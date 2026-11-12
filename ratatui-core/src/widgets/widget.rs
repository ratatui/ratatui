use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;

use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;

/// A `Widget` is a type that can be drawn on a [`Buffer`] in a given [`Rect`].
///
/// For a comprehensive guide to widgets, including trait explanations, implementation patterns,
/// and available widgets, see the [`widgets`] module documentation.
///
/// [`widgets`]: ../../ratatui/widgets/index.html
///
/// Prior to Ratatui 0.26.0, widgets generally were created for each frame as they were consumed
/// during rendering. This meant that they were not meant to be stored but used as *commands* to
/// draw common figures in the UI.
///
/// Starting with Ratatui 0.26.0, all the internal widgets implement Widget for a reference to
/// themselves. This allows you to store a reference to a widget and render it later. Widget crates
/// should consider also doing this to allow for more flexibility in how widgets are used.
///
/// In Ratatui 0.26.0, we also added an unstable `WidgetRef` trait and implemented this on all the
/// internal widgets. In addition to the above benefit of rendering references to widgets, this also
/// allows you to render boxed widgets. This is useful when you want to store a collection of
/// widgets with different types. You can then iterate over the collection and render each widget.
/// See <https://github.com/ratatui/ratatui/issues/1287> for more information.
///
/// In general where you expect a widget to immutably work on its data, we recommended to implement
/// `Widget` for a reference to the widget (`impl Widget for &MyWidget`). If you need to store state
/// between draw calls, implement `StatefulWidget` if you want the Widget to be immutable, or
/// implement `Widget` for a mutable reference to the widget (`impl Widget for &mut MyWidget`) if
/// you want the widget to be mutable. The mutable widget pattern is used infrequently in apps, but
/// can be quite useful.
///
/// A blanket implementation of `Widget` for `&W` where `W` implements `WidgetRef` is provided.
/// Widget is also implemented for `&str`, `String`, and `&String`.
///
/// `Cow<B>`, `&Cow<B>`, `&Box<W>`, `&Rc<W>`, and `&Arc<W>` render the value they wrap, so they
/// implement `Widget` whenever the wrapped type does when rendered by reference. For example,
/// `Cow<str>`, `&Box<str>`, `&Rc<str>`, and `&Arc<Paragraph>` can all be rendered directly.
///
/// # Examples
///
/// ```rust,ignore
/// use ratatui::{
///     backend::TestBackend,
///     widgets::{Clear, Widget},
///     Terminal,
/// };
/// # let backend = TestBackend::new(5, 5);
/// # let mut terminal = Terminal::new(backend).unwrap();
///
/// terminal.draw(|frame| {
///     frame.render_widget(Clear, frame.area());
/// });
/// ```
///
/// It's common to render widgets inside other widgets:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::text::Line;
/// use ratatui_core::widgets::Widget;
///
/// struct MyWidget;
///
/// impl Widget for MyWidget {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Hello").render(area, buf);
///     }
/// }
/// ```
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized;
}

/// Renders a string slice as a widget.
///
/// This implementation allows a string slice (`&str`) to act as a widget, meaning it can be drawn
/// onto a [`Buffer`] in a specified [`Rect`]. The slice represents a static string which can be
/// rendered by reference, thereby avoiding the need for string cloning or ownership transfer when
/// drawing the text to the screen.
impl Widget for &str {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        buf.set_stringn(area.x, area.y, self, area.width as usize, Style::new());
    }
}

/// Renders a `String` object as a widget.
///
/// This implementation enables an owned `String` to be treated as a widget, which can be rendered
/// on a [`Buffer`] within the bounds of a given [`Rect`].
impl Widget for String {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.as_str().render(area, buf);
    }
}

/// Renders a `String` reference as a widget.
///
/// This lets a `String` stored in app state be rendered from `&self` without moving it out or
/// calling `.as_str()` first.
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// struct App {
///     title: String,
/// }
///
/// impl App {
///     fn render(&self, area: Rect, buf: &mut Buffer) {
///         (&self.title).render(area, buf);
///     }
/// }
///
/// let app = App {
///     title: String::from("Inbox"),
/// };
/// let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
/// app.render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Inbox"]));
/// ```
impl Widget for &String {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.as_str().render(area, buf);
    }
}

