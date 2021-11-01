use crate::{
    backend::{Backend, ClearType},
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::io;
use tracing::{event, span, Level};

#[derive(Debug, Clone, PartialEq)]
pub enum ViewportVariant {
    Fullscreen,
    Inline(u16),
    Fixed(Rect),
}

#[derive(Debug, Clone, PartialEq)]
/// Options to pass to [`Terminal::with_options`]
pub struct TerminalOptions {
    /// Viewport used to draw to the terminal
    pub viewport: ViewportVariant,
}

#[derive(Debug, Clone, PartialEq)]
struct Viewport {
    variant: ViewportVariant,
    area: Rect,
}

/// Interface to the terminal backed by Termion
#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
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
    /// Last known size of the terminal. Used to detect if the internal buffers have to be resized.
    last_known_size: Rect,
    /// Last known position of the cursor. Used to find the new area when the viewport is inlined
    /// and the terminal resized.
    last_known_cursor_pos: (u16, u16),
}

/// Represents a consistent terminal interface for rendering.
pub struct Frame<'a, B: 'a>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,

    /// Where should the cursor be after drawing this frame?
    ///
    /// If `None`, the cursor is hidden and its position is controlled by the backend. If `Some((x,
    /// y))`, the cursor is shown and placed at `(x, y)` after the call to `Terminal::draw()`.
    cursor_position: Option<(u16, u16)>,
}

impl<'a, B> Frame<'a, B>
where
    B: Backend,
{
    /// Frame size, guaranteed not to change when rendering.
    pub fn size(&self) -> Rect {
        self.terminal.viewport.area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::Terminal;
    /// # use ratatui::backend::TestBackend;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Block;
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// let block = Block::default();
    /// let area = Rect::new(0, 0, 5, 5);
    /// let mut frame = terminal.get_frame();
    /// frame.render_widget(block, area);
    /// ```
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, self.terminal.current_buffer_mut());
    }

    /// Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
    ///
    /// The last argument should be an instance of the [`StatefulWidget::State`] associated to the
    /// given [`StatefulWidget`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::Terminal;
    /// # use ratatui::backend::TestBackend;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::{List, ListItem, ListState};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// let mut state = ListState::default();
    /// state.select(Some(1));
    /// let items = vec![
    ///     ListItem::new("Item 1"),
    ///     ListItem::new("Item 2"),
    /// ];
    /// let list = List::new(items);
    /// let area = Rect::new(0, 0, 5, 5);
    /// let mut frame = terminal.get_frame();
    /// frame.render_stateful_widget(list, area, &mut state);
    /// ```
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, self.terminal.current_buffer_mut(), state);
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
}

