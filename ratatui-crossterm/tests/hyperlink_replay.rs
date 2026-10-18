//! Screen-replay tests for OSC 8 hyperlinks.
//!
//! These tests draw buffers with [`CrosstermBackend`] the same way `Terminal` does (diff the
//! previous and next buffer, then draw the changed cells), replay the bytes in the [`vt100`]
//! terminal emulator, and check what a terminal would *show*: the screen text, and which cells
//! are linked. That catches cursor and width bugs that tests of the emitted cells miss.
//!
//! vt100 doesn't track OSC 8 itself, so [`Screen`] feeds it one token at a time and records
//! the link that is open when each character is printed.
#![cfg(feature = "cell-extensions")]

use pretty_assertions::assert_eq;
use ratatui_core::backend::Backend;
use ratatui_core::buffer::{Buffer, CellWidth, Hyperlink};
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style, Stylize};
use ratatui_core::text::Line;
use ratatui_crossterm::CrosstermBackend;

/// A terminal emulator that also tracks OSC 8 hyperlinks.
struct Screen {
    parser: vt100::Parser,
    /// The `(params, uri)` of the link open at each cell, indexed by `[row][col]`.
    links: Vec<Vec<Option<(String, String)>>>,
    /// The currently open link.
    open: Option<(String, String)>,
    /// The number of OSC 8 sequences that opened a link.
    opens: usize,
    /// The previous frame, for diffing.
    prev: Buffer,
}

impl Screen {
    fn new(width: u16, height: u16) -> Self {
        Self {
            parser: vt100::Parser::new(height, width, 0),
            links: vec![vec![None; usize::from(width)]; usize::from(height)],
            open: None,
            opens: 0,
            prev: Buffer::empty(Rect::new(0, 0, width, height)),
        }
    }

    /// Draws `next` like `Terminal::flush` does and replays the output. Returns the bytes.
    fn draw(&mut self, next: &Buffer) -> String {
        let mut out = Vec::new();
        CrosstermBackend::new(&mut out)
            .draw(self.prev.diff_iter(next))
            .unwrap();
        self.prev = next.clone();
        let out = String::from_utf8(out).unwrap();
        self.replay(&out);
        assert_eq!(
            self.open, None,
            "the link must be closed at the end of draw"
        );
        out
    }

    fn replay(&mut self, mut input: &str) {
        while !input.is_empty() {
            let len = token_len(input);
            let (token, rest) = input.split_at(len);
            input = rest;
            if let Some(osc8) = token.strip_prefix("\x1b]8;") {
                let osc8 = osc8
                    .strip_suffix("\x1b\\")
                    .or_else(|| osc8.strip_suffix('\x07'))
                    .unwrap();
                let (params, uri) = osc8.split_once(';').unwrap();
                self.open = (!uri.is_empty()).then(|| (params.to_string(), uri.to_string()));
                self.opens += usize::from(self.open.is_some());
            } else if token.starts_with('\x1b') || token.cell_width() == 0 {
                self.parser.process(token.as_bytes());
            } else {
                // A printed character: the cell under the cursor (and the one after it, for a
                // wide character) now has the current link, or none.
                let (row, col) = self.parser.screen().cursor_position();
                self.parser.process(token.as_bytes());
                let width = usize::from(token.cell_width());
                let row = &mut self.links[usize::from(row)];
                for cell in row.iter_mut().skip(usize::from(col)).take(width) {
                    cell.clone_from(&self.open);
                }
            }
        }
    }

    fn rows(&self) -> Vec<String> {
        let width = self.links[0].len() as u16;
        self.parser.screen().rows(0, width).collect()
    }

