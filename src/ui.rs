use std::collections::HashSet;

use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::bar;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, RenderDirection, Sparkline};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::app::{App, IfaceState};
use crate::constants::MIN_TILE_WIDTH;
use crate::model::{Group, SortMode};

fn format_rate(bytes_per_sec: f64, bits: bool) -> String {
    let step = if bits { 1000.0 } else { 1024.0 };
    let units = if bits {
        ["b/s", "Kb/s", "Mb/s", "Gb/s", "Tb/s"]
    } else {
        ["B/s", "KB/s", "MB/s", "GB/s", "TB/s"]
    };

    let mut value = if bits {
        bytes_per_sec * 8.0
    } else {
        bytes_per_sec
    };
    let mut idx = 0usize;

    while value >= step && idx < units.len() - 1 {
        value /= step;
        idx += 1;
    }

    if value >= 100.0 {
        format!("{:>6.0} {}", value, units[idx])
    } else if value >= 10.0 {
        format!("{:>6.1} {}", value, units[idx])
    } else {
        format!("{:>6.2} {}", value, units[idx])
    }
}

fn sparkline_data(history: &[u64], width: u16) -> Vec<u64> {
    let width = width as usize;
    if width == 0 {
        return Vec::new();
    }

    if history.is_empty() {
        return vec![0; width];
    }

    let mut data = history.iter().take(width).copied().collect::<Vec<u64>>();
    if data.len() < width {
        let pad = *data.last().unwrap_or(&0);
        data.resize(width, pad);
    }
    data
}

fn bar_symbol(level: u64) -> &'static str {
    match level {
        0 => bar::NINE_LEVELS.empty,
        1 => bar::NINE_LEVELS.one_eighth,
        2 => bar::NINE_LEVELS.one_quarter,
        3 => bar::NINE_LEVELS.three_eighths,
        4 => bar::NINE_LEVELS.half,
        5 => bar::NINE_LEVELS.five_eighths,
        6 => bar::NINE_LEVELS.three_quarters,
        7 => bar::NINE_LEVELS.seven_eighths,
        _ => bar::NINE_LEVELS.full,
    }
}

fn scale_units(value: u64, max: u64, total_units: u64) -> u64 {
    if total_units == 0 || max == 0 {
        return 0;
    }
    let scaled = ((value as f64 / max as f64) * total_units as f64).ceil() as u64;
    if value > 0 && scaled == 0 {
        1
    } else {
        scaled.min(total_units)
    }
}

fn row_level(units: u64, row: u64) -> u64 {
    let base = row * 8;
    if units >= base + 8 {
        8
    } else {
        units.saturating_sub(base)
    }
}

#[allow(clippy::too_many_arguments)]
fn render_split_sparkline(
    f: &mut Frame<'_>,
    area: ratatui::layout::Rect,
    block: Block<'_>,
    rx: &[u64],
    tx: &[u64],
    rx_style: Style,
    tx_style: Style,
    base_style: Style,
) {
    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.is_empty() {
        return;
    }

    let buf = f.buffer_mut();
    buf.set_style(inner, base_style);

    let rx_data = sparkline_data(rx, inner.width);
    let tx_data = sparkline_data(tx, inner.width);

    let mut max_rx = rx_data.iter().copied().max().unwrap_or(0);
    let mut max_tx = tx_data.iter().copied().max().unwrap_or(0);
    if max_rx == 0 {
        max_rx = 1;
    }
    if max_tx == 0 {
        max_tx = 1;
    }

    let height = inner.height;
    if height == 0 {
        return;
    }

    let up_rows = height / 2;
    let down_rows = height.saturating_sub(up_rows);
    let baseline_y = inner.top() + up_rows;
    let up_units = up_rows as u64 * 8;
    let down_units = down_rows as u64 * 8;
    let tx_invert_style = tx_style.add_modifier(Modifier::REVERSED);

    for i in 0..inner.width as usize {
        let x = inner.right().saturating_sub(1 + i as u16);
        let rx_val = rx_data.get(i).copied().unwrap_or(0);
        let tx_val = tx_data.get(i).copied().unwrap_or(0);

        let rx_units = scale_units(rx_val, max_rx, up_units);
        let tx_units = scale_units(tx_val, max_tx, down_units);

        for row in 0..up_rows {
            let level = row_level(rx_units, row as u64);
            if level == 0 {
                continue;
            }
            let y = baseline_y.saturating_sub(1 + row);
            if y < inner.top() {
                break;
            }
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_symbol(bar_symbol(level)).set_style(rx_style);
            }
        }

        for row in 0..down_rows {
            let level = row_level(tx_units, row as u64);
            if level == 0 {
                continue;
            }
            let y = baseline_y.saturating_add(row);
            if y >= inner.bottom() {
                break;
            }
            let inverted = 8 - level;
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_symbol(bar_symbol(inverted))
                    .set_style(tx_invert_style);
            }
        }
    }
}

