#[cfg(all(not(windows), feature = "termion"))]
#[test]
fn backend_termion_should_only_write_diffs() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fmt::Write, io::Cursor};

    let mut bytes = Vec::new();
    let mut stdout = Cursor::new(&mut bytes);
    {
        use ratatui::{
            backend::TermionBackend, layout::Rect, widgets::Paragraph, Terminal, TerminalOptions,
            Viewport,
        };
        let backend = TermionBackend::new(&mut stdout);
        let area = Rect::new(0, 0, 3, 1);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(area),
            },
        )?;
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("a"), area);
        })?;
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("ab"), area);
        })?;
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("abc"), area);
        })?;
    }

    let expected = {
        use ratatui::termion::{color, cursor, style};
        let mut s = String::new();
        // First draw
        write!(s, "{}", cursor::Goto(1, 1))?;
        s.push('a');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Second draw
        write!(s, "{}", cursor::Goto(2, 1))?;
        s.push('b');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Third draw
        write!(s, "{}", cursor::Goto(3, 1))?;
        s.push('c');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Terminal drop
        write!(s, "{}", cursor::Show)?;
        s
    };
    assert_eq!(std::str::from_utf8(&bytes)?, expected);

    Ok(())
}

/// Retrieving the cursor from an in-memory buffer will fail in
/// `termion::cursor::DetectCursorPos::cursor_pos()`. If there is no known last cursor, retrieving
/// the cursor will ulimately fail.
///
/// This simulates a usage of `termion` where retrieving the cursor fails in a multi-threaded
/// environment: [](https://gitlab.redox-os.org/redox-os/termion/-/issues/173)
#[cfg(feature = "termion")]
#[test]
fn backend_termion_should_fail_on_missing_last_known_cursor(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Cursor};

    use ratatui::{backend::TermionBackend, Terminal};

    let mut bytes = Vec::new();
    let mut stdout = Cursor::new(&mut bytes);

    let backend = TermionBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let cursor = terminal.get_cursor_position();
    let error_kind = cursor.unwrap_err().kind();
    let expected_kind = io::ErrorKind::Other;

    assert_eq!(error_kind, expected_kind);

    Ok(())
}

/// Retrieving the cursor from an in-memory buffer will fail in
/// `termion::cursor::DetectCursorPos::cursor_pos()`. If a cursor was set explicitly, retrieving
/// the cursor will fallback to its last known position.
///
/// This simulates a usage of `termion` where retrieving the cursor fails in a multi-threaded
/// environment: [](https://gitlab.redox-os.org/redox-os/termion/-/issues/173)
#[cfg(feature = "termion")]
#[test]
fn backend_termion_should_use_last_known_cursor() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Cursor;

    use ratatui::{backend::TermionBackend, layout::Position, Terminal};

    let mut bytes = Vec::new();
    let mut stdout = Cursor::new(&mut bytes);

    let backend = TermionBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.set_cursor_position(Position::new(13, 12))?;

    assert_eq!(terminal.get_cursor_position()?, Position::new(13, 12));

    Ok(())
}
