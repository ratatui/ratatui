// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
//! Render Ratatui frames through a [`termina::Terminal`].
//!
//! [`TerminaBackend`] writes Termina CSI/SGR escape sequences for Ratatui's [`Backend`] contract:
//! drawing cells, moving and querying the cursor, clearing regions, flushing output, and reading
//! terminal size. It wraps a caller-provided [`termina::Terminal`], so applications can keep using
//! Termina's event reader and typed terminal protocol surface alongside Ratatui rendering.
//! The Termina crate is re-exported as `ratatui_termina::termina`, so callers can use the same
//! Termina types as the backend.
//!
//! The backend does not enter raw mode, switch to the alternate screen, enable bracketed paste, or
//! install cleanup by itself. Configure those terminal modes with Termina before creating the
//! backend and restore them when the session ends. The `termina` example in this crate shows the
//! direct setup path with a small key-event loop.
//!
//! [`Backend`]: ratatui_core::backend::Backend
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]

use std::fmt::{self, Write as FmtWrite};
use std::io::{self, Write};

use ratatui_core::backend::{Backend, ClearType, WindowSize};
use ratatui_core::buffer::Cell;
use ratatui_core::layout::{Position, Size};
use ratatui_core::style::{Color, Modifier, Style};
pub use termina;
use termina::escape::csi::{
    Csi, Cursor, DecPrivateMode, DecPrivateModeCode, Edit, EraseInDisplay, EraseInLine, Mode, Sgr,
    SgrAttributes, SgrModifiers,
};
use termina::style::{Blink, ColorSpec, Intensity, RgbColor, Underline};
use termina::{Event, OneBased, Terminal};

macro_rules! decset {
    ($mode:ident) => {{
        let mode = DecPrivateMode::Code(DecPrivateModeCode::$mode);
        Csi::Mode(Mode::SetDecPrivateMode(mode))
    }};
}

macro_rules! decreset {
    ($mode:ident) => {{
        let mode = DecPrivateMode::Code(DecPrivateModeCode::$mode);
        Csi::Mode(Mode::ResetDecPrivateMode(mode))
    }};
}

/// A [`Backend`] implementation that renders through a [`termina::Terminal`].
///
/// `TerminaBackend` writes typed Termina escape sequences for drawing, cursor control, clearing,
/// and terminal-size queries.
///
/// This backend does not automatically enable raw mode or switch to the alternate screen. Use
/// Termina's terminal APIs and typed escape sequences to configure those application-level modes
/// before drawing.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui::Terminal;
/// use ratatui::backend::TerminaBackend;
/// use ratatui::termina::{PlatformTerminal, Terminal as _};
///
/// let mut output = PlatformTerminal::new()?;
/// output.enter_raw_mode()?;
///
/// let backend = TerminaBackend::new(output);
/// let mut terminal = Terminal::new(backend)?;
///
/// terminal.draw(|frame| {
///     // -- snip --
/// })?;
/// # std::io::Result::Ok(())
/// ```
pub struct TerminaBackend<T>
where
    T: Terminal,
{
    terminal: T,
}

impl<T> TerminaBackend<T>
where
    T: Terminal,
{
    /// Creates a backend that writes to the given Termina terminal.
    pub const fn new(terminal: T) -> Self {
        Self { terminal }
    }

    /// Returns the wrapped terminal.
    #[instability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui/ratatui/pull/991"
    )]
    pub const fn terminal(&self) -> &T {
        &self.terminal
    }

    /// Returns the wrapped terminal as a mutable reference.
    ///
    /// Direct writes can desynchronize Ratatui's diffing buffers from the visible terminal. Clear
    /// the terminal or force a full redraw before relying on Ratatui's next diff.
    #[instability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui/ratatui/pull/991"
    )]
    pub const fn terminal_mut(&mut self) -> &mut T {
        &mut self.terminal
    }
}

impl<T> Write for TerminaBackend<T>
where
    T: Terminal,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.terminal.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.terminal.flush()
    }
}

