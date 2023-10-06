#![deny(missing_docs)]
//! Provides the [`Terminal`], [`Frame`] and related types.
//!
//! The [`Terminal`] is the main interface of this library. It is responsible for drawing and
//! maintaining the state of the different widgets that compose your application.
//!
//! The [`Frame`] is a consistent view into the terminal state for rendering. It is obtained via
//! the closure argument of [`Terminal::draw`]. It is used to render widgets to the terminal and
//! control the cursor position.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::io::stdout;
//! use ratatui::{prelude::*, widgets::Paragraph};
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend)?;
//! terminal.draw(|frame| {
//!     let area = frame.size();
//!     frame.render_widget(Paragraph::new("Hello world!"), area);
//! })?;
//! # std::io::Result::Ok(())
//! ```
//!
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [`backend`]: crate::backend
//! [`Backend`]: crate::backend::Backend
//! [`Buffer`]: crate::buffer::Buffer
use std::{fmt, io};

use crate::{
    backend::{Backend, ClearType},
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

/// Represents the viewport of the terminal. The viewport is the area of the terminal that is
/// currently visible to the user. It can be either fullscreen, inline or fixed.
///
/// When the viewport is fullscreen, the whole terminal is used to draw the application.
///
/// When the viewport is inline, it is drawn inline with the rest of the terminal. The height of
/// the viewport is fixed, but the width is the same as the terminal width.
///
/// When the viewport is fixed, it is drawn in a fixed area of the terminal. The area is specified
/// by a [`Rect`].
///
/// See [`Terminal::with_options`] for more information.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum Viewport {
    /// The viewport is fullscreen
    #[default]
    Fullscreen,
    /// The viewport is inline with the rest of the terminal.
    ///
    /// The viewport's height is fixed and specified in number of lines. The width is the same as
    /// the terminal's width. The viewport is drawn below the cursor position.
    Inline(u16),
    /// The viewport is drawn in a fixed area of the terminal. The area is specified by a [`Rect`].
    Fixed(Rect),
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Viewport::Fullscreen => write!(f, "Fullscreen"),
            Viewport::Inline(height) => write!(f, "Inline({})", height),
            Viewport::Fixed(area) => write!(f, "Fixed({})", area),
        }
    }
}

/// Options to pass to [`Terminal::with_options`]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct TerminalOptions {
    /// Viewport used to draw to the terminal
    pub viewport: Viewport,
}

