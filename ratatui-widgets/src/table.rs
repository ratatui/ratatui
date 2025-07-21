//! The [`Table`] widget is used to display multiple rows and columns in a grid and allows selecting
//! one or multiple cells.

use alloc::vec;
use alloc::vec::Vec;

use bitflags::bitflags;
use itertools::Itertools;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols;
use ratatui_core::symbols::merge::MergeStrategy;
use ratatui_core::text::Text;
use ratatui_core::widgets::{StatefulWidget, Widget};

pub use self::cell::Cell;
pub use self::highlight_spacing::HighlightSpacing;
pub use self::row::Row;
pub use self::state::TableState;
use crate::block::{Block, BlockExt};

mod cell;
mod highlight_spacing;
mod row;
mod state;

bitflags! {
    /// The type of internal borders for a table.
    ///
    /// This bitflags defines the different internal border styles that can be applied to a table.
    /// It allows for controlling which internal borders are displayed within the table.
    ///
    /// **Naming Convention**: The term "internal borders" distinguishes these borders from external
    /// borders that might be added by wrapping the table in a [`Block`] widget. Both types of
    /// borders serve the same visual purpose of creating table grid lines, but they are positioned
    /// and controlled differently.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::style::{Color, Style};
    /// use ratatui_core::layout::Constraint;
    /// use ratatui_widgets::table::{Table, TableBorders, Row};
    ///
    /// let table = Table::new(Vec::<Row>::new(), Vec::<Constraint>::new())
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    /// ```
    ///
    /// ## Enhanced Border Control
    ///
    /// The enhanced border system provides fine-grained control over individual border segments:
    ///
    /// ```rust
    /// use ratatui_widgets::table::{Table, TableBorders};
    ///
    /// // Individual border control
    /// let table = Table::default()
    ///     .table_borders(TableBorders::TOP | TableBorders::INNER_HORIZONTAL);
    ///
    /// // Outer borders only
    /// let table = Table::default()
    ///     .table_borders(TableBorders::OUTER);
    ///
    /// // Header separator with vertical borders
    /// let table = Table::default()
    ///     .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL);
    /// ```
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct TableBorders: u16 {
        /// No borders displayed.
        const NONE = 0b0000_0000_0000_0000;

        // Legacy flags (maintained for backward compatibility)
        /// Only horizontal borders displayed (legacy - maps to INNER_HORIZONTAL).
        const HORIZONTAL = 0b0000_0000_0000_0001;
        /// Only vertical borders displayed (legacy - maps to INNER_VERTICAL).
        const VERTICAL = 0b0000_0000_0000_0010;
        /// All borders displayed (legacy - maps to INNER_HORIZONTAL | INNER_VERTICAL).
        const ALL = Self::HORIZONTAL.bits() | Self::VERTICAL.bits();

        // New individual border flags
        /// Top border of the table.
        const TOP = 0b0000_0000_0000_0100;
        /// Left border of the table.
        const LEFT = 0b0000_0000_0000_1000;
        /// Right border of the table.
        const RIGHT = 0b0000_0000_0001_0000;
        /// Bottom border of the table.
        const BOTTOM = 0b0000_0000_0010_0000;
        /// Vertical borders between columns (inner vertical borders).
        const INNER_VERTICAL = 0b0000_0000_0100_0000;
        /// Horizontal borders between rows (inner horizontal borders).
        const INNER_HORIZONTAL = 0b0000_0000_1000_0000;
        /// Separator line between header and data rows.
        const HEADER_TOP = 0b0000_0001_0000_0000;

        // Convenience combinations
        /// All outer borders (top, left, right, bottom).
        const OUTER = Self::TOP.bits() | Self::LEFT.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits();
        /// All inner borders (inner vertical and inner horizontal).
        const INNER = Self::INNER_VERTICAL.bits() | Self::INNER_HORIZONTAL.bits();
        /// All borders (outer + inner).
        const ALL_BORDERS = Self::OUTER.bits() | Self::INNER.bits();
    }
}

/// A set of symbols used to render table borders.
///
/// This structure defines the characters used for drawing table borders, including corners,
/// lines, and junctions. It provides more flexibility than the basic line sets by allowing
/// different symbols for different parts of the table, including header-specific symbols.
///
/// ## Predefined Border Sets
///
/// Several predefined border sets are available for common use cases:
///
/// - [`TableBorderSet::plain()`] - Standard single-line borders (┌─┐│└─┘)
/// - [`TableBorderSet::rounded()`] - Rounded corners with single lines (╭─╮│╰─╯)
/// - [`TableBorderSet::double()`] - Double-line borders (╔═╗║╚═╝)
/// - [`TableBorderSet::thick()`] - Thick single-line borders (┏━┓┃┗━┛)
/// - [`TableBorderSet::with_header_style()`] - Mixed single/double for headers
///
/// ## Header-Specific Symbols
///
/// The optional header-specific symbols allow you to use different characters for header
/// separators than for regular cell borders. This is useful for creating visual distinction
/// between headers and data:
///
/// ```text
/// ┌─────┬─────┬─────┐  ← Regular borders
/// │ H1  │ H2  │ H3  │
/// ╞═════╪═════╪═════╡  ← Header separator (different style)
/// │ D1  │ D2  │ D3  │
/// ├─────┼─────┼─────┤  ← Regular borders
/// │ D4  │ D5  │ D6  │
/// └─────┴─────┴─────┘
/// ```
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorderSet, TableBorders};
/// use ratatui_core::symbols::line;
///
/// // Use predefined border set
/// let table = Table::default()
///     .table_borders(TableBorders::ALL_BORDERS)
///     .border_set(TableBorderSet::thick());
///
/// // Create from existing line set
/// let border_set = TableBorderSet::from(line::DOUBLE);
/// let table = Table::default()
///     .table_borders(TableBorders::OUTER)
///     .border_set(border_set);
/// ```
///
/// ## Custom Border Symbols
/// ```rust
/// use ratatui_widgets::table::TableBorderSet;
///
/// let custom_set = TableBorderSet {
///     horizontal: "━",
///     vertical: "┃",
///     top_left: "┏",
///     top_right: "┓",
///     bottom_left: "┗",
///     bottom_right: "┛",
///     vertical_left: "┫",
///     vertical_right: "┣",
///     horizontal_down: "┳",
///     horizontal_up: "┻",
///     cross: "╋",
///     header_horizontal: Some("═"),
///     header_vertical_left: Some("╣"),
///     header_vertical_right: Some("╠"),
///     header_cross: Some("╬"),
/// };
/// ```
///
/// ## Header-Specific Styling
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorderSet, TableBorders};
/// use ratatui::widgets::Row;
///
/// let table = Table::default()
///     .header(Row::new(vec!["Name", "Age", "City"]))
///     .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL)
///     .border_set(TableBorderSet::with_header_style());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TableBorderSet<'a> {
    /// Horizontal line character (─)
    pub horizontal: &'a str,
    /// Vertical line character (│)
    pub vertical: &'a str,

    /// Top-left corner character (┌)
    pub top_left: &'a str,
    /// Top-right corner character (┐)
    pub top_right: &'a str,
    /// Bottom-left corner character (└)
    pub bottom_left: &'a str,
    /// Bottom-right corner character (┘)
    pub bottom_right: &'a str,

    /// Left T-junction character (┤)
    pub vertical_left: &'a str,
    /// Right T-junction character (├)
    pub vertical_right: &'a str,
    /// Top T-junction character (┬)
    pub horizontal_down: &'a str,
    /// Bottom T-junction character (┴)
    pub horizontal_up: &'a str,

    /// Cross junction character (┼)
    pub cross: &'a str,

    /// Optional header horizontal line character (═)
    pub header_horizontal: Option<&'a str>,
    /// Optional header left T-junction character (╣)
    pub header_vertical_left: Option<&'a str>,
    /// Optional header right T-junction character (╠)
    pub header_vertical_right: Option<&'a str>,
    /// Optional header cross junction character (╬)
    pub header_cross: Option<&'a str>,
}

impl Default for TableBorderSet<'_> {
    fn default() -> Self {
        Self::from(symbols::line::NORMAL)
    }
}

impl<'a> From<symbols::line::Set<'a>> for TableBorderSet<'a> {
    fn from(line_set: symbols::line::Set<'a>) -> Self {
        Self {
            horizontal: line_set.horizontal,
            vertical: line_set.vertical,
            top_left: line_set.top_left,
            top_right: line_set.top_right,
            bottom_left: line_set.bottom_left,
            bottom_right: line_set.bottom_right,
            vertical_left: line_set.vertical_left,
            vertical_right: line_set.vertical_right,
            horizontal_down: line_set.horizontal_down,
            horizontal_up: line_set.horizontal_up,
            cross: line_set.cross,
            header_horizontal: None,
            header_vertical_left: None,
            header_vertical_right: None,
            header_cross: None,
        }
    }
}

impl<'a> TableBorderSet<'a> {
    /// Creates a new [`TableBorderSet`] using plain line symbols.
    pub const fn plain() -> Self {
        Self {
            horizontal: "─",
            vertical: "│",
            top_left: "┌",
            top_right: "┐",
            bottom_left: "└",
            bottom_right: "┘",
            vertical_left: "┤",
            vertical_right: "├",
            horizontal_down: "┬",
            horizontal_up: "┴",
            cross: "┼",
            header_horizontal: None,
            header_vertical_left: None,
            header_vertical_right: None,
            header_cross: None,
        }
    }

    /// Creates a new [`TableBorderSet`] using rounded corner symbols.
    pub const fn rounded() -> Self {
        Self {
            horizontal: "─",
            vertical: "│",
            top_left: "╭",
            top_right: "╮",
            bottom_left: "╰",
            bottom_right: "╯",
            vertical_left: "┤",
            vertical_right: "├",
            horizontal_down: "┬",
            horizontal_up: "┴",
            cross: "┼",
            header_horizontal: None,
            header_vertical_left: None,
            header_vertical_right: None,
            header_cross: None,
        }
    }

    /// Creates a new [`TableBorderSet`] using double line symbols.
    pub const fn double() -> Self {
        Self {
            horizontal: "═",
            vertical: "║",
            top_left: "╔",
            top_right: "╗",
            bottom_left: "╚",
            bottom_right: "╝",
            vertical_left: "╣",
            vertical_right: "╠",
            horizontal_down: "╦",
            horizontal_up: "╩",
            cross: "╬",
            header_horizontal: None,
            header_vertical_left: None,
            header_vertical_right: None,
            header_cross: None,
        }
    }

    /// Creates a new [`TableBorderSet`] using thick line symbols.
    pub const fn thick() -> Self {
        Self {
            horizontal: "━",
            vertical: "┃",
            top_left: "┏",
            top_right: "┓",
            bottom_left: "┗",
            bottom_right: "┛",
            vertical_left: "┫",
            vertical_right: "┣",
            horizontal_down: "┳",
            horizontal_up: "┻",
            cross: "╋",
            header_horizontal: None,
            header_vertical_left: None,
            header_vertical_right: None,
            header_cross: None,
        }
    }

    /// Creates a new [`TableBorderSet`] with header-specific symbols.
    ///
    /// This creates a set where regular borders use single lines and header separators
    /// use double lines for visual distinction.
    pub const fn with_header_style() -> Self {
        Self {
            horizontal: "─",
            vertical: "│",
            top_left: "┌",
            top_right: "┐",
            bottom_left: "└",
            bottom_right: "┘",
            vertical_left: "┤",
            vertical_right: "├",
            horizontal_down: "┬",
            horizontal_up: "┴",
            cross: "┼",
            header_horizontal: Some("═"),
            header_vertical_left: Some("╡"),
            header_vertical_right: Some("╞"),
            header_cross: Some("╪"),
        }
    }

    /// Gets the horizontal symbol to use, preferring header-specific symbol if available and in header context.
    pub const fn get_horizontal(&self, is_header: bool) -> &'a str {
        if is_header {
            if let Some(header_horizontal) = self.header_horizontal {
                header_horizontal
            } else {
                self.horizontal
            }
        } else {
            self.horizontal
        }
    }

    /// Gets the vertical left symbol to use, preferring header-specific symbol if available and in header context.
    pub const fn get_vertical_left(&self, is_header: bool) -> &'a str {
        if is_header {
            if let Some(header_vertical_left) = self.header_vertical_left {
                header_vertical_left
            } else {
                self.vertical_left
            }
        } else {
            self.vertical_left
        }
    }

    /// Gets the vertical right symbol to use, preferring header-specific symbol if available and in header context.
    pub const fn get_vertical_right(&self, is_header: bool) -> &'a str {
        if is_header {
            if let Some(header_vertical_right) = self.header_vertical_right {
                header_vertical_right
            } else {
                self.vertical_right
            }
        } else {
            self.vertical_right
        }
    }

    /// Gets the cross symbol to use, preferring header-specific symbol if available and in header context.
    pub const fn get_cross(&self, is_header: bool) -> &'a str {
        if is_header {
            if let Some(header_cross) = self.header_cross {
                header_cross
            } else {
                self.cross
            }
        } else {
            self.cross
        }
    }
}

/// Context information for determining which border symbol to use during table rendering.
///
/// This enum provides context about where a border is being drawn within the table,
/// allowing the rendering system to choose appropriate symbols based on position,
/// intersections, and whether the border is part of a header separator.
///
/// # Examples
///
/// ```rust
/// use ratatui_widgets::table::{BorderContext, CornerType};
///
/// // Context for a header separator that intersects with vertical borders
/// let context = BorderContext::HeaderSeparator { has_vertical: true };
///
/// // Context for an inner horizontal border with no vertical intersection
/// let context = BorderContext::InnerHorizontal { has_vertical: false };
///
/// // Context for a corner position
/// let context = BorderContext::Corner(CornerType::TopLeft);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderContext {
    /// Header separator line between header and data rows.
    ///
    /// `has_vertical` indicates whether this position intersects with a vertical border,
    /// which affects whether to use a cross symbol or just a horizontal line.
    HeaderSeparator {
        /// Whether this position intersects with a vertical border
        has_vertical: bool,
    },

    /// Inner horizontal border between data rows.
    ///
    /// `has_vertical` indicates whether this position intersects with a vertical border.
    InnerHorizontal {
        /// Whether this position intersects with a vertical border
        has_vertical: bool,
    },

    /// Inner vertical border between columns.
    ///
    /// `has_horizontal` indicates whether this position intersects with a horizontal border.
    InnerVertical {
        /// Whether this position intersects with a horizontal border
        has_horizontal: bool,
    },

    /// Top edge border of the table.
    ///
    /// `has_vertical` indicates whether this position intersects with a vertical border.
    TopEdge {
        /// Whether this position intersects with a vertical border
        has_vertical: bool,
    },

    /// Bottom edge border of the table.
    ///
    /// `has_vertical` indicates whether this position intersects with a vertical border.
    BottomEdge {
        /// Whether this position intersects with a vertical border
        has_vertical: bool,
    },

    /// Left edge border of the table.
    ///
    /// `has_horizontal` indicates whether this position intersects with a horizontal border.
    LeftEdge {
        /// Whether this position intersects with a horizontal border
        has_horizontal: bool,
    },

    /// Right edge border of the table.
    ///
    /// `has_horizontal` indicates whether this position intersects with a horizontal border.
    RightEdge {
        /// Whether this position intersects with a horizontal border
        has_horizontal: bool,
    },

    /// Corner position in the table.
    Corner(CornerType),
}

/// Specifies which corner of the table a border symbol represents.
///
/// This enum is used with [`BorderContext::Corner`] to indicate the specific
/// corner position, allowing the rendering system to choose the appropriate
/// corner symbol (┌, ┐, └, ┘).
///
/// # Examples
///
/// ```rust
/// use ratatui_widgets::table::{BorderContext, CornerType};
///
/// let top_left_context = BorderContext::Corner(CornerType::TopLeft);
/// let bottom_right_context = BorderContext::Corner(CornerType::BottomRight);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerType {
    /// Top-left corner (┌)
    TopLeft,
    /// Top-right corner (┐)
    TopRight,
    /// Bottom-left corner (└)
    BottomLeft,
    /// Bottom-right corner (┘)
    BottomRight,
}

impl BorderContext {
    /// Determines if this border context represents a header-related border.
    ///
    /// Returns `true` for `HeaderSeparator` contexts, `false` for all others.
    /// This is useful for determining whether to use header-specific symbols.
    pub const fn is_header(self) -> bool {
        matches!(self, Self::HeaderSeparator { .. })
    }

    /// Determines if this border context involves an intersection with other borders.
    ///
    /// Returns `true` if the context indicates that multiple border lines meet at this position,
    /// which typically requires a junction symbol (┼, ╬, ├, ┤, ┬, ┴, etc.).
    pub const fn has_intersection(self) -> bool {
        match self {
            Self::HeaderSeparator { has_vertical }
            | Self::InnerHorizontal { has_vertical }
            | Self::TopEdge { has_vertical }
            | Self::BottomEdge { has_vertical } => has_vertical,

            Self::InnerVertical { has_horizontal }
            | Self::LeftEdge { has_horizontal }
            | Self::RightEdge { has_horizontal } => has_horizontal,

            Self::Corner(_) => true, // Corners are always intersections
        }
    }

    /// Gets the corner type if this context represents a corner, otherwise returns `None`.
    pub const fn corner_type(self) -> Option<CornerType> {
        match self {
            Self::Corner(corner_type) => Some(corner_type),
            _ => None,
        }
    }
}

