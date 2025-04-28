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
//! - [`RatatuiLogo`]: displays the Ratatui logo.
//! - [`RatatuiMascot`]: displays the Ratatui mascot.
//!
//! [`Canvas`]: crate::widgets::canvas::Canvas

pub use ratatui_core::widgets::{StatefulWidget, Widget};
pub use ratatui_widgets::barchart::{Bar, BarChart, BarGroup};
// TODO remove this module once title etc. are gone
pub use ratatui_widgets::block;
pub use ratatui_widgets::block::{Block, Padding};
pub use ratatui_widgets::borders::{BorderType, Borders};
#[cfg(feature = "widget-calendar")]
pub use ratatui_widgets::calendar;
pub use ratatui_widgets::canvas;
pub use ratatui_widgets::chart::{Axis, Chart, Dataset, GraphType, LegendPosition};
pub use ratatui_widgets::clear::Clear;
pub use ratatui_widgets::gauge::{Gauge, LineGauge};
pub use ratatui_widgets::list::{List, ListDirection, ListItem, ListState};
pub use ratatui_widgets::logo::{RatatuiLogo, Size as RatatuiLogoSize};
pub use ratatui_widgets::mascot::{MascotEyeColor, RatatuiMascot};
pub use ratatui_widgets::paragraph::{Paragraph, Wrap};
pub use ratatui_widgets::scrollbar::{
    ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
pub use ratatui_widgets::sparkline::{RenderDirection, Sparkline, SparklineBar};
pub use ratatui_widgets::table::{Cell, HighlightSpacing, Row, Table, TableState};
pub use ratatui_widgets::tabs::Tabs;
#[instability::unstable(feature = "widget-ref")]
pub use {stateful_widget_ref::StatefulWidgetRef, widget_ref::WidgetRef};

mod stateful_widget_ref;
mod widget_ref;

use ratatui_core::layout::Rect;

/// Extension trait for [`Frame`] that provides methods to render [`WidgetRef`] and
/// [`StatefulWidgetRef`] to the current buffer.
#[instability::unstable(feature = "widget-ref")]
pub trait FrameExt {
    /// Render a [`WidgetRef`] to the current buffer using [`WidgetRef::render_ref`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[cfg(feature = "unstable-widget-ref")] {
    /// # use ratatui::{backend::TestBackend, Terminal};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// use ratatui::layout::Rect;
    /// use ratatui::widgets::{Block, FrameExt};
    ///
    /// let block = Block::new();
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_widget_ref(&block, area);
    /// # }
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    fn render_widget_ref<W: WidgetRef>(&mut self, widget: W, area: Rect);

    /// Render a [`StatefulWidgetRef`] to the current buffer using
    /// [`StatefulWidgetRef::render_ref`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// The last argument should be an instance of the [`StatefulWidgetRef::State`] associated to
    /// the given [`StatefulWidgetRef`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[cfg(feature = "unstable-widget-ref")] {
    /// # use ratatui::{backend::TestBackend, Terminal};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// use ratatui::layout::Rect;
    /// use ratatui::widgets::{FrameExt, List, ListItem, ListState};
    ///
    /// let mut state = ListState::default().with_selected(Some(1));
    /// let list = List::new(vec![ListItem::new("Item 1"), ListItem::new("Item 2")]);
    /// let area = Rect::new(0, 0, 5, 5);
    /// frame.render_stateful_widget_ref(&list, area, &mut state);
    /// # }
    /// ```
    /// [`Layout`]: crate::layout::Layout
    fn render_stateful_widget_ref<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidgetRef;
}

#[cfg(feature = "unstable-widget-ref")]
impl FrameExt for ratatui_core::terminal::Frame<'_> {
    fn render_widget_ref<W: WidgetRef>(&mut self, widget: W, area: Rect) {
        widget.render_ref(area, self.buffer_mut());
    }

    fn render_stateful_widget_ref<W>(&mut self, widget: W, area: Rect, state: &mut W::State)
    where
        W: StatefulWidgetRef,
    {
        widget.render_ref(area, self.buffer_mut(), state);
    }
}
