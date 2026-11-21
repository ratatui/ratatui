use std::io::Write as _;

use color_eyre::Result;
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal};
use ratatui_termina::TerminaBackend;
use ratatui_termina::termina::escape::csi::{self, Csi};
use ratatui_termina::termina::event::{KeyCode, KeyEventKind};
use ratatui_termina::termina::{Event, EventReader, PlatformTerminal, Terminal as _};

type AppTerminal = Terminal<TerminaBackend<PlatformTerminal>>;

macro_rules! decset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::SetDecPrivateMode(mode))
    }};
}

macro_rules! decreset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::ResetDecPrivateMode(mode))
    }};
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let (mut terminal, events) = init_terminal()?;
    run(&mut terminal, &events)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn init_terminal() -> Result<(AppTerminal, EventReader)> {
    let mut output = PlatformTerminal::new()?;
    output.enter_raw_mode()?;

    let enter_alternate_screen = decset!(ClearAndEnableAlternateScreen);
    let show_cursor = decset!(ShowCursor);
    write!(output, "{enter_alternate_screen}{show_cursor}")?;
    output.flush()?;

    let events = output.event_reader();
    let backend = TerminaBackend::new(output);
    let terminal = Terminal::new(backend)?;
    Ok((terminal, events))
}

fn run(terminal: &mut AppTerminal, events: &EventReader) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if should_quit(&events.read(|event| matches!(event, Event::Key(_)))?) {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    let text = "Ratatui + Termina\n\nPress q or Esc to quit";
    frame.render_widget(Paragraph::new(text).cyan().centered(), frame.area());
}

fn restore_terminal(terminal: &mut AppTerminal) -> Result<()> {
    let leave_alternate_screen = decreset!(ClearAndEnableAlternateScreen);
    let show_cursor = decset!(ShowCursor);
    let backend = terminal.backend_mut();
    write!(backend, "{leave_alternate_screen}{show_cursor}")?;
    backend.flush()?;
    Ok(())
}

fn should_quit(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key)
            if key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Escape)
    )
}
