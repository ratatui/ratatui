use crate::backend::{Backend, ClearType};
use crate::layout::Rect;
use crate::terminal::inline::compute_inline_size;
use crate::terminal::{Terminal, Viewport};

impl<B: Backend> Terminal<B> {
    /// Updates the Terminal so that internal buffers match the requested area.
    ///
    /// This updates the buffer size used for rendering and triggers a full clear so the next
    /// [`Terminal::draw`] / [`Terminal::try_draw`] paints into a consistent area.
    ///
    /// When the viewport is [`Viewport::Inline`], the `area` argument is treated as the new
    /// terminal size and the viewport origin is recomputed relative to the current cursor position.
    /// Ratatui attempts to keep the cursor at the same relative row within the viewport across
    /// resizes.
    ///
    /// See also: [`Terminal::autoresize`] (automatic resizing during [`Terminal::draw`] /
    /// [`Terminal::try_draw`]).
    ///
    /// For [`Viewport::Fixed`] and [`Viewport::Fullscreen`], `area` becomes the new viewport area.
    /// For [`Viewport::Inline`], `area` is interpreted as the backend's new terminal size and the
    /// viewport origin may move to preserve the cursor's relative row within the inline UI.
    pub fn resize(&mut self, area: Rect) -> Result<(), B::Error> {
        let (mut next_area, cursor_to_restore) = match self.viewport {
            Viewport::Inline(height) => {
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .y
                    .saturating_sub(self.viewport_area.top());
                let (next_area, cursor_position) = compute_inline_size(
                    &mut self.backend,
                    height,
                    area.as_size(),
                    offset_in_previous_viewport,
                )?;
                (next_area, Some(cursor_position))
            }
            Viewport::Fixed(_) | Viewport::Fullscreen => (area, None),
        };

        // clear screen on horizontal shrink to avoid line wrapping issues
        if next_area.width < self.viewport_area.width {
            next_area.y = 0;
            // `clear_viewport` below already clears everything for `Fullscreen`.
            if !matches!(self.viewport, Viewport::Fullscreen) {
                self.backend.clear_region(ClearType::All)?;
            }
        }

        self.set_viewport_area(next_area);
        self.clear_viewport()?;
        if let Some(cursor_position) = cursor_to_restore {
            self.backend.set_cursor_position(cursor_position)?;
        }

        self.last_known_area = area;
        Ok(())
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    ///
    /// This is called automatically during [`Terminal::draw`] / [`Terminal::try_draw`] for
    /// fullscreen and inline viewports. Fixed viewports are not automatically resized.
    ///
    /// If the size changed, this calls [`Terminal::resize`] and therefore clears the affected
    /// region before the next frame is rendered.
    pub fn autoresize(&mut self) -> Result<(), B::Error> {
        // fixed viewports do not get autoresized
        if matches!(self.viewport, Viewport::Fullscreen | Viewport::Inline(_)) {
            let area = self.size()?.into();
            if area != self.last_known_area {
                self.resize(area)?;
            }
        }
        Ok(())
    }

    /// Resize internal buffers and update the current viewport area.
    ///
    /// This is an internal helper used by [`Terminal::with_options`] and [`Terminal::resize`].
    pub(crate) fn set_viewport_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::backend::{Backend, ClearType, TestBackend, WindowSize};
    use crate::buffer::{Buffer, Cell};
    use crate::layout::{Position, Rect, Size};
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    /// A [`TestBackend`] that records clear commands.
    ///
    /// [`TestBackend`] implements `clear_region(ClearType::All)` as a buffer reset, so one clear
    /// and two are indistinguishable in its buffer.
    #[derive(Debug)]
    struct CountingTestBackend {
        inner: TestBackend,
        clears: Vec<ClearType>,
    }

    impl CountingTestBackend {
        fn new(width: u16, height: u16) -> Self {
            Self {
                inner: TestBackend::new(width, height),
                clears: Vec::new(),
            }
        }
    }

    impl Backend for CountingTestBackend {
        type Error = core::convert::Infallible;

        fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
        where
            I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
        {
            self.inner.draw(content)
        }

        fn append_lines(&mut self, n: u16) -> Result<(), Self::Error> {
            self.inner.append_lines(n)
        }

        fn hide_cursor(&mut self) -> Result<(), Self::Error> {
            self.inner.hide_cursor()
        }

        fn show_cursor(&mut self) -> Result<(), Self::Error> {
            self.inner.show_cursor()
        }

        fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
            self.inner.get_cursor_position()
        }

        fn set_cursor_position<P: Into<Position>>(
            &mut self,
            position: P,
        ) -> Result<(), Self::Error> {
            self.inner.set_cursor_position(position)
        }

        fn clear(&mut self) -> Result<(), Self::Error> {
            self.clears.push(ClearType::All);
            self.inner.clear()
        }

        fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
            self.clears.push(clear_type);
            self.inner.clear_region(clear_type)
        }

