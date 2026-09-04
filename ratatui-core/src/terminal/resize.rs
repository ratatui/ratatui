use crate::backend::{Backend, ClearType};
use crate::layout::Rect;
use crate::terminal::inline::compute_inline_size;
use crate::terminal::{Terminal, Viewport};

impl<B: Backend> Terminal<B> {
    /// Updates the Terminal so that internal buffers match the requested area.
    ///
    /// This updates the buffer size used for rendering and clears the affected viewport so the
    /// next [`Terminal::draw`] / [`Terminal::try_draw`] paints into a consistent area.
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

        // Clear the screen on horizontal shrink to avoid line wrapping issues.
        //
        // Inline viewports are excluded: an inline viewport only owns the rows
        // from its origin down. The rows above it were written by
        // `insert_before`, which keeps no copy of them, so the application can
        // never repaint them. A full-screen erase either destroys that output
        // or, on terminals that move erased content into scrollback (Windows
        // Terminal, conhost), pushes a copy of the viewport into scrollback on
        // every resize event. The `clear_viewport` call below already erases
        // from the recomputed origin to the bottom of the screen, which covers
        // every row the inline viewport can legitimately own.
        if next_area.width < self.viewport_area.width {
            match self.viewport {
                Viewport::Inline(_) => {}
                Viewport::Fullscreen => next_area.y = 0,
                Viewport::Fixed(_) => {
                    next_area.y = 0;
                    self.backend.clear_region(ClearType::All)?;
                }
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
    use alloc::vec::Vec;

    use rstest::rstest;

    use crate::backend::{Backend, ClearType, TestBackend, WindowSize};
    use crate::buffer::Buffer;
    use crate::layout::{Position, Rect, Size};
    use crate::style::Style;
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[derive(Debug, Default)]
    struct ClearBackend(Vec<ClearType>);

    impl Backend for ClearBackend {
        type Error = core::convert::Infallible;

        fn draw<'a, I>(&mut self, _content: I) -> Result<(), Self::Error>
        where
            I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
        {
            Ok(())
        }

        fn hide_cursor(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn show_cursor(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
            Ok(Position::ORIGIN)
        }

        fn set_cursor_position<P: Into<Position>>(
            &mut self,
            _position: P,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn clear(&mut self) -> Result<(), Self::Error> {
            self.clear_region(ClearType::All)
        }

        fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
            self.0.push(clear_type);
            Ok(())
        }

        fn size(&self) -> Result<Size, Self::Error> {
            Ok(Size::new(80, 24))
        }

        fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
            Ok(WindowSize {
                columns_rows: self.size()?,
                pixels: Size::default(),
            })
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_up(
            &mut self,
            _region: core::ops::Range<u16>,
            _line_count: u16,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_down(
            &mut self,
            _region: core::ops::Range<u16>,
            _line_count: u16,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn clears_on_horizontal_shrink(viewport: Viewport) -> Vec<ClearType> {
        let backend = ClearBackend::default();
        let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport }).unwrap();
        terminal.backend_mut().0.clear();
        terminal.resize(Rect::new(0, 0, 40, 24)).unwrap();
        terminal.backend().0.clone()
    }

    #[rstest]
    #[case::fullscreen(Viewport::Fullscreen, &[ClearType::All])]
    #[case::inline(
        Viewport::Inline(5),
        &[ClearType::AfterCursor]
    )]
    #[case::fixed(
        Viewport::Fixed(Rect::new(0, 0, 80, 24)),
        &[ClearType::All]
    )]
    fn resize_horizontal_shrink_clears_expected_regions(
        #[case] viewport: Viewport,
        #[case] expected: &[ClearType],
    ) {
        assert_eq!(clears_on_horizontal_shrink(viewport), expected);
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

    // An inline viewport does not own the rows above its origin: they were
    // written by `insert_before`, which keeps no copy of them, so the
    // application cannot repaint them. A horizontal shrink must not erase them.
    #[test]
    fn resize_inline_horizontal_shrink_keeps_rows_above_the_viewport() {
        let mut backend = TestBackend::new(6, 4);
        backend
            .set_cursor_position(Position { x: 0, y: 0 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(1),
            },
        )
        .unwrap();

        for text in ["one", "two"] {
            terminal
                .insert_before(1, |buf| {
                    buf.set_string(0, 0, text, Style::default());
                })
                .unwrap();
        }
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame
                    .buffer_mut()
                    .set_string(area.x, area.y, "live", Style::default());
            })
            .unwrap();
        terminal
            .backend()
            .assert_buffer_lines(["one   ", "two   ", "live  ", "      "]);

        // A real backend leaves the cursor just past the last painted cell;
        // `TestBackend` only moves it on an explicit cursor command, so place it
        // where the viewport was drawn. `resize` anchors the new origin to it.
        terminal
            .backend_mut()
            .set_cursor_position(Position { x: 4, y: 2 })
            .unwrap();

        // Narrow the terminal. The backend buffer is left at its original width
        // so the assertions below read the same columns as the ones above;
        // `resize_inline_resets_the_buffers_on_horizontal_shrink` does the same.
        terminal.resize(Rect::new(0, 0, 5, 4)).unwrap();

        assert_eq!(terminal.viewport_area, Rect::new(0, 2, 5, 1));
        terminal
            .backend()
            .assert_buffer_lines(["one   ", "two   ", "      ", "      "]);
    }

    // This tests for the case where the new width is smaller than the old
    // width. The internal buffers are reset so the next draw repaints the
    // viewport into a consistent area.
    #[test]
    fn resize_inline_resets_the_buffers_on_horizontal_shrink() {
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

        let _ = terminal.resize(new_area);
        assert_eq!(terminal.viewport_area, new_area);
        let all_clear = terminal
            .current_buffer_mut()
            .content()
            .iter()
            .all(|cell| cell == &crate::buffer::Cell::EMPTY);
        assert!(all_clear, "not all buffer cells are empty");
    }
}