/// CompletedFrame represents the state of the terminal after all changes performed in the last
/// [`Terminal::draw`] call have been applied. Therefore, it is only valid until the next call to
/// [`Terminal::draw`].
pub struct CompletedFrame<'a> {
    pub buffer: &'a Buffer,
    pub area: Rect,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            if let Err(err) = self.show_cursor() {
                eprintln!("Failed to show the cursor: {}", err);
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Wrapper around Terminal initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: ViewportVariant::Fullscreen,
            },
        )
    }

    pub fn with_options(mut backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        let size = backend.size()?;
        let (viewport_area, cursor_pos) = match options.viewport {
            ViewportVariant::Fullscreen => (size, (0, 0)),
            ViewportVariant::Inline(height) => {
                let pos = backend.get_cursor()?;
                let mut row = pos.1;
                let max_height = size.height.min(height);
                backend.append_lines(max_height.saturating_sub(1))?;
                let missing_lines = row.saturating_add(max_height).saturating_sub(size.height);
                if missing_lines > 0 {
                    row = row.saturating_sub(missing_lines);
                }
                (
                    Rect {
                        x: 0,
                        y: row,
                        width: size.width,
                        height: max_height,
                    },
                    pos,
                )
            }
            ViewportVariant::Fixed(area) => (area, (area.left(), area.top())),
        };
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: Viewport {
                variant: options.viewport,
                area: viewport_area,
            },
            last_known_size: size,
            last_known_cursor_pos: cursor_pos,
        })
    }

    /// Get a Frame object which provides a consistent view into the terminal state for rendering.
    pub fn get_frame(&mut self) -> Frame<B> {
        Frame {
            terminal: self,
            cursor_position: None,
        }
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

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

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    pub fn resize(&mut self) -> io::Result<()> {
        let size = self.size()?;
        if self.last_known_size == size {
            return Ok(());
        }

        event!(Level::DEBUG, last_known_size = ?self.last_known_size, ?size, "terminal size changed");

        let next_area = match self.viewport.variant {
            ViewportVariant::Fullscreen => size,
            ViewportVariant::Inline(height) => {
                let (_, mut row) = self.get_cursor()?;
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .1
                    .saturating_sub(self.viewport.area.top());
                let max_height = height.min(size.height);
                let lines_after_cursor = height
                    .saturating_sub(offset_in_previous_viewport)
                    .saturating_sub(1);
                let available_lines = size.height.saturating_sub(row).saturating_sub(1);
                let missing_lines = lines_after_cursor.saturating_sub(available_lines);
                self.backend.append_lines(lines_after_cursor)?;
                if missing_lines > 0 {
                    row = row.saturating_sub(missing_lines);
                }
                row = row.saturating_sub(offset_in_previous_viewport);
                Rect {
                    x: 0,
                    y: row,
                    width: size.width,
                    height: max_height,
                }
            }
            ViewportVariant::Fixed(area) => area,
        };
        self.set_viewport_area(next_area);
        self.clear()?;

        self.last_known_size = size;
        Ok(())
    }

    fn set_viewport_area(&mut self, area: Rect) {
        self.viewport.area = area;
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        event!(Level::DEBUG, area = ?area, "viewport changed");
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        let span = span!(Level::DEBUG, "draw");
        let _guard = span.enter();

        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.resize()?;

        let mut frame = self.get_frame();
        f(&mut frame);
        // We can't change the cursor position right away because we have to flush the frame to
        // stdout first. But we also can't keep the frame around, since it holds a &mut to
        // Terminal. Thus, we're taking the important data out of the Frame and dropping it.
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

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        self.backend.flush()?;

        event!(Level::DEBUG, "completed frame");

        Ok(CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.last_known_size,
        })
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)?;
        self.last_known_cursor_pos = (x, y);
        Ok(())
    }

    /// Clear the terminal and force a full redraw on the next draw call.
    pub fn clear(&mut self) -> io::Result<()> {
        event!(Level::DEBUG, "clear");
        match self.viewport.variant {
            ViewportVariant::Fullscreen => self.backend.clear(ClearType::All)?,
            ViewportVariant::Inline(_) => {
                self.backend
                    .set_cursor(self.viewport.area.left(), self.viewport.area.top())?;
                self.backend.clear(ClearType::AfterCursor)?;
            }
            ViewportVariant::Fixed(area) => {
                for row in area.top()..area.bottom() {
                    self.backend.set_cursor(0, row)?;
                    self.backend.clear(ClearType::AfterCursor)?;
                }
            }
        }
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }

    /// Queries the real size of the backend.
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }

    /// Insert some content before the current inline viewport. This has no effect when the
    /// viewport is fullscreen.
    ///
    /// This function scrolls down the current viewport by the given height. The newly freed space is
    /// then made available to the `draw_fn` closure through a writable `Buffer`.
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
    /// # use tui::widgets::{Paragraph, Widget};
    /// # use tui::text::{Spans, Span};
    /// # use tui::style::{Color, Style};
    /// # use tui::{Terminal};
    /// # use tui::backend::TestBackend;
    /// # let backend = TestBackend::new(10, 10);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// terminal.insert_before(1, |buf| {
    ///     Paragraph::new(Spans::from(vec![
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
        let span = span!(Level::DEBUG, "insert_before");
        let _guard = span.enter();
        if !matches!(self.viewport.variant, ViewportVariant::Inline(_)) {
            return Ok(());
        }

        self.clear()?;
        let height = height.min(self.last_known_size.height);
        self.backend.append_lines(height)?;
        let missing_lines =
            height.saturating_sub(self.last_known_size.bottom() - self.viewport.area.top());
        let area = Rect {
            x: self.viewport.area.left(),
            y: self.viewport.area.top().saturating_sub(missing_lines),
            width: self.viewport.area.width,
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
        let missing_lines = self.viewport.area.height.saturating_sub(remaining_lines);
        self.backend.append_lines(self.viewport.area.height)?;

        self.set_viewport_area(Rect {
            x: area.left(),
            y: area.bottom().saturating_sub(missing_lines),
            width: area.width,
            height: self.viewport.area.height,
        });

        Ok(())
    }
}
