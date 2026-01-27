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

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, TestBackend};
    use crate::layout::Position;
    use crate::terminal::Terminal;

    #[test]
    fn hide_cursor_updates_terminal_state() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.hide_cursor().unwrap();

        assert!(terminal.hidden_cursor);
        assert!(!terminal.backend().cursor_visible());
    }

    #[test]
    fn show_cursor_updates_terminal_state() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.hide_cursor().unwrap();
        terminal.show_cursor().unwrap();

        assert!(!terminal.hidden_cursor);
        assert!(terminal.backend().cursor_visible());
    }

    #[test]
    fn set_cursor_position_updates_backend_and_tracking() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.set_cursor_position((3, 4)).unwrap();

        assert_eq!(terminal.last_known_cursor_pos, Position { x: 3, y: 4 });
        terminal
            .backend_mut()
            .assert_cursor_position(Position { x: 3, y: 4 });
    }

    #[test]
    fn get_cursor_position_queries_backend() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .backend_mut()
            .set_cursor_position(Position { x: 7, y: 2 })
            .unwrap();

        assert_eq!(
            terminal.get_cursor_position().unwrap(),
            Position { x: 7, y: 2 }
        );
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_cursor_wrappers_delegate_to_position_apis() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.set_cursor(4, 1).unwrap();

        assert_eq!(terminal.get_cursor().unwrap(), (4, 1));
        assert_eq!(terminal.last_known_cursor_pos, Position { x: 4, y: 1 });
        terminal
            .backend_mut()
            .assert_cursor_position(Position { x: 4, y: 1 });
    }
}