fn key_style(app: &App) -> Style {
    let mut style = Style::default().fg(app.theme().outer);
    if app.is_advanced_theme() {
        style = style.bg(app.theme().background);
    }
    style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

fn append_sep(spans: &mut Vec<Span<'static>>) {
    spans.push(Span::raw("  "));
}

fn cmd_bold_prefix(app: &App, label: &str) -> Vec<Span<'static>> {
    let mut chars = label.chars();
    let first = chars.next();
    let rest: String = chars.collect();
    match first {
        Some(ch) => vec![
            Span::styled(ch.to_string(), key_style(app)),
            Span::raw(rest),
        ],
        None => Vec::new(),
    }
}

fn title_line(label: &str) -> Line<'static> {
    Line::from(format!(" {label} "))
}

fn bold_title_style(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

fn title_line_bold(prefix: &str, rest: &str, color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    spans.push(Span::styled(format!(" {prefix}"), bold_title_style(color)));
    if !rest.is_empty() {
        spans.push(Span::raw(rest.to_string()));
    }
    spans.push(Span::raw(" "));
    Line::from(spans)
}

fn bordered_block<'a>(
    color: Color,
    border: BorderType,
    label_l: Option<Line<'a>>,
    label_r: Option<Line<'a>>,
    style: Style,
) -> Block<'a> {
    let mut block = Block::new()
        .borders(Borders::ALL)
        .border_type(border)
        .border_style(color)
        .style(style);

    if let Some(label) = label_l {
        block = block.title_top(label);
    }

    if let Some(label) = label_r {
        block = block.title_top(label);
    }

    block
}

fn render_overview(f: &mut Frame<'_>, area: ratatui::layout::Rect, app: &App) {
    let total = app.total_rx + app.total_tx;
    let colors = app.colors();
    let details = format!(
        "RX {}  TX {}",
        format_rate(app.total_rx, app.display.show_bits),
        format_rate(app.total_tx, app.display.show_bits)
    );
    let label_l = title_line_bold("All Interfaces", &format!(" {details}"), colors.pane);
    let label_r = Some(
        title_line(&format!(
            "Total {}",
            format_rate(total, app.display.show_bits)
        ))
        .alignment(Alignment::Right),
    );

    let block = bordered_block(
        colors.pane,
        BorderType::Thick,
        Some(label_l),
        label_r,
        app.block_style(),
    );

    if app.display.show_split {
        render_split_sparkline(
            f,
            area,
            block,
            &app.total_rx_history,
            &app.total_tx_history,
            app.rx_style(),
            app.tx_style(),
            app.block_style(),
        );
    } else {
        let inner = block.inner(area);
        let data = sparkline_data(&app.total_rx_history, inner.width)
            .into_iter()
            .zip(sparkline_data(&app.total_tx_history, inner.width))
            .map(|(rx, tx)| rx + tx)
            .collect::<Vec<u64>>();
        let spark = Sparkline::default()
            .block(block)
            .direction(RenderDirection::RightToLeft)
            .data(&data)
            .style(app.rx_style());
        f.render_widget(spark, area);
    }
}