impl<T> Backend for TerminaBackend<T>
where
    T: Terminal,
{
    type Error = io::Error;

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        #[cfg(feature = "underline-color")]
        let mut underline_color = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<Position> = None;
        for (x, y, cell) in content {
            if !matches!(last_pos, Some(p) if x == p.x + 1 && y == p.y) {
                let command = Csi::Cursor(cursor_position(Position { x, y })?);
                write!(string, "{command}").unwrap();
            }
            last_pos = Some(Position { x, y });

            let mut attributes = SgrAttributes::default();
            if cell.fg != fg {
                attributes.foreground = Some(cell.fg.into_termina());
                fg = cell.fg;
            }
            if cell.bg != bg {
                attributes.background = Some(cell.bg.into_termina());
                bg = cell.bg;
            }
            #[cfg(feature = "underline-color")]
            if cell.underline_color != underline_color {
                attributes.underline_color = Some(cell.underline_color.into_termina());
                underline_color = cell.underline_color;
            }
            if cell.modifier != modifier {
                attributes.modifiers = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                }
                .into_termina();
                modifier = cell.modifier;
            }
            if !attributes.is_empty() {
                write!(string, "{}", Csi::Sgr(Sgr::Attributes(attributes))).unwrap();
            }

            string.push_str(cell.symbol());
        }

        write!(self.terminal, "{string}{}", Csi::Sgr(Sgr::Reset))
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        let command = decreset!(ShowCursor);
        write!(self.terminal, "{command}")?;
        self.terminal.flush()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        let command = decset!(ShowCursor);
        write!(self.terminal, "{command}")?;
        self.terminal.flush()
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        let command = Csi::Cursor(Cursor::RequestActivePositionReport);
        write!(self.terminal, "{command}")?;
        self.terminal.flush()?;
        let event = self.terminal.read(|event| {
            matches!(
                event,
                Event::Csi(Csi::Cursor(Cursor::ActivePositionReport { .. }))
            )
        })?;
        let Event::Csi(Csi::Cursor(Cursor::ActivePositionReport { line, col })) = event else {
            return Err(io::Error::other(
                "termina returned a non-cursor-position event",
            ));
        };
        Ok(Position {
            x: col.get_zero_based(),
            y: line.get_zero_based(),
        })
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        let command = Csi::Cursor(cursor_position(position.into())?);
        write!(self.terminal, "{command}")?;
        self.terminal.flush()
    }

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        let edit = match clear_type {
            ClearType::All => Edit::EraseInDisplay(EraseInDisplay::EraseDisplay),
            ClearType::AfterCursor => Edit::EraseInDisplay(EraseInDisplay::EraseToEndOfDisplay),
            ClearType::BeforeCursor => Edit::EraseInDisplay(EraseInDisplay::EraseToStartOfDisplay),
            ClearType::CurrentLine => Edit::EraseInLine(EraseInLine::EraseLine),
            ClearType::UntilNewLine => Edit::EraseInLine(EraseInLine::EraseToEndOfLine),
        };
        let command = Csi::Edit(edit);
        write!(self.terminal, "{command}")?;
        self.terminal.flush()
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        for _ in 0..n {
            writeln!(self.terminal)?;
        }
        self.terminal.flush()
    }

    fn size(&self) -> io::Result<Size> {
        let size = self.terminal.get_dimensions()?;
        Ok(Size::new(size.cols, size.rows))
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        let size = self.terminal.get_dimensions()?;
        Ok(WindowSize {
            columns_rows: Size::new(size.cols, size.rows),
            pixels: Size::new(
                size.pixel_width.unwrap_or_default(),
                size.pixel_height.unwrap_or_default(),
            ),
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.terminal.flush()
    }

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_up(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
        let margins = Csi::Cursor(set_top_and_bottom_margins(region)?);
        let scroll = Csi::Edit(Edit::ScrollUp(amount.into()));
        let reset = Csi::Cursor(reset_top_and_bottom_margins());
        write!(self.terminal, "{margins}{scroll}{reset}")?;
        self.terminal.flush()
    }

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_down(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
        let margins = Csi::Cursor(set_top_and_bottom_margins(region)?);
        let scroll = Csi::Edit(Edit::ScrollDown(amount.into()));
        let reset = Csi::Cursor(reset_top_and_bottom_margins());
        write!(self.terminal, "{margins}{scroll}{reset}")?;
        self.terminal.flush()
    }
}

fn cursor_position(position: Position) -> io::Result<Cursor> {
    Ok(Cursor::Position {
        line: one_based(position.y)?,
        col: one_based(position.x)?,
    })
}

fn one_based(n: u16) -> io::Result<OneBased> {
    n.checked_add(1)
        .and_then(OneBased::new)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "position exceeds u16::MAX - 1"))
}

