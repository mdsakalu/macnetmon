use std::collections::{HashMap, HashSet};
use std::time::Instant;

use ratatui::style::{Color, Style};

use crate::args::Args;
use crate::config::Config;
use crate::constants::{HISTORY_LEN, INTERVAL_STEP_MS, MAX_INTERVAL_MS, MIN_INTERVAL_MS};
use crate::model::{DisplaySettings, Group, SortMode};
use crate::net::{
    get_hostname, is_physical_interface, is_up, load_interface_aliases, sample_interfaces,
    InterfaceSample,
};
use crate::theme::{build_themes, solid_tx_color, BorderColors, Theme, SOLID_THEMES};

#[derive(Clone, Debug)]
pub struct IfaceState {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_rate: f64,
    pub tx_rate: f64,
    pub total_rate: f64,
    pub flags: u32,
    pub is_loopback: bool,
    pub rx_history: Vec<u64>,
    pub tx_history: Vec<u64>,
    pub last_active_sample: u64,
    pub initialized: bool,
}

impl IfaceState {
    fn new(sample: &InterfaceSample) -> Self {
        Self {
            name: sample.name.clone(),
            rx_bytes: sample.rx_bytes,
            tx_bytes: sample.tx_bytes,
            rx_rate: 0.0,
            tx_rate: 0.0,
            total_rate: 0.0,
            flags: sample.flags,
            is_loopback: sample.is_loopback,
            rx_history: Vec::with_capacity(HISTORY_LEN),
            tx_history: Vec::with_capacity(HISTORY_LEN),
            last_active_sample: 0,
            initialized: false,
        }
    }
}

pub struct App {
    pub states: HashMap<String, IfaceState>,
    pub last_sample: Instant,
    pub sample_index: u64,
    pub hostname: String,
    pub display: DisplaySettings,
    pub theme_index: usize,
    pub themes: Vec<Theme>,
    pub sort_mode: SortMode,
    pub interval_ms: u64,
    pub last_error: Option<String>,
    pub name_error: Option<String>,
    pub aliases: HashMap<String, String>,
    pub visible_physical: HashSet<String>,
    pub visible_virtual: HashSet<String>,
    pub total_rx: f64,
    pub total_tx: f64,
    pub total_rx_history: Vec<u64>,
    pub total_tx_history: Vec<u64>,
}