    /// Returns `(row, first col, last col, uri, id)` for each run of identically linked cells,
    /// skipping the trailing halves of wide characters.
    fn link_runs(&self) -> Vec<(usize, usize, usize, String, String)> {
        let mut runs: Vec<(usize, usize, usize, String, String)> = vec![];
        for (y, row) in self.links.iter().enumerate() {
            for (x, link) in row.iter().enumerate() {
                let Some((params, uri)) = link else { continue };
                let id = params.strip_prefix("id=").unwrap_or(params).to_string();
                match runs.last_mut() {
                    Some(run) if run.0 == y && run.2 + 1 == x && run.3 == *uri && run.4 == id => {
                        run.2 = x;
                    }
                    _ => runs.push((y, x, x, uri.clone(), id)),
                }
            }
        }
        runs
    }
}

/// Returns the length of the first token: an escape sequence or one character.
fn token_len(input: &str) -> usize {
    let bytes = input.as_bytes();
    if bytes[0] != 0x1b || bytes.len() == 1 {
        return input.chars().next().unwrap().len_utf8();
    }
    match bytes[1] {
        // CSI: parameters, then a final byte in 0x40..=0x7e
        b'[' => {
            bytes[2..]
                .iter()
                .position(|b| (0x40..=0x7e).contains(b))
                .unwrap()
                + 3
        }
        // OSC: terminated by BEL or ST (ESC \)
        b']' => {
            let end = (2..bytes.len())
                .find(|&i| {
                    bytes[i] == 0x07 || (bytes[i] == 0x1b && bytes.get(i + 1) == Some(&b'\\'))
                })
                .unwrap();
            if bytes[end] == 0x07 { end + 1 } else { end + 2 }
        }
        _ => 2,
    }
}

fn line_buffer(width: u16, lines: &[&str]) -> Buffer {
    let mut buf = Buffer::empty(Rect::new(0, 0, width, lines.len() as u16));
    for (y, line) in lines.iter().enumerate() {
        buf.set_string(0, y as u16, line, Style::new());
    }
    buf
}

fn trimmed(rows: Vec<String>) -> Vec<String> {
    rows.into_iter()
        .map(|row| row.trim_end().to_string())
        .collect()
}

/// The upstream hyperlink example's text with a link over "hyperlink". Before cell extensions,
/// the example's two-graphemes-per-cell workaround printed `Exxammplhyypeerllinnk`.
#[test]
fn link_over_part_of_a_line() {
    let mut screen = Screen::new(30, 1);
    let mut buf = Buffer::empty(Rect::new(0, 0, 30, 1));
    let link = Hyperlink::new("https://example.com");
    let text = Line::from("Example hyperlink").blue().underlined();
    buf.set_line(0, 0, &text, 30);
    buf.set_hyperlink(Rect::new(8, 0, 9, 1), Some(&link));
    screen.draw(&buf);

    assert_eq!(trimmed(screen.rows()), ["Example hyperlink"]);
    let id = buf[(8, 0)].hyperlink().unwrap().id().unwrap().to_string();
    assert_eq!(
        screen.link_runs(),
        [(0, 8, 16, "https://example.com".into(), id)]
    );
    assert_eq!(screen.opens, 1, "one OSC 8 per contiguous run");
}

/// Text drawn over a link is drawn and is not linked. Before cell extensions, the `ForcedWidth`
/// workaround skipped the cells under the link, so the `X` was never drawn.
#[test]
fn overlay_over_a_link() {
    let link = Hyperlink::new("https://example.com").with_id("ex");
    let mut screen = Screen::new(20, 1);
    let mut frame1 = line_buffer(20, &["hyperlink after"]);
    frame1.set_hyperlink(Rect::new(0, 0, 9, 1), Some(&link));
    screen.draw(&frame1);

    let mut frame2 = frame1.clone();
    frame2.set_string(2, 0, "X", Style::new().fg(Color::Red));
    let out = screen.draw(&frame2);

    assert_eq!(trimmed(screen.rows()), ["hyXerlink after"]);
    assert_eq!(
        screen.link_runs(),
        [
            (0, 0, 1, "https://example.com".into(), "ex".into()),
            (0, 3, 8, "https://example.com".into(), "ex".into()),
        ]
    );
    assert!(
        !out.contains("\x1b]8;id"),
        "the overlay must not re-open the link: {out:?}"
    );
}