#[cfg(feature = "scrolling-regions")]
fn set_top_and_bottom_margins(region: std::ops::Range<u16>) -> io::Result<Cursor> {
    Ok(Cursor::SetTopAndBottomMargins {
        top: one_based(region.start)?,
        bottom: OneBased::new(region.end).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "scroll region end cannot be 0")
        })?,
    })
}

#[cfg(feature = "scrolling-regions")]
const fn reset_top_and_bottom_margins() -> Cursor {
    Cursor::SetTopAndBottomMargins {
        top: OneBased::from_zero_based(0),
        bottom: OneBased::new(u16::MAX).expect("u16::MAX is non-zero"),
    }
}

/// A trait for converting a Ratatui type to a Termina type.
pub trait IntoTermina<T> {
    /// Converts the ratatui type to a Termina type.
    fn into_termina(self) -> T;
}

/// A trait for converting a Termina type to a Ratatui type.
pub trait FromTermina<T> {
    /// Converts the Termina type to a ratatui type.
    fn from_termina(value: T) -> Self;
}

struct ModifierDiff {
    from: Modifier,
    to: Modifier,
}

impl IntoTermina<ColorSpec> for Color {
    fn into_termina(self) -> ColorSpec {
        match self {
            Self::Reset => ColorSpec::Reset,
            Self::Black => ColorSpec::BLACK,
            Self::Red => ColorSpec::RED,
            Self::Green => ColorSpec::GREEN,
            Self::Yellow => ColorSpec::YELLOW,
            Self::Blue => ColorSpec::BLUE,
            Self::Magenta => ColorSpec::MAGENTA,
            Self::Cyan => ColorSpec::CYAN,
            Self::Gray => ColorSpec::WHITE,
            Self::DarkGray => ColorSpec::BRIGHT_BLACK,
            Self::LightRed => ColorSpec::BRIGHT_RED,
            Self::LightGreen => ColorSpec::BRIGHT_GREEN,
            Self::LightYellow => ColorSpec::BRIGHT_YELLOW,
            Self::LightBlue => ColorSpec::BRIGHT_BLUE,
            Self::LightMagenta => ColorSpec::BRIGHT_MAGENTA,
            Self::LightCyan => ColorSpec::BRIGHT_CYAN,
            Self::White => ColorSpec::BRIGHT_WHITE,
            Self::Indexed(i) => ColorSpec::PaletteIndex(i),
            Self::Rgb(r, g, b) => ColorSpec::TrueColor(RgbColor::new(r, g, b).into()),
        }
    }
}

impl IntoTermina<SgrAttributes> for Style {
    fn into_termina(self) -> SgrAttributes {
        SgrAttributes {
            foreground: self.fg.map(IntoTermina::into_termina),
            background: self.bg.map(IntoTermina::into_termina),
            #[cfg(feature = "underline-color")]
            underline_color: self.underline_color.map(IntoTermina::into_termina),
            modifiers: ModifierDiff {
                from: self.sub_modifier,
                to: self.add_modifier,
            }
            .into_termina(),
            ..Default::default()
        }
    }
}

impl FromTermina<ColorSpec> for Color {
    fn from_termina(value: ColorSpec) -> Self {
        match value {
            ColorSpec::Reset => Self::Reset,
            ColorSpec::PaletteIndex(i) => match i {
                0 => Self::Black,
                1 => Self::Red,
                2 => Self::Green,
                3 => Self::Yellow,
                4 => Self::Blue,
                5 => Self::Magenta,
                6 => Self::Cyan,
                7 => Self::Gray,
                8 => Self::DarkGray,
                9 => Self::LightRed,
                10 => Self::LightGreen,
                11 => Self::LightYellow,
                12 => Self::LightBlue,
                13 => Self::LightMagenta,
                14 => Self::LightCyan,
                15 => Self::White,
                _ => Self::Indexed(i),
            },
            ColorSpec::TrueColor(color) => Self::Rgb(color.red, color.green, color.blue),
        }
    }
}

