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
