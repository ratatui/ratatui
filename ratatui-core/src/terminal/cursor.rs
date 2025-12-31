use crate::backend::Backend;
use crate::layout::Position;
use crate::terminal::Terminal;

impl<B: Backend> Terminal<B> {
    /// Hides the cursor.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. Mixing the APIs can lead to surprising results.
    ///
    /// [`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    pub fn hide_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    /// Shows the cursor.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. Mixing the APIs can lead to surprising results.
    ///
    /// [`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    pub fn show_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    /// Gets the current cursor position.
    ///
    /// This is the position of the cursor after the last draw call and is returned as a tuple of
    /// `(x, y)` coordinates.
    #[deprecated = "use `get_cursor_position()` instead which returns `Result<Position>`"]
    pub fn get_cursor(&mut self) -> Result<(u16, u16), B::Error> {
        let Position { x, y } = self.get_cursor_position()?;
        Ok((x, y))
    }

    /// Sets the cursor position.
    #[deprecated = "use `set_cursor_position((x, y))` instead which takes `impl Into<Position>`"]
    pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), B::Error> {
        self.set_cursor_position(Position { x, y })
    }

    /// Gets the current cursor position.
    ///
    /// This queries the backend for the current cursor position.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. For direct control, see [`Terminal::set_cursor_position`].
    ///
    /// [`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    pub fn get_cursor_position(&mut self) -> Result<Position, B::Error> {
        self.backend.get_cursor_position()
    }

    /// Sets the cursor position.
    ///
    /// This updates the backend cursor and Ratatui's internal cursor tracking. Inline viewports
    /// use that tracking when recomputing the viewport on resize.
    ///
    /// When using [`Terminal::draw`], consider using [`Frame::set_cursor_position`] instead so the
    /// cursor is updated as part of the normal rendering flow.
    ///
    /// [`Frame::set_cursor_position`]: crate::terminal::Frame::set_cursor_position
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), B::Error> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        Ok(())
    }
}