impl IntoTermina<SgrModifiers> for ModifierDiff {
    fn into_termina(self) -> SgrModifiers {
        let removed = self.from - self.to;
        let added = self.to - self.from;
        let mut modifiers = SgrModifiers::empty();

        if removed.contains(Modifier::BOLD) || removed.contains(Modifier::DIM) {
            modifiers |= SgrModifiers::INTENSITY_NORMAL;
        }
        if removed.contains(Modifier::ITALIC) {
            modifiers |= SgrModifiers::NO_ITALIC;
        }
        if removed.contains(Modifier::UNDERLINED) {
            modifiers |= SgrModifiers::UNDERLINE_NONE;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            modifiers |= SgrModifiers::BLINK_NONE;
        }
        if removed.contains(Modifier::REVERSED) {
            modifiers |= SgrModifiers::NO_REVERSE;
        }
        if removed.contains(Modifier::HIDDEN) {
            modifiers |= SgrModifiers::NO_INVISIBLE;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            modifiers |= SgrModifiers::NO_STRIKE_THROUGH;
        }

        if added.contains(Modifier::BOLD) {
            modifiers |= SgrModifiers::INTENSITY_BOLD;
        }
        if added.contains(Modifier::DIM) {
            modifiers |= SgrModifiers::INTENSITY_DIM;
        }
        if added.contains(Modifier::ITALIC) {
            modifiers |= SgrModifiers::ITALIC;
        }
        if added.contains(Modifier::UNDERLINED) {
            modifiers |= SgrModifiers::UNDERLINE_SINGLE;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            modifiers |= SgrModifiers::BLINK_SLOW;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            modifiers |= SgrModifiers::BLINK_RAPID;
        }
        if added.contains(Modifier::REVERSED) {
            modifiers |= SgrModifiers::REVERSE;
        }
        if added.contains(Modifier::HIDDEN) {
            modifiers |= SgrModifiers::INVISIBLE;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            modifiers |= SgrModifiers::STRIKE_THROUGH;
        }

        modifiers
    }
}

impl FromTermina<Intensity> for Modifier {
    fn from_termina(value: Intensity) -> Self {
        match value {
            Intensity::Normal => Self::empty(),
            Intensity::Bold => Self::BOLD,
            Intensity::Dim => Self::DIM,
        }
    }
}

impl FromTermina<Underline> for Modifier {
    fn from_termina(value: Underline) -> Self {
        match value {
            Underline::None => Self::empty(),
            _ => Self::UNDERLINED,
        }
    }
}

impl FromTermina<Blink> for Modifier {
    fn from_termina(value: Blink) -> Self {
        match value {
            Blink::None => Self::empty(),
            Blink::Slow => Self::SLOW_BLINK,
            Blink::Rapid => Self::RAPID_BLINK,
        }
    }
}

