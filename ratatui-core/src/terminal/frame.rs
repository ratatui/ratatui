use crate::buffer::Buffer;
use crate::layout::{Position, Rect};
use crate::widgets::{StatefulWidget, Widget};

/// A consistent view into the terminal state for rendering a single frame.
///
/// You usually get a `Frame` from the closure argument of [`Terminal::draw`] /
/// [`Terminal::try_draw`]. For manual rendering, use [`Terminal::get_frame`].
///
/// A `Frame` is used to render widgets into Ratatui's current buffer and request the cursor state
/// for the end of the render pass.
///
/// The changes drawn to the frame are applied only to the current [`Buffer`]. After the closure
/// returns, the current buffer is compared to the previous buffer and only the changed cells are
/// sent to the backend. This avoids drawing redundant cells.
///
/// [`Buffer`]: crate::buffer::Buffer
/// [`Terminal::draw`]: crate::terminal::Terminal::draw
/// [`Terminal::try_draw`]: crate::terminal::Terminal::try_draw
#[derive(Debug, Hash)]
pub struct Frame<'a> {
    /// Where should the cursor be after drawing this frame?
    ///
    /// If `None`, the cursor is hidden at the end of the render pass. If `Some((x, y))`, the
    /// cursor is shown and placed at `(x, y)` after the frame's buffer diff has been applied to
    /// the backend.
    pub(crate) cursor_position: Option<Position>,

    /// The area of the viewport
    pub(crate) viewport_area: Rect,

    /// The buffer that is used to draw the current frame
    pub(crate) buffer: &'a mut Buffer,

    /// The frame count indicating the sequence number of this frame.
    pub(crate) count: usize,
}

/// `CompletedFrame` represents the state of the terminal after the last successful
/// [`Terminal::draw`] / [`Terminal::try_draw`] render pass has been applied. Therefore, it is only
/// valid until the next successful draw call.
///
/// This lifetime follows Ratatui's double-buffering model: the next render pass swaps buffers via
/// [`Terminal::swap_buffers`], so the previously completed buffer is no longer the current output.
///
/// [`Terminal::draw`]: crate::terminal::Terminal::draw
/// [`Terminal::swap_buffers`]: crate::terminal::Terminal::swap_buffers
/// [`Terminal::try_draw`]: crate::terminal::Terminal::try_draw
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CompletedFrame<'a> {
    /// The buffer that was used to draw the last frame.
    pub buffer: &'a Buffer,
    /// The size of the last frame.
    pub area: Rect,
    /// The frame count indicating the sequence number of this frame.
    pub count: usize,
}

impl Frame<'_> {
    /// Returns the area of the current frame.
    ///
    /// This is guaranteed not to change during rendering, so may be called multiple times.
    ///
    /// If your app listens for a resize event from the backend, ignore that event's dimensions for
    /// calculations performed during the current render callback and use this value instead. It is
    /// the area of the buffer that is actually being rendered for this pass.
    pub const fn area(&self) -> Rect {
        self.viewport_area
    }

    /// Returns the area of the current frame.
    ///
    /// This is guaranteed not to change during rendering, so may be called multiple times.
    ///
    /// If your app listens for a resize event from the backend, ignore that event's dimensions for
    /// calculations performed during the current render callback and use this value instead. It is
    /// the area of the buffer that is actually being rendered for this pass.
    #[deprecated = "use `area()` instead"]
    pub const fn size(&self) -> Rect {
        self.viewport_area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// Rendering writes directly into the current frame buffer. If multiple widgets cover the same
    /// cells, later renders win for those cells.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// use ratatui_core::layout::Rect;
    ///
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_widget("Hello", area);
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
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
    /// Like [`Frame::render_widget`], this writes directly into the current frame buffer. The
    /// widget owns how it interprets and mutates the provided state.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_core::{backend::TestBackend, buffer::Buffer, layout::Rect, terminal::Terminal};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// use ratatui_core::widgets::StatefulWidget;
    ///
    /// struct DemoWidget;
    ///
    /// impl StatefulWidget for DemoWidget {
    ///     type State = bool;
    ///
    ///     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
    ///         let symbol = if *state { "Y" } else { "N" };
    ///         buf[(area.x, area.y)].set_symbol(symbol);
    ///     }
    /// }
    ///
    /// let mut state = true;
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_stateful_widget(DemoWidget, area, &mut state);
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, self.buffer, state);
    }

    /// After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`
    /// coordinates. If this method is not called, the cursor will be hidden.
    ///
    /// The cursor is applied after Ratatui flushes the frame's buffer diff to the backend.
    ///
    /// Note that this will interfere with calls to [`Terminal::hide_cursor`],
    /// [`Terminal::show_cursor`], and [`Terminal::set_cursor_position`]. Pick one of the APIs and
    /// stick with it.
    ///
    /// [`Terminal::hide_cursor`]: crate::terminal::Terminal::hide_cursor
    /// [`Terminal::show_cursor`]: crate::terminal::Terminal::show_cursor
    /// [`Terminal::set_cursor_position`]: crate::terminal::Terminal::set_cursor_position
    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) {
        self.cursor_position = Some(position.into());
    }

    /// After this frame is rendered, make the cursor visible and put it at the specified `(x, y)`
    /// coordinates. If this method is not called, the cursor will be hidden.
    ///
    /// Note that this will interfere with calls to [`Terminal::hide_cursor`],
    /// [`Terminal::show_cursor`], and [`Terminal::set_cursor_position`]. Pick one of the APIs and
    /// stick with it.
    ///
    /// [`Terminal::hide_cursor`]: crate::terminal::Terminal::hide_cursor
    /// [`Terminal::show_cursor`]: crate::terminal::Terminal::show_cursor
    /// [`Terminal::set_cursor_position`]: crate::terminal::Terminal::set_cursor_position
    #[deprecated = "use `set_cursor_position((x, y))` instead which takes `impl Into<Position>`"]
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.set_cursor_position(Position { x, y });
    }

    /// Gets the buffer that this `Frame` draws into as a mutable reference.
    ///
    /// This is an escape hatch for direct buffer manipulation. Prefer the widget rendering methods
    /// when possible so layout and rendering intent stay visible at the call site.
    ///
    /// Changes written here are not visible on the backend until the render pass is applied by
    /// [`Terminal::flush`] or a full [`Terminal::draw`] / [`Terminal::try_draw`] pass.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(5, 1);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// frame.buffer_mut()[(0, 0)].set_symbol("h");
    /// ```
    pub const fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    /// Returns the current frame count.
    ///
    /// This method provides access to the frame count, which is a sequence number indicating
    /// how many frames have been rendered up to (but not including) this one. It can be used
    /// for purposes such as animation, performance tracking, or debugging.
    ///
    /// Each time a frame has been rendered, this count is incremented,
    /// providing a consistent way to reference the order and number of frames processed by the
    /// terminal. When count reaches its maximum value (`usize::MAX`), it wraps around to zero.
    ///
    /// This count is particularly useful when dealing with dynamic content or animations where the
    /// state of the display changes over time. By tracking the frame count, developers can
    /// synchronize updates or changes to the content with the rendering process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// let current_count = frame.count();
    /// println!("Current frame count: {}", current_count);
    /// ```
    pub const fn count(&self) -> usize {
        self.count
    }
}
