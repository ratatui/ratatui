use unicode_width::UnicodeWidthStr;

use crate::{
    backend::Backend,
    buffer::{Buffer, Cell},
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::{cmp::min, io};

#[derive(Debug, Clone, PartialEq)]
/// Options to pass to [`Terminal::with_options`]
pub struct TerminalOptions {
    /// Viewport used to draw to the terminal
    pub viewport: Viewport,
}

/// Represents a consistent terminal interface for rendering.
pub struct Frame<'a, B: 'a>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
}

impl<'a, B> Frame<'a, B>
where
    B: Backend,
{
    /// Terminal size, guaranteed not to change when rendering.
    pub fn viewport_area(&self) -> Rect {
        self.terminal.viewport_area()
    }

    pub fn resize_buffers(&mut self, width: u16, height: u16) {
        self.terminal.resize_buffers(width, height)
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn clear_region(&mut self, area: Rect) {
        self.terminal.clear_region(area).unwrap()
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
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
    /// terminal.draw(|frame| frame.render_widget(block, area));
    /// ```
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, &mut self.terminal.buffers[self.terminal.current]);
    }

    /// Render a [`StatefulWidget`] to the current buffer using [`StatefulWidget::render`].
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
    /// terminal.draw(|frame| frame.render_stateful_widget(list, area, &mut state));
    /// ```
    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(
            area,
            &mut self.terminal.buffers[self.terminal.current],
            state,
        );
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.terminal.set_cursor(x, y)
    }
}

/// CompletedFrame represents the state of the terminal after all changes performed in the last
/// [`Terminal::draw`] call have been applied. Therefore, it is only valid until the next call to
/// [`Terminal::draw`].
pub struct CompletedFrame<'a> {
    pub buffer: &'a Buffer,
    pub area: Rect,
}