/// Renders a borrowed or owned `Cow<B>` as a widget.
///
/// A `Cow` renders whatever it holds, so this works for any `B` that renders by reference and
/// whose owned form renders by value: `Cow<str>` (holding a `&str` or a `String`), `Cow<Line>`,
/// and so on.
///
/// A common source of `Cow<str>` is decoding bytes that may not be valid UTF-8, such as process
/// output:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// let output: &[u8] = b"build ok";
/// let mut buf = Buffer::empty(Rect::new(0, 0, 8, 1));
/// String::from_utf8_lossy(output).render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["build ok"]));
/// ```
impl<B> Widget for Cow<'_, B>
where
    B: ?Sized + ToOwned,
    for<'a> &'a B: Widget,
    B::Owned: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Cow::Borrowed(widget) => widget.render(area, buf),
            Cow::Owned(widget) => widget.render(area, buf),
        }
    }
}

/// Renders a reference to a `Cow<B>` as a widget.
///
/// `Cow<'static, str>` is a common field type for text that is usually a string literal but is
/// sometimes built at runtime. This lets that field be rendered from `&self`.
///
/// ```rust
/// use std::borrow::Cow;
///
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// struct StatusBar {
///     message: Cow<'static, str>,
/// }
///
/// impl Widget for &StatusBar {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         (&self.message).render(area, buf);
///     }
/// }
///
/// let ready = StatusBar {
///     message: Cow::Borrowed("Ready"),
/// };
/// let saved = StatusBar {
///     message: Cow::Owned(format!("Saved {} files", 3)),
/// };
/// let mut buf = Buffer::empty(Rect::new(0, 0, 13, 1));
/// ready.render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Ready        "]));
/// saved.render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Saved 3 files"]));
/// ```
impl<'a, B> Widget for &'a Cow<'_, B>
where
    B: ?Sized + ToOwned,
    &'a B: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders a reference to a `Box<W>` as a widget.
///
/// Rendering a `&Box<W>` renders the boxed `W` by reference, so a boxed widget can be rendered
/// without unboxing it first. This also covers unsized values such as `Box<str>`.
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::text::Line;
/// use ratatui_core::widgets::Widget;
///
/// struct Header {
///     title: Box<Line<'static>>,
///     subtitle: Box<str>,
/// }
///
/// let header = Header {
///     title: Box::new(Line::from("Mail")),
///     subtitle: Box::from("3 unread"),
/// };
/// let mut buf = Buffer::empty(Rect::new(0, 0, 8, 2));
/// (&header.title).render(Rect::new(0, 0, 8, 1), &mut buf);
/// (&header.subtitle).render(Rect::new(0, 1, 8, 1), &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Mail    ", "3 unread"]));
/// ```
impl<'a, W> Widget for &'a Box<W>
where
    W: ?Sized,
    &'a W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders a reference to an `Rc<W>` as a widget.
///
/// Use `Rc` when several parts of the UI hold the same widget or text. Each holder can render it
/// without cloning the contents.
///
/// ```rust
/// use std::rc::Rc;
///
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// let user: Rc<str> = Rc::from("ferris");
/// let sidebar = Rc::clone(&user);
/// let title_bar = Rc::clone(&user);
///
/// let mut buf = Buffer::empty(Rect::new(0, 0, 6, 2));
/// (&sidebar).render(Rect::new(0, 0, 6, 1), &mut buf);
/// (&title_bar).render(Rect::new(0, 1, 6, 1), &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["ferris", "ferris"]));
/// ```
impl<'a, W> Widget for &'a Rc<W>
where
    W: ?Sized,
    &'a W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders a reference to an `Arc<W>` as a widget.
///
/// This is the thread-safe counterpart of the `Rc` impl. It is useful when the value is shared
/// with another thread, such as a status message set by a background task.
///
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use std::thread;
///
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// let status: Arc<Mutex<Arc<str>>> = Arc::new(Mutex::new(Arc::from("Loading")));
/// let worker_status = Arc::clone(&status);
/// thread::spawn(move || *worker_status.lock().unwrap() = Arc::from("Done"))
///     .join()
///     .unwrap();
///
/// let mut buf = Buffer::empty(Rect::new(0, 0, 7, 1));
/// let message = Arc::clone(&status.lock().unwrap());
/// (&message).render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Done   "]));
/// ```
#[cfg(target_has_atomic = "ptr")]
impl<'a, W> Widget for &'a Arc<W>
where
    W: ?Sized,
    &'a W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