impl App {
    pub fn new(args: &Args) -> Self {
        let cfg = Config::load();
        let mut app = Self {
            states: HashMap::new(),
            last_sample: Instant::now(),
            sample_index: 0,
            hostname: get_hostname().unwrap_or_else(|| "unknown".to_string()),
            display: cfg.display,
            theme_index: 0,
            themes: build_themes(),
            sort_mode: cfg.sort_mode,
            interval_ms: cfg.interval_ms,
            last_error: None,
            name_error: None,
            aliases: HashMap::new(),
            visible_physical: HashSet::new(),
            visible_virtual: HashSet::new(),
            total_rx: 0.0,
            total_tx: 0.0,
            total_rx_history: Vec::with_capacity(HISTORY_LEN),
            total_tx_history: Vec::with_capacity(HISTORY_LEN),
        };
        if let Some(idx) = app.themes.iter().position(|t| t.name == cfg.theme) {
            app.theme_index = idx;
        }
        if args.hide_loopback {
            app.display.show_loopback = false;
        }
        if args.hide_virtual {
            app.display.show_virtual = false;
        }
        if args.show_inactive {
            app.display.show_inactive = true;
        }
        if args.bits {
            app.display.show_bits = true;
        }
        if let Some(interval) = args.interval {
            app.interval_ms = interval;
        }
        app.interval_ms = app.interval_ms.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS);
        app.save_config();
        app.refresh_aliases();
        app
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.last_sample)
            .as_secs_f64()
            .max(0.001);
        self.last_sample = now;
        self.sample_index = self.sample_index.saturating_add(1);

        match sample_interfaces() {
            Ok(samples) => {
                self.last_error = None;
                let mut seen = HashSet::new();

                for sample in samples {
                    if !is_up(sample.flags) {
                        continue;
                    }

                    seen.insert(sample.name.clone());

                    let entry = self
                        .states
                        .entry(sample.name.clone())
                        .or_insert_with(|| IfaceState::new(&sample));

                    if entry.initialized {
                        let rx_delta = sample.rx_bytes.saturating_sub(entry.rx_bytes);
                        let tx_delta = sample.tx_bytes.saturating_sub(entry.tx_bytes);
                        entry.rx_rate = rx_delta as f64 / dt;
                        entry.tx_rate = tx_delta as f64 / dt;
                        entry.total_rate = entry.rx_rate + entry.tx_rate;
                    } else {
                        entry.rx_rate = 0.0;
                        entry.tx_rate = 0.0;
                        entry.total_rate = 0.0;
                    }

                    entry.rx_bytes = sample.rx_bytes;
                    entry.tx_bytes = sample.tx_bytes;
                    entry.flags = sample.flags;
                    entry.is_loopback = sample.is_loopback;
                    entry.initialized = true;

                    if entry.total_rate >= 1.0 {
                        entry.last_active_sample = self.sample_index;
                        if is_physical_interface(&entry.name) {
                            self.visible_physical.insert(entry.name.clone());
                        } else {
                            self.visible_virtual.insert(entry.name.clone());
                        }
                    }

                    entry
                        .rx_history
                        .insert(0, entry.rx_rate.round().max(0.0) as u64);
                    entry.rx_history.truncate(HISTORY_LEN);
                    entry
                        .tx_history
                        .insert(0, entry.tx_rate.round().max(0.0) as u64);
                    entry.tx_history.truncate(HISTORY_LEN);
                }

                self.states.retain(|name, _| seen.contains(name));
                self.visible_physical
                    .retain(|name| self.states.contains_key(name));
                self.visible_virtual
                    .retain(|name| self.states.contains_key(name));

                let mut total_rx = 0.0;
                let mut total_tx = 0.0;
                for iface in self.states.values() {
                    if !self.display.show_loopback && iface.is_loopback {
                        continue;
                    }
                    total_rx += iface.rx_rate;
                    total_tx += iface.tx_rate;
                }

                self.total_rx = total_rx;
                self.total_tx = total_tx;
                self.total_rx_history
                    .insert(0, total_rx.round().max(0.0) as u64);
                self.total_rx_history.truncate(HISTORY_LEN);
                self.total_tx_history
                    .insert(0, total_tx.round().max(0.0) as u64);
                self.total_tx_history.truncate(HISTORY_LEN);
            }
            Err(err) => {
                self.last_error = Some(err.to_string());
            }
        }
    }

    pub fn theme(&self) -> &Theme {
        &self.themes[self.theme_index % self.themes.len()]
    }

    pub fn theme_slot(&self) -> usize {
        self.theme_index % self.themes.len()
    }

    pub fn rx_color(&self) -> Color {
        self.theme().graph
    }

    pub fn tx_color(&self) -> Color {
        if self.is_advanced_theme() {
            self.theme().outer
        } else {
            solid_tx_color(self.theme().graph)
        }
    }

    pub fn save_config(&self) {
        let cfg = Config {
            theme: self.theme().name.to_string(),
            display: self.display,
            sort_mode: self.sort_mode,
            interval_ms: self.interval_ms,
        };
        cfg.save();
    }

    pub fn is_advanced_theme(&self) -> bool {
        self.theme_slot() >= SOLID_THEMES.len()
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % self.themes.len();
        self.save_config();
    }

    pub fn colors(&self) -> BorderColors {
        let t = self.theme();
        BorderColors {
            outer: t.outer,
            pane: t.pane,
            tile: t.graph,
        }
    }

    pub fn block_style(&self) -> Style {
        if self.is_advanced_theme() {
            Style::default().bg(self.theme().background)
        } else {
            Style::default()
        }
    }

    pub fn rx_style(&self) -> Style {
        let mut style = Style::default().fg(self.rx_color());
        if self.is_advanced_theme() {
            style = style.bg(self.theme().background);
        }
        style
    }

    pub fn tx_style(&self) -> Style {
        let mut style = Style::default().fg(self.tx_color());
        if self.is_advanced_theme() {
            style = style.bg(self.theme().background);
        }
        style
    }

    pub fn toggle_sort(&mut self) {
        self.sort_mode = match self.sort_mode {
            SortMode::Bandwidth => SortMode::Name,
            SortMode::Name => SortMode::Bandwidth,
        };
        self.save_config();
    }

    pub fn inc_interval(&mut self) {
        let next = self.interval_ms.saturating_add(INTERVAL_STEP_MS);
        self.interval_ms = next.min(MAX_INTERVAL_MS);
        self.save_config();
    }

    pub fn dec_interval(&mut self) {
        if self.interval_ms <= INTERVAL_STEP_MS {
            self.interval_ms = MIN_INTERVAL_MS;
            self.save_config();
            return;
        }
        let next = self.interval_ms.saturating_sub(INTERVAL_STEP_MS);
        self.interval_ms = next.max(MIN_INTERVAL_MS);
        self.save_config();
    }

    pub fn in_group(&self, iface: &IfaceState, group: Group) -> bool {
        match group {
            Group::Physical if !is_physical_interface(&iface.name) => return false,
            Group::Virtual if is_physical_interface(&iface.name) => return false,
            _ => {}
        }

        true
    }

    pub fn refresh_aliases(&mut self) {
        match load_interface_aliases() {
            Ok(map) => {
                self.aliases = map;
                self.name_error = None;
            }
            Err(err) => {
                self.name_error = Some(err.to_string());
            }
        }
    }
}