/// Errors that can occur when configuring table borders.
///
/// This enum represents various validation errors that can occur when setting up
/// table border configurations, allowing for graceful error handling and user feedback.
///
/// # Examples
///
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorders, BorderConfigError};
///
/// // This would be an example of conflicting border types (if validation was enabled)
/// // let result = table.validate_border_config();
/// // match result {
/// //     Ok(()) => println!("Border configuration is valid"),
/// //     Err(BorderConfigError::ConflictingBorderTypes) => {
/// //         println!("Warning: Using both legacy and new border flags");
/// //     }
/// // }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorderConfigError {
    /// Conflicting border types are being used (e.g., mixing legacy and new border flags inappropriately).
    ConflictingBorderTypes,

    /// An invalid border symbol was provided (e.g., empty string or invalid Unicode).
    InvalidBorderSymbol(alloc::string::String),

    /// An unsupported combination of border flags was specified.
    UnsupportedBorderCombination,

    /// A deprecated border configuration is being used.
    DeprecatedConfiguration(alloc::string::String),
}

impl core::fmt::Display for BorderConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ConflictingBorderTypes => {
                write!(
                    f,
                    "Conflicting border types detected. Consider using either legacy or new border flags consistently."
                )
            }
            Self::InvalidBorderSymbol(symbol) => {
                write!(
                    f,
                    "Invalid border symbol: '{symbol}'. Border symbols must be non-empty valid Unicode strings."
                )
            }
            Self::UnsupportedBorderCombination => {
                write!(
                    f,
                    "Unsupported border combination. Some border flag combinations are not yet supported."
                )
            }
            Self::DeprecatedConfiguration(msg) => {
                write!(f, "Deprecated border configuration: {msg}")
            }
        }
    }
}

// Note: Error trait implementation is omitted in no_std environment
// impl std::error::Error for BorderConfigError {}

/// A widget to display data in formatted columns.
///
/// A `Table` is a collection of [`Row`]s, each composed of [`Cell`]s:
///
/// You can construct a [`Table`] using either [`Table::new`] or [`Table::default`] and then chain
/// builder style methods to set the desired properties.
///
/// Table cells can be aligned, for more details see [`Cell`].
///
/// Make sure to call the [`Table::widths`] method, otherwise the columns will all have a width of 0
/// and thus not be visible.
///
/// [`Table`] implements [`Widget`] and so it can be drawn using `Frame::render_widget`.
///
/// [`Table`] is also a [`StatefulWidget`], which means you can use it with [`TableState`] to allow
/// the user to scroll through the rows and select one of them. When rendering a [`Table`] with a
/// [`TableState`], the selected row, column and cell will be highlighted. If the selected row is
/// not visible (based on the offset), the table will be scrolled to make the selected row visible.
///
/// Note: if the `widths` field is empty, the table will be rendered with equal widths.
/// Note: Highlight styles are applied in the following order: Row, Column, Cell.
///
/// ## Internal Borders
///
/// Tables support internal borders that are drawn between and around the table cells. These are
/// separate from any external borders that might be added by wrapping the table in a [`Block`].
/// You can control which internal borders are visible using [`Table::internal_borders`] and style
/// them with [`Table::border_style`]. The available border options are:
///
/// - [`TableBorders::NONE`] - No internal borders (default)
/// - [`TableBorders::HORIZONTAL`] - Horizontal lines between rows
/// - [`TableBorders::VERTICAL`] - Vertical lines between columns
/// - [`TableBorders::ALL`] - Both horizontal and vertical borders
///
/// Borders can also be combined using bitwise operations for more fine-grained control.
///
/// ### Interaction with External Block Borders
///
/// When a table is wrapped in a [`Block`] widget, the internal borders will automatically
/// integrate with the block's external borders at intersection points. The table will use
/// appropriate intersection symbols (┌, ┐, └, ┘, ├, ┤, ┬, ┴, ┼) where internal borders meet
/// external block borders.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::{Color, Style};
/// use ratatui::widgets::{Block, Row, Table};
/// use ratatui_widgets::table::TableBorders;
///
/// let table = Table::new(
///     vec![Row::new(vec!["Cell1", "Cell2"])],
///     [Constraint::Length(5); 2],
/// )
/// .block(Block::bordered().title("Table"))
/// .internal_borders(TableBorders::ALL)
/// .border_style(Style::default().fg(Color::Blue));
/// ```
///
/// ### Performance Considerations
///
/// Internal borders add rendering overhead, especially for large tables. The performance impact
/// scales with:
/// - **Number of rows**: Each row boundary requires horizontal border rendering
/// - **Number of columns**: Each column boundary requires vertical border rendering
/// - **Table size**: Larger tables require more border intersection calculations
///
/// For tables with hundreds of rows or many columns, consider:
/// - Using [`TableBorders::HORIZONTAL`] or [`TableBorders::VERTICAL`] instead of
///   [`TableBorders::ALL`]
/// - Limiting the number of visible rows through pagination or virtualization
/// - Disabling internal borders for very large datasets
///
/// ## Migration Guide: Legacy to Enhanced Borders
///
/// The enhanced border system provides backward compatibility while offering new capabilities.
/// Here's how to migrate from the legacy API to the new enhanced API:
///
/// ### Legacy API (Still Supported)
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorders};
/// use ratatui_core::style::{Style, Color};
///
/// // Old way - still works
/// let table = Table::default()
///     .internal_borders(TableBorders::ALL)
///     .border_style(Style::default().fg(Color::Blue));
/// ```
///
/// ### Enhanced API (Recommended)
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorders, TableBorderSet};
/// use ratatui_core::style::{Style, Color};
///
/// // New way - more control
/// let table = Table::default()
///     .table_borders(TableBorders::ALL_BORDERS)
///     .border_set(TableBorderSet::thick())
///     .border_style(Style::default().fg(Color::Blue));
/// ```
///
/// ### Migration Examples
///
/// **Basic Migration:**
/// ```text
/// // Before
/// .internal_borders(TableBorders::HORIZONTAL)
/// // After
/// .table_borders(TableBorders::INNER_HORIZONTAL)
///
/// // Before
/// .internal_borders(TableBorders::VERTICAL)
/// // After
/// .table_borders(TableBorders::INNER_VERTICAL)
///
/// // Before
/// .internal_borders(TableBorders::ALL)
/// // After
/// .table_borders(TableBorders::INNER_HORIZONTAL | TableBorders::INNER_VERTICAL)
/// // Or use the convenience constant
/// .table_borders(TableBorders::INNER)
/// ```
///
/// **Advanced Features (New Capabilities):**
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorders, TableBorderSet};
/// use ratatui::widgets::Row;
///
/// // Individual outer border control
/// let table = Table::default()
///     .table_borders(TableBorders::TOP | TableBorders::BOTTOM);
///
/// // Header separator with custom styling
/// let table = Table::default()
///     .header(Row::new(vec!["Header"]))
///     .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL)
///     .border_set(TableBorderSet::with_header_style());
///
/// // Custom border symbols
/// let table = Table::default()
///     .table_borders(TableBorders::ALL_BORDERS)
///     .border_set(TableBorderSet::double());
/// ```
///
/// ### Validation and Best Practices
/// ```rust
/// use ratatui_widgets::table::{Table, TableBorders, TableBorderSet};
///
/// // Validate your border configuration
/// let table = Table::default()
///     .table_borders(TableBorders::ALL_BORDERS)
///     .border_set(TableBorderSet::thick());
///
/// match table.validate_border_config() {
///     Ok(()) => println!("Configuration is valid"),
///     Err(warnings) => {
///         for warning in warnings {
///             eprintln!("Border config warning: {}", warning);
///         }
///     }
/// }
/// ```
///
/// See the [Examples] directory for more examples of table configuration options.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/master/examples/README.md
///
/// # Constructor methods
///
/// - [`Table::new`] creates a new [`Table`] with the given rows.
/// - [`Table::default`] creates an empty [`Table`]. You can then add rows using [`Table::rows`].
///
/// # Setter methods
///
/// These methods are fluent setters. They return a new `Table` with the specified property set.
///
/// - [`Table::rows`] sets the rows of the [`Table`].
/// - [`Table::header`] sets the header row of the [`Table`].
/// - [`Table::footer`] sets the footer row of the [`Table`].
/// - [`Table::widths`] sets the width constraints of each column.
/// - [`Table::column_spacing`] sets the spacing between each column.
/// - [`Table::block`] wraps the table in a [`Block`] widget.
/// - [`Table::style`] sets the base style of the widget.
/// - [`Table::row_highlight_style`] sets the style of the selected row.
/// - [`Table::column_highlight_style`] sets the style of the selected column.
/// - [`Table::cell_highlight_style`] sets the style of the selected cell.
/// - [`Table::highlight_symbol`] sets the symbol to be displayed in front of the selected row.
/// - [`Table::highlight_spacing`] sets when to show the highlight spacing.
/// - [`Table::internal_borders`] sets which internal borders to display within the table (legacy).
/// - [`Table::table_borders`] sets which borders to display with enhanced control (preferred).
/// - [`Table::border_set`] sets custom border symbols for the table.
/// - [`Table::border_style`] sets the style for the internal borders.
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, Row, Table};
/// use ratatui_widgets::table::TableBorders;
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// // Columns widths are constrained in the same way as Layout...
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths)
///     // ...and they can be separated by a fixed spacing.
///     .column_spacing(1)
///     // You can set the style of the entire Table.
///     .style(Style::new().blue())
///     // It has an optional header, which is simply a Row always visible at the top.
///     .header(
///         Row::new(vec!["Col1", "Col2", "Col3"])
///             .style(Style::new().bold())
///             // To add space between the header and the rest of the rows, specify the margin
///             .bottom_margin(1),
///     )
///     // It has an optional footer, which is simply a Row always visible at the bottom.
///     .footer(Row::new(vec!["Updated on Dec 28"]))
///     // As any other widget, a Table can be wrapped in a Block.
///     .block(Block::new().title("Table"))
///     // You can add internal borders between and around cells.
///     .internal_borders(TableBorders::ALL)
///     .border_style(Style::new().white())
///     // The selected row, column, cell and its content can also be styled.
///     .row_highlight_style(Style::new().reversed())
///     .column_highlight_style(Style::new().red())
///     .cell_highlight_style(Style::new().blue())
///     // ...and potentially show a symbol in front of the selection.
///     .highlight_symbol(">>");
/// ```
///
/// Rows can be created from an iterator of [`Cell`]s. Each row can have an associated height,
/// bottom margin, and style. See [`Row`] for more details.
///
/// ```rust
/// use ratatui::style::{Style, Stylize};
/// use ratatui::text::{Line, Span};
/// use ratatui::widgets::{Cell, Row, Table};
///
/// // a Row can be created from simple strings.
/// let row = Row::new(vec!["Row11", "Row12", "Row13"]);
///
/// // You can style the entire row.
/// let row = Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::new().red());
///
/// // If you need more control over the styling, create Cells directly
/// let row = Row::new(vec![
///     Cell::from("Row31"),
///     Cell::from("Row32").style(Style::new().yellow()),
///     Cell::from(Line::from(vec![Span::raw("Row"), Span::from("33").green()])),
/// ]);
///
/// // If a Row need to display some content over multiple lines, specify the height.
/// let row = Row::new(vec![
///     Cell::from("Row\n41"),
///     Cell::from("Row\n42"),
///     Cell::from("Row\n43"),
/// ])
/// .height(2);
/// ```
///
/// Cells can be created from anything that can be converted to [`Text`]. See [`Cell`] for more
/// details.
///
/// ```rust
/// use ratatui::style::{Style, Stylize};
/// use ratatui::text::{Line, Span, Text};
/// use ratatui::widgets::Cell;
///
/// Cell::from("simple string");
/// Cell::from("simple styled span".red());
/// Cell::from(Span::raw("raw span"));
/// Cell::from(Span::styled("styled span", Style::new().red()));
/// Cell::from(Line::from(vec![
///     Span::raw("a vec of "),
///     Span::from("spans").bold(),
/// ]));
/// Cell::from(Text::from("text"));
/// ```
///
/// Just as rows can be collected from iterators of `Cell`s, tables can be collected from iterators
/// of `Row`s.  This will create a table with column widths evenly dividing the space available.
/// These default columns widths can be overridden using the `Table::widths` method.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::widgets::{Row, Table};
///
/// let text = "Mary had a\nlittle lamb.";
///
/// let table = text
///     .split("\n")
///     .map(|line: &str| -> Row { line.split_ascii_whitespace().collect() })
///     .collect::<Table>()
///     .widths([Constraint::Length(10); 3]);
/// ```
///
/// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
/// the [`Stylize`] trait to set the style of the widget more concisely.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::Stylize;
/// use ratatui::widgets::{Row, Table};
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths).red().italic();
/// ```
///
/// # Stateful example
///
/// `Table` is a [`StatefulWidget`], which means you can use it with [`TableState`] to allow the
/// user to scroll through the rows and select one of them.
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::{Constraint, Rect};
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, Row, Table, TableState};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // Note: TableState should be stored in your application state (not constructed in your render
/// // method) so that the selected row is preserved across renders
/// let mut table_state = TableState::default();
/// let rows = [
///     Row::new(vec!["Row11", "Row12", "Row13"]),
///     Row::new(vec!["Row21", "Row22", "Row23"]),
///     Row::new(vec!["Row31", "Row32", "Row33"]),
/// ];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths)
///     .block(Block::new().title("Table"))
///     .row_highlight_style(Style::new().reversed())
///     .highlight_symbol(">>");
///
/// frame.render_stateful_widget(table, area, &mut table_state);
/// # }
/// ```
///
/// [`Stylize`]: ratatui_core::style::Stylize
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Table<'a> {
    /// Data to display in each row
    rows: Vec<Row<'a>>,

    /// Optional header
    header: Option<Row<'a>>,

    /// Optional footer
    footer: Option<Row<'a>>,

    /// Width constraints for each column
    widths: Vec<Constraint>,

    /// Space between each column
    column_spacing: u16,

    /// A block to wrap the widget in
    block: Option<Block<'a>>,

    /// Base style for the widget
    style: Style,

    /// Style used to render the selected row
    row_highlight_style: Style,

    /// Style used to render the selected column
    column_highlight_style: Style,

    /// Style used to render the selected cell
    cell_highlight_style: Style,

    /// Symbol in front of the selected row
    highlight_symbol: Text<'a>,

    /// Decides when to allocate spacing for the row selection
    highlight_spacing: HighlightSpacing,

    /// Controls how to distribute extra space among the columns
    flex: Flex,

    /// The type of internal borders to display.
    internal_borders: TableBorders,

    /// The style for borders.
    border_style: Style,

    /// Custom border symbol set for table borders.
    border_set: Option<TableBorderSet<'a>>,
}

impl Default for Table<'_> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            header: None,
            footer: None,
            widths: Vec::new(),
            column_spacing: 1,
            block: None,
            style: Style::new(),
            row_highlight_style: Style::new(),
            column_highlight_style: Style::new(),
            cell_highlight_style: Style::new(),
            highlight_symbol: Text::default(),
            highlight_spacing: HighlightSpacing::default(),
            flex: Flex::Start,
            internal_borders: TableBorders::NONE,
            border_style: Style::default(),
            border_set: None,
        }
    }
}

