use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::rc::Rc;
#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;

use super::Widget;
use crate::buffer::Buffer;
use crate::layout::Rect;

/// Renders a borrowed or owned `Cow<B>` as a widget.
///
/// Both variants render by reference, so this works for any `B` that renders by reference:
/// `Cow<str>` (holding a `&str` or a `String`), `Cow<Line>`, and so on. The owned value doesn't
/// need to implement `Widget` by value.
///
/// That includes your own widgets. If a widget implements `Widget for &MyWidget` and `Clone`, a
/// `Cow<MyWidget>` can be passed to `Frame::render_widget` or any API that takes a widget by
/// value. This suits a widget that is usually shared but is sometimes customized for one render:
///
/// ```rust
/// use std::borrow::Cow;
///
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// #[derive(Clone)]
/// struct Badge {
///     label: &'static str,
/// }
///
/// impl Widget for &Badge {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         self.label.render(area, buf);
///     }
/// }
///
/// let shared = Badge { label: "ok" };
/// let borrowed: Cow<'_, Badge> = Cow::Borrowed(&shared);
/// let owned: Cow<'_, Badge> = Cow::Owned(Badge { label: "ALERT" });
///
/// let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
/// borrowed.render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["ok   "]));
/// owned.render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["ALERT"]));
/// ```
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
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.as_ref().render(area, buf);
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
/// This is the thread-safe counterpart of the `Rc` impl, for values that are also shared with
/// another thread.
///
/// ```rust
/// use std::sync::Arc;
/// use std::thread;
///
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::widgets::Widget;
///
/// let status: Arc<str> = Arc::from("Done");
/// let worker_status = Arc::clone(&status);
/// thread::spawn(move || assert_eq!(&*worker_status, "Done"))
///     .join()
///     .unwrap();
///
/// let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
/// (&status).render(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Done"]));
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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::text::Line;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
    }

    /// A widget that only renders by reference, so these tests don't depend on the impls for
    /// text types.
    #[derive(Clone)]
    struct Greeting;

    impl Widget for &Greeting {
        fn render(self, area: Rect, buf: &mut Buffer) {
            Line::from("Hello").render(area, buf);
        }
    }

    #[rstest]
    fn render_cow_borrowed(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Borrowed(&Greeting);
        // Call by value so method resolution can't autoref to `&Cow`.
        Widget::render(cow, buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_cow_owned(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Owned(Greeting);
        // Call by value so method resolution can't autoref to `&Cow`.
        Widget::render(cow, buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_cow_ref_borrowed(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Borrowed(&Greeting);
        (&cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_cow_ref_owned(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Owned(Greeting);
        (&cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_option_cow(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Owned(Greeting);
        Some(cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_box(mut buf: Buffer) {
        let boxed = Box::new(Greeting);
        (&boxed).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_rc(mut buf: Buffer) {
        let rc = Rc::new(Greeting);
        (&rc).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[cfg(target_has_atomic = "ptr")]
    #[rstest]
    fn render_arc(mut buf: Buffer) {
        let arc = Arc::new(Greeting);
        (&arc).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_box_of_rc(mut buf: Buffer) {
        let nested = Box::new(Rc::new(Greeting));
        (&nested).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }
}