/// An interface to interact and draw [`Frame`]s on the user's terminal.
///
/// This is the main entry point for Ratatui. It is responsible for drawing and maintaining the
/// state of the buffers, cursor and viewport.
///
/// The [`Terminal`] is generic over a [`Backend`] implementation which is used to interface with
/// the underlying terminal library. The [`Backend`] trait is implemented for three popular Rust
/// terminal libraries: [Crossterm], [Termion] and [Termwiz]. See the [`backend`] module for more
/// information.
///
/// The `Terminal` struct maintains two buffers: the current and the previous.
/// When the widgets are drawn, the changes are accumulated in the current buffer.
/// At the end of each draw pass, the two buffers are compared, and only the changes
/// between these buffers are written to the terminal, avoiding any redundant operations.
/// After flushing these changes, the buffers are swapped to prepare for the next draw cycle./
///
/// The terminal also has a viewport which is the area of the terminal that is currently visible to
/// the user. It can be either fullscreen, inline or fixed. See [`Viewport`] for more information.
///
/// Applications should detect terminal resizes and call [`Terminal::draw`] to redraw the
/// application with the new size. This will automatically resize the internal buffers to match the
/// new size for inline and fullscreen viewports. Fixed viewports are not resized automatically.
///
/// # Examples
///
/// ```rust,no_run
/// use std::io::stdout;
/// use ratatui::{prelude::*, widgets::Paragraph};
///
/// let backend = CrosstermBackend::new(stdout());
/// let mut terminal = Terminal::new(backend)?;
/// terminal.draw(|frame| {
///     let area = frame.size();
///     frame.render_widget(Paragraph::new("Hello World!"), area);
///     frame.set_cursor(0, 0);
/// })?;
/// # std::io::Result::Ok(())
/// ```
///
/// [Crossterm]: https://crates.io/crates/crossterm
/// [Termion]: https://crates.io/crates/termion
/// [Termwiz]: https://crates.io/crates/termwiz
/// [`backend`]: crate::backend
/// [`Backend`]: crate::backend::Backend
/// [`Buffer`]: crate::buffer::Buffer
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Terminal<B>
where
    B: Backend,
{
    /// The backend used to interface with the terminal
    backend: B,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
    /// Whether the cursor is currently hidden
    hidden_cursor: bool,
    /// Viewport
    viewport: Viewport,
    /// Area of the viewport
    viewport_area: Rect,
    /// Last known size of the terminal. Used to detect if the internal buffers have to be resized.
    last_known_size: Rect,
    /// Last known position of the cursor. Used to find the new area when the viewport is inlined
    /// and the terminal resized.
    last_known_cursor_pos: (u16, u16),
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            if let Err(err) = self.show_cursor() {
                eprintln!("Failed to show the cursor: {err}");
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Creates a new [`Terminal`] with the given [`Backend`] with a full screen viewport.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::io::stdout;
    /// # use ratatui::prelude::*;
    /// let backend = CrosstermBackend::new(stdout());
    /// let terminal = Terminal::new(backend)?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fullscreen,
            },
        )
    }

    /// Creates a new [`Terminal`] with the given [`Backend`] and [`TerminalOptions`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::io::stdout;
    /// # use ratatui::{prelude::*, backend::TestBackend};
    /// let backend = CrosstermBackend::new(stdout());
    /// let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// let terminal = Terminal::with_options(
    ///     backend,
    ///     TerminalOptions { viewport },
    /// )?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_options(mut backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        let size = match options.viewport {
            Viewport::Fullscreen | Viewport::Inline(_) => backend.size()?,
            Viewport::Fixed(area) => area,
        };
        let (viewport_area, cursor_pos) = match options.viewport {
            Viewport::Fullscreen => (size, (0, 0)),
            Viewport::Inline(height) => compute_inline_size(&mut backend, height, size, 0)?,
            Viewport::Fixed(area) => (area, (area.left(), area.top())),
        };
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
            viewport_area,
            last_known_size: size,
            last_known_cursor_pos: cursor_pos,
        })
    }

    /// Get a Frame object which provides a consistent view into the terminal state for rendering.
    pub fn get_frame(&mut self) -> Frame {
        Frame {
            cursor_position: None,
            viewport_area: self.viewport_area,
            buffer: self.current_buffer_mut(),
        }
    }

    /// Gets the current buffer as a mutable reference.
    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    /// Gets the backend
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Gets the backend as a mutable reference
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Obtains a difference between the previous and the current buffer and passes it to the
    /// current backend for drawing.
    pub fn flush(&mut self) -> io::Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some((col, row, _)) = updates.last() {
            self.last_known_cursor_pos = (*col, *row);
        }
        self.backend.draw(updates.into_iter())
    }

    /// Updates the Terminal so that internal buffers match the requested size.
    ///
    /// Requested size will be saved so the size can remain consistent when rendering. This leads
    /// to a full clear of the screen.
    pub fn resize(&mut self, size: Rect) -> io::Result<()> {
        let next_area = match self.viewport {
            Viewport::Fullscreen => size,
            Viewport::Inline(height) => {
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .1
                    .saturating_sub(self.viewport_area.top());
                compute_inline_size(&mut self.backend, height, size, offset_in_previous_viewport)?.0
            }
            Viewport::Fixed(area) => area,
        };
        self.set_viewport_area(next_area);
        self.clear()?;

        self.last_known_size = size;
        Ok(())
    }

    fn set_viewport_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    pub fn autoresize(&mut self) -> io::Result<()> {
        // fixed viewports do not get autoresized
        if matches!(self.viewport, Viewport::Fullscreen | Viewport::Inline(_)) {
            let size = self.size()?;
            if size != self.last_known_size {
                self.resize(size)?;
            }
        };
        Ok(())
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    ///
    /// This is the main entry point for drawing to the terminal.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::io::stdout;
    /// # use ratatui::{prelude::*, widgets::Paragraph};
    /// let backend = CrosstermBackend::new(stdout());
    /// let mut terminal = Terminal::new(backend)?;
    /// terminal.draw(|frame| {
    ///     let area = frame.size();
    ///     frame.render_widget(Paragraph::new("Hello World!"), area);
    ///     frame.set_cursor(0, 0);
    /// })?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = self.get_frame();
        f(&mut frame);
        // We can't change the cursor position right away because we have to flush the frame to
        // stdout first. But we also can't keep the frame around, since it holds a &mut to
        // Buffer. Thus, we're taking the important data out of the Frame and dropping it.
        let cursor_position = frame.cursor_position;

        // Draw to stdout
        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some((x, y)) => {
                self.show_cursor()?;
                self.set_cursor(x, y)?;
            }
        }

        self.swap_buffers();

        // Flush
        self.backend.flush()?;

        Ok(CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.last_known_size,
        })
    }

    /// Hides the cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    /// Shows the cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    /// Gets the current cursor position.
    ///
    /// This is the position of the cursor after the last draw call and is returned as a tuple of
    /// `(x, y)` coordinates.
    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    /// Sets the cursor position.
    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)?;
        self.last_known_cursor_pos = (x, y);
        Ok(())
    }

    /// Clear the terminal and force a full redraw on the next draw call.
    pub fn clear(&mut self) -> io::Result<()> {
        match self.viewport {
            Viewport::Fullscreen => self.backend.clear_region(ClearType::All)?,
            Viewport::Inline(_) => {
                self.backend
                    .set_cursor(self.viewport_area.left(), self.viewport_area.top())?;
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(area) => {
                for row in area.top()..area.bottom() {
                    self.backend.set_cursor(0, row)?;
                    self.backend.clear_region(ClearType::AfterCursor)?;
                }
            }
        }
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }

    /// Clears the inactive buffer and swaps it with the current buffer
    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    /// Queries the real size of the backend.
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }

    /// Insert some content before the current inline viewport. This has no effect when the
    /// viewport is fullscreen.
    ///
    /// This function scrolls down the current viewport by the given height. The newly freed space
    /// is then made available to the `draw_fn` closure through a writable `Buffer`.
    ///
    /// Before:
    /// ```ignore
    /// +-------------------+
    /// |                   |
    /// |      viewport     |
    /// |                   |
    /// +-------------------+
    /// ```
    ///
    /// After:
    /// ```ignore
    /// +-------------------+
    /// |      buffer       |
    /// +-------------------+
    /// +-------------------+
    /// |                   |
    /// |      viewport     |
    /// |                   |
    /// +-------------------+
    /// ```
    ///
    /// # Examples
    ///
    /// ## Insert a single line before the current viewport
    ///
    /// ```rust
    /// # use ratatui::{backend::TestBackend, prelude::*, widgets::*};
    /// # let backend = TestBackend::new(10, 10);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// terminal.insert_before(1, |buf| {
    ///     Paragraph::new(Line::from(vec![
    ///         Span::raw("This line will be added "),
    ///         Span::styled("before", Style::default().fg(Color::Blue)),
    ///         Span::raw(" the current viewport")
    ///     ])).render(buf.area, buf);
    /// });
    /// ```
    pub fn insert_before<F>(&mut self, height: u16, draw_fn: F) -> io::Result<()>
    where
        F: FnOnce(&mut Buffer),
    {
        if !matches!(self.viewport, Viewport::Inline(_)) {
            return Ok(());
        }

        self.clear()?;
        let height = height.min(self.last_known_size.height);
        self.backend.append_lines(height)?;
        let missing_lines =
            height.saturating_sub(self.last_known_size.bottom() - self.viewport_area.top());
        let area = Rect {
            x: self.viewport_area.left(),
            y: self.viewport_area.top().saturating_sub(missing_lines),
            width: self.viewport_area.width,
            height,
        };
        let mut buffer = Buffer::empty(area);

        draw_fn(&mut buffer);

        let iter = buffer.content.iter().enumerate().map(|(i, c)| {
            let (x, y) = buffer.pos_of(i);
            (x, y, c)
        });
        self.backend.draw(iter)?;
        self.backend.flush()?;

        let remaining_lines = self.last_known_size.height - area.bottom();
        let missing_lines = self.viewport_area.height.saturating_sub(remaining_lines);
        self.backend.append_lines(self.viewport_area.height)?;

        self.set_viewport_area(Rect {
            x: area.left(),
            y: area.bottom().saturating_sub(missing_lines),
            width: area.width,
            height: self.viewport_area.height,
        });

        Ok(())
    }
}