impl<T> fmt::Debug for TerminaBackend<T>
where
    T: Terminal + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TerminaBackend")
            .field("terminal", &self.terminal)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ratatui_core::buffer::Cell;
    use termina::EventReader;
    use termina::escape::csi::Csi;

    use super::*;

    #[derive(Debug)]
    struct MockTerminal {
        output: Vec<u8>,
        size: termina::WindowSize,
        events: Vec<Event>,
    }

    impl MockTerminal {
        fn new() -> Self {
            Self {
                output: Vec::new(),
                size: termina::WindowSize {
                    cols: 80,
                    rows: 24,
                    pixel_width: Some(800),
                    pixel_height: Some(480),
                },
                events: Vec::new(),
            }
        }

        fn with_event(mut self, event: Event) -> Self {
            self.events.push(event);
            self
        }

        fn output(&self) -> String {
            String::from_utf8_lossy(&self.output).into_owned()
        }
    }

    impl Write for MockTerminal {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Terminal for MockTerminal {
        fn enter_raw_mode(&mut self) -> io::Result<()> {
            Ok(())
        }

        fn enter_cooked_mode(&mut self) -> io::Result<()> {
            Ok(())
        }

        fn get_dimensions(&self) -> io::Result<termina::WindowSize> {
            Ok(self.size)
        }

        fn event_reader(&self) -> EventReader {
            unimplemented!("backend tests do not use event_reader")
        }

        fn poll<F: Fn(&Event) -> bool>(
            &self,
            filter: F,
            _timeout: Option<Duration>,
        ) -> io::Result<bool> {
            Ok(self.events.iter().any(filter))
        }

        fn read<F: Fn(&Event) -> bool>(&self, filter: F) -> io::Result<Event> {
            self.events
                .iter()
                .find(|event| filter(event))
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::WouldBlock, "no matching event"))
        }

        fn set_panic_hook(
            &mut self,
            _f: impl Fn(&mut termina::PlatformHandle) + Send + Sync + 'static,
        ) {
        }
    }

    fn backend() -> TerminaBackend<MockTerminal> {
        TerminaBackend::new(MockTerminal::new())
    }

    #[test]
    fn writes_cursor_visibility_commands() {
        let mut backend = backend();
        backend.hide_cursor().unwrap();
        backend.show_cursor().unwrap();

        let hide_cursor = decreset!(ShowCursor);
        let show_cursor = decset!(ShowCursor);
        assert_eq!(
            backend.terminal.output(),
            format!("{hide_cursor}{show_cursor}")
        );
    }

    #[test]
    fn reads_cursor_position_reports() {
        let event = Event::Csi(Csi::Cursor(Cursor::ActivePositionReport {
            line: OneBased::new(5).unwrap(),
            col: OneBased::new(7).unwrap(),
        }));
        let mut backend = TerminaBackend::new(MockTerminal::new().with_event(event));

        assert_eq!(backend.get_cursor_position().unwrap(), Position::new(6, 4));
        let request = Csi::Cursor(Cursor::RequestActivePositionReport);
        assert_eq!(backend.terminal.output(), request.to_string());
    }

    #[test]
    fn rejects_non_cursor_position_reports() {
        let event = Event::FocusIn;
        let mut backend = TerminaBackend::new(MockTerminal::new().with_event(event));

        let error = backend.get_cursor_position().unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::WouldBlock);
    }

    #[test]
    fn sets_cursor_position() {
        let mut backend = backend();
        backend.set_cursor_position(Position::new(3, 4)).unwrap();

        let position = Cursor::Position {
            line: OneBased::new(5).unwrap(),
            col: OneBased::new(4).unwrap(),
        };
        assert_eq!(backend.terminal.output(), Csi::Cursor(position).to_string());
    }

    #[test]
    fn rejects_cursor_position_overflow() {
        let mut backend = backend();

        let error = backend.set_cursor_position(Position::new(u16::MAX, 0));
        assert_eq!(error.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn clears_regions() {
        let mut backend = backend();
        backend.clear_region(ClearType::All).unwrap();
        backend.clear_region(ClearType::AfterCursor).unwrap();
        backend.clear_region(ClearType::BeforeCursor).unwrap();
        backend.clear_region(ClearType::CurrentLine).unwrap();
        backend.clear_region(ClearType::UntilNewLine).unwrap();

        let expected = [
            Csi::Edit(Edit::EraseInDisplay(EraseInDisplay::EraseDisplay)),
            Csi::Edit(Edit::EraseInDisplay(EraseInDisplay::EraseToEndOfDisplay)),
            Csi::Edit(Edit::EraseInDisplay(EraseInDisplay::EraseToStartOfDisplay)),
            Csi::Edit(Edit::EraseInLine(EraseInLine::EraseLine)),
            Csi::Edit(Edit::EraseInLine(EraseInLine::EraseToEndOfLine)),
        ]
        .into_iter()
        .map(|command| command.to_string())
        .collect::<String>();
        assert_eq!(backend.terminal.output(), expected);
    }

    #[test]
    fn reports_terminal_size() {
        let mut backend = backend();

        assert_eq!(backend.size().unwrap(), Size::new(80, 24));
        assert_eq!(
            backend.window_size().unwrap(),
            WindowSize {
                columns_rows: Size::new(80, 24),
                pixels: Size::new(800, 480),
            }
        );
    }

    #[test]
    fn appends_lines() {
        let mut backend = backend();
        backend.append_lines(3).unwrap();

        assert_eq!(backend.terminal.output(), "\n\n\n");
    }

    #[test]
    fn draws_cells_with_grouped_sgr_attributes() {
        let mut backend = backend();
        let mut cell = Cell::new("x");
        cell.set_style(
            Style::new()
                .fg(Color::Red)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
        let content = [(2, 3, &cell)];

        backend.draw(content.into_iter()).unwrap();

        let output = backend.terminal.output();
        let cursor = Csi::Cursor(cursor_position(Position::new(2, 3)).unwrap());
        assert!(output.starts_with(&cursor.to_string()));
        assert!(output.contains('x'));
        assert!(output.ends_with(&Csi::Sgr(Sgr::Reset).to_string()));
    }

    #[test]
    fn converts_ratatui_colors_to_termina_colors() {
        assert_eq!(Color::Reset.into_termina(), ColorSpec::Reset);
        assert_eq!(Color::Red.into_termina(), ColorSpec::RED);
        assert_eq!(
            Color::Indexed(42).into_termina(),
            ColorSpec::PaletteIndex(42)
        );
        assert_eq!(
            Color::Rgb(1, 2, 3).into_termina(),
            ColorSpec::TrueColor(RgbColor::new(1, 2, 3).into())
        );
    }

    #[test]
    fn converts_termina_colors_to_ratatui_colors() {
        assert_eq!(Color::from_termina(ColorSpec::Reset), Color::Reset);
        assert_eq!(Color::from_termina(ColorSpec::PaletteIndex(1)), Color::Red);
        assert_eq!(
            Color::from_termina(ColorSpec::TrueColor(RgbColor::new(1, 2, 3).into())),
            Color::Rgb(1, 2, 3)
        );
    }

    #[test]
    fn converts_modifier_diffs_to_sgr_modifiers() {
        let from = Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED;
        let to = Modifier::DIM | Modifier::REVERSED | Modifier::CROSSED_OUT;
        let modifiers = ModifierDiff { from, to }.into_termina();

        assert!(modifiers.contains(SgrModifiers::INTENSITY_NORMAL));
        assert!(modifiers.contains(SgrModifiers::NO_ITALIC));
        assert!(modifiers.contains(SgrModifiers::UNDERLINE_NONE));
        assert!(modifiers.contains(SgrModifiers::INTENSITY_DIM));
        assert!(modifiers.contains(SgrModifiers::REVERSE));
        assert!(modifiers.contains(SgrModifiers::STRIKE_THROUGH));
    }

    #[test]
    fn converts_termina_modifiers_to_ratatui_modifiers() {
        assert_eq!(Modifier::from_termina(Intensity::Normal), Modifier::empty());
        assert_eq!(Modifier::from_termina(Intensity::Bold), Modifier::BOLD);
        assert_eq!(Modifier::from_termina(Underline::None), Modifier::empty());
        assert_eq!(
            Modifier::from_termina(Underline::Single),
            Modifier::UNDERLINED
        );
        assert_eq!(Modifier::from_termina(Blink::None), Modifier::empty());
        assert_eq!(Modifier::from_termina(Blink::Rapid), Modifier::RAPID_BLINK);
    }

    #[cfg(feature = "scrolling-regions")]
    #[test]
    fn scrolls_regions() {
        let mut backend = backend();
        backend.scroll_region_up(1..4, 2).unwrap();
        backend.scroll_region_down(1..4, 3).unwrap();

        let margins = Cursor::SetTopAndBottomMargins {
            top: OneBased::new(2).unwrap(),
            bottom: OneBased::new(4).unwrap(),
        };
        let reset = reset_top_and_bottom_margins();
        let up = Csi::Edit(Edit::ScrollUp(2_u16.into()));
        let down = Csi::Edit(Edit::ScrollDown(3_u16.into()));
        let expected = format!(
            "{}{up}{}{}{down}{}",
            Csi::Cursor(margins.clone()),
            Csi::Cursor(reset.clone()),
            Csi::Cursor(margins),
            Csi::Cursor(reset)
        );
        assert_eq!(backend.terminal.output(), expected);
    }

    #[cfg(feature = "scrolling-regions")]
    #[test]
    fn rejects_zero_ended_scroll_regions() {
        let mut backend = backend();

        let error = backend.scroll_region_up(0..0, 1).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn csi_helpers_use_one_based_coordinates() {
        assert_eq!(
            cursor_position(Position::new(1, 2)).unwrap(),
            Cursor::Position {
                line: OneBased::new(3).unwrap(),
                col: OneBased::new(2).unwrap(),
            }
        );
        assert_eq!(one_based(0).unwrap(), OneBased::new(1).unwrap());
    }
}
