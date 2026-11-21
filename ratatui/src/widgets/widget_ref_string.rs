use alloc::string::String;

use super::{Widget, WidgetRef};
use crate::buffer::Buffer;
use crate::layout::Rect;

/// Provides the ability to render a string slice by reference.
///
/// This trait implementation ensures that a string slice, which is an immutable view over a
/// `String`, can be drawn on demand without requiring ownership of the string itself. It utilizes
/// the default text style when rendering onto the provided [`Buffer`] at the position defined by
/// [`Rect`]. It renders the same way as the `Widget` impl for `&str`, including clipping the area
/// to the buffer.
impl WidgetRef for &str {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(*self, area, buf);
    }
}

/// Provides the ability to render a `String` by reference.
///
/// This trait allows for a `String` to be rendered onto the [`Buffer`], similarly using the default
/// style settings. It ensures that an owned `String` can be rendered efficiently by reference,
/// without the need to give up ownership of the underlying text. It renders the same way as the
/// `Widget` impl for `&str`.
impl WidgetRef for String {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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
    fn render_ref_str(mut buf: Buffer) {
        "hello world".render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_option_str(mut buf: Buffer) {
        Some("hello world").render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_string(mut buf: Buffer) {
        String::from("hello world").render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_option_string(mut buf: Buffer) {
        Some(String::from("hello world")).render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_cow_borrowed_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_cow_owned_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Owned(String::from("hello world"));
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_option_cow_str(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        Some(cow).render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_box_str(mut buf: Buffer) {
        let boxed: Box<str> = Box::from("hello world");
        boxed.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_rc_str(mut buf: Buffer) {
        let rc: Rc<str> = Rc::from("hello world");
        rc.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[cfg(target_has_atomic = "ptr")]
    #[rstest]
    fn render_ref_arc_str(mut buf: Buffer) {
        let arc: Arc<str> = Arc::from("hello world");
        arc.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    /// The string impls clip the area to the buffer like the `Widget` impls do, so text whose
    /// area starts outside the buffer still renders the part that overlaps it.
    #[test]
    fn render_ref_string_widgets_clip_to_buffer() {
        let area = Rect::new(10, 10, 4, 1);
        let mut buf = Buffer::empty(area);
        let mut expected = Buffer::with_lines(["hell"]);
        expected.area = area;
        let render_area = Rect::new(9, 9, 10, 10);

        "hello".render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        String::from("hello").render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        Cow::Borrowed("hello").render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        let boxed: Box<str> = Box::from("hello");
        boxed.render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        buf.reset();
        let rc: Rc<str> = Rc::from("hello");
        rc.render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        #[cfg(target_has_atomic = "ptr")]
        {
            buf.reset();
            let arc: Arc<str> = Arc::from("hello");
            arc.render_ref(render_area, &mut buf);
            assert_eq!(buf, expected);
        }
    }

    /// Areas that only touch the buffer at an edge, or miss it entirely, render nothing.
    #[rstest]
    fn render_ref_string_widgets_outside_buffer(
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

        "hello".render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        String::from("hello").render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        Some("hello").render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        Cow::Borrowed("hello").render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        let boxed: Box<str> = Box::from("hello");
        boxed.render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        let rc: Rc<str> = Rc::from("hello");
        rc.render_ref(render_area, &mut buf);
        assert_eq!(buf, expected);
        #[cfg(target_has_atomic = "ptr")]
        {
            let arc: Arc<str> = Arc::from("hello");
            arc.render_ref(render_area, &mut buf);
            assert_eq!(buf, expected);
        }
    }
}
