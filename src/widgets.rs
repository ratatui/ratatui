#![warn(missing_docs)]
//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! Widgets are created for each frame as they are consumed after rendered.
//! They are not meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]: a basic widget that draws a block with optional borders, titles and styles.
//! - [`BarChart`]: displays multiple datasets as bars with optional grouping.
//! - [`calendar::Monthly`]: displays a single month.
//! - [`Canvas`]: draws arbitrary shapes using drawing characters.
//! - [`Chart`]: displays multiple datasets as a lines or scatter graph.
//! - [`Clear`]: clears the area it occupies. Useful to render over previously drawn widgets.
//! - [`Gauge`]: displays progress percentage using block characters.
//! - [`LineGauge`]: display progress as a line.
//! - [`List`]: displays a list of items and allows selection.
//! - [`Paragraph`]: displays a paragraph of optionally styled and wrapped text.
//! - [`Scrollbar`]: displays a scrollbar.
//! - [`Sparkline`]: display a single data set as a sparkline.
//! - [`Table`]: displays multiple rows and columns in a grid and allows selection.
//! - [`Tabs`]: displays a tab bar and allows selection.
//!
//! [`Canvas`]: crate::widgets::canvas::Canvas
mod barchart;
pub mod block;
mod borders;
#[cfg(feature = "widget-calendar")]
pub mod calendar;
pub mod canvas;
mod chart;
mod clear;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod scrollbar;
mod sparkline;
mod table;
mod tabs;

pub use self::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, BorderType, Padding},
    borders::*,
    chart::{Axis, Chart, Dataset, GraphType, LegendPosition},
    clear::Clear,
    gauge::{Gauge, LineGauge},
    list::{List, ListDirection, ListItem, ListState},
    paragraph::{Paragraph, Wrap},
    scrollbar::{ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState},
    sparkline::{RenderDirection, Sparkline},
    table::{Cell, HighlightSpacing, Row, Table, TableState},
    tabs::Tabs,
};
use crate::{buffer::Buffer, layout::Rect, style::Style};

/// A `Widget` is a type that can be drawn on a [`Buffer`] in a given [`Rect`].
///
/// Prior to Ratatui 0.26.0, widgets generally were created for each frame as they were consumed
/// during rendering. This meant that they were not meant to be stored but used as *commands* to
/// draw common figures in the UI.
///
/// Starting with Ratatui 0.26.0, all the internal widgets implement Widget for a reference to
/// themselves. This allows you to store a reference to a widget and render it later. Widget crates
/// should consider also doing this to allow for more flexibility in how widgets are used.
///
/// In Ratatui 0.26.0, we also added an unstable [`WidgetRef`] trait and implemented this on all the
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
/// Widget is also implemented for `&str` and `String` types.
///
/// # Examples
///
/// ```rust,no_run
/// use ratatui::{backend::TestBackend, prelude::*, widgets::*};
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
/// use ratatui::{prelude::*, widgets::*};
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

