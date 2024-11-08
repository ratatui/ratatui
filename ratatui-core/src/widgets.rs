#![warn(missing_docs)]
//! The `widgets` module contains the `Widget` and `StatefulWidget` traits, which are used to
//! render UI elements on the screen.

pub use self::{stateful_widget::StatefulWidget, widget::Widget};
#[instability::unstable(feature = "widget-ref")]
pub use self::{stateful_widget_ref::StatefulWidgetRef, widget_ref::WidgetRef};

mod stateful_widget;
mod stateful_widget_ref;
mod widget;
mod widget_ref;