        fn size(&self) -> Result<crate::layout::Size, Self::Error> {
            self.inner.size()
        }

        fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
            self.inner.window_size()
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.inner.flush()
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_up(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner.scroll_region_up(region, line_count)
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_down(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner.scroll_region_down(region, line_count)
        }
    }

    fn clears_on_horizontal_shrink(viewport: Viewport) -> Vec<ClearType> {
        let backend = CountingTestBackend::new(80, 24);
        let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport }).unwrap();
        terminal.backend_mut().clears.clear();
        terminal.resize(Rect::new(0, 0, 40, 24)).unwrap();
        terminal.backend().clears.clone()
    }

    /// The resize assertions above only mean anything if the wrapper is a faithful pass-through,
    /// so exercise the methods those tests do not reach.
    #[test]
    fn counting_backend_delegates_everything_to_the_inner_backend() {
        let mut backend = CountingTestBackend::new(4, 2);

        backend.draw([(0, 0, &Cell::new("x"))].into_iter()).unwrap();
        backend.set_cursor_position(Position::new(1, 1)).unwrap();
        assert_eq!(backend.get_cursor_position().unwrap(), Position::new(1, 1));
        assert_eq!(backend.size().unwrap(), Size::new(4, 2));
        assert_eq!(backend.window_size().unwrap().columns_rows, Size::new(4, 2));
        backend.hide_cursor().unwrap();
        backend.show_cursor().unwrap();
        backend.append_lines(1).unwrap();
        backend.flush().unwrap();
        #[cfg(feature = "scrolling-regions")]
        {
            backend.scroll_region_up(0..2, 1).unwrap();
            backend.scroll_region_down(0..2, 1).unwrap();
        }

        backend.clear().unwrap();
        backend.clear_region(ClearType::CurrentLine).unwrap();
        assert_eq!(backend.clears, [ClearType::All, ClearType::CurrentLine]);
    }

    #[test]
    fn resize_fullscreen_clears_the_screen_once_on_horizontal_shrink() {
        assert_eq!(
            clears_on_horizontal_shrink(Viewport::Fullscreen),
            vec![ClearType::All]
        );
    }

    #[test]
    fn resize_inline_still_clears_above_the_viewport_on_horizontal_shrink() {
        // Both are needed: `clear_viewport` only reaches from the viewport origin down.
        assert_eq!(
            clears_on_horizontal_shrink(Viewport::Inline(5)),
            vec![ClearType::All, ClearType::AfterCursor]
        );
    }

    #[test]
    fn resize_fixed_still_clears_outside_the_viewport_on_horizontal_shrink() {
        // The one clear here is the explicit one: `clear_fixed_viewport` redraws cells instead.
        assert_eq!(
            clears_on_horizontal_shrink(Viewport::Fixed(Rect::new(0, 0, 80, 24))),
            vec![ClearType::All]
        );
    }

    #[test]
    fn resize_fullscreen_updates_viewport_and_buffer_areas() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.backend_mut().resize(4, 3);
        let new_area = Rect::new(0, 0, 4, 3);
        terminal.resize(new_area).unwrap();

        assert_eq!(terminal.viewport_area, new_area);
        assert_eq!(terminal.last_known_area, new_area);
        assert_eq!(terminal.buffers[terminal.current].area, new_area);
        assert_eq!(terminal.buffers[1 - terminal.current].area, new_area);
    }

    #[test]
    fn resize_fullscreen_triggers_clear_and_resets_back_buffer() {
        // This test is specifically about the side effects of `resize`:
        // - it calls `clear` to force a full redraw
        // - it resets the "previous" buffer
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        // Put visible content on the backend so we can tell whether a clear happened.
        {
            let frame = terminal.get_frame();
            frame.buffer[(0, 0)].set_symbol("x");
        }
        terminal.flush().unwrap();
        terminal.backend().assert_buffer_lines(["x  ", "   "]);

        terminal.backend_mut().resize(4, 3);
        let new_area = Rect::new(0, 0, 4, 3);
        terminal.resize(new_area).unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["    ", "    ", "    "]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(new_area)
        );
    }

    #[test]
    fn autoresize_fullscreen_uses_backend_size_when_changed() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        {
            let frame = terminal.get_frame();
            frame.buffer[(0, 0)].set_symbol("x");
        }
        terminal.flush().unwrap();

        terminal.backend_mut().resize(4, 3);
        terminal.autoresize().unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 4, 3));
        assert_eq!(terminal.last_known_area, Rect::new(0, 0, 4, 3));
        terminal
            .backend()
            .assert_buffer_lines(["    ", "    ", "    "]);
    }

    #[test]
    fn autoresize_fixed_does_not_change_viewport() {
        let backend = TestBackend::with_lines(["xxx", "yyy"]);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(1, 0, 2, 2)),
            },
        )
        .unwrap();

        terminal.autoresize().unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(1, 0, 2, 2));
        assert_eq!(terminal.last_known_area, Rect::new(1, 0, 2, 2));
        terminal.backend().assert_buffer_lines(["xxx", "yyy"]);
    }

    #[test]
    fn resize_fixed_changes_viewport_area_and_buffer_sizes() {
        let backend = TestBackend::new(5, 3);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(1, 1, 2, 1)),
            },
        )
        .unwrap();

        terminal.resize(Rect::new(0, 0, 3, 2)).unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 3, 2));
        assert_eq!(terminal.last_known_area, Rect::new(0, 0, 3, 2));
        assert_eq!(
            terminal.buffers[terminal.current].area,
            terminal.viewport_area
        );
        assert_eq!(
            terminal.buffers[1 - terminal.current].area,
            terminal.viewport_area
        );
    }

    #[test]
    fn resize_inline_recomputes_origin_using_previous_cursor_offset() {
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 4 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(4),
            },
        )
        .unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 4, 10, 4));

        // Characterization test:
        // This test simulates a terminal resize (increasing the terminal height) while an inline
        // viewport is active. The key behavior being exercised is that the viewport remains
        // anchored to the backend cursor row and preserves the cursor's relative offset within the
        // previous viewport.
        //
        // For inline viewports, `Terminal::resize(area)` interprets `area` as the *new terminal
        // size*, then recomputes the viewport origin based on:
        // - the backend cursor position at the time of the call
        // - the cursor offset within the *previous* viewport (`last_known_cursor_pos -
        //   viewport_top`)
        //
        // This means `resize(Rect { .. })` can update `viewport_area.y` even when the passed-in
        // `area.y` is 0, because `viewport_area` is anchored to the cursor row, not the terminal
        // origin.
        terminal.last_known_cursor_pos = Position { x: 0, y: 5 };
        terminal
            .backend_mut()
            .set_cursor_position(Position { x: 0, y: 6 })
            .unwrap();

        terminal.backend_mut().resize(10, 12);
        let new_terminal_area = Rect::new(0, 0, 10, 12);
        terminal.resize(new_terminal_area).unwrap();

        // Previous viewport top was y=4, and last_known_cursor_pos was y=5, so the cursor offset
        // within the viewport is 1 row. At the time of resize the backend cursor is at y=6, so the
        // new viewport top becomes 6 - 1 = 5.
        assert_eq!(terminal.viewport_area, Rect::new(0, 5, 10, 4));
        assert_eq!(terminal.last_known_area, new_terminal_area);
    }

    #[test]
    fn resize_inline_clamps_height_to_terminal_height() {
        // Characterization test:
        // This test simulates a terminal resize that *reduces* the terminal height. Inline
        // viewports clamp their height to the new terminal size so the viewport remains fully
        // visible.
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 0 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(10),
            },
        )
        .unwrap();

        terminal.backend_mut().resize(10, 3);
        terminal.resize(Rect::new(0, 0, 10, 3)).unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 10, 3));
    }

    #[test]
    fn resize_inline_preserves_backend_cursor_across_repeated_resizes() {
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 4 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(4),
            },
        )
        .unwrap();

        terminal.last_known_cursor_pos = Position { x: 0, y: 5 };
        terminal
            .backend_mut()
            .set_cursor_position(Position { x: 0, y: 6 })
            .unwrap();

        terminal.resize(Rect::new(0, 0, 10, 12)).unwrap();
        assert_eq!(terminal.viewport_area, Rect::new(0, 5, 10, 4));
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 0, y: 6 }
        );

        terminal.resize(Rect::new(0, 0, 10, 14)).unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 6, 10, 4));
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 0, y: 6 }
        );
    }

    // This tests for the case where the new width is smaller than the old
    // width. The screen should be cleared completely to avoid rendering
    // glitches caused by line wrap.
    #[test]
    fn resize_inline_clears_screen_on_horizontal_shrink() {
        let mut backend = TestBackend::with_lines(["0000", "1111"]);
        backend
            .set_cursor_position(Position { x: 0, y: 0 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(2),
            },
        )
        .unwrap();

        let old_area = terminal.backend().buffer().area;
        let new_area = Rect {
            width: old_area.width - 1,
            ..old_area
        };

        terminal.resize(new_area);
        assert_eq!(terminal.viewport_area, new_area);
        let all_clear = terminal
            .current_buffer_mut()
            .content()
            .iter()
            .all(|cell| cell == &crate::buffer::Cell::EMPTY);
        assert!(all_clear, "not all buffer cells are empty");
    }
}
