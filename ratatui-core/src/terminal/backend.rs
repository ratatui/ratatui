use crate::backend::Backend;
use crate::layout::Size;
use crate::terminal::Terminal;

impl<B: Backend> Terminal<B> {
    /// Returns a shared reference to the backend.
    ///
    /// This is primarily useful for backend-specific inspection in tests (e.g. reading
    /// [`TestBackend`]'s buffer) or for backend-specific APIs that Ratatui does not model.
    ///
    /// Reading from the backend does not desynchronize Ratatui, but values observed here may lag
    /// behind the current render callback because Ratatui does not apply a frame to the backend
    /// until the end of [`Terminal::draw`] / [`Terminal::try_draw`].
    ///
    /// [`TestBackend`]: crate::backend::TestBackend
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns a mutable reference to the backend.
    ///
    /// This is an advanced escape hatch. Normal applications should render through
    /// [`Terminal::draw`] / [`Terminal::try_draw`] instead of mutating the backend directly.
    ///
    /// Use this when integrating with backend-specific APIs that Ratatui does not model, or when
    /// tests need direct control over backend state.
    ///
    /// Mutating the backend directly can desynchronize Ratatui's internal buffers, cursor
    /// tracking, or viewport assumptions from what's on-screen. If you do this, call
    /// [`Terminal::clear`] or perform a full draw pass before relying on Ratatui's view of the
    /// terminal again.
    ///
    /// [`Terminal::clear`]: crate::terminal::Terminal::clear
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    /// [`Terminal::try_draw`]: crate::terminal::Terminal::try_draw
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Queries the real size of the backend.
    ///
    /// This returns the backend's current terminal size and does not update Ratatui's internal
    /// viewport bookkeeping by itself. The current renderable area depends on the configured
    /// [`Viewport`]; use [`Frame::area`] inside [`Terminal::draw`] / [`Terminal::try_draw`] if you
    /// want the area you should render into for the current pass.
    ///
    /// To make Ratatui observe backend size changes for fullscreen or inline viewports, see
    /// [`Terminal::autoresize`].
    ///
    /// [`Frame::area`]: crate::terminal::Frame::area
    /// [`Terminal::autoresize`]: crate::terminal::Terminal::autoresize
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    /// [`Terminal::try_draw`]: crate::terminal::Terminal::try_draw
    /// [`Viewport`]: crate::terminal::Viewport
    pub fn size(&self) -> Result<Size, B::Error> {
        self.backend.size()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::TestBackend;
    use crate::layout::{Position, Size};
    use crate::terminal::Terminal;

    #[test]
    fn backend_returns_shared_reference() {
        let backend = TestBackend::new(3, 2);
        let terminal = Terminal::new(backend).unwrap();

        assert_eq!(terminal.backend().cursor_position(), Position::ORIGIN);
    }

    #[test]
    fn backend_mut_allows_mutating_backend_state() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.backend_mut().resize(4, 3);

        assert_eq!(terminal.size().unwrap(), Size::new(4, 3));
        terminal
            .backend()
            .assert_buffer_lines(["    ", "    ", "    "]);
    }

    #[test]
    fn size_queries_underlying_backend_size() {
        let mut backend = TestBackend::new(3, 2);
        backend.resize(4, 3);
        let terminal = Terminal::new(backend).unwrap();

        assert_eq!(terminal.size().unwrap(), Size::new(4, 3));
    }
}
