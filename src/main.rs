mod app;
mod args;
mod config;
mod constants;
mod model;
mod net;
mod theme;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::app::App;
use crate::args::Args;
use crate::ui::ui;

fn run_app(terminal: &mut DefaultTerminal, args: Args) -> io::Result<()> {
    let mut app = App::new(&args);

    app.update();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let tick_rate = Duration::from_millis(app.interval_ms);
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('b') => {
                            app.display.show_bits = !app.display.show_bits;
                            app.save_config();
                        }
                        KeyCode::Char('l') => {
                            app.display.show_loopback = !app.display.show_loopback;
                            app.save_config();
                        }
                        KeyCode::Char('t') => app.next_theme(),
                        KeyCode::Char('i') => {
                            app.display.show_inactive = !app.display.show_inactive;
                            app.save_config();
                        }
                        KeyCode::Char('v') => {
                            app.display.show_virtual = !app.display.show_virtual;
                            app.save_config();
                        }
                        KeyCode::Char('a') => {
                            app.display.show_overview = !app.display.show_overview;
                            app.save_config();
                        }
                        KeyCode::Char('s') => app.toggle_sort(),
                        KeyCode::Char('g') => {
                            app.display.show_split = !app.display.show_split;
                            app.save_config();
                        }
                        KeyCode::Char('r') => app.refresh_aliases(),
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.inc_interval();
                            last_tick = Instant::now();
                        }
                        KeyCode::Char('-') => {
                            app.dec_interval();
                            last_tick = Instant::now();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal, args);
    ratatui::restore();
    if let Err(err) = result {
        eprintln!("error: {}", err);
    }
    Ok(())
}
