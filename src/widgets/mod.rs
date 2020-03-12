use bitflags::bitflags;
use std::borrow::Cow;

mod barchart;
mod block;
pub mod canvas;
mod chart;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod sparkline;
mod table;
mod tabs;

pub use self::barchart::BarChart;
pub use self::block::{Block, BorderType};
pub use self::chart::{Axis, Chart, Dataset, GraphType, Marker};
pub use self::gauge::Gauge;
pub use self::list::{List, ListState};
pub use self::paragraph::Paragraph;
pub use self::sparkline::Sparkline;
pub use self::table::{Row, Table, TableState};
pub use self::tabs::Tabs;

use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    pub struct Borders: u32 {
        /// Show no border (default)
        const NONE  = 0b0000_0001;
        /// Show the top border
        const TOP   = 0b0000_0010;
        /// Show the right border
        const RIGHT = 0b0000_0100;
        /// Show the bottom border
        const BOTTOM = 0b000_1000;
        /// Show the left border
        const LEFT = 0b0001_0000;
        /// Show all borders
        const ALL = Self::TOP.bits | Self::RIGHT.bits | Self::BOTTOM.bits | Self::LEFT.bits;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Text<'b> {
    Raw(Cow<'b, str>),
    Styled(Cow<'b, str>, Style),
}

impl<'b> Text<'b> {
    pub fn raw<D: Into<Cow<'b, str>>>(data: D) -> Text<'b> {
        Text::Raw(data.into())
    }

    pub fn styled<D: Into<Cow<'b, str>>>(data: D, style: Style) -> Text<'b> {
        Text::Styled(data.into(), style)
    }
}

/// Base requirements for a Widget
pub trait Widget {
    /// Draws the current state of the widget in the given buffer. That the only method required to
    /// implement a custom widget.
    fn render(self, area: Rect, buf: &mut Buffer);
}

pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