fn compute_inline_size<B: Backend>(
    backend: &mut B,
    height: u16,
    size: Rect,
    offset_in_previous_viewport: u16,
) -> io::Result<(Rect, (u16, u16))> {
    let pos = backend.get_cursor()?;
    let mut row = pos.1;

    let max_height = size.height.min(height);

    let lines_after_cursor = height
        .saturating_sub(offset_in_previous_viewport)
        .saturating_sub(1);

    backend.append_lines(lines_after_cursor)?;

    let available_lines = size.height.saturating_sub(row).saturating_sub(1);
    let missing_lines = lines_after_cursor.saturating_sub(available_lines);
    if missing_lines > 0 {
        row = row.saturating_sub(missing_lines);
    }
    row = row.saturating_sub(offset_in_previous_viewport);

    Ok((
        Rect {
            x: 0,
            y: row,
            width: size.width,
            height: max_height,
        },
        pos,
    ))
}

/// A consistent view into the terminal state for rendering a single frame.
///
/// This is obtained via the closure argument of [`Terminal::draw`]. It is used to render widgets
/// to the terminal and control the cursor position.
///
/// The changes drawn to the frame are applied only to the current [`Buffer`].
/// After the closure returns, the current buffer is compared to the previous
/// buffer and only the changes are applied to the terminal.
///
/// [`Buffer`]: crate::buffer::Buffer
#[derive(Debug, Hash)]
pub struct Frame<'a> {
    /// Where should the cursor be after drawing this frame?
    ///
    /// If `None`, the cursor is hidden and its position is controlled by the backend. If `Some((x,
    /// y))`, the cursor is shown and placed at `(x, y)` after the call to `Terminal::draw()`.
    cursor_position: Option<(u16, u16)>,
    /// The area of the viewport
    viewport_area: Rect,

    /// The buffer that is used to draw the current frame
    buffer: &'a mut Buffer,
}

