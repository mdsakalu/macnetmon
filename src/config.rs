use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::model::{DisplaySettings, SortMode};
use crate::theme::DEFAULT_THEME;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub theme: String,
    #[serde(flatten)]
    pub display: DisplaySettings,
    pub sort_mode: SortMode,
    pub interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: DEFAULT_THEME.to_string(),
            display: DisplaySettings::default(),
            sort_mode: SortMode::Name,
            interval_ms: 1000,
        }
    }
}

impl Config {
    fn path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let path = PathBuf::from(home).join(".config").join("macnetmon.json");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Some(path)
    }

    pub fn load() -> Self {
        if let Some(path) = Self::path() {
            if let Ok(file) = std::fs::File::open(path) {
                let reader = std::io::BufReader::new(file);
                if let Ok(cfg) = serde_json::from_reader(reader) {
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::path() {
            if let Ok(file) = std::fs::File::create(path) {
                let writer = std::io::BufWriter::new(file);
                let _ = serde_json::to_writer_pretty(writer, self);
            }
        }
    }
}