/// A wide-character link drawn over previous text. Before cell extensions, per-cell links around
/// wide characters printed `リ ン ク !`.
#[test]
fn wide_characters_in_a_link() {
    let link = Hyperlink::new("https://example.com/リンク").with_id("jp");
    let mut screen = Screen::new(10, 1);
    screen.draw(&line_buffer(10, &["abcdefg"]));

    let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
    buf.set_line_with_hyperlink(0, 0, &Line::from("リンク!"), 10, &link);
    screen.draw(&buf);

    assert_eq!(trimmed(screen.rows()), ["リンク!"]);
    assert_eq!(
        screen.link_runs(),
        [(0, 0, 6, "https://example.com/リンク".into(), "jp".into())]
    );
}

/// A diff that only changes the middle of a link re-opens it with the same `id`.
#[test]
fn partial_redraw_keeps_the_link() {
    let link = Hyperlink::new("https://example.com");
    let mut screen = Screen::new(10, 1);
    let mut frame1 = line_buffer(10, &["abcdef"]);
    frame1.set_hyperlink(Rect::new(0, 0, 6, 1), Some(&link));
    screen.draw(&frame1);

    let mut frame2 = frame1.clone();
    frame2.set_style(Rect::new(2, 0, 2, 1), Style::new().bold());
    let out = screen.draw(&frame2);

    assert_eq!(trimmed(screen.rows()), ["abcdef"]);
    assert_eq!(screen.opens, 2);
    let id = frame1[(0, 0)]
        .hyperlink()
        .unwrap()
        .id()
        .unwrap()
        .to_string();
    assert!(out.contains(&format!("\x1b]8;id={id};https://example.com\x1b\\")));
    assert_eq!(
        screen.link_runs(),
        [(0, 0, 5, "https://example.com".into(), id)]
    );
}

/// A link over several rows keeps one `id`, so terminals highlight it as one link.
#[test]
fn link_spanning_rows_shares_an_id() {
    let link = Hyperlink::new("https://example.com");
    let mut screen = Screen::new(8, 2);
    let mut buf = line_buffer(8, &["wrapped ", "link"]);
    buf.set_hyperlink(Rect::new(0, 0, 8, 1), Some(&link));
    buf.set_hyperlink(Rect::new(0, 1, 4, 1), Some(&link.clone().with_id("w")));
    screen.draw(&buf);

    assert_eq!(trimmed(screen.rows()), ["wrapped", "link"]);
    let runs = screen.link_runs();
    assert_eq!(runs.len(), 2);
    assert_eq!((runs[0].0, runs[0].1, runs[0].2), (0, 0, 7));
    assert_eq!((runs[1].0, runs[1].1, runs[1].2), (1, 0, 3));
    assert_eq!(runs[1].4, "w");

    // The same area always gets the same derived id.
    let mut other = line_buffer(8, &["wrapped ", "link"]);
    other.set_hyperlink(Rect::new(0, 0, 8, 1), Some(&link));
    assert_eq!(runs[0].4, other[(0, 0)].hyperlink().unwrap().id().unwrap());
}

/// Removing a link redraws the cells without it.
#[test]
fn removing_a_link() {
    let link = Hyperlink::new("https://example.com");
    let mut screen = Screen::new(10, 1);
    let mut frame1 = line_buffer(10, &["link"]);
    frame1.set_hyperlink(Rect::new(0, 0, 4, 1), Some(&link));
    screen.draw(&frame1);
    assert_eq!(screen.link_runs().len(), 1);

    let frame2 = line_buffer(10, &["link"]);
    let out = screen.draw(&frame2);
    assert_eq!(trimmed(screen.rows()), ["link"]);
    assert_eq!(screen.link_runs(), []);
    assert!(
        out.contains("link"),
        "the unlinked cells are redrawn: {out:?}"
    );
}
