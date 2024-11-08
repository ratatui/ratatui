#![warn(missing_docs)]
//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! The widgets provided with Ratatui are implemented in the [`ratatui_widgets`] crate, and are
//! re-exported here. The [`Widget`] and [`StatefulWidget`] traits are implemented in the
//! [`ratatui_core`] crate and are also re-exported in this module. This means that you can use
//! these types directly from the `ratatui` crate without having to import the `ratatui_widgets`
//! crate.
//!
//! Widgets are created for each frame as they are consumed after rendered. They are not meant to be
//! stored but used as *commands* to draw common figures in the UI.
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

pub use ratatui_widgets::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, Padding},
    borders::{BorderType, Borders},
    canvas,
    chart::{Axis, Chart, Dataset, GraphType, LegendPosition},
    clear::Clear,
    gauge::{Gauge, LineGauge},
    list::{List, ListDirection, ListItem, ListState},
    logo::{RatatuiLogo, Size as RatatuiLogoSize},
    paragraph::{Paragraph, Wrap},
    scrollbar::{ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState},
    sparkline::{RenderDirection, Sparkline, SparklineBar},
    table::{Cell, HighlightSpacing, Row, Table, TableState},
    tabs::Tabs,
};

// TODO remove this module once title etc. are gone
pub use ratatui_widgets::block;

#[cfg(feature = "widget-calendar")]
pub use ratatui_widgets::calendar;

pub use ratatui_core::widgets::{StatefulWidget, Widget};
#[instability::unstable(feature = "widget-ref")]
pub use ratatui_core::widgets::{StatefulWidgetRef, WidgetRef};
