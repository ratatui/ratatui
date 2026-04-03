use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::Position;
use crate::terminal::inline::compute_inline_size;
use crate::terminal::{Terminal, TerminalOptions, Viewport};

impl<B: Backend> Terminal<B> {
    /// Creates a new [`Terminal`] with the given [`Backend`] with a full screen viewport.
    ///
    /// This is a convenience for [`Terminal::with_options`] with [`Viewport::Fullscreen`].
    ///
    /// After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
    /// render your UI.
    ///
    /// Note that unlike [`ratatui::init`], this does not install a panic hook, so it is
    /// recommended to do that manually when using this function, otherwise any panic messages will
    /// be printed to the alternate screen and the terminal may be left in an unusable state.
    ///
    /// See [how to set up panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/) and
    /// [`better-panic` example](https://ratatui.rs/recipes/apps/better-panic/) for more
    /// information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io::stdout;
    ///
    /// use ratatui::Terminal;
    /// use ratatui::backend::CrosstermBackend;
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let _terminal = Terminal::new(backend)?;
    ///
    /// // Optionally set up a panic hook to restore the terminal on panic.
    /// let old_hook = std::panic::take_hook();
    /// std::panic::set_hook(Box::new(move |info| {
    ///     ratatui::restore();
    ///     old_hook(info);
    /// }));
    /// # }
    /// # #[cfg(not(feature = "crossterm"))]
    /// # {
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(10, 10);
    /// # let _terminal = Terminal::new(backend)?;
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`ratatui::init`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
    pub fn new(backend: B) -> Result<Self, B::Error> {
        Self::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fullscreen,
            },
        )
    }

    /// Creates a new [`Terminal`] with the given [`Backend`] and [`TerminalOptions`].
    ///
    /// The viewport determines what area is exposed to widgets via [`Frame::area`]. See
    /// [`Viewport`] for an overview of the available modes.
    ///
    /// [`Frame::area`]: crate::terminal::Frame::area
    ///
    /// After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
    /// render your UI.
    ///
    /// Resize behavior depends on the selected viewport:
    ///
    /// - [`Viewport::Fullscreen`] and [`Viewport::Inline`] are automatically resized during
    ///   [`Terminal::draw`] (via [`Terminal::autoresize`]).
    /// - [`Viewport::Fixed`] is not automatically resized; call [`Terminal::resize`] if the region
    ///   should change.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io::stdout;
    ///
    /// use ratatui::backend::CrosstermBackend;
    /// use ratatui::layout::Rect;
    /// use ratatui::{Terminal, TerminalOptions, Viewport};
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    /// # }
    /// # #[cfg(not(feature = "crossterm"))]
    /// # {
    /// # use ratatui_core::{
    /// #     backend::TestBackend,
    /// #     layout::Rect,
    /// #     terminal::{Terminal, TerminalOptions, Viewport},
    /// # };
    /// # let backend = TestBackend::new(10, 10);
    /// # let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// # let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// When the viewport is [`Viewport::Inline`], Ratatui anchors the viewport to the current
    /// cursor row at initialization time (always starting at column 0). Ratatui may scroll the
    /// terminal to make enough room for the requested height so the viewport stays fully visible.
    pub fn with_options(mut backend: B, options: TerminalOptions) -> Result<Self, B::Error> {
        let area = match options.viewport {
            Viewport::Fullscreen | Viewport::Inline(_) => backend.size()?.into(),
            Viewport::Fixed(area) => area,
        };
        let (viewport_area, cursor_pos) = match options.viewport {
            Viewport::Fullscreen => (area, Position::ORIGIN),
            Viewport::Inline(height) => {
                compute_inline_size(&mut backend, height, area.as_size(), 0)?
            }
            Viewport::Fixed(area) => (area, area.as_position()),
        };
        Ok(Self {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
            viewport_area,
            last_known_area: area,
            last_known_cursor_pos: cursor_pos,
            frame_count: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, TestBackend};
    use crate::layout::{Position, Rect};
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[test]
    fn new_fullscreen_initializes_state() {
        let backend = TestBackend::new(10, 5);
        let terminal = Terminal::new(backend).unwrap();

        assert_eq!(terminal.viewport, Viewport::Fullscreen);
        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 10, 5));
        assert_eq!(terminal.last_known_area, Rect::new(0, 0, 10, 5));
        assert_eq!(terminal.last_known_cursor_pos, Position::ORIGIN);
        assert_eq!(terminal.current, 0);
        assert!(!terminal.hidden_cursor);
        assert_eq!(terminal.frame_count, 0);
        assert_eq!(terminal.buffers[0].area, terminal.viewport_area);
        assert_eq!(terminal.buffers[1].area, terminal.viewport_area);
    }

    #[test]
    fn with_options_fixed_uses_fixed_area() {
        let backend = TestBackend::new(10, 10);
        let viewport = Viewport::Fixed(Rect::new(2, 3, 5, 4));
        let terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: viewport.clone(),
            },
        )
        .unwrap();

        assert_eq!(terminal.viewport, viewport);
        assert_eq!(terminal.viewport_area, Rect::new(2, 3, 5, 4));
        assert_eq!(terminal.last_known_area, Rect::new(2, 3, 5, 4));
        assert_eq!(terminal.last_known_cursor_pos, Position { x: 2, y: 3 });
        assert_eq!(terminal.buffers[0].area, terminal.viewport_area);
        assert_eq!(terminal.buffers[1].area, terminal.viewport_area);
    }

    #[test]
    fn with_options_inline_anchors_to_cursor_when_space_available() {
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 3 })
            .unwrap();

        let terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(4),
            },
        )
        .unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 3, 10, 4));
        assert_eq!(terminal.last_known_cursor_pos, Position { x: 0, y: 3 });
    }

    #[test]
    fn with_options_inline_shifts_up_when_near_bottom() {
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 8 })
            .unwrap();

        let terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(4),
            },
        )
        .unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 6, 10, 4));
        assert_eq!(terminal.last_known_cursor_pos, Position { x: 0, y: 8 });
    }

    #[test]
    fn with_options_inline_clamps_height_to_terminal() {
        let mut backend = TestBackend::new(10, 3);
        backend
            .set_cursor_position(Position { x: 0, y: 0 })
            .unwrap();

        let terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(10),
            },
        )
        .unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 10, 3));
    }
}