fn render_interface(f: &mut Frame<'_>, area: ratatui::layout::Rect, app: &App, iface: &IfaceState) {
    let bsd_name = iface.name.as_str();
    let friendly_name = app.aliases.get(bsd_name).map(|alias| {
        if alias == bsd_name || alias.contains(&format!("({})", bsd_name)) {
            alias.to_string()
        } else {
            format!("{alias} ({bsd_name})")
        }
    });

    let label_r = format_rate(iface.total_rate, app.display.show_bits);
    let right_width = UnicodeWidthStr::width(label_r.as_str()) + 2;
    let available = area.width.saturating_sub(2) as usize;
    let max_left = available.saturating_sub(right_width + 1);

    let label_l_full = friendly_name.as_ref().map(|name| {
        format!(
            "{}  RX {}  TX {}",
            name,
            format_rate(iface.rx_rate, app.display.show_bits),
            format_rate(iface.tx_rate, app.display.show_bits)
        )
    });
    let use_full = if let Some(full) = label_l_full.as_ref() {
        let left_width = UnicodeWidthStr::width(full.as_str()) + 2;
        left_width <= max_left
    } else {
        false
    };
    let name_display = if use_full {
        friendly_name.unwrap_or_else(|| bsd_name.to_string())
    } else {
        bsd_name.to_string()
    };

    let colors = app.colors();
    let label_l_line = {
        let rest = format!(
            "  RX {}  TX {}",
            format_rate(iface.rx_rate, app.display.show_bits),
            format_rate(iface.tx_rate, app.display.show_bits)
        );
        title_line_bold(&name_display, &rest, colors.tile)
    };
    let label_r_line = Some(title_line(&label_r).alignment(Alignment::Right));
    let block = bordered_block(
        colors.tile,
        BorderType::Plain,
        Some(label_l_line),
        label_r_line,
        app.block_style(),
    );
    if app.display.show_split {
        render_split_sparkline(
            f,
            area,
            block,
            &iface.rx_history,
            &iface.tx_history,
            app.rx_style(),
            app.tx_style(),
            app.block_style(),
        );
    } else {
        let inner = block.inner(area);
        let data = sparkline_data(&iface.rx_history, inner.width)
            .into_iter()
            .zip(sparkline_data(&iface.tx_history, inner.width))
            .map(|(rx, tx)| rx + tx)
            .collect::<Vec<u64>>();
        let spark = Sparkline::default()
            .block(block)
            .direction(RenderDirection::RightToLeft)
            .data(&data)
            .style(app.rx_style());
        f.render_widget(spark, area);
    }
}

