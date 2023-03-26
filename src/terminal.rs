use crate::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::io::{self, Error, ErrorKind};

/// Interface to the terminal backed by Termion
#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    buffer: Buffer,
    viewport: Rect,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if let Err(err) = self.show_cursor() {
            eprintln!("Failed to show the cursor: {}", err);
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Initializes a terminal with a buffer size the same as the backend size.
    /// Buffer cells are initialized with empty strings using
    /// default foreground and the background color.
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let (width, height) = backend.dimensions()?;
        Ok(Terminal {
            backend,
            buffer: Buffer::empty(width, height),
            viewport: Rect {
                x: 0,
                y: 0,
                width,
                height,
            },
        })
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    /// Should be proceeded with a call to terminal.flush().
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
    /// terminal.render_widget(block, area);
    /// ```
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, &mut self.buffer);
    }

    /// Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
    /// Should be proceeded with a call to terminal.flush().
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
    /// terminal.render_stateful_widget(list, area, &mut state);
    /// ```
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, &mut self.buffer, state);
    }

    pub fn viewport_area(&self) -> Rect {
        self.viewport
    }

    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// `Ok` content is `Result<()>`, Nested `Ok` representing a sucessfull scroll.
    /// Nested `Err` representing a viewport scroll overflowing buffer attempt.
    pub fn viewport_scroll(&mut self, x_step: i16, y_step: i16) -> io::Result<io::Result<()>> {
        let new_x_offset = self.viewport.x.saturating_add_signed(x_step);
        let new_y_offset = self.viewport.y.saturating_add_signed(y_step);
        if new_x_offset + self.viewport.width > self.buffer.get_width()
            || new_y_offset + self.viewport.height > self.buffer.get_height()
        {
            return Ok(Err(Error::new(
                ErrorKind::Other,
                "Viewport scroll overflows buffer",
            )));
        }
        self.viewport.x = new_x_offset;
        self.viewport.y = new_y_offset;
        self.flush_viewport_region()?;
        Ok(Ok(()))
    }

    pub fn resize_buffer(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height)
    }

    /// Queries the backend for its viewport size and resizes frontend viewport size
    /// if it doesn't match.
    fn autoresize(&mut self) -> io::Result<()> {
        let (b_width, b_height) = self.backend.dimensions()?;
        if self.backend.size()? != self.viewport.size() {
            self.viewport.width = b_width;
            self.viewport.height = b_height;
            if self.buffer.cells.len() < self.backend.size()? {
                self.buffer.resize(b_width, b_height)
            }
            self.clear()?
        }

        Ok(())
    }

    /// Clears buffer and backend.
    fn clear(&mut self) -> io::Result<()> {
        self.buffer.clear();
        self.backend.clear()
    }

    pub fn clear_region(&mut self, area: Rect) {
        self.buffer.clear_region(area);
    }

    /// Flush buffer content to backend.
    /// Content flushed is based on the viewport offset and backend terminal size.
    pub fn flush(&mut self) -> io::Result<()> {
        self.autoresize()?;
        self.flush_viewport_region()
    }

    fn flush_viewport_region(&mut self) -> io::Result<()> {
        let mut buffer_region = self.buffer.get_region(self.viewport_area());
        buffer_region.iter_mut().for_each(|(x, y, _)| {
            *x -= self.viewport.x;
            *y -= self.viewport.y;
        });

        self.backend.draw(buffer_region.iter())
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()
    }

    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)
    }
}
