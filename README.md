<div align="center">

# macnetmon

Network interface bandwidth monitor for macOS

[<img src="https://img.shields.io/github/actions/workflow/status/mdsakalu/macnetmon/check.yml?label=build&logo=github" />](https://github.com/mdsakalu/macnetmon/actions)
[<img src="https://img.shields.io/github/v/release/mdsakalu/macnetmon?label=release&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgc3Ryb2tlPSJ3aGl0ZSIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI%2BCiAgPHBhdGggZD0iTTIgNyBMNyAyIEgxNCBWOSBMOSAxNCBaIi8%2BCiAgPGNpcmNsZSBjeD0iMTEiIGN5PSI1IiByPSIxIi8%2BCjwvc3ZnPg%3D%3D" />](https://github.com/mdsakalu/macnetmon/releases/latest)
[<img src="https://img.shields.io/crates/v/macnetmon?label=crates.io&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMzJweCIgaGVpZ2h0PSIzMnB4IiB2aWV3Qm94PSI4IDggMTYgMTYiIHZlcnNpb249IjEuMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayI%2BCiAgICA8cGF0aCBkPSJNMjAuMjQ2ODI0NSwxMi41NDgwOTQ3IEwxNiwxMCBMMTEuNzc1ODA3OSwxMi41MzQ1MTUyIEwxNi4wMjMwNzY0LDE0Ljk4NjY3NjggTDIwLjI0NjgyNDUsMTIuNTQ4MDk0NyBaIE0yMSwxNC40MjI2NDk3IEwxNywxNi43MzIwNTA4IEwxNywyMS40IEwyMSwxOSBMMjEsMTQuNDIyNjQ5NyBaIE0xMSwxNC4zOTYwMDM0IEwxMSwxOSBMMTUsMjEuNCBMMTUsMTYuNzA1NDA0NSBMMTEsMTQuMzk2MDAzNCBaIE0xNiw4IEwyMywxMiBMMjMsMjAgTDE2LDI0IEw5LDIwIEw5LDEyIEwxNiw4IFoiIGZpbGw9IiNGRkZGRkYiPjwvcGF0aD4KPC9zdmc%2BCg%3D%3D&logoWidth=20" />](https://crates.io/crates/macnetmon)
[<img src="https://img.shields.io/github/downloads/mdsakalu/macnetmon/total?label=downloads&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgc3Ryb2tlPSJ3aGl0ZSIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI%2BCiAgPHBhdGggZD0iTTggMiBWMTAiLz4KICA8cGF0aCBkPSJNNSA3IEw4IDEwIEwxMSA3Ii8%2BCiAgPHBhdGggZD0iTTMgMTMgSDEzIi8%2BCjwvc3ZnPg%3D%3D" />](https://github.com/mdsakalu/macnetmon/releases)
[<img src="https://img.shields.io/badge/Homebrew-mdsakalu/tap/macnetmon-orange?logo=homebrew" />](https://github.com/mdsakalu/homebrew-tap)
[<img src="https://img.shields.io/badge/%F0%9F%A6%80%20Rust-1.70+-orange" />](https://www.rust-lang.org)
[<img src="https://img.shields.io/badge/platform-macOS-lightgrey?logo=apple" />](https://www.apple.com/macos)
[<img src="https://img.shields.io/github/license/mdsakalu/macnetmon?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNCAxNiI%2BPHBhdGggZmlsbD0id2hpdGUiIGZpbGwtcnVsZT0iZXZlbm9kZCIgZD0iTTcgNGMtLjgzIDAtMS41LS42Ny0xLjUtMS41UzYuMTcgMSA3IDFzMS41LjY3IDEuNSAxLjVTNy44MyA0IDcgNHptNyA2YzAgMS4xMS0uODkgMi0yIDJoLTFjLTEuMTEgMC0yLS44OS0yLTJsMi00aC0xYy0uNTUgMC0xLS40NS0xLTFIOHY4Yy40MiAwIDEgLjQ1IDEgMWgxYy40MiAwIDEgLjQ1IDEgMUgzYzAtLjU1LjU4LTEgMS0xaDFjMC0uNTUuNTgtMSAxLTFoLjAzTDYgNUg1YzAgLjU1LS40NSAxLTEgMUgzbDIgNGMwIDEuMTEtLjg5IDItMiAySDJjLTEuMTEgMC0yLS44OS0yLTJsMi00SDFWNWgzYzAtLjU1LjQ1LTEgMS0xaDRjLjU1IDAgMSAuNDUgMSAxaDN2MWgtMWwyIDR6TTIuNSA3TDEgMTBoM0wyLjUgN3pNMTMgMTBsLTEuNS0zLTEuNSAzaDN6Ii8%2BPC9zdmc%2B" />](LICENSE)
[<img src="https://img.shields.io/badge/Built_With-Ratatui-blue?logo=ratatui" />](https://ratatui.rs/)

<table>
  <tr>
    <td><a href="assets/screenshot-split.png"><img src="assets/screenshot-split.png" alt="macnetmon split view" width="420" /></a></td>
    <td><a href="assets/screenshot-total.png"><img src="assets/screenshot-total.png" alt="macnetmon total view" width="420" /></a></td>
  </tr>
  <tr>
    <td align="center"><sub>Split RX/TX view</sub></td>
    <td align="center"><sub>Total view</sub></td>
  </tr>
</table>

</div>

## About

macnetmon is a TUI (Terminal User Interface) application that displays real-time network interface bandwidth usage on macOS. It monitors incoming (RX) and outgoing (TX) traffic rates for all network interfaces with colorful visualizations.

Inspired by [macmon](https://github.com/vladkens/macmon), which monitors Apple Silicon power consumption.

## Features

- Real-time network bandwidth monitoring
- Split sparkline visualization for RX/TX traffic
- Multiple color themes (7 solid colors + 10 advanced themes including Catppuccin, Dracula, Nord, Tokyo Night)
- Friendly interface names from macOS `networksetup`
- Toggle display of loopback, virtual, and inactive interfaces
- Sort by bandwidth or interface name
- Overview panel showing total system bandwidth
- Bits/s and Bytes/s display modes
- Persists settings (theme, toggles, interval) across runs
- 512-sample history depth

## Installation

### Homebrew

```sh
brew install mdsakalu/tap/macnetmon
```

### Cargo

```sh
cargo install macnetmon
```

### Build from Source

```sh
git clone https://github.com/mdsakalu/macnetmon.git
cd macnetmon
cargo build --release
./target/release/macnetmon
```

## Usage

```sh
macnetmon [OPTIONS]
```

### Options

| Option                | Description                          |
| --------------------- | ------------------------------------ |
| `-i, --interval <MS>` | Update interval in milliseconds      |
| `--hide-loopback`     | Hide loopback interfaces             |
| `--hide-virtual`      | Hide virtual interfaces              |
| `--show-inactive`     | Show inactive interfaces             |
| `--bits`              | Display in bits/s instead of bytes/s |
| `-h, --help`          | Print help                           |
| `-V, --version`       | Print version                        |

### Keyboard Controls

| Key | Action                        |
| --- | ----------------------------- |
| `q` | Quit                          |
| `t` | Cycle through themes          |
| `g` | Toggle graph (split/total)    |
| `b` | Toggle bits/bytes display     |
| `s` | Toggle sort (bandwidth/name)  |
| `a` | Toggle “All Interfaces” panel |
| `i` | Toggle inactive interfaces    |
| `v` | Toggle virtual interfaces     |
| `l` | Toggle loopback interfaces    |
| `r` | Refresh interface aliases     |
| `+` | Increase refresh interval     |
| `-` | Decrease refresh interval     |

## Configuration

macnetmon persists settings to:

```
~/.config/macnetmon.json
```

CLI flags override saved settings for that run, and the updated values are saved back to the config.

Default interval is 1000ms if no config exists.

## Requirements

- macOS (uses macOS-specific APIs via libc)
- Rust 1.70+ (for building from source)

## Contributing

Contributions are welcome! Whether you have ideas, suggestions, or bug reports, feel free to [open an issue](https://github.com/mdsakalu/macnetmon/issues) or submit a pull request.

## License

[MIT](LICENSE)

## See Also

- [macmon](https://github.com/vladkens/macmon) - Apple Silicon power monitor (inspiration for this project)