impl<W: Widget> Widget for Option<W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(widget) = self {
            widget.render(area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::buffer::Buffer;
    use crate::layout::Rect;
    use crate::text::Line;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
    }

    struct Greeting;

    impl Widget for Greeting {
        fn render(self, area: Rect, buf: &mut Buffer) {
            Line::from("Hello").render(area, buf);
        }
    }

    #[rstest]
    fn render(mut buf: Buffer) {
        let widget = Greeting;
        widget.render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_str(mut buf: Buffer) {
        "hello world".render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_str_truncate(mut buf: Buffer) {
        let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
        "hello world, just hello".render(area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_option_str(mut buf: Buffer) {
        Some("hello world").render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_string(mut buf: Buffer) {
        String::from("hello world").render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_string_truncate(mut buf: Buffer) {
        let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
        String::from("hello world, just hello").render(area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_option_string(mut buf: Buffer) {
        Some(String::from("hello world")).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_string_ref(mut buf: Buffer) {
        let s = String::from("hello world");
        (&s).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_borrowed(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        cow.render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_owned(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Owned(String::from("hello world"));
        cow.render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_ref(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        (&cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_truncate(mut buf: Buffer) {
        let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
        let cow: Cow<'_, str> = Cow::Borrowed("hello world, just hello");
        cow.render(area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_line(mut buf: Buffer) {
        let cow: Cow<'_, Line> = Cow::Owned(Line::from("hello world"));
        (&cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        buf.reset();
        cow.render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_box_str(mut buf: Buffer) {
        let boxed: Box<str> = Box::from("hello world");
        (&boxed).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_rc_str(mut buf: Buffer) {
        let rc: Rc<str> = Rc::from("hello world");
        (&rc).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_arc_line(mut buf: Buffer) {
        let arc = Arc::new(Line::from("hello world"));
        (&arc).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_option_cow(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        Some(cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_string_widgets_outside_buffer(
        #[values(
            (Rect::new(10, 10, 5, 1), Rect::new(0, 10, 5, 1)),
            (Rect::new(10, 10, 5, 1), Rect::new(15, 10, 5, 1)),
            (Rect::new(10, 10, 5, 1), Rect::new(10, 9, 5, 1)),
            (Rect::new(10, 10, 5, 1), Rect::new(10, 11, 5, 1)),
            (Rect::new(10, 10, 5, 1), Rect::new(10, 10, 5, 0)),
            (Rect::new(10, 10, 5, 1), Rect::new(10, 10, 0, 1)),
            (Rect::new(10, 10, 0, 1), Rect::new(10, 10, 5, 1)),
            (Rect::new(10, 10, 5, 0), Rect::new(10, 10, 5, 1)),
            (Rect::ZERO, Rect::new(0, 0, 5, 1)),
            (Rect::new(0, 0, 57, 1), Rect::new(0, 1, 57, 0))
        )]
        bounds: (Rect, Rect),
    ) {
        let (buffer_area, render_area) = bounds;
        let mut buf = Buffer::empty(buffer_area);
        let expected = buf.clone();

        "hello".render(render_area, &mut buf);
        assert_eq!(buf, expected);
        String::from("hello").render(render_area, &mut buf);
        assert_eq!(buf, expected);
        Some("hello").render(render_area, &mut buf);
        assert_eq!(buf, expected);
        Some(String::from("hello")).render(render_area, &mut buf);
        assert_eq!(buf, expected);
        let s = String::from("hello");
        (&s).render(render_area, &mut buf);
        assert_eq!(buf, expected);
        let cow = Cow::Borrowed("hello");
        (&cow).render(render_area, &mut buf);
        assert_eq!(buf, expected);
        cow.render(render_area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn render_string_widgets_clip_to_buffer() {
        let area = Rect::new(10, 10, 4, 1);
        let mut buf = Buffer::empty(area);
        let mut expected = Buffer::with_lines(["hell"]);
        expected.area = area;

        "hello".render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        String::from("hello").render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        let s = String::from("hello");
        (&s).render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        Cow::Borrowed("hello").render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
    }
}
