//! Nested routing and focus-scope proof app.
//!
//! The app stays immediate-mode. Rendering produces frame-local route, focus, mouse, and cursor
//! data; the next input event uses those previous data to decide whether a leaf, local scope, or
//! page-level handler sees the event.

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use layout_routing_lab::App;
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

/// Runs the terminal event loop.
fn run() -> Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !app.handle_key(key.code) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    })
}
