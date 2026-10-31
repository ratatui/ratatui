use alloc::string::String;

use super::Widget;
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;

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

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    #[cfg(target_has_atomic = "ptr")]
    use alloc::sync::Arc;

    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
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
    fn render_cow_borrowed_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        // Call by value so method resolution can't autoref to `&Cow`.
        Widget::render(cow, buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_owned_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Owned(String::from("hello world"));
        // Call by value so method resolution can't autoref to `&Cow`.
        Widget::render(cow, buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_ref_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        (&cow).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_cow_str_truncate(mut buf: Buffer) {
        let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
        let cow: Cow<'_, str> = Cow::Borrowed("hello world, just hello");
        Widget::render(cow, area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_option_cow_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        Some(cow).render(buf.area, &mut buf);
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

    #[cfg(target_has_atomic = "ptr")]
    #[rstest]
    fn render_arc_str(mut buf: Buffer) {
        let arc: Arc<str> = Arc::from("hello world");
        (&arc).render(buf.area, &mut buf);
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
        let boxed: Box<str> = Box::from("hello");
        (&boxed).render(render_area, &mut buf);
        assert_eq!(buf, expected);
        let rc: Rc<str> = Rc::from("hello");
        (&rc).render(render_area, &mut buf);
        assert_eq!(buf, expected);
        #[cfg(target_has_atomic = "ptr")]
        {
            let arc: Arc<str> = Arc::from("hello");
            (&arc).render(render_area, &mut buf);
            assert_eq!(buf, expected);
        }
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
        buf.reset();
        let boxed: Box<str> = Box::from("hello");
        (&boxed).render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        let rc: Rc<str> = Rc::from("hello");
        (&rc).render(Rect::new(9, 9, 10, 10), &mut buf);
        assert_eq!(buf, expected);
        #[cfg(target_has_atomic = "ptr")]
        {
            buf.reset();
            let arc: Arc<str> = Arc::from("hello");
            (&arc).render(Rect::new(9, 9, 10, 10), &mut buf);
            assert_eq!(buf, expected);
        }
    }
}
