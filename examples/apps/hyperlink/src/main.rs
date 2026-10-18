/// A Ratatui example that shows how to create hyperlinks in the terminal using [OSC 8].
///
/// Links are attached to the cells of the buffer after rendering the text, with
/// [`Buffer::set_hyperlink`] or [`Buffer::set_line_with_hyperlink`]. The backend writes the
/// OSC 8 escape sequences around the linked cells, so links never affect the layout. This
/// needs the `cell-extensions` feature of `ratatui`.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
/// [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
use color_eyre::Result;
use crossterm::event;
use ratatui::buffer::{Buffer, Hyperlink};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame.area(), frame.buffer_mut()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

fn render(area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .spacing(1);
    let [part_of_line, whole_line, wrapped] = area.layout(&layout);

    // Render a line, then link part of it.
    Line::from(vec!["Example ".into(), "hyperlink".blue().underlined()]).render(part_of_line, buf);
    let example_area = Rect {
        x: part_of_line.x + 8,
        width: 9,
        height: 1,
        ..part_of_line
    };
    let example = Hyperlink::new("https://example.com");
    buf.set_hyperlink(example_area, Some(&example));

    // Render and link a whole line in one call.
    let website = Hyperlink::new("https://ratatui.rs");
    let (x, y, width) = (whole_line.x, whole_line.y, whole_line.width);
    let text = Line::from("Ratatui website").blue().underlined();
    buf.set_line_with_hyperlink(x, y, &text, width, &website);

    // Link a wrapped paragraph. All its rows get the same link id, so terminals highlight them
    // as one link.
    Paragraph::new(
        "This paragraph wraps onto several lines when the terminal is narrow, \
         and all of it links to the Ratatui repository.",
    )
    .blue()
    .underlined()
    .wrap(Wrap { trim: true })
    .render(wrapped, buf);
    let repository = Hyperlink::new("https://github.com/ratatui/ratatui");
    buf.set_hyperlink(wrapped, Some(&repository));
}
