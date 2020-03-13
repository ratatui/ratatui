use std::io;

use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::widgets::{StatefulWidget, Widget};

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
    /// Terminal size used for rendering.
    known_size: Rect,
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
    pub fn size(&self) -> Rect {
        self.terminal.known_size
    }

    /// Calls the draw method of a given widget on the current buffer
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, self.terminal.current_buffer_mut());
    }

    pub fn render_stateful_widget<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidget,
    {
        widget.render(area, self.terminal.current_buffer_mut(), state);
    }
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
    /// Wrapper around Termion initialization. Each buffer is initialized with a blank string and
    /// default colors for the foreground and the background
    pub fn new(backend: B) -> io::Result<Terminal<B>> {
        let size = backend.size()?;
        Ok(Terminal {
            backend,
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
            hidden_cursor: false,
            known_size: size,
        })
    }

    /// Get a Frame object which provides a consistent view into the terminal state for rendering.
    pub fn get_frame(&mut self) -> Frame<B> {
        Frame { terminal: self }
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Obtains a difference between the previous and the current buffer and passes it to the
    /// current backend for drawing.
    pub fn flush(&mut self) -> io::Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        self.backend.draw(updates.into_iter())
    }

    /// Updates the Terminal so that internal buffers match the requested size. Requested size will
    /// be saved so the size can remain consistent when rendering.
    /// This leads to a full clear of the screen.
    pub fn resize(&mut self, area: Rect) -> io::Result<()> {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].reset();
        self.buffers[1 - self.current].resize(area);
        self.known_size = area;
        self.backend.clear()
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    pub fn autoresize(&mut self) -> io::Result<()> {
        let size = self.size()?;
        if self.known_size != size {
            self.resize(size)?;
        }
        Ok(())
    }

    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state
    /// and prepares for the next draw call.
    pub fn draw<F>(&mut self, f: F) -> io::Result<()>
    where
        F: FnOnce(Frame<B>),
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        f(self.get_frame());

        // Draw to stdout
        self.flush()?;

        // Swap buffers
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;

        // Flush
        self.backend.flush()?;
        Ok(())
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
    pub fn clear(&mut self) -> io::Result<()> {
        self.backend.clear()
    }
    /// Queries the real size of the backend.
    pub fn size(&self) -> io::Result<Rect> {
        self.backend.size()
    }
}
