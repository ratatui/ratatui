use crate::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use std::io::{self, Error, ErrorKind};

/// Interface to the terminal backed by Termion
pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    buffer: Buffer,
    viewport_slit_constaints: Vec<Constraint>,
    viewport_split_direction: Direction,
    viewports: Vec<Viewport>,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if let Err(err) = self.show_cursor() {
            eprintln!("Failed to show the cursor: {}", err);
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Initializes a terminal with a buffer size the same as the backend size.
    /// Buffer cells are initialized with empty strings using
    /// default foreground and the background color.
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        // NOTE: Direction arbitrary for single viewport setup
        Self::new_split(
            backend,
            vec![Constraint::Percentage(100)],
            Direction::Horizontal,
        )
    }

    /// Terminal can be initialized with split viewports.
    /// Useful when parts of the buffer are to be individually scrolled,
    /// without re-rendering buffer at all.
    ///
    /// Indidivual viewport scrolls are then controlled with `split_viewport_scroll`.
    pub fn new_split(
        backend: B,
        constraints: Vec<Constraint>,
        direction: Direction,
    ) -> io::Result<Terminal<B>> {
        let (width, height) = backend.dimensions()?;
        let base_area = Rect {
            x: 0,
            y: 0,
            width,
            height,
        };
        Ok(Terminal {
            backend,
            buffer: Buffer::empty(width, height),
            viewports: Layout::default()
                .direction(direction.clone())
                .constraints(constraints.as_ref())
                .split(&base_area)
                .into_iter()
                .map(|region| Viewport {
                    region,
                    scroll: (0, 0),
                })
                .collect(),
            viewport_slit_constaints: constraints,
            viewport_split_direction: direction,
        })
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    /// Should be proceeded with a call to terminal.flush().
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
    /// terminal.render_widget(block, &area);
    /// ```
    pub fn render_widget<W: Widget>(&mut self, widget: W, area: &Rect) {
        widget.render(area, &mut self.buffer);
    }

    /// Render the widget on a given viewport.
    pub fn render_widget_on_viewport<W: Widget>(&mut self, widget: W, viewport_index: usize) {
        widget.render(&self.viewports[viewport_index].region, &mut self.buffer)
    }

    /// Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
    /// Should be proceeded with a call to terminal.flush().
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
    /// terminal.render_stateful_widget(list, &area, &mut state);
    /// ```
    pub fn render_stateful_widget<W: StatefulWidget>(
        &mut self,
        widget: W,
        area: &Rect,
        state: &mut W::State,
    ) {
        widget.render(area, &mut self.buffer, state);
    }

    pub fn render_stateful_widget_on_viewport<W: StatefulWidget>(
        &mut self,
        widget: W,
        state: &mut W::State,
        viewport_index: usize,
    ) {
        widget.render(
            &self.viewports[viewport_index].region,
            &mut self.buffer,
            state,
        );
    }

    pub fn viewport_areas(&self) -> Vec<&Rect> {
        self.viewports
            .iter()
            .map(|viewport| &viewport.region)
            .collect()
    }

    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Returns Ok(Ok()) if viewport did not overflow and buffer could be flushed.
    /// Returns Err() no overflow was detected by buffer flush failed.
    /// Returns Ok(Err()) if scroll overflows or overlaps overviewports.
    /// Nested Results allow for fairly simple silent fail of scroll overflows
    /// without disregarding the more serious flush errors with:
    /// `terminal.viewport_scroll(-10, -10)?.unwrap_or(())`.
    /*
        IMPROVEMENT: Introduce a separate ViewportOverflow struct for the error?
        Might require the introduction of the anyhow crate both internally and externally
        as the library would no longer only return io::Results.

        Might be a more Rust idiomatic way of handling the nested Results shenanigans.
    */
    pub fn viewport_scroll(&mut self, x_step: i16, y_step: i16) -> io::Result<io::Result<()>> {
        self.split_viewport_scroll(|_| (x_step, y_step))
    }

    /// Used in combination with Terminal::new_split() to individually scroll split viewports.
    /// Call is passed with a closure that returns the scroll for a given split viewport index.
    /// The viewport index represents the respective viewport created by constraints given in Terminal::new_split([Constraint]).
    pub fn split_viewport_scroll<F: Fn(usize) -> (i16, i16)>(
        &mut self,
        step_closure: F,
    ) -> io::Result<io::Result<()>> {
        let mut viewports_to_flush: ViewportToFlush = Vec::with_capacity(self.viewports.len());
        for index in 0..self.viewports.len() {
            let (x_step, y_step) = step_closure(index);
            // We don't want to re-flush a region which hasn't moved.
            if (x_step, y_step) == (0, 0) {
                continue;
            }

            let (x_scroll, y_scroll) = self.viewports[index].scroll;
            let new_scroll = (x_scroll + x_step, y_scroll + y_step);
            if let Err(err) = self.assert_viewport_within_buffer(index, new_scroll) {
                return Ok(Err(err));
            }
            // We don't want to update scroll and flush immediatedly as viewport overlap must first be checked.
            // This has be done on all at he same time, otherwise it's not possible to scroll multiple viewports
            // in unison.
            viewports_to_flush.push((index, new_scroll));
        }

        match self.assert_nonoverlapping_viewports(viewports_to_flush) {
            Ok(viewports_to_flush) => {
                for (index, new_scroll) in viewports_to_flush {
                    self.viewports[index].scroll = new_scroll;
                    self.flush_viewport_region(index)?;
                }
                Ok(Ok(()))
            }
            Err(err) => Ok(Err(err)),
        }
    }

    /// The absolute version of slip_viewport_scroll
    /// Useful if you are keeping track of the offset externally.
    pub fn split_viewport_offset<F: Fn(usize) -> (u16, u16)>(
        &mut self,
        offset_closure: F,
    ) -> io::Result<io::Result<()>> {
        let mut viewports_to_flush: ViewportToFlush = Vec::with_capacity(self.viewports.len());
        for index in 0..self.viewports.len() {
            let (x_offset, y_offset) = offset_closure(index);
            // We don't want to re-flush a region which hasn't moved.
            let curr_viewport = &self.viewports[index];
            let (curr_x_scroll, curr_y_scroll) = curr_viewport.scroll;
            // Displacement from origo
            let (curr_x_disclacement, curr_y_displacement) = (
                curr_viewport.region.x.saturating_add_signed(curr_x_scroll),
                curr_viewport.region.y.saturating_add_signed(curr_y_scroll),
            );
            if (x_offset, y_offset) == (curr_x_disclacement, curr_y_displacement) {
                continue;
            }

            // Operation doesn't actually change the true offset, as it is still needed
            // reposition back the split viewports back into one piece for the backend.
            let new_scroll = (
                x_offset as i16 - curr_viewport.region.x as i16,
                y_offset as i16 - curr_viewport.region.y as i16,
            );
            if let Err(err) = self.assert_viewport_within_buffer(index, new_scroll) {
                return Ok(Err(err));
            }
            // We don't want to update scroll and flush immediatedly as viewport overlap must first be checked.
            // This has be done on all at he same time, otherwise it's not possible to scroll multiple viewports
            // in unison.
            viewports_to_flush.push((index, new_scroll));
        }

        match self.assert_nonoverlapping_viewports(viewports_to_flush) {
            Ok(viewports_to_flush) => {
                for (index, new_scroll) in viewports_to_flush {
                    self.viewports[index].scroll = new_scroll;
                    self.flush_viewport_region(index)?;
                }
                Ok(Ok(()))
            }
            Err(err) => Ok(Err(err)),
        }
    }

    fn assert_viewport_within_buffer(
        &self,
        viewport_index: usize,
        new_scroll: (i16, i16),
    ) -> io::Result<()> {
        let viewport_region = &self.viewports[viewport_index].region;
        let (new_x_scroll, new_y_scroll) = new_scroll;
        let error = |side: &str, expected: i16, actual: i16| {
            let msg = format!(
                "Viewport scroll overflows buffer, index: {}, side: {}, expected: {}, actual: {}",
                viewport_index, side, expected, actual
            );
            Error::new(ErrorKind::Other, msg)
        };

        // Check left
        let new_x_begin = viewport_region.x as i16 + new_x_scroll;
        let min = 0;
        if new_x_begin < min {
            return Err(error("left", min, new_x_begin));
        }
        // Check top
        let new_y_begin = viewport_region.y as i16 + new_y_scroll;
        if new_y_begin < min {
            return Err(error("top", min, new_y_begin));
        }
        // Check right
        let new_x_end = (viewport_region.width + viewport_region.x) as i16 + new_x_scroll;
        let max = self.buffer.get_width() as i16;
        if new_x_end > max {
            return Err(error("right", max, new_x_end));
        }
        // Check bottom
        let new_y_end = (viewport_region.height + viewport_region.y) as i16 + new_y_scroll;
        let max = self.buffer.get_height() as i16;
        if new_y_end > max {
            return Err(error("bottom", max, new_y_end));
        }

        Ok(())
    }

    fn assert_nonoverlapping_viewports(
        &self,
        viewports_to_flush: ViewportToFlush,
    ) -> io::Result<ViewportToFlush> {
        let error = |index_one: usize, index_two: usize| {
            let msg = format!("Viewport {} overlaps with {}.", index_one, index_two);
            Error::new(ErrorKind::Other, msg)
        };

        // Create a new viewports array with all the scroll values applied
        let mut next_viewport_regions = Vec::with_capacity(self.viewports.len());
        for (viewport_index, viewport) in self.viewports.iter().enumerate() {
            let scroll: (i16, i16);
            if let Some((_, new_scroll)) = viewports_to_flush
                .iter()
                .find(|(index, _)| *index == viewport_index)
            {
                scroll = *new_scroll;
            } else {
                scroll = viewport.scroll;
            }

            let (x_scroll, y_scroll) = scroll;
            let mut next_viewport_region = viewport.clone().region;
            next_viewport_region.x = next_viewport_region.x.saturating_add_signed(x_scroll);
            next_viewport_region.y = next_viewport_region.y.saturating_add_signed(y_scroll);
            next_viewport_regions.push(next_viewport_region);
        }

        // Check if any viewport overlaps
        for index_one in 0..self.viewports.len() {
            for index_two in (index_one + 1)..self.viewports.len() {
                let region_one = &next_viewport_regions[index_one];
                let region_two = &next_viewport_regions[index_two];
                if region_one.intersects(region_two) {
                    return Err(error(index_one, index_two));
                }
            }
        }

        Ok(viewports_to_flush)
    }

    pub fn resize_buffer_abs(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height)
    }

    /// Shorthand for: resize_buffer_abs(curr_width + width, curr_height + height),
    /// but which also doesn't require any explicit copying.
    /// Addition is saturating add signed, so negative values can also be supplied.
    pub fn resize_buffer_rel(&mut self, width: i16, height: i16) {
        self.buffer.resize(
            self.buffer.get_width().saturating_add_signed(width),
            self.buffer.get_height().saturating_add_signed(height),
        )
    }

    /// Queries the backend for its viewport size and  resizes frontend viewport
    /// size if it doesn't match.
    fn autoresize(&mut self) -> io::Result<()> {
        let sum_viewport_sizes = self
            .viewports
            .iter()
            .fold(0, |acc, viewport| acc + viewport.region.size());
        if self.backend.size()? != sum_viewport_sizes {
            let (b_width, b_height) = self.backend.dimensions()?;
            let new_viewports = Layout::default()
                .direction(self.viewport_split_direction.clone())
                .constraints(self.viewport_slit_constaints.clone())
                .split(&Rect {
                    x: 0,
                    y: 0,
                    width: b_width,
                    height: b_height,
                });
            // Only change height and width for scroll to persist
            for (index, new_viewport) in new_viewports.iter().enumerate() {
                self.viewports[index].region.width = new_viewport.width;
                self.viewports[index].region.height = new_viewport.height;
            }

            if self.buffer.cells.len() < self.backend.size()? {
                self.buffer.resize(b_width, b_height)
            }
            self.clear()?
        }

        Ok(())
    }

    /// Clears buffer and backend.
    fn clear(&mut self) -> io::Result<()> {
        self.buffer.clear();
        self.backend.clear()
    }

    pub fn clear_region(&mut self, area: &Rect) {
        self.buffer.clear_region(area);
    }

    pub fn clear_viewport(&mut self, viewport_index: usize) {
        self.buffer
            .clear_region(&self.viewports[viewport_index].region)
    }

    pub fn clear_all_viewports(&mut self) {
        for viewport in &self.viewports {
            self.buffer.clear_region(&viewport.region)
        }
    }

    /// Flush buffer content to backend.
    /// Content flushed is based on the viewport offset and backend terminal size.
    pub fn flush(&mut self) -> io::Result<()> {
        self.autoresize()?;
        for index in 0..self.viewports.len() {
            self.flush_viewport_region(index)?
        }
        Ok(())
    }

    /// Fine grained version of flush.
    pub fn flush_viewport_region(&mut self, viewport_index: usize) -> io::Result<()> {
        let (x_scroll, y_scroll) = self.viewports[viewport_index].scroll;
        let mut scrolled_viewport_region = self.viewports[viewport_index].region.clone();
        scrolled_viewport_region.x = scrolled_viewport_region.x.saturating_add_signed(x_scroll);
        scrolled_viewport_region.y = scrolled_viewport_region.y.saturating_add_signed(y_scroll);

        let mut buffer_region = self.buffer.get_region(&scrolled_viewport_region);
        // Translate each cell so that it is placed inside the backend buffer.
        // *_scroll values won't be unsound as they are checked for in viewport_scroll_split
        // with assert_viewport_within_buffer.
        buffer_region.iter_mut().for_each(|(x, y, _)| {
            *x = x.saturating_add_signed(-x_scroll);
            *y = y.saturating_add_signed(-y_scroll);
        });

        self.backend.draw(buffer_region.iter())
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()
    }

    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)
    }
}

type ViewportToFlush = Vec<(usize, (i16, i16))>;

#[derive(Clone)]
pub struct Viewport {
    region: Rect,
    pub scroll: (i16, i16),
}
