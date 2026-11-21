use alloc::borrow::Cow;
use alloc::string::String;

use super::WidgetRef;
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;

/// Provides the ability to render a string slice by reference.
///
/// This trait implementation ensures that a string slice, which is an immutable view over a
/// `String`, can be drawn on demand without requiring ownership of the string itself. It utilizes
/// the default text style when rendering onto the provided [`Buffer`] at the position defined by
/// [`Rect`].
impl WidgetRef for &str {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_stringn(area.x, area.y, self, area.width as usize, Style::new());
    }
}

/// Provides the ability to render a `String` by reference.
///
/// This trait allows for a `String` to be rendered onto the [`Buffer`], similarly using the default
/// style settings. It ensures that an owned `String` can be rendered efficiently by reference,
/// without the need to give up ownership of the underlying text.
impl WidgetRef for String {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_stringn(area.x, area.y, self, area.width as usize, Style::new());
    }
}

/// Provides the ability to render a `Cow<str>` by reference.
///
/// This trait allows for a borrowed or owned `Cow<str>` to be rendered onto the [`Buffer`],
/// similarly using the default style settings.
impl WidgetRef for Cow<'_, str> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_stringn(
            area.x,
            area.y,
            self.as_ref(),
            area.width as usize,
            Style::new(),
        );
    }
}

#[cfg(test)]
mod tests {
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
    fn render_ref_cow_borrowed(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_cow_owned(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Owned(String::from("hello world"));
        cow.render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }

    #[rstest]
    fn render_ref_option_cow(mut buf: Buffer) {
        let cow: Cow<'_, str> = Cow::Borrowed("hello world");
        Some(cow).render_ref(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["hello world         "]));
    }
}