impl Frame<'_> {
    /// The size of the current frame
    ///
    /// This is guaranteed not to change when rendering.
    pub fn size(&self) -> Rect {
        self.viewport_area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{backend::TestBackend, prelude::*, widgets::Block};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// let block = Block::default();
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_widget(block, area);
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, self.buffer);
    }

    /// Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// The last argument should be an instance of the [`StatefulWidget::State`] associated to the
    /// given [`StatefulWidget`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{backend::TestBackend, prelude::*, widgets::*};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// let mut state = ListState::default().with_selected(Some(1));
    /// let list = List::new(vec![
    ///     ListItem::new("Item 1"),
    ///     ListItem::new("Item 2"),
    /// ]);
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_stateful_widget(list, area, &mut state);
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, self.buffer, state);
    }

    /// After drawing this frame, make the cursor visible and put it at the specified (x, y)
    /// coordinates. If this method is not called, the cursor will be hidden.
    ///
    /// Note that this will interfere with calls to `Terminal::hide_cursor()`,
    /// `Terminal::show_cursor()`, and `Terminal::set_cursor()`. Pick one of the APIs and stick
    /// with it.
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor_position = Some((x, y));
    }

    /// Gets the buffer that this `Frame` draws into as a mutable reference.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }
}

/// `CompletedFrame` represents the state of the terminal after all changes performed in the last
/// [`Terminal::draw`] call have been applied. Therefore, it is only valid until the next call to
/// [`Terminal::draw`].
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CompletedFrame<'a> {
    /// The buffer that was used to draw the last frame.
    pub buffer: &'a Buffer,
    /// The size of the last frame.
    pub area: Rect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_to_string() {
        assert_eq!(Viewport::Fullscreen.to_string(), "Fullscreen");
        assert_eq!(Viewport::Inline(5).to_string(), "Inline(5)");
        assert_eq!(
            Viewport::Fixed(Rect::new(0, 0, 5, 5)).to_string(),
            "Fixed(5x5+0+0)"
        );
    }
}
