use crate::backend::Backend;
use crate::layout::Size;
use crate::terminal::Terminal;

impl<B: Backend> Terminal<B> {
    /// Returns a shared reference to the backend.
    ///
    /// This is primarily useful for backend-specific inspection in tests (e.g. reading
    /// [`TestBackend`]'s buffer). Most applications should interact with the terminal via
    /// [`Terminal::draw`] rather than calling backend methods directly.
    ///
    /// [`TestBackend`]: crate::backend::TestBackend
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns a mutable reference to the backend.
    ///
    /// This is an advanced escape hatch. Mutating the backend directly can desynchronize Ratatui's
    /// internal buffers from what's on-screen; if you do this, you may need to call
    /// [`Terminal::clear`] to force a full redraw.
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Queries the real size of the backend.
    ///
    /// This returns the size of the underlying terminal. The current renderable area depends on
    /// the configured [`Viewport`]; use [`Frame::area`] inside [`Terminal::draw`] if you want the
    /// area you should render into.
    ///
    /// [`Frame::area`]: crate::terminal::Frame::area
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
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
