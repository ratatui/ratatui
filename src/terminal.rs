use crate::{
    backend::Backend,
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::io;

#[derive(Debug, Clone, PartialEq)]
/// Options to pass to [`Terminal::with_options`]
pub struct TerminalOptions {
    /// Viewport used to draw to the terminal
    pub viewport: Viewport,
}

/// Represents a consistent terminal interface for rendering.
pub struct Frame<'a, B: 'a>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
}

impl<'a, B> Frame<'a, B>
where
    B: Backend,
{
    /// Terminal size, guaranteed not to change when rendering.
    pub fn viewport_area(&self) -> Rect {
        self.terminal.viewport_area()
    }

    pub fn resize_buffers(&mut self, width: u16, height: u16) {
        self.terminal.resize_buffer(width, height)
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn clear_region(&mut self, area: Rect) {
        self.terminal.clear_buffer_region(area)
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
    /// terminal.draw(|frame| frame.render_widget(block, area));
    /// ```
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, &mut self.terminal.buffer);
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
    /// terminal.draw(|frame| frame.render_stateful_widget(list, area, &mut state));
    /// ```
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, &mut self.terminal.buffer, state);
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.terminal.set_cursor(x, y)
    }
}

/// CompletedFrame represents the state of the terminal after all changes performed in the last
/// [`Terminal::draw`] call have been applied. Therefore, it is only valid until the next call to
/// [`Terminal::draw`].
pub struct CompletedFrame<'a> {
    pub buffer: &'a Buffer,
    pub area: Rect,
}

/// Interface to the terminal backed by Termion
#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    buffer: Buffer,
    /// Whether the cursor is currently hidden
    hidden_cursor: bool,
    /// Viewport
    viewport: Viewport,
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
        let (width, height) = backend.dimensions()?;
        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport {
                    area: Rect::new(0, 0, width, height),
                    resize_behavior: ResizeBehavior::Auto,
                },
            },
        )
    }

    /// UNSTABLE
    pub fn with_options(backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        let width = options.viewport.area.width;
        let height = options.viewport.area.height;
        Ok(Terminal {
            backend,
            buffer: Buffer::empty(width, height),
            hidden_cursor: false,
            viewport: options.viewport,
        })
    }

    pub fn viewport_area(&self) -> Rect {
        self.viewport.area
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn viewport_scroll(&mut self, x_step: u16, y_step: u16) -> io::Result<()> {
        self.viewport.area.x += x_step;
        self.viewport.area.y += y_step;
        self.clear()
    }

    pub fn resize_buffer(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height)
    }

    /// Queries the backend for its viewport size and resizes frontend viewport size
    /// if it doesn't match.
    fn autoresize(&mut self) -> io::Result<()> {
        if self.viewport.resize_behavior == ResizeBehavior::Auto {
            let (b_width, b_height) = self.backend.dimensions()?;
            if self.backend.size()? != self.viewport.area.size() {
                self.viewport.area.width = b_width;
                self.viewport.area.height = b_height;
                if self.buffer.cells.len() < self.backend.size()? {
                    self.buffer.resize(b_width, b_height)
                }
                self.clear()?
            }
        }
        Ok(())
    }

    /// Clears buffer and backend.
    fn clear(&mut self) -> io::Result<()> {
        self.buffer.clear();
        self.backend.clear()
    }

    fn clear_buffer_region(&mut self, area: Rect) {
        self.buffer.clear_region(area);
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = Frame { terminal: self };
        f(&mut frame);

        self.backend
            .draw(self.buffer.get_region(self.viewport_area()).iter())?;

        Ok(CompletedFrame {
            buffer: &self.buffer,
            area: self.viewport.area,
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
        self.backend.set_cursor(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
pub struct Viewport {
    area: Rect,
    resize_behavior: ResizeBehavior,
}

impl Viewport {
    /// UNSTABLE
    pub fn fixed(width: u16, height: u16) -> Viewport {
        Viewport {
            area: Rect::new(0, 0, width, height),
            resize_behavior: ResizeBehavior::Fixed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
enum ResizeBehavior {
    Fixed,
    Auto,
}
