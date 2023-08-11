//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! All widgets are implemented using the builder pattern and are consumable objects. They are not
//! meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]
//! - [`Tabs`]
//! - [`List`]
//! - [`Table`]
//! - [`Paragraph`]
//! - [`Chart`]
//! - [`BarChart`]
//! - [`Gauge`]
//! - [`Sparkline`]
//! - [`calendar::Monthly`]
//! - [`Clear`]

mod barchart;
pub mod block;
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

use std::fmt::{self, Debug};

use bitflags::bitflags;

pub use self::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, BorderType, Padding},
    chart::{Axis, Chart, Dataset, GraphType},
    clear::Clear,
    gauge::{Gauge, LineGauge},
    list::{List, ListItem, ListState},
    paragraph::{Paragraph, Wrap},
    scrollbar::{ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState},
    sparkline::{RenderDirection, Sparkline},
    table::{Cell, HighlightSpacing, Row, Table, TableState},
    tabs::Tabs,
};
use crate::{buffer::Buffer, layout::Rect};

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Borders: u8 {
        /// Show no border (default)
        const NONE   = 0b0000;
        /// Show the top border
        const TOP    = 0b0001;
        /// Show the right border
        const RIGHT  = 0b0010;
        /// Show the bottom border
        const BOTTOM = 0b0100;
        /// Show the left border
        const LEFT   = 0b1000;
        /// Show all borders
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

/// Implement the `Debug` trait for the `Borders` bitflags. This is a manual implementation to
/// display the flags in a more readable way. The default implementation would display the
/// flags as 'Border(0x0)' for `Borders::NONE` for example.
impl Debug for Borders {
    /// Display the Borders bitflags as a list of names. For example, `Borders::NONE` will be
    /// displayed as `NONE` and `Borders::ALL` will be displayed as `ALL`. If multiple flags are
    /// set, they will be displayed separated by a pipe character.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        if self.is_all() {
            return write!(f, "ALL");
        }
        let mut first = true;
        for (name, border) in self.iter_names() {
            if border == Borders::NONE {
                continue;
            }
            if first {
                write!(f, "{name}")?;
                first = false;
            } else {
                write!(f, " | {name}")?;
            }
        }
        Ok(())
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That is the only method required
    /// to implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer);
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
/// # use std::io;
/// # use ratatui::Terminal;
/// # use ratatui::backend::{Backend, TestBackend};
/// # use ratatui::widgets::{Widget, List, ListItem, ListState};
///
/// // Let's say we have some events to display.
/// struct Events {
///     // `items` is the state managed by your application.
///     items: Vec<String>,
///     // `state` is the state that can be modified by the UI. It stores the index of the selected
///     // item as well as the offset computed during the previous draw call (used to implement
///     // natural scrolling).
///     state: ListState
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
/// let mut events = Events::new(vec![
///     String::from("Item 1"),
///     String::from("Item 2")
/// ]);
///
/// loop {
///     terminal.draw(|f| {
///         // The items managed by the application are transformed to something
///         // that is understood by ratatui.
///         let items: Vec<ListItem>= events.items.iter().map(|i| ListItem::new(i.as_str())).collect();
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
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}

/// Macro that constructs and returns a [`Borders`] object from TOP, BOTTOM, LEFT, RIGHT, NONE, and
/// ALL. Internally it creates an empty `Borders` object and then inserts each bit flag specified
/// into it using `Borders::insert()`.
///
/// ## Examples
///
///```
/// # use ratatui::widgets::{Block, Borders};
/// # use ratatui::style::{Style, Color};
/// # use ratatui::border;
///
/// Block::default()
///     //Construct a `Borders` object and use it in place
///     .borders(border!(TOP, BOTTOM));
///
/// //`border!` can be called with any order of individual sides
/// let bottom_first = border!(BOTTOM, LEFT, TOP);
/// //with the ALL keyword which works as expected
/// let all = border!(ALL);
/// //or with nothing to return a `Borders::NONE' bitflag.
/// let none = border!(NONE);
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! border {
    ( $($b:tt), +) => {{
            let mut border = Borders::empty();
            $(
               border.insert(Borders::$b);
            )*
            border
    }};
    () =>{
        Borders::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borders_debug() {
        assert_eq!(format!("{:?}", Borders::empty()), "NONE");
        assert_eq!(format!("{:?}", Borders::NONE), "NONE");
        assert_eq!(format!("{:?}", Borders::TOP), "TOP");
        assert_eq!(format!("{:?}", Borders::BOTTOM), "BOTTOM");
        assert_eq!(format!("{:?}", Borders::LEFT), "LEFT");
        assert_eq!(format!("{:?}", Borders::RIGHT), "RIGHT");
        assert_eq!(format!("{:?}", Borders::ALL), "ALL");
        assert_eq!(format!("{:?}", Borders::all()), "ALL");

        assert_eq!(
            format!("{:?}", Borders::TOP | Borders::BOTTOM),
            "TOP | BOTTOM"
        );
    }
}