/// A `StatefulWidget` is a widget that can take advantage of some local state to remember things
/// between two draw calls.
///
/// Most widgets can be drawn directly based on the input parameters. However, some features may
/// require some kind of associated state to be implemented.
///
/// For example, the [`List`] widget can highlight the item currently selected. This can be
/// translated in an offset, which is the number of elements to skip in order to have the selected
/// item within the viewport currently allocated to this widget. The widget can therefore only
/// provide the following behavior: whenever the selected item is out of the viewport scroll to a
/// predefined position (making the selected item the last viewable item or the one in the middle
/// for example). Nonetheless, if the widget has access to the last computed offset then it can
/// implement a natural scrolling experience where the last offset is reused until the selected
/// item is out of the viewport.
///
/// ## Examples
///
/// ```rust,no_run
/// use std::io;
///
/// use ratatui::{backend::TestBackend, prelude::*, widgets::*};
///
/// // Let's say we have some events to display.
/// struct Events {
///     // `items` is the state managed by your application.
///     items: Vec<String>,
///     // `state` is the state that can be modified by the UI. It stores the index of the selected
///     // item as well as the offset computed during the previous draw call (used to implement
///     // natural scrolling).
///     state: ListState,
/// }
///
/// impl Events {
///     fn new(items: Vec<String>) -> Events {
///         Events {
///             items,
///             state: ListState::default(),
///         }
///     }
///
///     pub fn set_items(&mut self, items: Vec<String>) {
///         self.items = items;
///         // We reset the state as the associated items have changed. This effectively reset
///         // the selection as well as the stored offset.
///         self.state = ListState::default();
///     }
///
///     // Select the next item. This will not be reflected until the widget is drawn in the
///     // `Terminal::draw` callback using `Frame::render_stateful_widget`.
///     pub fn next(&mut self) {
///         let i = match self.state.selected() {
///             Some(i) => {
///                 if i >= self.items.len() - 1 {
///                     0
///                 } else {
///                     i + 1
///                 }
///             }
///             None => 0,
///         };
///         self.state.select(Some(i));
///     }
///
///     // Select the previous item. This will not be reflected until the widget is drawn in the
///     // `Terminal::draw` callback using `Frame::render_stateful_widget`.
///     pub fn previous(&mut self) {
///         let i = match self.state.selected() {
///             Some(i) => {
///                 if i == 0 {
///                     self.items.len() - 1
///                 } else {
///                     i - 1
///                 }
///             }
///             None => 0,
///         };
///         self.state.select(Some(i));
///     }
///
///     // Unselect the currently selected item if any. The implementation of `ListState` makes
///     // sure that the stored offset is also reset.
///     pub fn unselect(&mut self) {
///         self.state.select(None);
///     }
/// }
///
/// # let backend = TestBackend::new(5, 5);
/// # let mut terminal = Terminal::new(backend).unwrap();
///
/// let mut events = Events::new(vec![String::from("Item 1"), String::from("Item 2")]);
///
/// loop {
///     terminal.draw(|f| {
///         // The items managed by the application are transformed to something
///         // that is understood by ratatui.
///         let items: Vec<ListItem> = events
///             .items
///             .iter()
///             .map(|i| ListItem::new(i.as_str()))
///             .collect();
///         // The `List` widget is then built with those items.
///         let list = List::new(items);
///         // Finally the widget is rendered using the associated state. `events.state` is
///         // effectively the only thing that we will "remember" from this draw call.
///         f.render_stateful_widget(list, f.size(), &mut events.state);
///     });
///
///     // In response to some input events or an external http request or whatever:
///     events.next();
/// }
/// ```
pub trait StatefulWidget {
    /// State associated with the stateful widget.
    ///
    /// If you don't need this then you probably want to implement [`Widget`] instead.
    type State;
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom stateful widget.
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}

/// A `WidgetRef` is a trait that allows rendering a widget by reference.
///
/// This trait is useful when you want to store a reference to a widget and render it later. It also
/// allows you to render boxed widgets.
///
/// Boxed widgets allow you to store widgets with a type that is not known at compile time. This is
/// useful when you want to store a collection of widgets with different types. You can then iterate
/// over the collection and render each widget.
///
/// This trait was introduced in Ratatui 0.26.0 and is implemented for all the internal widgets. It
/// is currently marked as unstable as we are still evaluating the API and may make changes in the
/// future. See <https://github.com/ratatui/ratatui/issues/1287> for more information.
///
/// A blanket implementation of `Widget` for `&W` where `W` implements `WidgetRef` is provided.
///
/// A blanket implementation of `WidgetRef` for `Option<W>` where `W` implements `WidgetRef` is
/// provided. This is a convenience approach to make it easier to attach child widgets to parent
/// widgets. It allows you to render an optional widget by reference.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use ratatui::{prelude::*, widgets::*};
///
/// struct Greeting;
///
/// struct Farewell;
///
/// impl WidgetRef for Greeting {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Hello").render(area, buf);
///     }
/// }
///
/// /// Only needed for backwards compatibility
/// impl Widget for Greeting {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         self.render_ref(area, buf);
///     }
/// }
///
/// impl WidgetRef for Farewell {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Goodbye").right_aligned().render(area, buf);
///     }
/// }
///
/// /// Only needed for backwards compatibility
/// impl Widget for Farewell {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         self.render_ref(area, buf);
///     }
/// }
///
/// # fn render(area: Rect, buf: &mut Buffer) {
/// let greeting = Greeting;
/// let farewell = Farewell;
///
/// // these calls do not consume the widgets, so they can be used again later
/// greeting.render_ref(area, buf);
/// farewell.render_ref(area, buf);
///
/// // a collection of widgets with different types
/// let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(greeting), Box::new(farewell)];
/// for widget in widgets {
///     widget.render_ref(area, buf);
/// }
/// # }
/// # }
/// ```
#[instability::unstable(feature = "widget-ref")]
pub trait WidgetRef {
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom widget.
    fn render_ref(&self, area: Rect, buf: &mut Buffer);
}

/// This allows you to render a widget by reference.
impl<W: WidgetRef> Widget for &W {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

/// A blanket implementation of `WidgetExt` for `Option<W>` where `W` implements `WidgetRef`.
///
/// This is a convenience implementation that makes it easy to attach child widgets to parent
/// widgets. It allows you to render an optional widget by reference.
///
/// The internal widgets use this pattern to render the optional `Block` widgets that are included
/// on most widgets.
/// Blanket implementation of `WidgetExt` for `Option<W>` where `W` implements `WidgetRef`.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "unstable-widget-ref")] {
/// use ratatui::{prelude::*, widgets::*};
///
/// struct Parent {
///     child: Option<Child>,
/// }
///
/// struct Child;
///
/// impl WidgetRef for Child {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         Line::raw("Hello from child").render(area, buf);
///     }
/// }
///
/// impl WidgetRef for Parent {
///     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
///         self.child.render_ref(area, buf);
///     }
/// }
/// # }
/// ```
impl<W: WidgetRef> WidgetRef for Option<W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if let Some(widget) = self {
            widget.render_ref(area, buf);
        }
    }
}

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
/// use ratatui::{prelude::*, widgets::*};
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