/// Interface to the terminal backed by Termion
#[derive(Debug)]
pub struct Terminal<B>
where
    B: Backend,
{
    backend: B,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
    /// Whether the cursor is currently hidden
    hidden_cursor: bool,
    /// Viewport
    viewport: Viewport,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            if let Err(err) = self.show_cursor() {
                eprintln!("Failed to show the cursor: {}", err);
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Wrapper around Terminal initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let (width, height) = backend.dimensions()?;
        Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport {
                    area: Rect::new(0, 0, width, height),
                    resize_behavior: ResizeBehavior::Auto,
                },
            },
        )
    }

    /// UNSTABLE
    pub fn with_options(backend: B, options: TerminalOptions) -> io::Result<Terminal<B>> {
        let width = options.viewport.area.width;
        let height = options.viewport.area.height;
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(width, height), Buffer::empty(width, height)],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
        })
    }

    pub fn viewport_area(&self) -> Rect {
        self.viewport.area
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn viewport_scroll(&mut self, x_step: u16, y_step: u16) -> io::Result<()> {
        self.viewport.area.x += x_step;
        self.viewport.area.y += y_step;
        self.clear()
    }

    pub fn resize_buffers(&mut self, width: u16, height: u16) {
        self.buffers
            .iter_mut()
            .for_each(|buffer| buffer.resize(width, height))
    }

    /// Obtains a difference between the previous and the current buffer and passes it to the
    /// current backend for drawing.
    pub fn flush(&mut self) -> io::Result<()> {
        let updates = &self.diff();
        self.backend.draw(updates.iter())?;
        self.backend.flush()
    }

    /// Queries the backend for its viewport size and resizes frontend viewport size
    /// if it doesn't match.
    fn autoresize(&mut self) -> io::Result<()> {
        if self.viewport.resize_behavior == ResizeBehavior::Auto {
            let (b_width, b_height) = self.backend.dimensions()?;
            if self.backend.size()? != self.viewport.area.size() {
                self.viewport.area.width = b_width;
                self.viewport.area.height = b_height;
                if self.buffers[0].cells.len() < self.backend.size()? {
                    self.buffers
                        .iter_mut()
                        .for_each(|buffer| buffer.resize(b_width, b_height))
                }
                self.clear()?
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.buffers.iter_mut().for_each(|buffer| buffer.reset());
        self.backend.clear()
    }

    fn clear_region(&mut self, area: Rect) -> io::Result<()> {
        self.buffers
            .iter_mut()
            .for_each(|buffer| Self::map_buffer_region(area, |x, y| buffer.get_mut(x, y).clear()));
        let (backend_width, backend_height) = self.backend.dimensions()?;
        let backend_area = Rect {
            width: min(area.width, backend_width),
            height: min(area.height, backend_height),
            ..area
        };
        Self::map_buffer_region(backend_area, |x, y| {
            //
            self.backend.
        });
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = Frame { terminal: self };
        f(&mut frame);

        // Draw to stdout
        self.flush()?;

        // Swap buffers
        self.current = 1 - self.current;

        Ok(CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.viewport.area,
        })
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    pub fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.backend.get_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.backend.set_cursor(x, y)
    }

    // /// Clear the terminal and force a full redraw on the next draw call.
    // pub fn clear(&mut self) -> io::Result<()> {
    //     self.backend.clear()?;
    //     // Reset the back buffer to make sure the next update will redraw everything.
    //     self.buffers[1 - self.current].reset();
    //     Ok(())
    // }

    /// Builds a minimal sequence of coordinates and Cells necessary to update the UI from
    /// self to other. Will only scan part of buffer that is shown through the viewport.
    ///
    /// We're assuming that buffers are well-formed, that is no double-width cell is followed by
    /// a non-blank cell, (grapheme alligned).
    ///
    /// # Multi-width characters handling:
    ///
    /// ```text
    /// (Index:) `01`
    /// Prev:    `コ`
    /// Next:    `aa`
    /// Updates: `0: a, 1: a'
    /// ```
    ///
    /// ```text
    /// (Index:) `01`
    /// Prev:    `a `
    /// Next:    `コ`
    /// Updates: `0: コ` (double width symbol at index 0 - skip index 1)
    /// ```
    ///
    /// ```text
    /// (Index:) `012`
    /// Prev:    `aaa`
    /// Next:    `aコ`
    /// Updates: `0: a, 1: コ` (double width symbol at index 1 - skip index 2)
    /// ```
    pub fn diff(&self) -> Vec<(u16, u16, &Cell)> {
        let prev_buffer = &self.buffers[1 - self.current];
        let next_buffer = &self.buffers[self.current];

        let mut updates: Vec<(u16, u16, &Cell)> = Vec::new();
        // Cells invalidated by drawing/replacing preceding multi-width characters:
        let mut invalidated: usize = 0;
        // Cells from the current buffer to skip due to preceding multi-width characters taking their
        // place (the skipped cells should be blank anyway):
        let mut to_skip: usize = 0;
        let area = self.viewport.area;
        Self::map_buffer_region(area, |i_x, i_y| {
            let curr_cell = prev_buffer.get(i_x, i_y);
            let next_cell = next_buffer.get(i_x, i_y);

            if (curr_cell != next_cell || invalidated > 0) && to_skip == 0 {
                updates.push((i_x, i_y, next_cell));
            }

            to_skip = next_cell.symbol.width().saturating_sub(1);

            let affected_width = std::cmp::max(curr_cell.symbol.width(), next_cell.symbol.width());
            invalidated = std::cmp::max(affected_width, invalidated).saturating_sub(1);
        });
        updates
    }

    fn map_buffer_region<F: FnMut(u16, u16)>(area: Rect, mut closure: F) {
        for i_y in area.y..(area.y + area.height) {
            for i_x in area.x..(area.x + area.width) {
                closure(i_x, i_y)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
pub struct Viewport {
    area: Rect,
    resize_behavior: ResizeBehavior,
}

impl Viewport {
    /// UNSTABLE
    pub fn fixed(width: u16, height: u16) -> Viewport {
        Viewport {
            area: Rect::new(0, 0, width, height),
            resize_behavior: ResizeBehavior::Fixed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// UNSTABLE
enum ResizeBehavior {
    Fixed,
    Auto,
}
