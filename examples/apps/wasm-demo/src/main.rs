//! A minimal Ratatui application that renders a WASM plugin widget.
//!
//! Press `+`/`-` (or `Up`/`Down`) to adjust the host-side counter, `q`/`Esc`
//! to quit. The guest widget is reloaded on every frame, so editing and
//! recompiling `hello-rust` is visible immediately.

use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_wasm::WasmWidget;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let widget_path = find_widget()?;
    let mut counter: i64 = 0;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(1)])
                    .split(area);

                let widget_block = Block::default().title("WASM Widget").borders(Borders::ALL);
                let inner = widget_block.inner(chunks[0]);
                frame.render_widget(widget_block, chunks[0]);
                frame.render_widget(WasmWidget::from_file(&widget_path, &[]), inner);

                frame.render_widget(
                    Paragraph::new(format!("Counter: {counter}  |  +/- adjust  |  q quit")),
                    chunks[1],
                );
            })?;

            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Char('+') | KeyCode::Up => counter += 1,
                    KeyCode::Char('-') | KeyCode::Down => counter -= 1,
                    _ => {}
                }
            }
        }
    })
}

/// Locate the `hello-rust` `.wasm` artifact.
///
/// The path can be overridden with the `WASM_WIDGET` environment variable.
fn find_widget() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(path) = std::env::var("WASM_WIDGET") {
        return Ok(path.into());
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest_dir
        .join("../../wasm-widgets/hello-rust/target/wasm32-wasip2/release/hello_rust.wasm"))
}