fn render_group(
    f: &mut Frame<'_>,
    area: ratatui::layout::Rect,
    app: &mut App,
    group: Group,
    title: &str,
) {
    let candidates_all: Vec<&IfaceState> = app
        .states
        .values()
        .filter(|s| app.in_group(s, group))
        .collect();

    let filter_loopback = group == Group::Virtual && !app.display.show_loopback;

    let total_in_group = if filter_loopback {
        candidates_all.iter().filter(|s| !s.is_loopback).count()
    } else {
        candidates_all.len()
    };

    let visible_names: Vec<String> = if app.display.show_inactive {
        candidates_all
            .iter()
            .filter(|s| !filter_loopback || !s.is_loopback)
            .map(|s| s.name.clone())
            .collect()
    } else {
        let visible_set = match group {
            Group::Physical => &mut app.visible_physical,
            Group::Virtual => &mut app.visible_virtual,
        };

        let candidate_set: HashSet<&str> = candidates_all.iter().map(|s| s.name.as_str()).collect();
        visible_set.retain(|name| candidate_set.contains(name.as_str()));

        if !visible_set.is_empty() {
            let inner_width = area.width.saturating_sub(2);
            let max_cols = (inner_width / MIN_TILE_WIDTH).max(1) as usize;
            let display_count = visible_set
                .iter()
                .filter(|name| {
                    if let Some(iface) = app.states.get(*name) {
                        app.display.show_loopback || !iface.is_loopback
                    } else {
                        false
                    }
                })
                .count()
                .max(1);
            let cols = display_count.min(max_cols).max(1);
            let tile_width = inner_width / cols as u16;
            let window_samples = tile_width.saturating_sub(2).max(1) as u64;

            visible_set.retain(|name| {
                if let Some(iface) = app.states.get(name) {
                    app.sample_index.saturating_sub(iface.last_active_sample) < window_samples
                } else {
                    false
                }
            });
        }

        visible_set
            .iter()
            .filter(|name| {
                if let Some(iface) = app.states.get(*name) {
                    app.display.show_loopback || !iface.is_loopback
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    };

    let mut visible: Vec<&IfaceState> = visible_names
        .iter()
        .filter_map(|name| app.states.get(name))
        .collect();

    match app.sort_mode {
        SortMode::Bandwidth => visible.sort_by(|a, b| {
            b.total_rate
                .partial_cmp(&a.total_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.name.cmp(&b.name))
        }),
        SortMode::Name => visible.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    let mut group_rx = 0.0;
    let mut group_tx = 0.0;
    for iface in &visible {
        group_rx += iface.rx_rate;
        group_tx += iface.tx_rate;
    }
    let label_r = format!(
        "Total {}",
        format_rate(group_rx + group_tx, app.display.show_bits)
    );

    let colors = app.colors();
    let label_l_line = title_line_bold(
        title,
        &format!(" ({}/{})", visible.len(), total_in_group),
        colors.pane,
    );
    let label_r_line = Some(title_line(&label_r).alignment(Alignment::Right));
    let block = bordered_block(
        colors.pane,
        BorderType::Thick,
        Some(label_l_line),
        label_r_line,
        app.block_style(),
    );

    let list_area = block.inner(area);
    f.render_widget(block, area);

    if visible.is_empty() {
        let empty = Paragraph::new("No active interfaces")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(empty, list_area);
        return;
    }

    let max_cols = (list_area.width / MIN_TILE_WIDTH).max(1) as usize;
    let cols = visible.len().min(max_cols).max(1);
    let rows = (visible.len() + cols - 1) / cols;
    if rows == 0 {
        return;
    }

    let base_height = list_area.height / rows as u16;
    if base_height == 0 {
        return;
    }

    let extra = list_area.height % rows as u16;
    let mut row_constraints = Vec::with_capacity(rows);
    for idx in 0..rows {
        let mut height = base_height;
        if (idx as u16) < extra {
            height = height.saturating_add(1);
        }
        row_constraints.push(Constraint::Length(height));
    }

    let row_rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(list_area);

    let mut col_constraints = Vec::with_capacity(cols);
    let mut remaining_w = list_area.width;
    let per_col = list_area.width / cols as u16;
    for idx in 0..cols {
        let width = if idx + 1 == cols {
            remaining_w
        } else {
            per_col
        };
        col_constraints.push(Constraint::Length(width));
        remaining_w = remaining_w.saturating_sub(width);
    }

    let mut idx = 0usize;
    for row_area in row_rects.iter().take(rows) {
        let col_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints.clone())
            .split(*row_area);

        for col_area in col_rects.iter().take(cols) {
            if idx >= visible.len() {
                break;
            }
            render_interface(f, *col_area, app, visible[idx]);
            idx += 1;
        }
    }
}

pub fn ui(f: &mut Frame<'_>, app: &mut App) {
    let mut footer: Vec<Span<'static>> = Vec::new();
    footer.push(Span::raw(" "));
    footer.extend(cmd_bold_prefix(app, "quit"));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        &format!("theme: {}", app.theme().name),
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        if app.display.show_split {
            "graph: split"
        } else {
            "graph: total"
        },
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        if app.display.show_bits {
            "bits: b/s"
        } else {
            "bytes: B/s"
        },
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        match app.sort_mode {
            SortMode::Bandwidth => "sort: name",
            SortMode::Name => "sort: rate",
        },
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        &format!(
            "all interfaces {}",
            if app.display.show_overview {
                "●"
            } else {
                "○"
            }
        ),
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        &format!(
            "inactive {}",
            if app.display.show_inactive {
                "●"
            } else {
                "○"
            }
        ),
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        &format!(
            "virtual {}",
            if app.display.show_virtual {
                "●"
            } else {
                "○"
            }
        ),
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(
        app,
        &format!(
            "loopback {}",
            if app.display.show_loopback {
                "●"
            } else {
                "○"
            }
        ),
    ));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(app, "refresh names"));
    append_sep(&mut footer);
    footer.extend(cmd_bold_prefix(app, &format!("+/- {}ms", app.interval_ms)));

    if let Some(err) = &app.last_error {
        append_sep(&mut footer);
        footer.push(Span::raw(format!("error: {err}")));
    }
    if let Some(err) = &app.name_error {
        append_sep(&mut footer);
        footer.push(Span::raw(format!("names: {err}")));
    }
    footer.push(Span::raw(" "));

    let colors = app.colors();
    let outer = bordered_block(
        colors.outer,
        BorderType::Plain,
        Some(title_line_bold(&app.hostname, "", colors.outer)),
        Some(
            title_line(&format!(
                "{} v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .alignment(Alignment::Right),
        ),
        app.block_style(),
    )
    .title_bottom(Line::from(footer).alignment(Alignment::Right));
    let inner = outer.inner(f.area());
    f.render_widget(outer, f.area());

    let mut section_count = 1; // physical is always shown
    if app.display.show_overview {
        section_count += 1;
    }
    if app.display.show_virtual {
        section_count += 1;
    }

    let mut constraints = Vec::with_capacity(section_count);
    for _ in 0..section_count {
        constraints.push(Constraint::Ratio(1, section_count as u32));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    let mut idx = 0usize;
    if app.display.show_overview {
        render_overview(f, chunks[idx], app);
        idx += 1;
    }

    render_group(f, chunks[idx], app, Group::Physical, "Physical Interfaces");

    if app.display.show_virtual {
        render_group(
            f,
            chunks[idx + 1],
            app,
            Group::Virtual,
            "Virtual / Loopback",
        );
    }
}