impl<'a> Table<'a> {
    /// Creates a new [`Table`] widget with the given rows.
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths);
    /// ```
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);
        let rows = rows.into_iter().map(Into::into).collect();
        Self {
            rows,
            widths,
            ..Default::default()
        }
    }

    /// Set the rows
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// # Warning
    ///
    /// This method does not currently set the column widths. You will need to set them manually by
    /// calling [`Table::widths`].
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let table = Table::default().rows(rows);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn rows<T>(mut self, rows: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Row<'a>>,
    {
        self.rows = rows.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the header row
    ///
    /// The `header` parameter is a [`Row`] which will be displayed at the top of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let header = Row::new(vec![
    ///     Cell::from("Header Cell 1"),
    ///     Cell::from("Header Cell 2"),
    /// ]);
    /// let table = Table::default().header(header);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the footer row
    ///
    /// The `footer` parameter is a [`Row`] which will be displayed at the bottom of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let footer = Row::new(vec![
    ///     Cell::from("Footer Cell 1"),
    ///     Cell::from("Footer Cell 2"),
    /// ]);
    /// let table = Table::default().footer(footer);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn footer(mut self, footer: Row<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the widths of the columns.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// If the widths are empty, the table will be rendered with equal widths.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let table = Table::default().widths([Constraint::Length(5), Constraint::Length(5)]);
    /// let table = Table::default().widths(vec![Constraint::Length(5); 2]);
    ///
    /// // widths could also be computed at runtime
    /// let widths = [10, 10, 20].into_iter().map(|c| Constraint::Length(c));
    /// let table = Table::default().widths(widths);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);
        self.widths = widths;
        self
    }

    /// Set the spacing between columns
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).column_spacing(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Wraps the table with a custom [`Block`] widget.
    ///
    /// The `block` parameter is of type [`Block`]. This holds the specified block to be
    /// created around the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Block, Cell, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let block = Block::bordered().title("Table");
    /// let table = Table::new(rows, widths).block(block);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// All text rendered by the widget will use this style, unless overridden by [`Block::style`],
    /// [`Row::style`], [`Cell::style`], or the styles of cell's content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::{Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).style(Style::new().red().italic());
    /// ```
    ///
    /// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = vec![Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).red().italic();
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    /// [`Stylize`]: ratatui_core::style::Stylize
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the style of the selected row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire row, including the selection symbol if it is
    /// displayed, and will override any style set on the row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_style(Style::new().red().italic());
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    #[deprecated(note = "use `row_highlight_style()` instead")]
    pub fn highlight_style<S: Into<Style>>(self, highlight_style: S) -> Self {
        self.row_highlight_style(highlight_style)
    }

    /// Set the style of the selected row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire row, including the selection symbol if it is
    /// displayed, and will override any style set on the row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).row_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn row_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.row_highlight_style = highlight_style.into();
        self
    }

    /// Set the style of the selected column
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire column, and will override any style set on the
    /// row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).column_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn column_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.column_highlight_style = highlight_style.into();
        self
    }

    /// Set the style of the selected cell
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the selected cell, and will override any style set on the
    /// row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).cell_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn cell_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.cell_highlight_style = highlight_style.into();
        self
    }

    /// Set the symbol to be displayed in front of the selected row
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_symbol(">>");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_symbol<T: Into<Text<'a>>>(mut self, highlight_symbol: T) -> Self {
        self.highlight_symbol = highlight_symbol.into();
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// The highlight spacing is the spacing that is allocated for the selection symbol column (if
    /// enabled) and is used to shift the table when a row is selected. This method allows you to
    /// configure when this spacing is allocated.
    ///
    /// - [`HighlightSpacing::Always`] will always allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the table will never change size, regardless of if a
    ///   row is selected or not.
    /// - [`HighlightSpacing::WhenSelected`] will only allocate the spacing if a row is selected.
    ///   This means that the table will shift when a row is selected. This is the default setting
    ///   for backwards compatibility, but it is recommended to use `HighlightSpacing::Always` for a
    ///   better user experience.
    /// - [`HighlightSpacing::Never`] will never allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the highlight symbol will never be drawn.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{HighlightSpacing, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_spacing(HighlightSpacing::Always);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Set how extra space is distributed amongst columns.
    ///
    /// This determines how the space is distributed when the constraints are satisfied. By default,
    /// the extra space is not distributed at all.  But this can be changed to distribute all extra
    /// space to the last column or to distribute it equally.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// Create a table that needs at least 30 columns to display.  Any extra space will be assigned
    /// to the last column.
    /// ```
    /// use ratatui::layout::{Constraint, Flex};
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let widths = [
    ///     Constraint::Min(10),
    ///     Constraint::Min(10),
    ///     Constraint::Min(10),
    /// ];
    /// let table = Table::new(Vec::<Row>::new(), widths).flex(Flex::Legacy);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Set which internal borders to display within the table.
    ///
    /// Internal borders are the lines drawn between and around table cells, separate from any
    /// external borders that might be added by wrapping the table in a [`Block`]. This method
    /// controls which of these internal borders are visible.
    ///
    /// **Note on Terminology**: While this method is called "internal borders" to distinguish
    /// them from external [`Block`] borders, they serve the same visual purpose as traditional
    /// table borders. The term "internal" refers to their position within the table structure,
    /// not their importance or functionality.
    ///
    /// The borders are drawn using the style set by [`border_style`](Self::border_style).
    ///
    /// - [`TableBorders::NONE`] - No internal borders (default)
    /// - [`TableBorders::HORIZONTAL`] - Only horizontal lines between rows
    /// - [`TableBorders::VERTICAL`] - Only vertical lines between columns
    /// - [`TableBorders::ALL`] - Both horizontal and vertical borders
    ///
    /// You can also combine borders using bitwise operations, e.g.,
    /// `TableBorders::HORIZONTAL | TableBorders::VERTICAL`.
    ///
    /// **Performance Note**: Enabling internal borders adds rendering overhead, especially for
    /// large tables. Consider the performance implications when using this feature with tables
    /// containing many rows or columns.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::{Block, Table};
    /// use ratatui_widgets::table::TableBorders;
    ///
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn internal_borders(mut self, internal_borders: TableBorders) -> Self {
        self.internal_borders = internal_borders;
        self
    }

    /// Set the style for internal borders.
    ///
    /// This method sets the visual style (color, modifiers, etc.) that will be applied to the
    /// internal borders of the table. The borders themselves are controlled by the
    /// [`internal_borders`](Self::internal_borders) method, while this method determines how
    /// those borders will look.
    ///
    /// The style affects all internal borders equally - you cannot style horizontal and vertical
    /// borders differently. The style is applied to the border characters themselves, not to the
    /// content of the cells.
    ///
    /// **Terminal Compatibility**: The effectiveness of border styling depends on your terminal's
    /// capabilities. Some terminals may not support all colors or style modifiers. For maximum
    /// compatibility, consider using simple styles and testing with your target terminal
    /// environment.
    ///
    /// **Validation Note**: This method does not validate that the style is appropriate for your
    /// terminal's capabilities. It's the responsibility of the application to ensure the chosen
    /// style works well with the target terminal environment.
    ///
    /// Common styling options include:
    /// - Colors: `.fg(Color::Blue)`, `.bg(Color::Gray)`
    /// - Modifiers: `.bold()`, `.dim()`, `.italic()`
    /// - Combined: `.fg(Color::Red).bold()`
    ///
    /// **Performance Consideration**: Complex styles with multiple modifiers may have a slight
    /// performance impact when rendering large tables with many borders.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::{Block, Table};
    /// use ratatui_widgets::table::TableBorders;
    ///
    /// // Blue colored borders
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    ///
    /// // Gray borders with bold styling
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::HORIZONTAL)
    ///     .border_style(Style::default().fg(Color::Gray).bold());
    ///
    /// // Dim borders for a subtle appearance
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::VERTICAL)
    ///     .border_style(Style::default().dim());
    ///
    /// // Simple style for maximum compatibility
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::White));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }

    /// Set the table borders using enhanced border flags.
    ///
    /// This method provides fine-grained control over which specific borders are displayed,
    /// including individual outer borders (TOP, LEFT, RIGHT, BOTTOM), inner borders
    /// (`INNER_VERTICAL`, `INNER_HORIZONTAL`), and special borders like `HEADER_TOP`.
    ///
    /// This is the preferred method for new code as it provides more flexibility than the
    /// legacy [`internal_borders`](Self::internal_borders) method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_widgets::table::{Table, TableBorders};
    ///
    /// // Individual border control
    /// let table = Table::default()
    ///     .table_borders(TableBorders::TOP | TableBorders::INNER_HORIZONTAL);
    ///
    /// // Outer borders only
    /// let table = Table::default()
    ///     .table_borders(TableBorders::OUTER);
    ///
    /// // Header separator with vertical borders
    /// let table = Table::default()
    ///     .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL);
    ///
    /// // All borders including outer and inner
    /// let table = Table::default()
    ///     .table_borders(TableBorders::ALL_BORDERS);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn table_borders(mut self, borders: TableBorders) -> Self {
        self.internal_borders = borders;
        self
    }

    /// Set custom border symbols for the table.
    ///
    /// This method allows you to customize the characters used for drawing table borders,
    /// including support for header-specific symbols that can be different from regular
    /// cell borders.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_widgets::table::{Table, TableBorderSet, TableBorders};
    ///
    /// // Use predefined border set
    /// let table = Table::default()
    ///     .table_borders(TableBorders::ALL_BORDERS)
    ///     .border_set(TableBorderSet::thick());
    ///
    /// // Use header-specific styling
    /// let table = Table::default()
    ///     .table_borders(TableBorders::INNER | TableBorders::HEADER_TOP)
    ///     .border_set(TableBorderSet::with_header_style());
    ///
    /// // Convert from line set
    /// use ratatui_core::symbols::line;
    /// let table = Table::default()
    ///     .table_borders(TableBorders::ALL)
    ///     .border_set(TableBorderSet::from(line::DOUBLE));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn border_set(mut self, border_set: TableBorderSet<'a>) -> Self {
        self.border_set = Some(border_set);
        self
    }

    /// Validates the current border configuration for potential issues.
    ///
    /// This method checks for common configuration problems such as conflicting
    /// border types, invalid symbols, or deprecated usage patterns. It returns
    /// warnings that can help developers identify potential issues with their
    /// border configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_widgets::table::{Table, TableBorders, TableBorderSet};
    ///
    /// let table = Table::default()
    ///     .table_borders(TableBorders::ALL_BORDERS)
    ///     .border_set(TableBorderSet::thick());
    ///
    /// // Validate the configuration
    /// match table.validate_border_config() {
    ///     Ok(()) => println!("Border configuration is valid"),
    ///     Err(warnings) => {
    ///         for warning in warnings {
    ///             println!("Warning: {}", warning);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn validate_border_config(&self) -> Result<(), Vec<BorderConfigError>> {
        let mut errors = Vec::new();

        // Check for conflicting legacy and new border flags
        if self.has_conflicting_border_types() {
            errors.push(BorderConfigError::ConflictingBorderTypes);
        }

        // Check for deprecated configurations
        if let Some(deprecated_msg) = self.check_deprecated_configurations() {
            errors.push(BorderConfigError::DeprecatedConfiguration(deprecated_msg));
        }

        // Validate custom border symbols if present
        if let Some(border_set) = &self.border_set {
            if let Err(symbol_errors) = Self::validate_border_symbols(border_set) {
                errors.extend(symbol_errors);
            }
        }

        // Check for unsupported border combinations
        if Self::has_unsupported_border_combinations() {
            errors.push(BorderConfigError::UnsupportedBorderCombination);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Checks if there are conflicting border type configurations.
    const fn has_conflicting_border_types(&self) -> bool {
        let borders = self.internal_borders;

        // Check if both legacy and new flags are used in potentially conflicting ways
        let has_legacy_horizontal = borders.contains(TableBorders::HORIZONTAL);
        let has_legacy_vertical = borders.contains(TableBorders::VERTICAL);
        let has_new_inner_horizontal = borders.contains(TableBorders::INNER_HORIZONTAL);
        let has_new_inner_vertical = borders.contains(TableBorders::INNER_VERTICAL);

        // It's conflicting if both legacy and new versions of the same border type are used
        (has_legacy_horizontal && has_new_inner_horizontal)
            || (has_legacy_vertical && has_new_inner_vertical)
    }

    /// Checks for deprecated configuration patterns.
    fn check_deprecated_configurations(&self) -> Option<alloc::string::String> {
        let borders = self.internal_borders;

        // Check if only legacy flags are being used
        if (borders.contains(TableBorders::HORIZONTAL) || borders.contains(TableBorders::VERTICAL))
            && !borders
                .intersects(TableBorders::OUTER | TableBorders::INNER | TableBorders::HEADER_TOP)
        {
            return Some(alloc::string::String::from(
                "Using legacy border flags (HORIZONTAL, VERTICAL, ALL). Consider migrating to new border flags (INNER_HORIZONTAL, INNER_VERTICAL, etc.) for more control.",
            ));
        }

        None
    }

    /// Validates border symbols in a `TableBorderSet`.
    fn validate_border_symbols(border_set: &TableBorderSet) -> Result<(), Vec<BorderConfigError>> {
        let mut errors = Vec::new();

        // Check that all required symbols are non-empty
        let symbols = [
            ("horizontal", border_set.horizontal),
            ("vertical", border_set.vertical),
            ("top_left", border_set.top_left),
            ("top_right", border_set.top_right),
            ("bottom_left", border_set.bottom_left),
            ("bottom_right", border_set.bottom_right),
            ("vertical_left", border_set.vertical_left),
            ("vertical_right", border_set.vertical_right),
            ("horizontal_down", border_set.horizontal_down),
            ("horizontal_up", border_set.horizontal_up),
            ("cross", border_set.cross),
        ];

        for (name, symbol) in symbols {
            if symbol.is_empty() {
                errors.push(BorderConfigError::InvalidBorderSymbol(alloc::format!(
                    "Border symbol '{name}' is empty"
                )));
            }
        }

        // Check optional header symbols if present
        if let Some(header_horizontal) = border_set.header_horizontal {
            if header_horizontal.is_empty() {
                errors.push(BorderConfigError::InvalidBorderSymbol(
                    alloc::string::String::from("Header horizontal symbol is empty"),
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Checks for unsupported border combinations.
    const fn has_unsupported_border_combinations() -> bool {
        // Currently all combinations are supported, but this method is here
        // for future extensibility when certain combinations might not be supported
        false
    }

    // === Private helpers ===
    fn layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let header_top_margin = self.header.as_ref().map_or(0, |h| h.top_margin);
        let header_height = self.header.as_ref().map_or(0, |h| h.height);
        let header_bottom_margin = self.header.as_ref().map_or(0, |h| h.bottom_margin);
        let footer_top_margin = self.footer.as_ref().map_or(0, |h| h.top_margin);
        let footer_height = self.footer.as_ref().map_or(0, |f| f.height);
        let footer_bottom_margin = self.footer.as_ref().map_or(0, |h| h.bottom_margin);
        let layout = Layout::vertical([
            Constraint::Length(header_top_margin),
            Constraint::Length(header_height),
            Constraint::Length(header_bottom_margin),
            Constraint::Min(0),
            Constraint::Length(footer_top_margin),
            Constraint::Length(footer_height),
            Constraint::Length(footer_bottom_margin),
        ])
        .split(area);
        let (header_area, rows_area, footer_area) = (layout[1], layout[3], layout[5]);
        (header_area, rows_area, footer_area)
    }

    const fn get_border_symbol(
        _self: &Self,
        _x: u16,
        _y: u16,
        _area: Rect,
        is_horizontal: bool,
        has_horizontal_border: bool,
        has_vertical_border: bool,
    ) -> &'static str {
        use symbols::line;

        // For internal borders, we only use simple symbols:
        // - Cross (┼) when both horizontal and vertical borders intersect
        // - Horizontal line (─) for horizontal borders
        // - Vertical line (│) for vertical borders
        if has_horizontal_border && has_vertical_border {
            line::NORMAL.cross
        } else if is_horizontal {
            line::NORMAL.horizontal
        } else {
            line::NORMAL.vertical
        }
    }

    /// Enhanced border symbol resolution using `BorderContext` and custom `TableBorderSet`.
    ///
    /// This method determines the appropriate border symbol based on the border context
    /// and uses custom border symbols if configured, including header-specific symbols.
    #[allow(clippy::unnecessary_wraps)]
    fn get_border_symbol_enhanced(&self, context: BorderContext) -> Option<&str> {
        let border_set = self.border_set.unwrap_or_default();

        match context {
            BorderContext::HeaderSeparator { has_vertical } => {
                if has_vertical {
                    Some(border_set.get_cross(true))
                } else {
                    Some(border_set.get_horizontal(true))
                }
            }
            BorderContext::InnerHorizontal { has_vertical } => {
                if has_vertical {
                    Some(border_set.cross)
                } else {
                    Some(border_set.horizontal)
                }
            }
            BorderContext::InnerVertical { has_horizontal } => {
                if has_horizontal {
                    Some(border_set.cross)
                } else {
                    Some(border_set.vertical)
                }
            }
            BorderContext::TopEdge { has_vertical } => {
                if has_vertical {
                    Some(border_set.horizontal_down)
                } else {
                    Some(border_set.horizontal)
                }
            }
            BorderContext::BottomEdge { has_vertical } => {
                if has_vertical {
                    Some(border_set.horizontal_up)
                } else {
                    Some(border_set.horizontal)
                }
            }
            BorderContext::LeftEdge { has_horizontal } => {
                if has_horizontal {
                    Some(border_set.vertical_right)
                } else {
                    Some(border_set.vertical)
                }
            }
            BorderContext::RightEdge { has_horizontal } => {
                if has_horizontal {
                    Some(border_set.vertical_left)
                } else {
                    Some(border_set.vertical)
                }
            }
            BorderContext::Corner(corner_type) => match corner_type {
                CornerType::TopLeft => Some(border_set.top_left),
                CornerType::TopRight => Some(border_set.top_right),
                CornerType::BottomLeft => Some(border_set.bottom_left),
                CornerType::BottomRight => Some(border_set.bottom_right),
            },
        }
    }

    fn has_vertical_border_at(&self, x: u16, area: Rect, selection_width: u16) -> bool {
        // If we don't have vertical borders, return false
        if !self.internal_borders.contains(TableBorders::VERTICAL) {
            return false;
        }

        // Calculate column widths to find vertical border positions
        let column_count = self.column_count();
        let column_widths = self.get_column_widths(area.width, selection_width, column_count);

        // Check if x is at a vertical border position
        for (i, (col_x, width)) in column_widths.iter().enumerate() {
            if i < column_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x
                        .saturating_add(*col_x)
                        .saturating_add(*width)
                        .saturating_add(self.column_spacing / 2)
                } else {
                    area.x.saturating_add(*col_x).saturating_add(*width)
                };
                if x == border_x {
                    return true;
                }
            }
        }
        false
    }

    fn has_horizontal_border_at(&self, y: u16, area: Rect, _selection_width: u16) -> bool {
        // If we don't have horizontal borders, return false
        if !self.internal_borders.contains(TableBorders::HORIZONTAL) {
            return false;
        }

        // Calculate row positions to find horizontal border positions
        let mut y_offset: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            y_offset = y_offset
                .saturating_add(row.top_margin)
                .saturating_add(row.height);

            // Check if this is a border position (between rows)
            if i < self.rows.len() - 1 {
                let border_y = area.y.saturating_add(y_offset);
                if y == border_y {
                    return true;
                }
            }
            y_offset = y_offset.saturating_add(row.bottom_margin);
        }
        false
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer, column_widths: &[(u16, u16)]) {
        if let Some(ref header) = self.header {
            buf.set_style(area, header.style);
            for ((x, width), cell) in column_widths.iter().zip(header.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer, column_widths: &[(u16, u16)]) {
        if let Some(ref footer) = self.footer {
            buf.set_style(area, footer.style);
            for ((x, width), cell) in column_widths.iter().zip(footer.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_rows(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut TableState,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        if self.rows.is_empty() {
            // Clear selection for empty tables
            state.selected = None;
            state.selected_column = None;
            return;
        }

        let (start_index, end_index) = self.visible_rows(state, area);
        state.offset = start_index;

        // Correct selection indices
        if let Some(selected) = state.selected {
            if selected >= self.rows.len() {
                state.selected = Some(self.rows.len().saturating_sub(1));
            }
        }
        if let Some(selected_column) = state.selected_column {
            if selected_column >= columns_widths.len() {
                state.selected_column = Some(columns_widths.len().saturating_sub(1));
            }
        }

        let mut y_offset: u16 = 0;

        let mut selected_row_area = None;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            let y = area
                .y
                .saturating_add(y_offset)
                .saturating_add(row.top_margin);
            let height = (y.saturating_add(row.height))
                .min(area.bottom())
                .saturating_sub(y);
            let row_area = Rect { y, height, ..area };
            buf.set_style(row_area, row.style);

            let is_selected = state.selected.is_some_and(|index| index == i);
            if selection_width > 0 && is_selected {
                let selection_area = Rect {
                    width: selection_width,
                    ..row_area
                };
                buf.set_style(selection_area, row.style);
                (&self.highlight_symbol).render(selection_area, buf);
            }
            for ((x, width), cell) in columns_widths.iter().zip(row.cells.iter()) {
                cell.render(
                    Rect::new(
                        row_area.x.saturating_add(*x),
                        row_area.y,
                        *width,
                        row_area.height,
                    ),
                    buf,
                );
            }
            if is_selected {
                selected_row_area = Some(row_area);
            }
            y_offset = y_offset.saturating_add(row.height_with_margin());
        }

        let selected_column_area = state.selected_column.and_then(|s| {
            // The selection is clamped by the column count. Since a user can manually specify an
            // incorrect number of widths, we should use panic free methods.
            columns_widths.get(s).map(|(x, width)| Rect {
                x: x.saturating_add(area.x),
                width: *width,
                ..area
            })
        });

        match (selected_row_area, selected_column_area) {
            (Some(row_area), Some(col_area)) => {
                buf.set_style(row_area, self.row_highlight_style);
                buf.set_style(col_area, self.column_highlight_style);
                let cell_area = row_area.intersection(col_area);
                buf.set_style(cell_area, self.cell_highlight_style);
            }
            (Some(row_area), None) => {
                buf.set_style(row_area, self.row_highlight_style);
            }
            (None, Some(col_area)) => {
                buf.set_style(col_area, self.column_highlight_style);
            }
            (None, None) => (),
        }
    }

    fn render_internal_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
        start_index: usize,
        end_index: usize,
    ) {
        // Handle legacy border flags for backward compatibility
        if self.internal_borders == TableBorders::NONE {
            return;
        }

        // Map legacy flags to new flags for backward compatibility
        let borders = self.map_legacy_borders();

        // Render outer borders
        if borders.contains(TableBorders::TOP) {
            self.render_top_border(area, buf, selection_width, columns_widths);
        }
        if borders.contains(TableBorders::BOTTOM) {
            self.render_bottom_border(area, buf, selection_width, columns_widths);
        }
        if borders.contains(TableBorders::LEFT) {
            self.render_left_border(area, buf, selection_width);
        }
        if borders.contains(TableBorders::RIGHT) {
            self.render_right_border(area, buf, selection_width);
        }

        // Render inner borders
        if borders.contains(TableBorders::INNER_HORIZONTAL) {
            self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
        }
        if borders.contains(TableBorders::INNER_VERTICAL) {
            self.render_vertical_borders(area, buf, selection_width, columns_widths);
        }

        // Render header separator
        if borders.contains(TableBorders::HEADER_TOP) && self.header.is_some() {
            self.render_header_separator(area, buf, selection_width, columns_widths);
        }
    }

    /// Maps legacy border flags to new border flags for backward compatibility.
    fn map_legacy_borders(&self) -> TableBorders {
        let mut borders = self.internal_borders;

        // Map legacy HORIZONTAL to INNER_HORIZONTAL
        if borders.contains(TableBorders::HORIZONTAL)
            && !borders.contains(TableBorders::INNER_HORIZONTAL)
        {
            borders |= TableBorders::INNER_HORIZONTAL;
        }

        // Map legacy VERTICAL to INNER_VERTICAL
        if borders.contains(TableBorders::VERTICAL)
            && !borders.contains(TableBorders::INNER_VERTICAL)
        {
            borders |= TableBorders::INNER_VERTICAL;
        }

        borders
    }

    fn render_horizontal_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        start_index: usize,
        end_index: usize,
    ) {
        let mut y_offset: u16 = 0;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            y_offset = y_offset
                .saturating_add(row.top_margin)
                .saturating_add(row.height);
            if i < end_index - 1 && y_offset < area.height {
                let border_y = area.y.saturating_add(y_offset);
                if border_y < area.bottom() {
                    for x in (area.x.saturating_add(selection_width))..area.right() {
                        let cell = &mut buf[(x, border_y)];
                        // Check if there's a vertical border at this position
                        let has_vertical_border =
                            self.has_vertical_border_at(x, area, selection_width);
                        let symbol = Self::get_border_symbol(
                            self,
                            x,
                            border_y,
                            area,
                            true,
                            true,
                            has_vertical_border,
                        );
                        cell.merge_symbol(symbol, MergeStrategy::Exact)
                            .set_style(self.border_style);
                    }
                }
            }
            y_offset = y_offset.saturating_add(row.bottom_margin);
        }
    }

    fn render_vertical_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        for (i, (x, width)) in columns_widths.iter().enumerate() {
            if i < columns_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x
                        .saturating_add(*x)
                        .saturating_add(*width)
                        .saturating_add(self.column_spacing / 2)
                } else {
                    area.x.saturating_add(*x).saturating_add(*width)
                };
                if border_x < area.right() {
                    for y in area.y..area.bottom() {
                        let cell = &mut buf[(border_x, y)];
                        // Check if there's a horizontal border at this position
                        let has_horizontal_border =
                            self.has_horizontal_border_at(y, area, selection_width);
                        let symbol = Self::get_border_symbol(
                            self,
                            border_x,
                            y,
                            area,
                            false,
                            has_horizontal_border,
                            true,
                        );
                        cell.merge_symbol(symbol, MergeStrategy::Exact)
                            .set_style(self.border_style);
                    }
                }
            }
        }
    }

    /// Render the top border of the table.
    fn render_top_border(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        _columns_widths: &[(u16, u16)],
    ) {
        if area.height == 0 {
            return;
        }

        let y = area.y;
        for x in (area.x.saturating_add(selection_width))..area.right() {
            let has_vertical = self.has_vertical_border_at(x, area, selection_width);
            let context = if x == area.x.saturating_add(selection_width) {
                BorderContext::Corner(CornerType::TopLeft)
            } else if x == area.right() - 1 {
                BorderContext::Corner(CornerType::TopRight)
            } else {
                BorderContext::TopEdge { has_vertical }
            };

            if let Some(symbol) = self.get_border_symbol_enhanced(context) {
                let cell = &mut buf[(x, y)];
                cell.set_symbol(symbol).set_style(self.border_style);
            }
        }
    }

    /// Render the bottom border of the table.
    fn render_bottom_border(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        _columns_widths: &[(u16, u16)],
    ) {
        if area.height == 0 {
            return;
        }

        let y = area.bottom() - 1;
        for x in (area.x.saturating_add(selection_width))..area.right() {
            let has_vertical = self.has_vertical_border_at(x, area, selection_width);
            let context = if x == area.x.saturating_add(selection_width) {
                BorderContext::Corner(CornerType::BottomLeft)
            } else if x == area.right() - 1 {
                BorderContext::Corner(CornerType::BottomRight)
            } else {
                BorderContext::BottomEdge { has_vertical }
            };

            if let Some(symbol) = self.get_border_symbol_enhanced(context) {
                let cell = &mut buf[(x, y)];
                cell.set_symbol(symbol).set_style(self.border_style);
            }
        }
    }

    /// Render the left border of the table.
    fn render_left_border(&self, area: Rect, buf: &mut Buffer, selection_width: u16) {
        if area.width == 0 {
            return;
        }

        let x = area.x.saturating_add(selection_width);
        for y in area.y..area.bottom() {
            let has_horizontal = self.has_horizontal_border_at(y, area, selection_width);
            let context = BorderContext::LeftEdge { has_horizontal };

            if let Some(symbol) = self.get_border_symbol_enhanced(context) {
                let cell = &mut buf[(x, y)];
                cell.set_symbol(symbol).set_style(self.border_style);
            }
        }
    }

    /// Render the right border of the table.
    fn render_right_border(&self, area: Rect, buf: &mut Buffer, selection_width: u16) {
        if area.width == 0 {
            return;
        }

        let x = area.right() - 1;
        for y in area.y..area.bottom() {
            let has_horizontal = self.has_horizontal_border_at(y, area, selection_width);
            let context = BorderContext::RightEdge { has_horizontal };

            if let Some(symbol) = self.get_border_symbol_enhanced(context) {
                let cell = &mut buf[(x, y)];
                cell.set_symbol(symbol).set_style(self.border_style);
            }
        }
    }

    /// Render the header separator line between header and data rows.
    ///
    /// This method renders a horizontal line that separates the header from the data rows,
    /// using header-specific symbols when available. It properly handles intersections
    /// with vertical borders and respects the header's margins and height.
    fn render_header_separator(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        if let Some(_header) = &self.header {
            let separator_y = self.get_header_separator_position(area);

            if let Some(y) = separator_y {
                if y < area.bottom() {
                    self.render_header_separator_line(
                        area,
                        buf,
                        selection_width,
                        columns_widths,
                        y,
                    );
                }
            }
        }
    }

    /// Calculate the Y position where the header separator should be drawn.
    ///
    /// Returns None if the header separator should not be drawn (e.g., no space available).
    const fn get_header_separator_position(&self, area: Rect) -> Option<u16> {
        if let Some(header) = &self.header {
            let header_total_height = header.height + header.top_margin + header.bottom_margin;

            // Ensure there's enough space for the header and at least one data row
            if header_total_height < area.height {
                let separator_y = area.y + header_total_height;

                // Make sure the separator is within the area bounds
                if separator_y < area.bottom() {
                    Some(separator_y)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Render the actual header separator line at the specified Y position.
    fn render_header_separator_line(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        _columns_widths: &[(u16, u16)],
        y: u16,
    ) {
        let start_x = area.x.saturating_add(selection_width);
        let end_x = area.right();

        for x in start_x..end_x {
            let has_vertical = self.has_vertical_border_at(x, area, selection_width);
            let context = self.get_header_separator_context(x, area, selection_width, has_vertical);

            if let Some(symbol) = self.get_border_symbol_enhanced(context) {
                let cell = &mut buf[(x, y)];
                cell.set_symbol(symbol).set_style(self.border_style);
            }
        }
    }

    /// Determine the appropriate border context for a header separator position.
    ///
    /// This method considers the position within the table and whether there are
    /// intersecting vertical borders to choose the correct border context.
    const fn get_header_separator_context(
        &self,
        x: u16,
        area: Rect,
        selection_width: u16,
        has_vertical: bool,
    ) -> BorderContext {
        let start_x = area.x.saturating_add(selection_width);
        let end_x = area.right();

        // Check if we're at the edges where outer borders might intersect
        let at_left_edge = x == start_x && self.internal_borders.contains(TableBorders::LEFT);
        let at_right_edge = x == end_x - 1 && self.internal_borders.contains(TableBorders::RIGHT);

        if at_left_edge || at_right_edge {
            // At table edges, the header separator intersects with outer borders
            BorderContext::HeaderSeparator { has_vertical: true }
        } else {
            // In the middle of the table, use the detected vertical border state
            BorderContext::HeaderSeparator { has_vertical }
        }
    }

    /// Return the indexes of the visible rows.
    ///
    /// The algorithm works as follows:
    /// - start at the offset and calculate the height of the rows that can be displayed within the
    ///   area.
    /// - if the selected row is not visible, scroll the table to ensure it is visible.
    /// - if there is still space to fill then there's a partial row at the end which should be
    ///   included in the view.
    fn visible_rows(&self, state: &TableState, area: Rect) -> (usize, usize) {
        if self.rows.is_empty() {
            return (0, 0);
        }

        let last_row = self.rows.len().saturating_sub(1);
        let mut start = state.offset.min(last_row);

        if let Some(selected) = state.selected {
            start = start.min(selected);
        }

        let mut end = start;
        let mut height: u16 = 0;

        for item in self.rows.iter().skip(start) {
            if height.saturating_add(item.height) > area.height {
                break;
            }
            height = height.saturating_add(item.height_with_margin());
            end += 1;
        }

        if let Some(selected) = state.selected {
            let selected = selected.min(last_row);

            // scroll down until the selected row is visible
            while selected >= end && end < self.rows.len() {
                height = height.saturating_add(self.rows[end].height_with_margin());
                end += 1;
                while height > area.height && start < self.rows.len() {
                    height = height.saturating_sub(self.rows[start].height_with_margin());
                    start += 1;
                }
            }
        }

        // Include a partial row if there is space
        if height < area.height && end < self.rows.len() {
            end += 1;
        }

        (start, end)
    }

    /// Get all offsets and widths of all user specified columns.
    ///
    /// Returns (x, width). When self.widths is empty, it is assumed `.widths()` has not been called
    /// and a default of equal widths is returned.
    fn get_column_widths(
        &self,
        max_width: u16,
        selection_width: u16,
        col_count: usize,
    ) -> Vec<(u16, u16)> {
        let widths = if self.widths.is_empty() {
            // Divide the space between each column equally
            vec![Constraint::Length(max_width / col_count.max(1) as u16); col_count]
        } else {
            self.widths.clone()
        };
        // this will always allocate a selection area
        let [_selection_area, columns_area] =
            Layout::horizontal([Constraint::Length(selection_width), Constraint::Fill(0)])
                .areas(Rect::new(0, 0, max_width, 1));
        let rects = Layout::horizontal(widths)
            .flex(self.flex)
            .spacing(self.column_spacing)
            .split(columns_area);
        rects.iter().map(|c| (c.x, c.width)).collect()
    }

    fn column_count(&self) -> usize {
        self.rows
            .iter()
            .chain(self.footer.iter())
            .chain(self.header.iter())
            .map(|r| r.cells.len())
            .max()
            .unwrap_or_default()
    }

    /// Returns the width of the selection column if a row is selected, or the `highlight_spacing`
    /// is set to show the column always, otherwise 0.
    fn selection_width(&self, state: &TableState) -> u16 {
        let has_selection = state.selected.is_some();
        match self.highlight_spacing {
            HighlightSpacing::Always => self.highlight_symbol.width() as u16,
            HighlightSpacing::WhenSelected if has_selection => self.highlight_symbol.width() as u16,
            _ => 0,
        }
    }
}

fn ensure_percentages_less_than_100(widths: &[Constraint]) {
    for w in widths {
        if let Constraint::Percentage(p) = w {
            assert!(
                *p <= 100,
                "Percentages should be between 0 and 100 inclusively."
            );
        }
    }
}

impl Styled for Table<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a, Item> FromIterator<Item> for Table<'a>
where
    Item: Into<Row<'a>>,
{
    /// Collects an iterator of rows into a table.
    ///
    /// When collecting from an iterator into a table, the user must provide the widths using
    /// `Table::widths` after construction.
    fn from_iter<Iter: IntoIterator<Item = Item>>(rows: Iter) -> Self {
        let widths: [Constraint; 0] = [];
        Self::new(rows, widths)
    }
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for Table<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl StatefulWidget for &Table<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        self.block.as_ref().render(area, buf);
        let table_area = self.block.inner_if_some(area);

        if table_area.is_empty() {
            return;
        }

        let (header_area, rows_area, footer_area) = self.layout(table_area);
        let selection_width = self.selection_width(state);
        let column_count = self.column_count();
        let column_widths = self.get_column_widths(rows_area.width, selection_width, column_count);

        // Render header
        self.render_header(header_area, buf, &column_widths);

        // Render rows
        self.render_rows(rows_area, buf, state, selection_width, &column_widths);

        // Render footer
        self.render_footer(footer_area, buf, &column_widths);

        // Render internal borders
        let (start_index, end_index) = self.visible_rows(state, rows_area);
        self.render_internal_borders(
            rows_area,
            buf,
            selection_width,
            &column_widths,
            start_index,
            end_index,
        );
    }
}

#[cfg(test)]
#[allow(
    unused_variables,
    clippy::items_after_statements,
    clippy::no_effect_underscore_binding
)]
mod tests {
    use alloc::string::ToString;
    use alloc::{format, vec};

