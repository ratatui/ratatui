use crate::{buffer::Buffer, layout::Rect};

/// A `StatefulWidgetRef` is a trait that allows rendering a stateful widget by reference.
///
/// This is the stateful equivalent of `WidgetRef`. It is useful when you want to store a reference
/// to a stateful widget and render it later. It also allows you to render boxed stateful widgets.
///
/// This trait was introduced in Ratatui 0.26.0 and is implemented for all the internal stateful
/// widgets. It is currently marked as unstable as we are still evaluating the API and may make
/// changes in the future. See <https://github.com/ratatui/ratatui/issues/1287> for more
/// information.
///
/// A blanket implementation of `StatefulWidget` for `&W` where `W` implements `StatefulWidgetRef`
/// is provided.
///
/// See the documentation for [`WidgetRef`] for more information on boxed widgets.
/// See the documentation for [`StatefulWidget`] for more information on stateful widgets.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use ratatui_core::{
///     buffer::Buffer,
///     layout::Rect,
///     style::Stylize,
///     text::Line,
///     widgets::{StatefulWidget, StatefulWidgetRef, Widget},
/// };
///
/// struct PersonalGreeting;
///
/// impl StatefulWidgetRef for PersonalGreeting {
///     type State = String;
///     fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
///         Line::raw(format!("Hello {}", state)).render(area, buf);
///     }
/// }
///
/// impl StatefulWidget for PersonalGreeting {
///     type State = String;
///     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
///         (&self).render_ref(area, buf, state);
///     }
/// }
///
/// fn render(area: Rect, buf: &mut Buffer) {
///     let widget = PersonalGreeting;
///     let mut state = "world".to_string();
///     widget.render(area, buf, &mut state);
/// }
/// # }
/// ```
#[instability::unstable(feature = "widget-ref")]
pub trait StatefulWidgetRef {
    /// State associated with the stateful widget.
    ///
    /// If you don't need this then you probably want to implement [`WidgetRef`] instead.
    ///
    /// [`WidgetRef`]: super::WidgetRef
    type State;
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom stateful widget.
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}

// Note: while StatefulWidgetRef is marked as unstable, the blanket implementation of StatefulWidget
// cannot be implemented as W::State is effectively pub(crate) and not accessible from outside the
// crate. Once stabilized, this blanket implementation can be added and the specific implementations
// on Table and List can be removed.
//
// /// Blanket implementation of `StatefulWidget` for `&W` where `W` implements `StatefulWidgetRef`.
// ///
// /// This allows you to render a stateful widget by reference.
// impl<W: StatefulWidgetRef> StatefulWidget for &W {
//     type State = W::State;
//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         StatefulWidgetRef::render_ref(self, area, buf, state);
//     }
// }

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
    }

    #[fixture]
    fn state() -> String {
        "world".to_string()
    }

    struct PersonalGreeting;

    impl StatefulWidgetRef for PersonalGreeting {
        type State = String;
        fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
            Line::from(format!("Hello {state}")).render(area, buf);
        }
    }

    #[rstest]
    fn render_ref(mut buf: Buffer, mut state: String) {
        let widget = PersonalGreeting;
        widget.render_ref(buf.area, &mut buf, &mut state);
        assert_eq!(buf, Buffer::with_lines(["Hello world         "]));
    }

    // Note this cannot be tested until the blanket implementation of StatefulWidget for &W
    // where W implements StatefulWidgetRef is added. (see the comment in the blanket
    // implementation for more).
    // /// This test is to ensure that the blanket implementation of `StatefulWidget` for `&W`
    // where /// `W` implements `StatefulWidgetRef` works as expected.
    // #[rstest]
    // fn stateful_widget_blanket_render(mut buf: Buffer, mut state: String) {
    //     let widget = &PersonalGreeting;
    //     widget.render(buf.area, &mut buf, &mut state);
    //     assert_eq!(buf, Buffer::with_lines(["Hello world         "]));
    // }

    #[rstest]
    fn box_render_render(mut buf: Buffer, mut state: String) {
        let widget = Box::new(PersonalGreeting);
        widget.render_ref(buf.area, &mut buf, &mut state);
        assert_eq!(buf, Buffer::with_lines(["Hello world         "]));
    }
}
