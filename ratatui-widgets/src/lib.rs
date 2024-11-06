#![warn(missing_docs)]
//! `ratatui-widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or
//! both.
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
mod barchart;
pub mod block;
mod borders;
pub mod canvas;
mod chart;
mod clear;
mod gauge;
mod list;
mod logo;
mod paragraph;
mod reflow;
mod scrollbar;
mod sparkline;
mod table;
mod tabs;

#[cfg(feature = "calendar")]
pub mod calendar;

pub use ratatui_core::widgets::{StatefulWidget, Widget};
#[instability::unstable(feature = "widget-ref")]
pub use ratatui_core::widgets::{StatefulWidgetRef, WidgetRef};

pub use self::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, BorderType, Padding},
    borders::*,
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