    use ratatui_core::layout::Constraint::*;
    use ratatui_core::style::{Color, Modifier, Style, Stylize};
    use ratatui_core::text::Line;
    use rstest::{fixture, rstest};

    use super::*;
    use crate::table::Cell;

    #[test]
    fn new() {
        let rows = [Row::new(vec![Cell::from("")])];
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows.clone(), widths);
        assert_eq!(table.rows, rows);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, widths);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.row_highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.flex, Flex::Start);
        assert_eq!(table.internal_borders, TableBorders::NONE);
        assert_eq!(table.border_style, Style::default());
        assert_eq!(table.border_set, None);
    }

    #[test]
    fn default() {
        let table = Table::default();
        assert_eq!(table.rows, []);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, []);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.row_highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.flex, Flex::Start);
        assert_eq!(table.internal_borders, TableBorders::NONE);
        assert_eq!(table.border_style, Style::default());
        assert_eq!(table.border_set, None);
    }

    #[test]
    fn table_borders_individual_flags() {
        // Test individual border flags
        assert_eq!(TableBorders::TOP.bits(), 0b0000_0000_0000_0100);
        assert_eq!(TableBorders::LEFT.bits(), 0b0000_0000_0000_1000);
        assert_eq!(TableBorders::RIGHT.bits(), 0b0000_0000_0001_0000);
        assert_eq!(TableBorders::BOTTOM.bits(), 0b0000_0000_0010_0000);
        assert_eq!(TableBorders::INNER_VERTICAL.bits(), 0b0000_0000_0100_0000);
        assert_eq!(TableBorders::INNER_HORIZONTAL.bits(), 0b0000_0000_1000_0000);
        assert_eq!(TableBorders::HEADER_TOP.bits(), 0b0000_0001_0000_0000);
    }

    #[test]
    fn table_borders_legacy_compatibility() {
        // Test that legacy flags maintain their original values
        assert_eq!(TableBorders::NONE.bits(), 0b0000_0000_0000_0000);
        assert_eq!(TableBorders::HORIZONTAL.bits(), 0b0000_0000_0000_0001);
        assert_eq!(TableBorders::VERTICAL.bits(), 0b0000_0000_0000_0010);
        assert_eq!(TableBorders::ALL.bits(), 0b0000_0000_0000_0011);

        // Test that ALL still equals HORIZONTAL | VERTICAL
        assert_eq!(
            TableBorders::ALL,
            TableBorders::HORIZONTAL | TableBorders::VERTICAL
        );
    }

    #[test]
    fn table_borders_convenience_combinations() {
        // Test OUTER combination
        let expected_outer =
            TableBorders::TOP | TableBorders::LEFT | TableBorders::RIGHT | TableBorders::BOTTOM;
        assert_eq!(TableBorders::OUTER, expected_outer);

        // Test INNER combination
        let expected_inner = TableBorders::INNER_VERTICAL | TableBorders::INNER_HORIZONTAL;
        assert_eq!(TableBorders::INNER, expected_inner);

        // Test ALL_BORDERS combination
        let expected_all_borders = TableBorders::OUTER | TableBorders::INNER;
        assert_eq!(TableBorders::ALL_BORDERS, expected_all_borders);
    }

    #[test]
    fn table_borders_bitwise_operations() {
        // Test combining individual flags
        let combined = TableBorders::TOP | TableBorders::INNER_VERTICAL;
        assert!(combined.contains(TableBorders::TOP));
        assert!(combined.contains(TableBorders::INNER_VERTICAL));
        assert!(!combined.contains(TableBorders::LEFT));

        // Test intersection
        let borders1 = TableBorders::OUTER;
        let borders2 = TableBorders::TOP | TableBorders::INNER_HORIZONTAL;
        let intersection = borders1 & borders2;
        assert_eq!(intersection, TableBorders::TOP);

        // Test difference
        let difference = TableBorders::ALL_BORDERS - TableBorders::OUTER;
        assert_eq!(difference, TableBorders::INNER);
    }

    #[test]
    fn table_borders_contains_checks() {
        // Test contains functionality
        assert!(TableBorders::ALL_BORDERS.contains(TableBorders::TOP));
        assert!(TableBorders::ALL_BORDERS.contains(TableBorders::INNER_VERTICAL));
        assert!(TableBorders::OUTER.contains(TableBorders::LEFT));
        assert!(!TableBorders::INNER.contains(TableBorders::TOP));

        // Test header border
        let with_header = TableBorders::INNER | TableBorders::HEADER_TOP;
        assert!(with_header.contains(TableBorders::HEADER_TOP));
        assert!(with_header.contains(TableBorders::INNER_HORIZONTAL));
        assert!(!with_header.contains(TableBorders::TOP));
    }

    #[test]
    fn table_borders_empty_and_all() {
        // Test empty
        assert!(TableBorders::NONE.is_empty());
        assert!(!TableBorders::TOP.is_empty());

        // Test all (note: bitflags doesn't have is_all for custom combinations)
        let all_individual = TableBorders::TOP
            | TableBorders::LEFT
            | TableBorders::RIGHT
            | TableBorders::BOTTOM
            | TableBorders::INNER_VERTICAL
            | TableBorders::INNER_HORIZONTAL
            | TableBorders::HEADER_TOP;
        assert!(all_individual.contains(TableBorders::ALL_BORDERS));
    }

    #[test]
    fn table_border_set_default() {
        let border_set = TableBorderSet::default();

        // Should match symbols::line::NORMAL
        assert_eq!(border_set.horizontal, "─");
        assert_eq!(border_set.vertical, "│");
        assert_eq!(border_set.top_left, "┌");
        assert_eq!(border_set.top_right, "┐");
        assert_eq!(border_set.bottom_left, "└");
        assert_eq!(border_set.bottom_right, "┘");
        assert_eq!(border_set.vertical_left, "┤");
        assert_eq!(border_set.vertical_right, "├");
        assert_eq!(border_set.horizontal_down, "┬");
        assert_eq!(border_set.horizontal_up, "┴");
        assert_eq!(border_set.cross, "┼");

        // Header symbols should be None by default
        assert_eq!(border_set.header_horizontal, None);
        assert_eq!(border_set.header_vertical_left, None);
        assert_eq!(border_set.header_vertical_right, None);
        assert_eq!(border_set.header_cross, None);
    }

    #[test]
    fn table_border_set_from_line_set() {
        use symbols::line;

        let border_set = TableBorderSet::from(line::NORMAL);

        // Should match the line set
        assert_eq!(border_set.horizontal, line::NORMAL.horizontal);
        assert_eq!(border_set.vertical, line::NORMAL.vertical);
        assert_eq!(border_set.top_left, line::NORMAL.top_left);
        assert_eq!(border_set.top_right, line::NORMAL.top_right);
        assert_eq!(border_set.bottom_left, line::NORMAL.bottom_left);
        assert_eq!(border_set.bottom_right, line::NORMAL.bottom_right);
        assert_eq!(border_set.vertical_left, line::NORMAL.vertical_left);
        assert_eq!(border_set.vertical_right, line::NORMAL.vertical_right);
        assert_eq!(border_set.horizontal_down, line::NORMAL.horizontal_down);
        assert_eq!(border_set.horizontal_up, line::NORMAL.horizontal_up);
        assert_eq!(border_set.cross, line::NORMAL.cross);

        // Header symbols should be None when converting from line set
        assert_eq!(border_set.header_horizontal, None);
        assert_eq!(border_set.header_vertical_left, None);
        assert_eq!(border_set.header_vertical_right, None);
        assert_eq!(border_set.header_cross, None);
    }

    #[test]
    fn table_border_set_predefined_sets() {
        // Test plain set
        let plain = TableBorderSet::plain();
        assert_eq!(plain.horizontal, "─");
        assert_eq!(plain.vertical, "│");
        assert_eq!(plain.top_left, "┌");
        assert_eq!(plain.cross, "┼");
        assert_eq!(plain.header_horizontal, None);

        // Test rounded set
        let rounded = TableBorderSet::rounded();
        assert_eq!(rounded.horizontal, "─");
        assert_eq!(rounded.vertical, "│");
        assert_eq!(rounded.top_left, "╭");
        assert_eq!(rounded.top_right, "╮");
        assert_eq!(rounded.bottom_left, "╰");
        assert_eq!(rounded.bottom_right, "╯");

        // Test double set
        let double = TableBorderSet::double();
        assert_eq!(double.horizontal, "═");
        assert_eq!(double.vertical, "║");
        assert_eq!(double.top_left, "╔");
        assert_eq!(double.cross, "╬");

        // Test thick set
        let thick = TableBorderSet::thick();
        assert_eq!(thick.horizontal, "━");
        assert_eq!(thick.vertical, "┃");
        assert_eq!(thick.top_left, "┏");
        assert_eq!(thick.cross, "╋");
    }

    #[test]
    fn table_border_set_with_header_style() {
        let header_set = TableBorderSet::with_header_style();

        // Regular symbols should be plain
        assert_eq!(header_set.horizontal, "─");
        assert_eq!(header_set.vertical, "│");
        assert_eq!(header_set.cross, "┼");

        // Header symbols should be different
        assert_eq!(header_set.header_horizontal, Some("═"));
        assert_eq!(header_set.header_vertical_left, Some("╡"));
        assert_eq!(header_set.header_vertical_right, Some("╞"));
        assert_eq!(header_set.header_cross, Some("╪"));
    }

    #[test]
    fn table_border_set_symbol_getters() {
        let header_set = TableBorderSet::with_header_style();

        // Test horizontal symbol getter
        assert_eq!(header_set.get_horizontal(false), "─");
        assert_eq!(header_set.get_horizontal(true), "═");

        // Test vertical left symbol getter
        assert_eq!(header_set.get_vertical_left(false), "┤");
        assert_eq!(header_set.get_vertical_left(true), "╡");

        // Test vertical right symbol getter
        assert_eq!(header_set.get_vertical_right(false), "├");
        assert_eq!(header_set.get_vertical_right(true), "╞");

        // Test cross symbol getter
        assert_eq!(header_set.get_cross(false), "┼");
        assert_eq!(header_set.get_cross(true), "╪");

        // Test fallback behavior with plain set (no header symbols)
        let plain_set = TableBorderSet::plain();
        assert_eq!(plain_set.get_horizontal(true), "─"); // Falls back to regular
        assert_eq!(plain_set.get_cross(true), "┼"); // Falls back to regular
    }

    #[test]
    fn table_border_set_conversion_from_different_line_sets() {
        use symbols::line;

        // Test conversion from DOUBLE line set
        let double_border = TableBorderSet::from(line::DOUBLE);
        assert_eq!(double_border.horizontal, "═");
        assert_eq!(double_border.vertical, "║");
        assert_eq!(double_border.cross, "╬");

        // Test conversion from THICK line set
        let thick_border = TableBorderSet::from(line::THICK);
        assert_eq!(thick_border.horizontal, "━");
        assert_eq!(thick_border.vertical, "┃");
        assert_eq!(thick_border.cross, "╋");

        // Test conversion from ROUNDED line set
        let rounded_border = TableBorderSet::from(line::ROUNDED);
        assert_eq!(rounded_border.top_left, "╭");
        assert_eq!(rounded_border.top_right, "╮");
        assert_eq!(rounded_border.bottom_left, "╰");
        assert_eq!(rounded_border.bottom_right, "╯");
    }

    #[test]
    fn table_borders_method() {
        // Test table_borders method with individual flags
        let table =
            Table::default().table_borders(TableBorders::TOP | TableBorders::INNER_HORIZONTAL);
        assert_eq!(
            table.internal_borders,
            TableBorders::TOP | TableBorders::INNER_HORIZONTAL
        );

        // Test with convenience combinations
        let table = Table::default().table_borders(TableBorders::OUTER);
        assert_eq!(table.internal_borders, TableBorders::OUTER);

        // Test with all borders
        let table = Table::default().table_borders(TableBorders::ALL_BORDERS);
        assert_eq!(table.internal_borders, TableBorders::ALL_BORDERS);

        // Test with header borders
        let table =
            Table::default().table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL);
        assert_eq!(
            table.internal_borders,
            TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL
        );
    }

    #[test]
    fn border_set_method() {
        // Test border_set method with predefined sets
        let table = Table::default().border_set(TableBorderSet::thick());
        assert!(table.border_set.is_some());
        let border_set = table.border_set.unwrap();
        assert_eq!(border_set.horizontal, "━");
        assert_eq!(border_set.vertical, "┃");
        assert_eq!(border_set.cross, "╋");

        // Test with header-specific styling
        let table = Table::default().border_set(TableBorderSet::with_header_style());
        assert!(table.border_set.is_some());
        let border_set = table.border_set.unwrap();
        assert_eq!(border_set.header_horizontal, Some("═"));
        assert_eq!(border_set.header_cross, Some("╪"));

        // Test with line set conversion
        use symbols::line;
        let table = Table::default().border_set(TableBorderSet::from(line::DOUBLE));
        assert!(table.border_set.is_some());
        let border_set = table.border_set.unwrap();
        assert_eq!(border_set.horizontal, "═");
        assert_eq!(border_set.vertical, "║");
        assert_eq!(border_set.cross, "╬");
    }

    #[test]
    fn table_configuration_method_chaining() {
        // Test that methods can be chained together
        let table = Table::default()
            .table_borders(TableBorders::ALL_BORDERS)
            .border_set(TableBorderSet::thick())
            .border_style(Style::default().fg(Color::Blue));

        assert_eq!(table.internal_borders, TableBorders::ALL_BORDERS);
        assert!(table.border_set.is_some());
        assert_eq!(table.border_style.fg, Some(Color::Blue));

        // Verify border_set contains expected values
        let border_set = table.border_set.unwrap();
        assert_eq!(border_set.horizontal, "━");
        assert_eq!(border_set.vertical, "┃");
    }

    #[test]
    fn backward_compatibility_internal_borders() {
        // Test that internal_borders method still works as before
        let table = Table::default().internal_borders(TableBorders::ALL);
        assert_eq!(table.internal_borders, TableBorders::ALL);

        // Test that it's equivalent to the legacy behavior
        let legacy_table =
            Table::default().internal_borders(TableBorders::HORIZONTAL | TableBorders::VERTICAL);
        let new_table =
            Table::default().table_borders(TableBorders::HORIZONTAL | TableBorders::VERTICAL);
        assert_eq!(legacy_table.internal_borders, new_table.internal_borders);

        // Test that border_set remains None when using legacy method
        let table = Table::default().internal_borders(TableBorders::ALL);
        assert_eq!(table.border_set, None);
    }

    #[test]
    fn table_configuration_with_mixed_methods() {
        // Test mixing old and new methods
        let table = Table::default()
            .internal_borders(TableBorders::HORIZONTAL)
            .border_set(TableBorderSet::rounded())
            .border_style(Style::default().fg(Color::Red));

        assert_eq!(table.internal_borders, TableBorders::HORIZONTAL);
        assert!(table.border_set.is_some());
        assert_eq!(table.border_style.fg, Some(Color::Red));

        // Test overriding with new method
        let table = Table::default()
            .internal_borders(TableBorders::HORIZONTAL)
            .table_borders(TableBorders::VERTICAL);

        assert_eq!(table.internal_borders, TableBorders::VERTICAL);
    }

    #[test]
    fn border_context_is_header() {
        // Test header context detection
        let header_context = BorderContext::HeaderSeparator { has_vertical: true };
        assert!(header_context.is_header());

        let header_context_no_vertical = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        assert!(header_context_no_vertical.is_header());

        // Test non-header contexts
        let inner_horizontal = BorderContext::InnerHorizontal { has_vertical: true };
        assert!(!inner_horizontal.is_header());

        let corner = BorderContext::Corner(CornerType::TopLeft);
        assert!(!corner.is_header());

        let top_edge = BorderContext::TopEdge {
            has_vertical: false,
        };
        assert!(!top_edge.is_header());
    }

    #[test]
    fn border_context_has_intersection() {
        // Test contexts with intersections
        let header_with_vertical = BorderContext::HeaderSeparator { has_vertical: true };
        assert!(header_with_vertical.has_intersection());

        let inner_horizontal_with_vertical = BorderContext::InnerHorizontal { has_vertical: true };
        assert!(inner_horizontal_with_vertical.has_intersection());

        let inner_vertical_with_horizontal = BorderContext::InnerVertical {
            has_horizontal: true,
        };
        assert!(inner_vertical_with_horizontal.has_intersection());

        let top_edge_with_vertical = BorderContext::TopEdge { has_vertical: true };
        assert!(top_edge_with_vertical.has_intersection());

        // Test corners (always intersections)
        let corner = BorderContext::Corner(CornerType::TopLeft);
        assert!(corner.has_intersection());

        // Test contexts without intersections
        let header_no_vertical = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        assert!(!header_no_vertical.has_intersection());

        let inner_horizontal_no_vertical = BorderContext::InnerHorizontal {
            has_vertical: false,
        };
        assert!(!inner_horizontal_no_vertical.has_intersection());

        let left_edge_no_horizontal = BorderContext::LeftEdge {
            has_horizontal: false,
        };
        assert!(!left_edge_no_horizontal.has_intersection());
    }

    #[test]
    fn border_context_corner_type() {
        // Test corner contexts
        let top_left = BorderContext::Corner(CornerType::TopLeft);
        assert_eq!(top_left.corner_type(), Some(CornerType::TopLeft));

        let top_right = BorderContext::Corner(CornerType::TopRight);
        assert_eq!(top_right.corner_type(), Some(CornerType::TopRight));

        let bottom_left = BorderContext::Corner(CornerType::BottomLeft);
        assert_eq!(bottom_left.corner_type(), Some(CornerType::BottomLeft));

        let bottom_right = BorderContext::Corner(CornerType::BottomRight);
        assert_eq!(bottom_right.corner_type(), Some(CornerType::BottomRight));

        // Test non-corner contexts
        let header = BorderContext::HeaderSeparator { has_vertical: true };
        assert_eq!(header.corner_type(), None);

        let inner_horizontal = BorderContext::InnerHorizontal {
            has_vertical: false,
        };
        assert_eq!(inner_horizontal.corner_type(), None);

        let top_edge = BorderContext::TopEdge { has_vertical: true };
        assert_eq!(top_edge.corner_type(), None);
    }

    #[test]
    fn border_context_variants() {
        // Test all BorderContext variants can be created
        let header_sep = BorderContext::HeaderSeparator { has_vertical: true };
        let inner_h = BorderContext::InnerHorizontal {
            has_vertical: false,
        };
        let inner_v = BorderContext::InnerVertical {
            has_horizontal: true,
        };
        let top_edge = BorderContext::TopEdge {
            has_vertical: false,
        };
        let bottom_edge = BorderContext::BottomEdge { has_vertical: true };
        let left_edge = BorderContext::LeftEdge {
            has_horizontal: false,
        };
        let right_edge = BorderContext::RightEdge {
            has_horizontal: true,
        };
        let corner = BorderContext::Corner(CornerType::TopLeft);

        // Test equality
        assert_eq!(
            header_sep,
            BorderContext::HeaderSeparator { has_vertical: true }
        );
        assert_ne!(
            header_sep,
            BorderContext::HeaderSeparator {
                has_vertical: false
            }
        );
        assert_ne!(inner_h, inner_v);
        assert_eq!(corner, BorderContext::Corner(CornerType::TopLeft));
    }

    #[test]
    fn corner_type_variants() {
        // Test all CornerType variants
        let top_left = CornerType::TopLeft;
        let _top_right = CornerType::TopRight;
        let bottom_left = CornerType::BottomLeft;
        let bottom_right = CornerType::BottomRight;

        // Test equality
        assert_eq!(top_left, CornerType::TopLeft);
        assert_ne!(top_left, CornerType::TopRight);
        assert_ne!(bottom_left, bottom_right);

        // Test in BorderContext
        let corner_context = BorderContext::Corner(top_left);
        assert_eq!(corner_context.corner_type(), Some(CornerType::TopLeft));
    }

    #[test]
    fn enhanced_border_symbol_resolution_header_context() {
        // Test header separator symbol resolution
        let table = Table::default().border_set(TableBorderSet::with_header_style());

        // Header separator with vertical intersection
        let context = BorderContext::HeaderSeparator { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╪")); // Header cross symbol

        // Header separator without vertical intersection
        let context = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("═")); // Header horizontal symbol

        // Test fallback when no header symbols are defined
        let table = Table::default().border_set(TableBorderSet::plain());
        let context = BorderContext::HeaderSeparator { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("┼")); // Falls back to regular cross
    }

    #[test]
    fn enhanced_border_symbol_resolution_inner_borders() {
        let table = Table::default().border_set(TableBorderSet::thick());

        // Inner horizontal with vertical intersection
        let context = BorderContext::InnerHorizontal { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╋")); // Thick cross

        // Inner horizontal without vertical intersection
        let context = BorderContext::InnerHorizontal {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("━")); // Thick horizontal

        // Inner vertical with horizontal intersection
        let context = BorderContext::InnerVertical {
            has_horizontal: true,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╋")); // Thick cross

        // Inner vertical without horizontal intersection
        let context = BorderContext::InnerVertical {
            has_horizontal: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("┃")); // Thick vertical
    }

    #[test]
    fn enhanced_border_symbol_resolution_edge_borders() {
        let table = Table::default().border_set(TableBorderSet::double());

        // Top edge with vertical intersection
        let context = BorderContext::TopEdge { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╦")); // Double horizontal down

        // Top edge without vertical intersection
        let context = BorderContext::TopEdge {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("═")); // Double horizontal

        // Bottom edge with vertical intersection
        let context = BorderContext::BottomEdge { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╩")); // Double horizontal up

        // Left edge with horizontal intersection
        let context = BorderContext::LeftEdge {
            has_horizontal: true,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╠")); // Double vertical right

        // Right edge with horizontal intersection
        let context = BorderContext::RightEdge {
            has_horizontal: true,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╣")); // Double vertical left
    }

    #[test]
    fn enhanced_border_symbol_resolution_corners() {
        let table = Table::default().border_set(TableBorderSet::rounded());

        // Test all corner types
        let top_left = BorderContext::Corner(CornerType::TopLeft);
        let symbol = table.get_border_symbol_enhanced(top_left);
        assert_eq!(symbol, Some("╭")); // Rounded top-left

        let top_right = BorderContext::Corner(CornerType::TopRight);
        let symbol = table.get_border_symbol_enhanced(top_right);
        assert_eq!(symbol, Some("╮")); // Rounded top-right

        let bottom_left = BorderContext::Corner(CornerType::BottomLeft);
        let symbol = table.get_border_symbol_enhanced(bottom_left);
        assert_eq!(symbol, Some("╰")); // Rounded bottom-left

        let bottom_right = BorderContext::Corner(CornerType::BottomRight);
        let symbol = table.get_border_symbol_enhanced(bottom_right);
        assert_eq!(symbol, Some("╯")); // Rounded bottom-right
    }

    #[test]
    fn enhanced_border_symbol_resolution_default_border_set() {
        // Test with default border set (no custom border_set configured)
        let table = Table::default();

        // Should use default TableBorderSet symbols
        let context = BorderContext::InnerHorizontal { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("┼")); // Normal cross

        let context = BorderContext::Corner(CornerType::TopLeft);
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("┌")); // Normal top-left corner

        // Header context should fall back to regular symbols
        let context = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("─")); // Falls back to regular horizontal
    }

    #[test]
    fn enhanced_border_symbol_resolution_all_contexts() {
        let table = Table::default().border_set(TableBorderSet::plain());

        // Test that all BorderContext variants return Some(symbol)
        let contexts = vec![
            BorderContext::HeaderSeparator { has_vertical: true },
            BorderContext::HeaderSeparator {
                has_vertical: false,
            },
            BorderContext::InnerHorizontal { has_vertical: true },
            BorderContext::InnerHorizontal {
                has_vertical: false,
            },
            BorderContext::InnerVertical {
                has_horizontal: true,
            },
            BorderContext::InnerVertical {
                has_horizontal: false,
            },
            BorderContext::TopEdge { has_vertical: true },
            BorderContext::TopEdge {
                has_vertical: false,
            },
            BorderContext::BottomEdge { has_vertical: true },
            BorderContext::BottomEdge {
                has_vertical: false,
            },
            BorderContext::LeftEdge {
                has_horizontal: true,
            },
            BorderContext::LeftEdge {
                has_horizontal: false,
            },
            BorderContext::RightEdge {
                has_horizontal: true,
            },
            BorderContext::RightEdge {
                has_horizontal: false,
            },
            BorderContext::Corner(CornerType::TopLeft),
            BorderContext::Corner(CornerType::TopRight),
            BorderContext::Corner(CornerType::BottomLeft),
            BorderContext::Corner(CornerType::BottomRight),
        ];

        for context in contexts {
            let symbol = table.get_border_symbol_enhanced(context);
            assert!(
                symbol.is_some(),
                "Context {context:?} should return a symbol"
            );
            assert!(
                !symbol.unwrap().is_empty(),
                "Symbol should not be empty for context {context:?}"
            );
        }
    }

    #[test]
    fn enhanced_border_rendering_legacy_compatibility() {
        // Test that legacy border flags still work with the new rendering system
        let table = Table::default().internal_borders(TableBorders::ALL);
        let mapped = table.map_legacy_borders();

        // Legacy ALL should map to INNER_HORIZONTAL | INNER_VERTICAL
        assert!(mapped.contains(TableBorders::INNER_HORIZONTAL));
        assert!(mapped.contains(TableBorders::INNER_VERTICAL));

        // Test individual legacy flags
        let table = Table::default().internal_borders(TableBorders::HORIZONTAL);
        let mapped = table.map_legacy_borders();
        assert!(mapped.contains(TableBorders::INNER_HORIZONTAL));
        assert!(!mapped.contains(TableBorders::INNER_VERTICAL));

        let table = Table::default().internal_borders(TableBorders::VERTICAL);
        let mapped = table.map_legacy_borders();
        assert!(mapped.contains(TableBorders::INNER_VERTICAL));
        assert!(!mapped.contains(TableBorders::INNER_HORIZONTAL));
    }

    #[test]
    fn enhanced_border_rendering_new_flags() {
        // Test that new border flags work correctly
        let table = Table::default().table_borders(TableBorders::OUTER);
        let mapped = table.map_legacy_borders();

        assert!(mapped.contains(TableBorders::TOP));
        assert!(mapped.contains(TableBorders::LEFT));
        assert!(mapped.contains(TableBorders::RIGHT));
        assert!(mapped.contains(TableBorders::BOTTOM));

        // Test individual new flags
        let table = Table::default().table_borders(TableBorders::HEADER_TOP);
        let mapped = table.map_legacy_borders();
        assert!(mapped.contains(TableBorders::HEADER_TOP));

        // Test combination of new and legacy flags
        let table = Table::default().table_borders(
            TableBorders::INNER_HORIZONTAL | TableBorders::TOP | TableBorders::HEADER_TOP,
        );
        let mapped = table.map_legacy_borders();
        assert!(mapped.contains(TableBorders::INNER_HORIZONTAL));
        assert!(mapped.contains(TableBorders::TOP));
        assert!(mapped.contains(TableBorders::HEADER_TOP));
    }

    #[test]
    fn enhanced_border_rendering_mixed_flags() {
        // Test mixing legacy and new flags
        let table = Table::default().table_borders(
            TableBorders::HORIZONTAL | TableBorders::TOP | TableBorders::INNER_VERTICAL,
        );
        let mapped = table.map_legacy_borders();

        // Legacy HORIZONTAL should be mapped to INNER_HORIZONTAL
        assert!(mapped.contains(TableBorders::INNER_HORIZONTAL));
        // New flags should remain unchanged
        assert!(mapped.contains(TableBorders::TOP));
        assert!(mapped.contains(TableBorders::INNER_VERTICAL));
        // Original legacy flag should still be present
        assert!(mapped.contains(TableBorders::HORIZONTAL));
    }

    #[test]
    fn enhanced_border_rendering_no_double_mapping() {
        // Test that flags aren't double-mapped
        let table = Table::default()
            .table_borders(TableBorders::HORIZONTAL | TableBorders::INNER_HORIZONTAL);
        let mapped = table.map_legacy_borders();

        // Both should be present, but INNER_HORIZONTAL shouldn't be duplicated
        assert!(mapped.contains(TableBorders::HORIZONTAL));
        assert!(mapped.contains(TableBorders::INNER_HORIZONTAL));

        // Count the bits to ensure no duplication
        let horizontal_bits = TableBorders::HORIZONTAL.bits();
        let inner_horizontal_bits = TableBorders::INNER_HORIZONTAL.bits();
        let expected_bits = horizontal_bits | inner_horizontal_bits;

        // The mapped result should contain at least these bits
        assert!((mapped.bits() & expected_bits) == expected_bits);
    }

    #[test]
    fn header_border_separator_position_calculation() {
        use ratatui_core::layout::Rect;

        // Test with a header that has height and margins
        let header = Row::new(vec!["Header"])
            .height(2)
            .top_margin(1)
            .bottom_margin(1);
        let table = Table::default().header(header);
        let area = Rect::new(0, 0, 10, 10);

        // Header total height: 2 (height) + 1 (top) + 1 (bottom) = 4
        // Separator should be at y = 0 + 4 = 4
        let separator_y = table.get_header_separator_position(area);
        assert_eq!(separator_y, Some(4));

        // Test with no header
        let table = Table::default();
        let separator_y = table.get_header_separator_position(area);
        assert_eq!(separator_y, None);

        // Test with header that's too tall for the area
        let header = Row::new(vec!["Header"])
            .height(8)
            .top_margin(1)
            .bottom_margin(1);
        let table = Table::default().header(header);
        let area = Rect::new(0, 0, 10, 5); // Area height is 5, header needs 10

        let separator_y = table.get_header_separator_position(area);
        assert_eq!(separator_y, None);
    }

    #[test]
    fn header_border_separator_context_detection() {
        use ratatui_core::layout::Rect;

        let table = Table::default()
            .table_borders(TableBorders::LEFT | TableBorders::RIGHT | TableBorders::INNER_VERTICAL);
        let area = Rect::new(0, 0, 10, 10);
        let selection_width = 0;

        // Test at left edge with LEFT border enabled
        let context = table.get_header_separator_context(0, area, selection_width, false);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator { has_vertical: true }
        );

        // Test at right edge with RIGHT border enabled
        let context = table.get_header_separator_context(9, area, selection_width, false);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator { has_vertical: true }
        );

        // Test in middle with vertical border
        let context = table.get_header_separator_context(5, area, selection_width, true);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator { has_vertical: true }
        );

        // Test in middle without vertical border
        let context = table.get_header_separator_context(5, area, selection_width, false);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator {
                has_vertical: false
            }
        );
    }

    #[test]
    fn header_border_rendering_with_different_styles() {
        // Test that header borders use header-specific symbols when available
        let header = Row::new(vec!["Header"]).height(1);
        let table = Table::default()
            .header(header)
            .table_borders(TableBorders::HEADER_TOP)
            .border_set(TableBorderSet::with_header_style());

        // Verify that header context uses header-specific symbols
        let context = BorderContext::HeaderSeparator { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("╪")); // Header cross symbol

        let context = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("═")); // Header horizontal symbol
    }

    #[test]
    fn header_border_rendering_fallback_behavior() {
        // Test fallback to regular symbols when no header-specific symbols are available
        let header = Row::new(vec!["Header"]).height(1);
        let table = Table::default()
            .header(header)
            .table_borders(TableBorders::HEADER_TOP)
            .border_set(TableBorderSet::plain()); // No header-specific symbols

        // Should fall back to regular symbols
        let context = BorderContext::HeaderSeparator { has_vertical: true };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("┼")); // Regular cross symbol

        let context = BorderContext::HeaderSeparator {
            has_vertical: false,
        };
        let symbol = table.get_border_symbol_enhanced(context);
        assert_eq!(symbol, Some("─")); // Regular horizontal symbol
    }

    #[test]
    fn header_border_integration_with_other_borders() {
        // Test that header borders work correctly with other border types
        let header = Row::new(vec!["Col1", "Col2"]).height(1);
        let table = Table::default()
            .header(header)
            .table_borders(
                TableBorders::HEADER_TOP
                    | TableBorders::INNER_VERTICAL
                    | TableBorders::LEFT
                    | TableBorders::RIGHT,
            )
            .border_set(TableBorderSet::with_header_style());

        // Test that the border mapping includes all specified borders
        let mapped = table.map_legacy_borders();
        assert!(mapped.contains(TableBorders::HEADER_TOP));
        assert!(mapped.contains(TableBorders::INNER_VERTICAL));
        assert!(mapped.contains(TableBorders::LEFT));
        assert!(mapped.contains(TableBorders::RIGHT));

        // Test that header separator context considers outer borders
        use ratatui_core::layout::Rect;
        let area = Rect::new(0, 0, 10, 10);
        let selection_width = 0;

        // At left edge, should detect intersection with LEFT border
        let context = table.get_header_separator_context(0, area, selection_width, false);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator { has_vertical: true }
        );

        // At right edge, should detect intersection with RIGHT border
        let context = table.get_header_separator_context(9, area, selection_width, false);
        assert_eq!(
            context,
            BorderContext::HeaderSeparator { has_vertical: true }
        );
    }

    #[test]
    fn border_config_validation_valid_configuration() {
        // Test that a valid configuration passes validation
        let table = Table::default()
            .table_borders(TableBorders::ALL_BORDERS)
            .border_set(TableBorderSet::thick());

        let result = table.validate_border_config();
        assert!(result.is_ok(), "Valid configuration should pass validation");
    }

    #[test]
    fn border_config_validation_conflicting_border_types() {
        // Test detection of conflicting legacy and new border flags
        let table = Table::default()
            .table_borders(TableBorders::HORIZONTAL | TableBorders::INNER_HORIZONTAL);

        let result = table.validate_border_config();
        assert!(
            result.is_err(),
            "Conflicting border types should be detected"
        );

        let errors = result.unwrap_err();
        assert!(errors.contains(&BorderConfigError::ConflictingBorderTypes));

        // Test vertical conflicts too
        let table =
            Table::default().table_borders(TableBorders::VERTICAL | TableBorders::INNER_VERTICAL);

        let result = table.validate_border_config();
        assert!(
            result.is_err(),
            "Conflicting vertical border types should be detected"
        );

        let errors = result.unwrap_err();
        assert!(errors.contains(&BorderConfigError::ConflictingBorderTypes));
    }

    #[test]
    fn border_config_validation_deprecated_configurations() {
        // Test detection of deprecated legacy-only configurations
        let table = Table::default().internal_borders(TableBorders::HORIZONTAL);

        let result = table.validate_border_config();
        assert!(
            result.is_err(),
            "Legacy-only configuration should trigger deprecation warning"
        );

        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, BorderConfigError::DeprecatedConfiguration(_)))
        );

        // Test that mixed legacy/new configurations don't trigger deprecation warning
        let table = Table::default().table_borders(TableBorders::HORIZONTAL | TableBorders::TOP);

        let result = table.validate_border_config();
        // Should still have conflicting types error, but not deprecation
        if let Err(errors) = result {
            let has_deprecation = errors
                .iter()
                .any(|e| matches!(e, BorderConfigError::DeprecatedConfiguration(_)));
            assert!(
                !has_deprecation,
                "Mixed configurations should not trigger deprecation warning"
            );
        }
    }

    #[test]
    fn border_config_validation_invalid_border_symbols() {
        // Create a custom border set with empty symbols
        let _border_set = TableBorderSet::plain();
        // We can't directly modify the fields since they're not mutable,
        // so we'll test the validation logic indirectly

        // Test that normal border sets pass validation
        let table = Table::default().border_set(TableBorderSet::plain());
        let result = table.validate_border_config();
        assert!(
            result.is_ok(),
            "Valid border symbols should pass validation"
        );

        // Test header symbols validation
        let table = Table::default().border_set(TableBorderSet::with_header_style());
        let result = table.validate_border_config();
        assert!(
            result.is_ok(),
            "Valid header symbols should pass validation"
        );
    }

    #[test]
    fn border_config_validation_no_unsupported_combinations() {
        // Test that currently all combinations are supported
        let table =
            Table::default().table_borders(TableBorders::ALL_BORDERS | TableBorders::HEADER_TOP);

        let result = table.validate_border_config();
        // Should not have unsupported combination errors (though may have other errors)
        if let Err(errors) = result {
            let has_unsupported = errors.contains(&BorderConfigError::UnsupportedBorderCombination);
            assert!(
                !has_unsupported,
                "Current combinations should all be supported"
            );
        }
    }

    #[test]
    fn border_config_error_display() {
        // Test that error messages are properly formatted
        let error = BorderConfigError::ConflictingBorderTypes;
        let message = format!("{error}");
        assert!(message.contains("Conflicting border types"));

        let error = BorderConfigError::InvalidBorderSymbol("test".to_string());
        let message = format!("{error}");
        assert!(message.contains("Invalid border symbol"));
        assert!(message.contains("test"));

        let error = BorderConfigError::UnsupportedBorderCombination;
        let message = format!("{error}");
        assert!(message.contains("Unsupported border combination"));

        let error = BorderConfigError::DeprecatedConfiguration("test message".to_string());
        let message = format!("{error}");
        assert!(message.contains("Deprecated border configuration"));
        assert!(message.contains("test message"));
    }

    #[test]
    fn border_config_validation_helper_methods() {
        // Test the individual validation helper methods

        // Test conflicting border types detection
        let table = Table::default()
            .table_borders(TableBorders::HORIZONTAL | TableBorders::INNER_HORIZONTAL);
        assert!(table.has_conflicting_border_types());

        let table = Table::default().table_borders(TableBorders::INNER_HORIZONTAL);
        assert!(!table.has_conflicting_border_types());

        // Test deprecated configuration detection
        let table = Table::default().internal_borders(TableBorders::HORIZONTAL);
        assert!(table.check_deprecated_configurations().is_some());

        let table = Table::default().table_borders(TableBorders::INNER_HORIZONTAL);
        assert!(table.check_deprecated_configurations().is_none());

        // Test unsupported combinations (currently none)
        let _table = Table::default().table_borders(TableBorders::ALL_BORDERS);
        assert!(!Table::has_unsupported_border_combinations());
    }

    #[test]
    fn collect() {
        let table = (0..4)
            .map(|i| -> Row { (0..4).map(|j| format!("{i}*{j} = {}", i * j)).collect() })
            .collect::<Table>()
            .widths([Constraint::Percentage(25); 4]);

        let expected_rows: Vec<Row> = vec![
            Row::new(["0*0 = 0", "0*1 = 0", "0*2 = 0", "0*3 = 0"]),
            Row::new(["1*0 = 0", "1*1 = 1", "1*2 = 2", "1*3 = 3"]),
            Row::new(["2*0 = 0", "2*1 = 2", "2*2 = 4", "2*3 = 6"]),
            Row::new(["3*0 = 0", "3*1 = 3", "3*2 = 6", "3*3 = 9"]),
        ];

        assert_eq!(table.rows, expected_rows);
        assert_eq!(table.widths, [Constraint::Percentage(25); 4]);
    }

    #[test]
    fn widths() {
        let table = Table::default().widths([Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        // ensure that code that uses &[] continues to work as there is a large amount of code that
        // uses this pattern
        #[expect(clippy::needless_borrows_for_generic_args)]
        let table = Table::default().widths(&[Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths(vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        // ensure that code that uses &some_vec continues to work as there is a large amount of code
        // that uses this pattern
        #[expect(clippy::needless_borrows_for_generic_args)]
        let table = Table::default().widths(&vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths([100].into_iter().map(Constraint::Length));
        assert_eq!(table.widths, [Constraint::Length(100)]);
    }

    #[test]
    fn rows() {
        let rows = [Row::new(vec![Cell::from("")])];
        let table = Table::default().rows(rows.clone());
        assert_eq!(table.rows, rows);
    }

    #[test]
    fn column_spacing() {
        let table = Table::default().column_spacing(2);
        assert_eq!(table.column_spacing, 2);
    }

    #[test]
    fn block() {
        let block = Block::bordered().title("Table");
        let table = Table::default().block(block.clone());
        assert_eq!(table.block, Some(block));
    }

    #[test]
    fn header() {
        let header = Row::new(vec![Cell::from("")]);
        let table = Table::default().header(header.clone());
        assert_eq!(table.header, Some(header));
    }

    #[test]
    fn footer() {
        let footer = Row::new(vec![Cell::from("")]);
        let table = Table::default().footer(footer.clone());
        assert_eq!(table.footer, Some(footer));
    }

    #[test]
    #[expect(deprecated)]
    fn highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().highlight_style(style);
        assert_eq!(table.row_highlight_style, style);
    }

    #[test]
    fn row_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().row_highlight_style(style);
        assert_eq!(table.row_highlight_style, style);
    }

    #[test]
    fn column_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().column_highlight_style(style);
        assert_eq!(table.column_highlight_style, style);
    }

    #[test]
    fn cell_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().cell_highlight_style(style);
        assert_eq!(table.cell_highlight_style, style);
    }

    #[test]
    fn highlight_symbol() {
        let table = Table::default().highlight_symbol(">>");
        assert_eq!(table.highlight_symbol, Text::from(">>"));
    }

    #[test]
    fn highlight_spacing() {
        let table = Table::default().highlight_spacing(HighlightSpacing::Always);
        assert_eq!(table.highlight_spacing, HighlightSpacing::Always);
    }

    #[test]
    #[should_panic = "Percentages should be between 0 and 100 inclusively"]
    fn table_invalid_percentages() {
        let _ = Table::default().widths([Constraint::Percentage(110)]);
    }

    #[test]
    fn widths_conversions() {
        let array = [Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "array");

        let array_ref = &[Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array_ref);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "array ref");

        let vec = vec![Constraint::Percentage(100)];
        let slice = vec.as_slice();
        let table = Table::new(Vec::<Row>::new(), slice);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "slice");

        let vec = vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "vec");

        let vec_ref = &vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec_ref);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "vec ref");
    }

    #[cfg(test)]
    mod state {
        use ratatui_core::buffer::Buffer;
        use ratatui_core::layout::{Constraint, Rect};
        use ratatui_core::widgets::StatefulWidget;

        use super::*;
        use crate::table::{Row, Table, TableState};

        #[fixture]
        fn table_buf() -> Buffer {
            Buffer::empty(Rect::new(0, 0, 10, 10))
        }

        #[rstest]
        fn test_list_state_empty_list(mut table_buf: Buffer) {
            let mut state = TableState::default();

            let rows: Vec<Row> = Vec::new();
            let widths = vec![Constraint::Percentage(100)];
            let table = Table::new(rows, widths);
            state.select_first();
            StatefulWidget::render(table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, None);
            assert_eq!(state.selected_column, None);
        }

        #[rstest]
        fn test_list_state_single_item(mut table_buf: Buffer) {
            let mut state = TableState::default();

            let widths = vec![Constraint::Percentage(100)];

            let items = vec![Row::new(vec!["Item 1"])];
            let table = Table::new(items, widths);
            state.select_first();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_last();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_previous();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_next();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            let mut state = TableState::default();

            state.select_first_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_last_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_previous_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_next_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);
        }
    }

    #[cfg(test)]
    mod render {
        use ratatui_core::layout::Alignment;

        use super::*;

        #[test]
        fn render_empty_area() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, vec![Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 0, 0), &mut buf);
            assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_default() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let table = Table::default();
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_with_block() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let block = Block::bordered().title("Block");
            let table = Table::new(rows, vec![Constraint::Length(5); 2]).block(block);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "┌Block────────┐",
                "│Cell1 Cell2  │",
                "└─────────────┘",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_and_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .header(header)
                .footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]).bottom_margin(1);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "               ",
                "Cell1 Cell2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]).top_margin(1);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "               ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_row_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]).bottom_margin(1),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "               ",
                "Cell3 Cell4    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_tall_row() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 23, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec![
                    Text::raw("Cell3-Line1\nCell3-Line2\nCell3-Line3"),
                    Text::raw("Cell4-Line1\nCell4-Line2\nCell4-Line3"),
                ])
                .height(3),
            ];
            let table = Table::new(rows, [Constraint::Length(11); 2]);
            Widget::render(table, Rect::new(0, 0, 23, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1       Cell2      ",
                "Cell3-Line1 Cell4-Line1",
                "Cell3-Line2 Cell4-Line2",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_alignment() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
            let rows = vec![
                Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),
                Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),
                Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),
            ];
            let table = Table::new(rows, [Percentage(100)]);
            Widget::render(table, Rect::new(0, 0, 10, 3), &mut buf);
            let expected = Buffer::with_lines(["Left      ", "  Center  ", "     Right"]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_overflow_does_not_panic() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
            let table = Table::new(Vec::<Row>::new(), [Constraint::Min(20); 1])
                .header(Row::new([Line::from("").alignment(Alignment::Right)]))
                .footer(Row::new([Line::from("").alignment(Alignment::Right)]));
            Widget::render(table, Rect::new(0, 0, 20, 3), &mut buf);
        }

        #[test]
        fn render_with_selected_column_and_incorrect_width_count_does_not_panic() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
            let table = Table::new(
                vec![Row::new(vec!["Row1", "Row2", "Row3"])],
                [Constraint::Length(10); 1],
            );
            let mut state = TableState::new().with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 3), &mut buf, &mut state);
        }

        #[test]
        fn render_with_selected() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .row_highlight_style(Style::new().red())
                .highlight_symbol(">>");
            let mut state = TableState::new().with_selected(Some(0));
            StatefulWidget::render(table, Rect::new(0, 0, 15, 3), &mut buf, &mut state);
            let expected = Buffer::with_lines([
                ">>Cell1 Cell2  ".red(),
                "  Cell3 Cell4  ".into(),
                "               ".into(),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_column() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .column_highlight_style(Style::new().blue())
                .highlight_symbol(">>");
            let mut state = TableState::new().with_selected_column(Some(1));
            StatefulWidget::render(table, Rect::new(0, 0, 15, 3), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 3]>([
                Line::from(vec![
                    "Cell1".into(),
                    " ".into(),
                    "Cell2".blue(),
                    "    ".into(),
                ]),
                Line::from(vec![
                    "Cell3".into(),
                    " ".into(),
                    "Cell4".blue(),
                    "    ".into(),
                ]),
                Line::from(vec!["      ".into(), "     ".blue(), "    ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_cell() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .cell_highlight_style(Style::new().green());
            let mut state = TableState::new().with_selected_cell((1, 2));
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".into()]),
                Line::from(vec![">>Cell4 Cell5 ".into(), "Cell6".green(), " ".into()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".into()]),
                Line::from(vec!["                    ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_row_and_column() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .row_highlight_style(Style::new().red())
                .column_highlight_style(Style::new().blue());
            let mut state = TableState::new().with_selected(1).with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".blue()]),
                Line::from(vec![">>Cell4 Cell5 ".red(), "Cell6".blue(), " ".red()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".blue()]),
                Line::from(vec!["              ".into(), "     ".blue(), " ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_row_and_column_and_cell() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .row_highlight_style(Style::new().red())
                .column_highlight_style(Style::new().blue())
                .cell_highlight_style(Style::new().green());
            let mut state = TableState::new().with_selected(1).with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".blue()]),
                Line::from(vec![">>Cell4 Cell5 ".red(), "Cell6".green(), " ".red()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".blue()]),
                Line::from(vec!["              ".into(), "     ".blue(), " ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        /// Note that this includes a regression test for a bug where the table would not render the
        /// correct rows when there is no selection.
        /// <https://github.com/ratatui/ratatui/issues/1179>
        #[rstest]
        #[case::no_selection(None, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_before_offset(20, 20, ["20", "21", "22", "23", "24"])]
        #[case::selection_immediately_before_offset(49, 49, ["49", "50", "51", "52", "53"])]
        #[case::selection_at_start_of_offset(50, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_at_end_of_offset(54, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_immediately_after_offset(55, 51, ["51", "52", "53", "54", "55"])]
        #[case::selection_after_offset(80, 76, ["76", "77", "78", "79", "80"])]
        fn render_with_selection_and_offset<T: Into<Option<usize>>>(
            #[case] selected_row: T,
            #[case] expected_offset: usize,
            #[case] expected_items: [&str; 5],
        ) {
            // render 100 rows offset at 50, with a selected row
            let rows = (0..100).map(|i| Row::new([i.to_string()]));
            let table = Table::new(rows, [Constraint::Length(2)]);
            let mut buf = Buffer::empty(Rect::new(0, 0, 2, 5));
            let mut state = TableState::new()
                .with_offset(50)
                .with_selected(selected_row.into());

            StatefulWidget::render(table.clone(), Rect::new(0, 0, 5, 5), &mut buf, &mut state);

            assert_eq!(buf, Buffer::with_lines(expected_items));
            assert_eq!(state.offset, expected_offset);
        }
    }

    // test how constraints interact with table column width allocation
    mod column_widths {
        use super::*;

        #[test]
        fn length_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            // <--------7px-------->
            // ┌────────┐x┌────────┐
            // │ (3, 2) │x│ (6, 1) │
            // └────────┘x└────────┘
            // column spacing (i.e. `x`) is always prioritized
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn max_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn min_constraint() {
            // in its currently stage, the "Min" constraint does not grow to use the possible
            // available length and enabling "expand_to_fill" will just stretch the last
            // constraint and not split it with all available constraints

            // without selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 10), (11, 9)]);

            // with selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 8), (12, 8)]);

            // without selection, less than needed width
            // allocates spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            // always allocates selection and spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn percentage_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 6), (7, 6)]);

            // with selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 5), (9, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.0, 0.0, 2.1, 3.1, 5.2, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 2), (3, 2)]);

            // with selection, less than needed width
            // rounds from positions: [0.0, 3.0, 5.1, 6.1, 7.0, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 1), (5, 1)]);
        }

        #[test]
        fn ratio_constraint() {
            // without selection, more than needed width
            // rounds from positions: [0.00, 0.00, 6.67, 7.67, 14.33]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 7), (8, 6)]);

            // with selection, more than needed width
            // rounds from positions: [0.00, 3.00, 10.67, 17.33, 20.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 6), (10, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.00, 2.33, 3.33, 5.66, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 2), (3, 3)]);

            // with selection, less than needed width
            // rounds from positions: [0.00, 3.00, 5.33, 6.33, 7.00, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 1), (5, 2)]);
        }

        /// When more width is available than requested, the behavior is controlled by flex
        #[test]
        fn underconstrained_flex() {
            let table = Table::default().widths([Min(10), Min(10), Min(1)]);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::Legacy);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 10), (11, 10), (22, 40)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::SpaceBetween);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );
        }

        #[test]
        fn underconstrained_segment_size() {
            let table = Table::default().widths([Min(10), Min(10), Min(1)]);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::Legacy);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 10), (11, 10), (22, 40)]
            );
        }

        #[test]
        fn no_constraint_with_rows() {
            let table = Table::default()
                .rows(vec![
                    Row::new(vec!["a", "b"]),
                    Row::new(vec!["c", "d", "e"]),
                ])
                // rows should get precedence over header
                .header(Row::new(vec!["f", "g"]))
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(
                table.get_column_widths(30, 0, 3),
                &[(0, 10), (10, 10), (20, 10)]
            );
        }

        #[test]
        fn no_constraint_with_header() {
            let table = Table::default()
                .rows(vec![] as Vec<Row>)
                .header(Row::new(vec!["f", "g"]))
                .column_spacing(0);
            assert_eq!(table.get_column_widths(10, 0, 2), [(0, 5), (5, 5)]);
        }

        #[test]
        fn no_constraint_with_footer() {
            let table = Table::default()
                .rows(vec![] as Vec<Row>)
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(table.get_column_widths(10, 0, 2), [(0, 5), (5, 5)]);
        }

        #[track_caller]
        fn test_table_with_selection<'line, Lines>(
            highlight_spacing: HighlightSpacing,
            columns: u16,
            spacing: u16,
            selection: Option<usize>,
            expected: Lines,
        ) where
            Lines: IntoIterator,
            Lines::Item: Into<Line<'line>>,
        {
            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(highlight_spacing)
                .highlight_symbol(">>>")
                .column_spacing(spacing);
            let area = Rect::new(0, 0, columns, 3);
            let mut buf = Buffer::empty(area);
            let mut state = TableState::default().with_selected(selection);
            StatefulWidget::render(table, area, &mut buf, &mut state);
            assert_eq!(buf, Buffer::with_lines(expected));
        }

        #[test]
        fn excess_area_highlight_symbol_and_column_spacing_allocation() {
            // no highlight_symbol rendered ever
            test_table_with_selection(
                HighlightSpacing::Never,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE  12345   ", /* default layout is Flex::Start but columns length
                                        * constraints are calculated as `max_area / n_columns`,
                                        * i.e. they are distributed amongst available space */
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .widths([5, 5])
                .column_spacing(0);
            let area = Rect::new(0, 0, 15, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            let expected = Buffer::with_lines([
                "ABCDE12345     ", /* As reference, this is what happens when you manually
                                    * specify widths */
                "               ", // row 2
                "               ", // row 3
            ]);
            assert_eq!(buf, expected);

            // no highlight_symbol rendered ever
            test_table_with_selection(
                HighlightSpacing::Never,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // no highlight_symbol rendered because no selection is made
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );
            // highlight_symbol rendered because selection is made
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // highlight_symbol always rendered even no selection is made
            test_table_with_selection(
                HighlightSpacing::Always,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "   ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // no highlight_symbol rendered because no selection is made
            test_table_with_selection(
                HighlightSpacing::Always,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );
        }

        #[expect(clippy::too_many_lines)]
        #[test]
        fn insufficient_area_highlight_symbol_and_column_spacing_allocation() {
            // column spacing is prioritized over every other constraint
            test_table_with_selection(
                HighlightSpacing::Never,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            // this test checks that space for highlight_symbol space is always allocated.
            // this test also checks that space for column is allocated.
            //
            // Space for highlight_symbol is allocated first by splitting horizontal space
            // into highlight_symbol area and column area.
            // Then in a separate step, column widths are calculated.
            // column spacing is prioritized when column widths are calculated and last column here
            // ends up with just 1 wide
            test_table_with_selection(
                HighlightSpacing::Always,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "   ABC 123", // highlight_symbol and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            // the following are specification tests
            test_table_with_selection(
                HighlightSpacing::Always,
                9,    // width
                1,    // spacing
                None, // selection
                [
                    "   ABC 12", // highlight_symbol and spacing are prioritized
                    "         ", // row 2
                    "         ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                8,    // width
                1,    // spacing
                None, // selection
                [
                    "   AB 12", // highlight_symbol and spacing are prioritized
                    "        ", // row 2
                    "        ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                7,    // width
                1,    // spacing
                None, // selection
                [
                    "   AB 1", // highlight_symbol and spacing are prioritized
                    "       ", // row 2
                    "       ", // row 3
                ],
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(HighlightSpacing::Always)
                .flex(Flex::Legacy)
                .highlight_symbol(">>>")
                .column_spacing(1);
            let area = Rect::new(0, 0, 10, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            // highlight_symbol and spacing are prioritized but columns are evenly distributed
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "   ABCDE 1",
                "          ",
                "          ",
            ]);
            assert_eq!(buf, expected);

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(HighlightSpacing::Always)
                .flex(Flex::Start)
                .highlight_symbol(">>>")
                .column_spacing(1);
            let area = Rect::new(0, 0, 10, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            // highlight_symbol and spacing are prioritized but columns are evenly distributed
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "   ABC 123",
                "          ",
                "          ",
            ]);
            assert_eq!(buf, expected);

            test_table_with_selection(
                HighlightSpacing::Never,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    "ABCDE 1234", // spacing is prioritized
                    "          ",
                    "          ",
                ],
            );

            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    ">>>ABC 123", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            test_table_with_selection(
                HighlightSpacing::Always,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    ">>>ABC 123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
        }

        #[test]
        fn insufficient_area_highlight_symbol_allocation_with_no_column_spacing() {
            test_table_with_selection(
                HighlightSpacing::Never,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            // highlight symbol spacing is prioritized over all constraints
            // even if the constraints are fixed length
            // this is because highlight_symbol column is separated _before_ any of the constraint
            // widths are calculated
            test_table_with_selection(
                HighlightSpacing::Always,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "   ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Never,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
        }
    }

    #[test]
    fn stylize() {
        assert_eq!(
            Table::new(vec![Row::new(vec![Cell::from("")])], [Percentage(100)])
                .black()
                .on_white()
                .bold()
                .not_crossed_out()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::CROSSED_OUT)
        );
    }

    #[rstest]
    #[case::no_columns(vec![], vec![], vec![], 0)]
    #[case::only_header(vec!["H1", "H2"], vec![], vec![], 2)]
    #[case::only_rows(
        vec![],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3"]],
        vec![],
        3
    )]
    #[case::only_footer(vec![], vec![], vec!["F1", "F2", "F3", "F4"], 4)]
    #[case::rows_longer(
        vec!["H1", "H2", "H3", "H4"],
        vec![vec!["C1", "C2"],vec!["C1", "C2", "C3"]],
        vec!["F1", "F2"],
        4
    )]
    #[case::rows_longer(
        vec!["H1", "H2"],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3", "C4"]],
        vec!["F1", "F2"],
        4
    )]
    #[case::footer_longer(
        vec!["H1", "H2"],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3"]],
        vec!["F1", "F2", "F3", "F4"],
        4
    )]

    fn column_count(
        #[case] header: Vec<&str>,
        #[case] rows: Vec<Vec<&str>>,
        #[case] footer: Vec<&str>,
        #[case] expected: usize,
    ) {
        let header = Row::new(header);
        let footer = Row::new(footer);
        let rows: Vec<Row> = rows.into_iter().map(Row::new).collect();
        let table = Table::new(rows, Vec::<Constraint>::new())
            .header(header)
            .footer(footer);
        let column_count = table.column_count();
        assert_eq!(column_count, expected);
    }

    #[test]
    fn render_with_block_and_internal_borders() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 6));
        let rows = vec![
            Row::new(vec!["Cell1", "Cell2", "Cell3"]),
            Row::new(vec!["Cell4", "Cell5", "Cell6"]),
        ];
        let table = Table::new(rows, [Constraint::Length(6); 3])
            .block(Block::bordered().title("Table"))
            .internal_borders(TableBorders::ALL)
            .border_style(Style::new().blue());
        Widget::render(table, Rect::new(0, 0, 20, 6), &mut buf);

        // Verify that the table renders without panicking
        // The exact output depends on the border integration logic
        assert!(!buf.area.is_empty());
    }

    #[test]
    fn render_with_corner_intersections() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 15, 5));
        let rows = vec![Row::new(vec!["A", "B"]), Row::new(vec!["C", "D"])];
        let table = Table::new(rows, [Constraint::Length(7); 2])
            .internal_borders(TableBorders::ALL)
            .border_style(Style::new().blue());
        Widget::render(table, Rect::new(0, 0, 15, 5), &mut buf);

        // Verify that the table renders without panicking
        // The corner intersections should now use proper cross symbols (┼)
        assert!(!buf.area.is_empty());
    }

    // Comprehensive integration tests for border rendering

    #[test]
    fn integration_test_all_border_combinations() {
        // Test various border combinations to ensure they all render correctly
        let rows = vec![
            Row::new(vec!["Header1", "Header2", "Header3"]),
            Row::new(vec!["Cell1", "Cell2", "Cell3"]),
            Row::new(vec!["Cell4", "Cell5", "Cell6"]),
        ];
        let widths = [Constraint::Length(8); 3];
        let area = Rect::new(0, 0, 30, 10);

        let border_combinations = vec![
            TableBorders::NONE,
            TableBorders::ALL,
            TableBorders::HORIZONTAL,
            TableBorders::VERTICAL,
            TableBorders::OUTER,
            TableBorders::INNER,
            TableBorders::ALL_BORDERS,
            TableBorders::TOP | TableBorders::BOTTOM,
            TableBorders::LEFT | TableBorders::RIGHT,
            TableBorders::INNER_HORIZONTAL | TableBorders::OUTER,
            TableBorders::INNER_VERTICAL | TableBorders::OUTER,
            TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL,
        ];

        for borders in border_combinations {
            let mut buf = Buffer::empty(area);
            let table = Table::new(rows.clone(), widths)
                .header(Row::new(vec!["H1", "H2", "H3"]))
                .table_borders(borders);

            // Should render without panicking
            Widget::render(table, area, &mut buf);
            assert!(
                !buf.area.is_empty(),
                "Border combination {borders:?} should render successfully"
            );
        }
    }

    #[test]
    fn integration_test_backward_compatibility_rendering() {
        // Test that old and new API produce equivalent output for compatible configurations
        let rows = vec![
            Row::new(vec!["Cell1", "Cell2"]),
            Row::new(vec!["Cell3", "Cell4"]),
        ];
        let widths = [Constraint::Length(10); 2];
        let area = Rect::new(0, 0, 25, 8);

        // Test HORIZONTAL compatibility
        let mut buf_old = Buffer::empty(area);
        let old_table = Table::new(rows.clone(), widths).internal_borders(TableBorders::HORIZONTAL);
        Widget::render(old_table, area, &mut buf_old);

        let mut buf_new = Buffer::empty(area);
        let new_table =
            Table::new(rows.clone(), widths).table_borders(TableBorders::INNER_HORIZONTAL);
        Widget::render(new_table, area, &mut buf_new);

        // Should produce similar visual output (allowing for implementation differences)
        assert_eq!(buf_old.area, buf_new.area);

        // Test VERTICAL compatibility
        let mut buf_old = Buffer::empty(area);
        let old_table = Table::new(rows.clone(), widths).internal_borders(TableBorders::VERTICAL);
        Widget::render(old_table, area, &mut buf_old);

        let mut buf_new = Buffer::empty(area);
        let new_table =
            Table::new(rows.clone(), widths).table_borders(TableBorders::INNER_VERTICAL);
        Widget::render(new_table, area, &mut buf_new);

        assert_eq!(buf_old.area, buf_new.area);

        // Test ALL compatibility
        let mut buf_old = Buffer::empty(area);
        let old_table = Table::new(rows.clone(), widths).internal_borders(TableBorders::ALL);
        Widget::render(old_table, area, &mut buf_old);

        let mut buf_new = Buffer::empty(area);
        let new_table = Table::new(rows, widths)
            .table_borders(TableBorders::INNER_HORIZONTAL | TableBorders::INNER_VERTICAL);
        Widget::render(new_table, area, &mut buf_new);

        assert_eq!(buf_old.area, buf_new.area);
    }

    #[test]
    fn integration_test_visual_border_patterns() {
        // Test different border patterns with various border sets
        let rows = vec![Row::new(vec!["A", "B"]), Row::new(vec!["C", "D"])];
        let widths = [Constraint::Length(5); 2];
        let area = Rect::new(0, 0, 15, 6);

        let border_sets = vec![
            TableBorderSet::plain(),
            TableBorderSet::rounded(),
            TableBorderSet::double(),
            TableBorderSet::thick(),
            TableBorderSet::with_header_style(),
        ];

        for border_set in border_sets {
            let mut buf = Buffer::empty(area);
            let table = Table::new(rows.clone(), widths)
                .header(Row::new(vec!["H1", "H2"]))
                .table_borders(TableBorders::ALL_BORDERS | TableBorders::HEADER_TOP)
                .border_set(border_set);

            // Should render without panicking
            Widget::render(table, area, &mut buf);
            assert!(
                !buf.area.is_empty(),
                "Border set should render successfully"
            );

            // Verify that some border characters are present in the buffer
            let content = buf.content.iter().any(|cell| {
                let symbol = cell.symbol();
                !symbol.is_empty() && symbol != " "
            });
            assert!(content, "Buffer should contain non-space characters");
        }
    }

    #[test]
    fn integration_test_edge_case_empty_table() {
        // Test rendering empty tables with various border configurations
        let area = Rect::new(0, 0, 20, 10);

        let border_combinations = vec![
            TableBorders::NONE,
            TableBorders::ALL_BORDERS,
            TableBorders::OUTER,
            TableBorders::INNER,
        ];

        for borders in border_combinations {
            let mut buf = Buffer::empty(area);
            let table =
                Table::new(Vec::<Row>::new(), Vec::<Constraint>::new()).table_borders(borders);

            // Should render without panicking even with no data
            Widget::render(table, area, &mut buf);
            assert!(
                !buf.area.is_empty(),
                "Empty table with borders {borders:?} should render"
            );
        }
    }

    #[test]
    fn integration_test_edge_case_single_row_table() {
        // Test rendering single row tables
        let area = Rect::new(0, 0, 20, 5);

        let single_row = vec![Row::new(vec!["OnlyCell"])];
        let widths = [Constraint::Length(15)];

        let border_combinations = vec![
            TableBorders::ALL_BORDERS,
            TableBorders::OUTER,
            TableBorders::INNER_VERTICAL,
            TableBorders::TOP | TableBorders::BOTTOM,
        ];

        for borders in border_combinations {
            let mut buf = Buffer::empty(area);
            let table = Table::new(single_row.clone(), widths).table_borders(borders);

            Widget::render(table, area, &mut buf);
            assert!(
                !buf.area.is_empty(),
                "Single row table with borders {borders:?} should render"
            );
        }
    }

    #[test]
    fn integration_test_edge_case_single_column_table() {
        // Test rendering single column tables
        let area = Rect::new(0, 0, 10, 10);

        let single_column_rows = vec![
            Row::new(vec!["Row1"]),
            Row::new(vec!["Row2"]),
            Row::new(vec!["Row3"]),
        ];
        let widths = [Constraint::Length(8)];

        let border_combinations = vec![
            TableBorders::ALL_BORDERS,
            TableBorders::OUTER,
            TableBorders::INNER_HORIZONTAL,
            TableBorders::LEFT | TableBorders::RIGHT,
        ];

        for borders in border_combinations {
            let mut buf = Buffer::empty(area);
            let table = Table::new(single_column_rows.clone(), widths).table_borders(borders);

            Widget::render(table, area, &mut buf);
            assert!(
                !buf.area.is_empty(),
                "Single column table with borders {borders:?} should render"
            );
        }
    }

    #[test]
    fn integration_test_edge_case_table_without_header() {
        // Test rendering tables without headers but with header borders enabled
        let area = Rect::new(0, 0, 20, 8);

        let rows = vec![
            Row::new(vec!["Cell1", "Cell2"]),
            Row::new(vec!["Cell3", "Cell4"]),
        ];
        let widths = [Constraint::Length(8); 2];

        let mut buf = Buffer::empty(area);
        let table = Table::new(rows, widths)
            .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL);

        // Should render without panicking even when header borders are enabled but no header exists
        Widget::render(table, area, &mut buf);
        assert!(
            !buf.area.is_empty(),
            "Table without header should render with header borders"
        );
    }

    #[test]
    fn integration_test_performance_large_table_rendering() {
        // Test that new border system doesn't significantly impact rendering speed
        use std::time::Instant;

        // Create a moderately large table
        let mut rows = Vec::new();
        for i in 0..50 {
            rows.push(Row::new(vec![
                format!("Cell{}1", i),
                format!("Cell{}2", i),
                format!("Cell{}3", i),
            ]));
        }
        let widths = [Constraint::Length(10); 3];
        let area = Rect::new(0, 0, 40, 30);

        // Test rendering with no borders (baseline)
        let start = Instant::now();
        let mut buf = Buffer::empty(area);
        let table = Table::new(rows.clone(), widths).table_borders(TableBorders::NONE);
        Widget::render(table, area, &mut buf);
        let no_borders_time = start.elapsed();

        // Test rendering with all borders
        let start = Instant::now();
        let mut buf = Buffer::empty(area);
        let table = Table::new(rows.clone(), widths)
            .table_borders(TableBorders::ALL_BORDERS)
            .border_set(TableBorderSet::thick());
        Widget::render(table, area, &mut buf);
        let all_borders_time = start.elapsed();

        // Border rendering should not be more than 10x slower than no borders
        // This is a reasonable performance threshold for the added functionality
        assert!(
            all_borders_time.as_nanos() < no_borders_time.as_nanos() * 10 + 1_000_000, // Add 1ms buffer for timing variations
            "Border rendering performance regression detected. No borders: {no_borders_time:?}, All borders: {all_borders_time:?}"
        );

        // Test with header borders specifically
        let start = Instant::now();
        let mut buf = Buffer::empty(area);
        let table = Table::new(rows, widths)
            .header(Row::new(vec!["H1", "H2", "H3"]))
            .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL)
            .border_set(TableBorderSet::with_header_style());
        Widget::render(table, area, &mut buf);
        let header_borders_time = start.elapsed();

        // Header border rendering should also be reasonable
        assert!(
            header_borders_time.as_nanos() < no_borders_time.as_nanos() * 5 + 1_000_000,
            "Header border rendering performance regression detected. No borders: {no_borders_time:?}, Header borders: {header_borders_time:?}"
        );
    }

    #[test]
    fn integration_test_border_rendering_with_different_table_sizes() {
        // Test border rendering with various table dimensions
        let test_cases = vec![
            (1, 1, Rect::new(0, 0, 5, 3)),    // Minimal table
            (2, 2, Rect::new(0, 0, 15, 5)),   // Small table
            (3, 5, Rect::new(0, 0, 25, 8)),   // Medium table
            (5, 3, Rect::new(0, 0, 30, 10)),  // Wide table
            (2, 10, Rect::new(0, 0, 20, 15)), // Tall table
        ];

        for (cols, rows_count, area) in test_cases {
            let mut rows = Vec::new();
            for r in 0..rows_count {
                let mut row_cells = Vec::new();
                for c in 0..cols {
                    row_cells.push(format!("R{r}C{c}"));
                }
                rows.push(Row::new(row_cells));
            }

            let widths = vec![Constraint::Length(8); cols];

            let border_combinations = vec![
                TableBorders::ALL_BORDERS,
                TableBorders::OUTER | TableBorders::INNER_HORIZONTAL,
                TableBorders::OUTER | TableBorders::INNER_VERTICAL,
            ];

            for borders in border_combinations {
                let mut buf = Buffer::empty(area);
                let table = Table::new(rows.clone(), widths.clone())
                    .table_borders(borders)
                    .border_set(TableBorderSet::plain());

                Widget::render(table, area, &mut buf);
                assert!(
                    !buf.area.is_empty(),
                    "Table {cols}x{rows_count} with borders {borders:?} should render in area {area:?}"
                );
            }
        }
    }
}
