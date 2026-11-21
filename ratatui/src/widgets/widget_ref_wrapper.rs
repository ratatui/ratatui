use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::rc::Rc;
#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;

use super::{Widget, WidgetRef};
use crate::buffer::Buffer;
use crate::layout::Rect;

/// Provides the ability to render a `Cow<B>` by reference.
///
/// This renders the borrowed or owned value the same way as rendering `&B`, so `Cow<str>`,
/// `Cow<Line>`, and similar types can be stored and rendered without giving up ownership. This
/// includes your own `Clone` widgets that only implement `Widget for &MyWidget`.
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use std::borrow::Cow;
///
/// use ratatui::widgets::WidgetRef;
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
///
/// let title: Cow<'static, str> = Cow::Owned(format!("{} unread", 3));
/// let mut buf = Buffer::empty(Rect::new(0, 0, 8, 1));
/// title.render_ref(buf.area, &mut buf);
/// // `title` is still usable after rendering
/// assert_eq!(title, "3 unread");
/// assert_eq!(buf, Buffer::with_lines(["3 unread"]));
/// # }
/// ```
impl<B> WidgetRef for Cow<'_, B>
where
    B: ?Sized + ToOwned,
    for<'a> &'a B: Widget,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders a `Box<W>` by reference.
///
/// This renders the boxed value the same way as rendering `&W`, so `Box<str>`, `Box<Line>`, and
/// boxed widgets that implement `Widget for &MyWidget` can be rendered without unboxing them.
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use ratatui::widgets::WidgetRef;
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
///
/// let title: Box<str> = Box::from("Inbox");
/// let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
/// title.render_ref(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Inbox"]));
/// # }
/// ```
impl<W> WidgetRef for Box<W>
where
    W: ?Sized,
    for<'a> &'a W: Widget,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders an `Rc<W>` by reference.
///
/// This renders the shared value the same way as rendering `&W`, so each holder of an `Rc` can
/// render it without cloning the contents.
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use std::rc::Rc;
///
/// use ratatui::widgets::WidgetRef;
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
///
/// let title: Rc<str> = Rc::from("Inbox");
/// let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
/// title.render_ref(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Inbox"]));
/// # }
/// ```
impl<W> WidgetRef for Rc<W>
where
    W: ?Sized,
    for<'a> &'a W: Widget,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        (**self).render(area, buf);
    }
}

/// Renders an `Arc<W>` by reference.
///
/// This is the thread-safe counterpart of the `Rc` impl.
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use std::sync::Arc;
///
/// use ratatui::widgets::WidgetRef;
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
///
/// let title: Arc<str> = Arc::from("Inbox");
/// let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
/// title.render_ref(buf.area, &mut buf);
/// assert_eq!(buf, Buffer::with_lines(["Inbox"]));
/// # }
/// ```
#[cfg(target_has_atomic = "ptr")]
impl<W> WidgetRef for Arc<W>
where
    W: ?Sized,
    for<'a> &'a W: Widget,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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
    fn render_ref_cow_borrowed(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Borrowed(&Greeting);
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_ref_cow_owned(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Owned(Greeting);
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_ref_option_cow(mut buf: Buffer) {
        let cow: Cow<'_, Greeting> = Cow::Owned(Greeting);
        Some(cow).render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_ref_box(mut buf: Buffer) {
        let boxed = Box::new(Greeting);
        boxed.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_ref_rc(mut buf: Buffer) {
        let rc = Rc::new(Greeting);
        rc.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[cfg(target_has_atomic = "ptr")]
    #[rstest]
    fn render_ref_arc(mut buf: Buffer) {
        let arc = Arc::new(Greeting);
        arc.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }
}
