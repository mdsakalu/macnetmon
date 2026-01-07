use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortMode {
    Bandwidth,
    Name,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Physical,
    Virtual,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplaySettings {
    pub show_loopback: bool,
    pub show_virtual: bool,
    pub show_overview: bool,
    pub show_inactive: bool,
    pub show_bits: bool,
    pub show_split: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            show_loopback: true,
            show_virtual: true,
            show_overview: true,
            show_inactive: false,
            show_bits: false,
            show_split: true,
        }
    }
}
