use ratatui::style::Color;

pub const SOLID_THEMES: [(&str, Color); 7] = [
    ("Green", Color::Green),
    ("Yellow", Color::Yellow),
    ("Red", Color::Red),
    ("Blue", Color::Blue),
    ("Magenta", Color::Magenta),
    ("Cyan", Color::Cyan),
    ("White", Color::White),
];

pub const DEFAULT_THEME: &str = "Green";

#[derive(Clone, Copy, Debug)]
pub struct BorderColors {
    pub outer: Color,
    pub pane: Color,
    pub tile: Color,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub outer: Color,
    pub pane: Color,
    pub graph: Color,
    pub background: Color,
}

fn rgb(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::Rgb(r, g, b)
}

pub fn solid_tx_color(color: Color) -> Color {
    match color {
        Color::Red => rgb("#b22222"),
        Color::Green => rgb("#228b22"),
        Color::Yellow => rgb("#b8860b"),
        Color::Blue => rgb("#1e3a8a"),
        Color::Magenta => rgb("#8b1c62"),
        Color::Cyan => rgb("#0f766e"),
        Color::White => rgb("#a1a1aa"),
        _ => color,
    }
}

pub fn build_themes() -> Vec<Theme> {
    let mut themes = Vec::new();
    for (name, color) in SOLID_THEMES {
        themes.push(Theme {
            name,
            outer: color,
            pane: color,
            graph: color,
            background: Color::Reset,
        });
    }

    themes.extend([
        Theme {
            name: "Catppuccin Latte",
            outer: rgb("#7287fd"),
            pane: rgb("#ea76cb"),
            graph: rgb("#40a02b"),
            background: rgb("#eff1f5"),
        },
        Theme {
            name: "Catppuccin Frappe",
            outer: rgb("#8caaee"),
            pane: rgb("#f4b8e4"),
            graph: rgb("#a6d189"),
            background: rgb("#303446"),
        },
        Theme {
            name: "Catppuccin Macchiato",
            outer: rgb("#8aadf4"),
            pane: rgb("#f5bde6"),
            graph: rgb("#a6da95"),
            background: rgb("#24273a"),
        },
        Theme {
            name: "Catppuccin Mocha",
            outer: rgb("#89b4fa"),
            pane: rgb("#f5c2e7"),
            graph: rgb("#a6e3a1"),
            background: rgb("#1e1e2e"),
        },
        Theme {
            name: "Dracula",
            outer: rgb("#bd93f9"),
            pane: rgb("#ff79c6"),
            graph: rgb("#50fa7b"),
            background: rgb("#282a36"),
        },
        Theme {
            name: "Nord",
            outer: rgb("#88c0d0"),
            pane: rgb("#81a1c1"),
            graph: rgb("#a3be8c"),
            background: rgb("#2e3440"),
        },
        Theme {
            name: "Tokyo Night",
            outer: rgb("#7aa2f7"),
            pane: rgb("#bb9af7"),
            graph: rgb("#9ece6a"),
            background: rgb("#1a1b26"),
        },
        Theme {
            name: "Tokyo Storm",
            outer: rgb("#7aa2f7"),
            pane: rgb("#bb9af7"),
            graph: rgb("#9ece6a"),
            background: rgb("#24283b"),
        },
        Theme {
            name: "Tokyo Moon",
            outer: rgb("#82aaff"),
            pane: rgb("#c099ff"),
            graph: rgb("#c3e88d"),
            background: rgb("#222436"),
        },
        Theme {
            name: "Tokyo Day",
            outer: rgb("#2e7de9"),
            pane: rgb("#9854f1"),
            graph: rgb("#587539"),
            background: rgb("#e1e2e7"),
        },
    ]);

    themes
}
