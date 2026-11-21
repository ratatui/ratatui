use crate::buffer::Buffer;
use crate::layout::Rect;

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
/// Widget is implemented for `&str`, `String`, and `&String`.
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
    fn render(mut buf: Buffer) {
        (&Greeting).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_option_some(mut buf: Buffer) {
        Some(&Greeting).render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["Hello               "]));
    }

    #[rstest]
    fn render_option_none(mut buf: Buffer) {
        None::<&Greeting>.render(buf.area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(["                    "]));
    }
}