/// Renders a string slice as a widget.
///
/// This implementation allows a string slice (`&str`) to act as a widget, meaning it can be drawn
/// onto a [`Buffer`] in a specified [`Rect`]. The slice represents a static string which can be
/// rendered by reference, thereby avoiding the need for string cloning or ownership transfer when
/// drawing the text to the screen.
impl Widget for &str {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

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

/// Renders a `String` object as a widget.
///
/// This implementation enables an owned `String` to be treated as a widget, which can be rendered
/// on a [`Buffer`] within the bounds of a given [`Rect`].
impl Widget for String {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;
    use crate::text::Line;

    #[fixture]
    fn buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 20, 1))
    }

    mod widget {
        use super::*;

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
    }

    mod widget_ref {
        use super::*;

        struct Greeting;
        struct Farewell;

        impl WidgetRef for Greeting {
            fn render_ref(&self, area: Rect, buf: &mut Buffer) {
                Line::from("Hello").render(area, buf);
            }
        }

        impl WidgetRef for Farewell {
            fn render_ref(&self, area: Rect, buf: &mut Buffer) {
                Line::from("Goodbye").right_aligned().render(area, buf);
            }
        }

        #[rstest]
        fn render_ref(mut buf: Buffer) {
            let widget = Greeting;
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        /// Ensure that the blanket implementation of `Widget` for `&W` where `W` implements
        /// `WidgetRef` works as expected.
        #[rstest]
        fn blanket_render(mut buf: Buffer) {
            let widget = &Greeting;
            widget.render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        #[rstest]
        fn box_render_ref(mut buf: Buffer) {
            let widget: Box<dyn WidgetRef> = Box::new(Greeting);
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        #[rstest]
        fn vec_box_render(mut buf: Buffer) {
            let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(Greeting), Box::new(Farewell)];
            for widget in widgets {
                widget.render_ref(buf.area, &mut buf);
            }
            assert_eq!(buf, Buffer::with_lines(["Hello        Goodbye"]));
        }
    }

    #[fixture]
    fn state() -> String {
        "world".to_string()
    }

    mod stateful_widget {
        use super::*;

        struct PersonalGreeting;

        impl StatefulWidget for PersonalGreeting {
            type State = String;
            fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
                Line::from(format!("Hello {state}")).render(area, buf);
            }
        }

        #[rstest]
        fn render(mut buf: Buffer, mut state: String) {
            let widget = PersonalGreeting;
            widget.render(buf.area, &mut buf, &mut state);
            assert_eq!(buf, Buffer::with_lines(["Hello world         "]));
        }
    }

    mod stateful_widget_ref {
        use super::*;

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

    mod option_widget_ref {
        use super::*;

        struct Greeting;

        impl WidgetRef for Greeting {
            fn render_ref(&self, area: Rect, buf: &mut Buffer) {
                Line::from("Hello").render(area, buf);
            }
        }

        #[rstest]
        fn render_ref_some(mut buf: Buffer) {
            let widget = Some(Greeting);
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello               "]));
        }

        #[rstest]
        fn render_ref_none(mut buf: Buffer) {
            let widget: Option<Greeting> = None;
            widget.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["                    "]));
        }
    }

    mod str {
        use super::*;

        #[rstest]
        fn render(mut buf: Buffer) {
            "hello world".render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn render_area(mut buf: Buffer) {
            let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
            "hello world, just hello".render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn render_ref(mut buf: Buffer) {
            "hello world".render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn option_render(mut buf: Buffer) {
            Some("hello world").render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn option_render_ref(mut buf: Buffer) {
            Some("hello world").render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }
    }

    mod string {
        use super::*;
        #[rstest]
        fn render(mut buf: Buffer) {
            String::from("hello world").render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn render_area(mut buf: Buffer) {
            let area = Rect::new(buf.area.x, buf.area.y, 11, buf.area.height);
            String::from("hello world, just hello").render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn render_ref(mut buf: Buffer) {
            String::from("hello world").render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn option_render(mut buf: Buffer) {
            Some(String::from("hello world")).render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }

        #[rstest]
        fn option_render_ref(mut buf: Buffer) {
            Some(String::from("hello world")).render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["hello world         "]));
        }
    }
}
