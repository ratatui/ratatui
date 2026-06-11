//! Linear row and column regions.
//!
//! This module adapts Ratatui's existing [`Layout`] solver into inspectable
//! [`Regions`](crate::regions::Regions) values. It does not replace [`Layout`]. Instead, it keeps
//! the same constraint behavior and adds region identifiers, render-order metadata, and hit testing
//! through the returned region set.
//!
//! If the solved rectangles are consumed immediately and never inspected again, use [`Layout`]
//! directly. Use this module when the result needs to be passed around as data, hit tested, or
//! associated with external ids.
//! Ordinary `Layout::areas` remains the better tool for local render-only splits because it avoids
//! introducing ids and stored regions where no later coordination is needed.
//!
//! # Types
//!
//! - [`RegionsExt`](crate::linear::RegionsExt) adds
//!   [`RegionsExt::regions`](crate::linear::RegionsExt::regions) to Ratatui [`Layout`] for
//!   incremental adoption.
//! - [`Row`](crate::linear::Row) is a horizontal wrapper that can return indexed or named
//!   left-to-right regions.
//! - [`Column`](crate::linear::Column) is a vertical wrapper that can return indexed or named
//!   top-to-bottom regions.
//!
//! See [`crate::docs::regions`] for the frame-local geometry model and
//! [`crate::docs::containers`] for how rows and columns fit into larger container composition.
//!
//! [`Layout`]: ratatui_core::layout::Layout
//! [`Regions`](crate::regions::Regions): crate::regions::Regions
//!
//! # Examples
//!
//! Pair ids with constraints when the split is page structure. This keeps the region name beside
//! the sizing rule and avoids later numeric indexing:
//!
//! ```rust
//! use ratatui_core::layout::{Constraint, Rect};
//! use ratatui_layout::linear::{Column, Row};
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq)]
//! enum PageSlot {
//!     Header,
//!     Body,
//! }
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq)]
//! enum BodySlot {
//!     Sidebar,
//!     Content,
//! }
//!
//! let page_slots = [
//!     (PageSlot::Header, Constraint::Length(1)),
//!     (PageSlot::Body, Constraint::Fill(1)),
//! ];
//! let page = Column::named(page_slots).regions(Rect::new(0, 0, 20, 5));
//!
//! let body_slots = [
//!     (BodySlot::Sidebar, Constraint::Length(6)),
//!     (BodySlot::Content, Constraint::Fill(1)),
//! ];
//! let body_area = page.area_for(PageSlot::Body).unwrap();
//! let body = Row::named(body_slots).regions(body_area);
//!
//! assert_eq!(body.hit_test((8, 1)).unwrap().id, BodySlot::Content);
//! ```

use alloc::vec::Vec;

use ratatui_core::layout::{Constraint, Direction, Flex, Layout, Rect, Spacing};

use crate::regions::{Region, Regions};

/// Extension methods for producing inspectable region sets from Ratatui layouts.
///
/// This trait is the lowest-friction bridge from existing Ratatui code. Any [`Layout`] can still be
/// used normally with `split` or `areas`; calling
/// [`RegionsExt::regions`](crate::linear::RegionsExt::regions) returns the same solved rectangles
/// wrapped in a [`Regions`](crate::regions::Regions) with `usize` region ids.
///
/// This is useful for incremental adoption: start with the layout you already have, then switch the
/// call from `areas` or `split` to [`RegionsExt::regions`](crate::linear::RegionsExt::regions) only
/// where the solved rectangles need hit testing or metadata.
///
/// # Method
///
/// - [`RegionsExt::regions`](crate::linear::RegionsExt::regions) solves a Ratatui [`Layout`] and
///   wraps the rectangles in a [`Regions`](crate::regions::Regions) with region ids assigned by
///   split order.
///
/// # Examples
///
/// Convert an existing layout call site into a region set without changing its constraints:
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Layout, Rect};
/// use ratatui_layout::linear::RegionsExt;
///
/// let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
/// let regions = layout.regions(Rect::new(0, 0, 20, 5));
///
/// assert_eq!(regions.regions()[0].area.height, 1);
/// assert_eq!(regions.regions()[1].id, 1);
/// ```
pub trait RegionsExt {
    /// Splits an area and returns the result as a [`Regions`](crate::regions::Regions).
    ///
    /// Region ids are assigned by split order starting at zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    /// use ratatui_layout::linear::RegionsExt;
    ///
    /// let layout = Layout::horizontal([Constraint::Length(4), Constraint::Fill(1)]);
    /// let plan = layout.regions(Rect::new(0, 0, 10, 1));
    ///
    /// assert_eq!(plan.regions()[0].area.width, 4);
    /// assert_eq!(plan.regions()[1].id, 1);
    /// ```
    fn regions(&self, area: Rect) -> Regions;
}

impl RegionsExt for Layout {
    fn regions(&self, area: Rect) -> Regions {
        let regions: Vec<_> = self
            .split(area)
            .iter()
            .copied()
            .enumerate()
            .map(|(id, area)| Region::new(id, area))
            .collect();
        Regions::from_regions(area, regions)
    }
}

/// Horizontal region builder that produces [`Regions`](crate::regions::Regions) values.
///
/// Use [`Row`](crate::linear::Row) when a horizontal split needs to become inspectable regions. A
/// common case is a row with left content and a right status label: normal [`Layout`] can split the
/// row, while [`Row`](crate::linear::Row) returns those split rectangles as regions that can be
/// rendered, tested, or hit-tested.
///
/// `Row` is a small convenience wrapper around a horizontal Ratatui [`Layout`]. It is useful when
/// code wants to speak in terms of row regions instead of generic direction-based layout.
///
/// # Constructors and setters
///
/// - [`Row::new`](crate::linear::Row::new) creates horizontal regions from Ratatui constraints and
///   assigns `usize` ids.
/// - [`Row::named`](crate::linear::Row::named) creates horizontal regions from `(id, constraint)`
///   pairs.
/// - [`Row::margin`](crate::linear::Row::margin),
///   [`Row::horizontal_margin`](crate::linear::Row::horizontal_margin), and
///   [`Row::vertical_margin`](crate::linear::Row::vertical_margin) delegate to the wrapped
///   [`Layout`] margin behavior.
/// - [`Row::flex`](crate::linear::Row::flex) delegates excess-space placement to Ratatui's solver.
/// - [`Row::spacing`](crate::linear::Row::spacing) delegates fixed spacing or overlap between
///   cells.
///
/// # Inspection and solving
///
/// - [`Row::layout`](crate::linear::Row::layout) returns the wrapped Ratatui [`Layout`] for APIs
///   not mirrored here.
/// - [`Row::regions`](crate::linear::Row::regions) solves the row and returns a
///   [`Regions`](crate::regions::Regions) with left-to-right ids.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::linear::Row;
///
/// let row = Row::new([Constraint::Length(8), Constraint::Fill(1)]).spacing(1);
/// let regions = row.regions(Rect::new(0, 0, 20, 1));
///
/// assert_eq!(regions.hit_test((2, 0)).unwrap().id, 0);
/// assert_eq!(regions.hit_test((12, 0)).unwrap().id, 1);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Row<Id = usize> {
    layout: Layout,
    ids: Vec<Id>,
}

impl Row {
    /// Creates a row from horizontal constraints.
    ///
    /// The constraints use the same semantics as [`ratatui_core::layout::Layout::horizontal`].
    ///
    /// # Examples
    ///
    /// Split a status row into app-owned left and right regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Fill(1), Constraint::Length(6)])
    ///     .spacing(1)
    ///     .regions(Rect::new(0, 0, 20, 1));
    ///
    /// assert_eq!(plan.regions()[1].id, 1);
    /// assert_eq!(plan.regions()[1].area.width, 6);
    /// ```
    pub fn new<I>(constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        let constraints = constraints.into_iter().map(Into::into).collect::<Vec<_>>();
        let ids = (0..constraints.len()).collect();
        Self {
            layout: Layout::new(Direction::Horizontal, constraints),
            ids,
        }
    }
}

impl<Id> Row<Id> {
    /// Creates a row from `(id, constraint)` pairs.
    ///
    /// Use this for structural layouts where each region already has a meaning at the point where
    /// its constraint is declared. Pairing the id with the constraint keeps the layout
    /// vocabulary local and avoids fragile numeric indexing such as `regions()[1]`.
    ///
    /// For repeated generated regions, [`Row::new`](crate::linear::Row::new) is still a good fit
    /// because numeric ids often line up naturally with item indexes.
    ///
    /// # Examples
    ///
    /// Name page body regions at the same time their widths are declared:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum BodySlot {
    ///     Sidebar,
    ///     Content,
    /// }
    ///
    /// let body_slots = [
    ///     (BodySlot::Sidebar, Constraint::Length(20)),
    ///     (BodySlot::Content, Constraint::Fill(1)),
    /// ];
    /// let body = Row::named(body_slots)
    ///     .spacing(1)
    ///     .regions(Rect::new(0, 0, 50, 1));
    ///
    /// assert_eq!(body.area_for(BodySlot::Content).unwrap().x, 21);
    /// ```
    pub fn named<I, C>(regions: I) -> Self
    where
        I: IntoIterator<Item = (Id, C)>,
        C: Into<Constraint>,
    {
        let (ids, constraints): (Vec<_>, Vec<_>) = regions
            .into_iter()
            .map(|(id, constraint)| (id, constraint.into()))
            .unzip();
        Self {
            layout: Layout::new(Direction::Horizontal, constraints),
            ids,
        }
    }

    /// Returns the inner Ratatui layout.
    ///
    /// Use this when a caller needs access to existing `Layout` configuration or methods that are
    /// not mirrored by `Row`.
    ///
    /// # Examples
    ///
    /// Inspect the wrapped layout when interoperating with existing Ratatui code:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let row = Row::new([Constraint::Length(4), Constraint::Fill(1)]);
    /// let areas = row.layout().split(Rect::new(0, 0, 10, 1));
    ///
    /// assert_eq!(areas[0].width, 4);
    /// ```
    pub const fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Sets the row margin on all edges before solving constraints.
    ///
    /// # Examples
    ///
    /// Keep planned regions away from the border area of a one-line toolbar:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Fill(1)])
    ///     .margin(1)
    ///     .regions(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(1, 1, 8, 1));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn margin(mut self, margin: u16) -> Self {
        self.layout = self.layout.margin(margin);
        self
    }

    /// Sets the row horizontal margin before solving constraints.
    ///
    /// # Examples
    ///
    /// Inset row items horizontally while preserving the full row height:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Fill(1)])
    ///     .horizontal_margin(2)
    ///     .regions(Rect::new(0, 0, 12, 2));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(2, 0, 8, 2));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn horizontal_margin(mut self, horizontal: u16) -> Self {
        self.layout = self.layout.horizontal_margin(horizontal);
        self
    }

    /// Sets the row vertical margin before solving constraints.
    ///
    /// # Examples
    ///
    /// Reserve top and bottom space for surrounding chrome:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Fill(1)])
    ///     .vertical_margin(1)
    ///     .regions(Rect::new(0, 0, 12, 3));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(0, 1, 12, 1));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn vertical_margin(mut self, vertical: u16) -> Self {
        self.layout = self.layout.vertical_margin(vertical);
        self
    }

    /// Sets how excess horizontal space is distributed.
    ///
    /// # Examples
    ///
    /// Center fixed-width command regions in the available row:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Flex, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Length(4), Constraint::Length(4)])
    ///     .flex(Flex::Center)
    ///     .regions(Rect::new(0, 0, 12, 1));
    ///
    /// assert_eq!(plan.regions()[0].area.x, 2);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.layout = self.layout.flex(flex);
        self
    }

    /// Sets spacing or overlap between row items.
    ///
    /// # Examples
    ///
    /// Separate adjacent toolbar commands with one cell of spacing:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let plan = Row::new([Constraint::Length(4), Constraint::Length(4)])
    ///     .spacing(1)
    ///     .regions(Rect::new(0, 0, 10, 1));
    ///
    /// assert_eq!(plan.regions()[1].area.x, 5);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn spacing<T>(mut self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.layout = self.layout.spacing(spacing);
        self
    }

    /// Solves the row into regions.
    ///
    /// The returned regions use ids in left-to-right order.
    ///
    /// # Examples
    ///
    /// Store the solved row for later hit testing:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Row;
    ///
    /// let previous_frame =
    ///     Row::new([Constraint::Length(6), Constraint::Length(6)]).regions(Rect::new(0, 0, 12, 1));
    ///
    /// assert_eq!(previous_frame.hit_test((8, 0)).unwrap().id, 1);
    /// ```
    pub fn regions(&self, area: Rect) -> Regions<Id>
    where
        Id: Clone,
    {
        let regions = self
            .layout
            .split(area)
            .iter()
            .copied()
            .zip(self.ids.iter().cloned())
            .map(|(area, id)| Region::new(id, area))
            .collect::<Vec<_>>();
        Regions::from_regions(area, regions)
    }
}

/// Vertical region builder that produces [`Regions`](crate::regions::Regions) values.
///
/// Use [`Column`](crate::linear::Column) when a vertical stack of app-owned items needs stable
/// region ids. For ordinary page structure,
/// [`Layout::vertical`](ratatui_core::layout::Layout::vertical) followed by `areas(area)` is
/// simpler. [`Column`](crate::linear::Column) is useful when each visible row is handed to an
/// external participant or later hit tested.
///
/// `Column` is the vertical counterpart to [`Row`](crate::linear::Row). It assigns region ids from
/// top to bottom.
///
/// # Constructors and setters
///
/// - [`Column::new`](crate::linear::Column::new) creates vertical regions from Ratatui constraints
///   and assigns `usize` ids.
/// - [`Column::named`](crate::linear::Column::named) creates vertical regions from `(id,
///   constraint)` pairs.
/// - [`Column::margin`](crate::linear::Column::margin),
///   [`Column::horizontal_margin`](crate::linear::Column::horizontal_margin), and
///   [`Column::vertical_margin`](crate::linear::Column::vertical_margin) delegate to the wrapped
///   [`Layout`] margin behavior.
/// - [`Column::flex`](crate::linear::Column::flex) delegates excess-space placement to Ratatui's
///   solver.
/// - [`Column::spacing`](crate::linear::Column::spacing) delegates fixed spacing or overlap between
///   cells.
///
/// # Inspection and solving
///
/// - [`Column::layout`](crate::linear::Column::layout) returns the wrapped Ratatui [`Layout`].
/// - [`Column::regions`](crate::linear::Column::regions) solves the column and returns a
///   [`Regions`](crate::regions::Regions) with top-to-bottom ids.
///
/// # Examples
///
/// Plan a named vertical stack and pass the body region to a nested row:
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::linear::{Column, Row};
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum PageSlot {
///     Header,
///     Body,
///     Footer,
/// }
///
/// let page_slots = [
///     (PageSlot::Header, Constraint::Length(1)),
///     (PageSlot::Body, Constraint::Fill(1)),
///     (PageSlot::Footer, Constraint::Length(1)),
/// ];
/// let column = Column::named(page_slots);
/// let page = column.regions(Rect::new(0, 0, 30, 10));
/// let body_area = page.area_for(PageSlot::Body).unwrap();
/// let body_row = Row::new([Constraint::Length(10), Constraint::Fill(1)]).regions(body_area);
///
/// assert_eq!(body_row.regions()[0].area.width, 10);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Column<Id = usize> {
    layout: Layout,
    ids: Vec<Id>,
}

impl Column {
    /// Creates a column from vertical constraints.
    ///
    /// The constraints use the same semantics as [`ratatui_core::layout::Layout::vertical`].
    ///
    /// # Examples
    ///
    /// Split a panel into header, body, and footer regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let rows = [
    ///     Constraint::Length(1),
    ///     Constraint::Fill(1),
    ///     Constraint::Length(1),
    /// ];
    /// let plan = Column::new(rows).regions(Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(plan.regions()[1].area, Rect::new(0, 1, 20, 3));
    /// ```
    pub fn new<I>(constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        let constraints = constraints.into_iter().map(Into::into).collect::<Vec<_>>();
        let ids = (0..constraints.len()).collect();
        Self {
            layout: Layout::new(Direction::Vertical, constraints),
            ids,
        }
    }
}

impl<Id> Column<Id> {
    /// Creates a column from `(id, constraint)` pairs.
    ///
    /// Use this for page structure, dialogs, and other vertical layouts where each row has a stable
    /// role. The id travels with the constraint, so later code can ask for
    /// [`Regions::area_for`](crate::regions::Regions::area_for) by enum value instead of relying on
    /// numeric region positions.
    ///
    /// Keep using [`Column::new`](crate::linear::Column::new) for repeated generated rows where an
    /// index is already the natural identity.
    ///
    /// # Examples
    ///
    /// Name page rows at the same time their heights are declared:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum PageSlot {
    ///     Header,
    ///     Body,
    ///     Footer,
    /// }
    ///
    /// let page_slots = [
    ///     (PageSlot::Header, Constraint::Length(1)),
    ///     (PageSlot::Body, Constraint::Fill(1)),
    ///     (PageSlot::Footer, Constraint::Length(1)),
    /// ];
    /// let page = Column::named(page_slots).regions(Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(
    ///     page.area_for(PageSlot::Body).unwrap(),
    ///     Rect::new(0, 1, 20, 3)
    /// );
    /// ```
    pub fn named<I, C>(regions: I) -> Self
    where
        I: IntoIterator<Item = (Id, C)>,
        C: Into<Constraint>,
    {
        let (ids, constraints): (Vec<_>, Vec<_>) = regions
            .into_iter()
            .map(|(id, constraint)| (id, constraint.into()))
            .unzip();
        Self {
            layout: Layout::new(Direction::Vertical, constraints),
            ids,
        }
    }

    /// Returns the inner Ratatui layout.
    ///
    /// Use this when code needs Ratatui's full [`Layout`] API while still using
    /// [`Column`](crate::linear::Column) in the places that need
    /// [`Regions`](crate::regions::Regions) output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let column = Column::new([Constraint::Length(1), Constraint::Fill(1)]);
    /// let areas = column.layout().split(Rect::new(0, 0, 10, 4));
    ///
    /// assert_eq!(areas[0].height, 1);
    /// ```
    pub const fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Sets the column margin on all edges before solving constraints.
    ///
    /// # Examples
    ///
    /// Inset a vertical stack inside surrounding chrome:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let plan = Column::new([Constraint::Fill(1)])
    ///     .margin(1)
    ///     .regions(Rect::new(0, 0, 10, 5));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(1, 1, 8, 3));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn margin(mut self, margin: u16) -> Self {
        self.layout = self.layout.margin(margin);
        self
    }

    /// Sets the column horizontal margin before solving constraints.
    ///
    /// # Examples
    ///
    /// Keep rows aligned with content while using the full vertical area:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let plan = Column::new([Constraint::Fill(1)])
    ///     .horizontal_margin(2)
    ///     .regions(Rect::new(0, 0, 12, 4));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(2, 0, 8, 4));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn horizontal_margin(mut self, horizontal: u16) -> Self {
        self.layout = self.layout.horizontal_margin(horizontal);
        self
    }

    /// Sets the column vertical margin before solving constraints.
    ///
    /// # Examples
    ///
    /// Reserve top and bottom space while planning the middle stack:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let plan = Column::new([Constraint::Fill(1)])
    ///     .vertical_margin(1)
    ///     .regions(Rect::new(0, 0, 12, 5));
    ///
    /// assert_eq!(plan.regions()[0].area, Rect::new(0, 1, 12, 3));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn vertical_margin(mut self, vertical: u16) -> Self {
        self.layout = self.layout.vertical_margin(vertical);
        self
    }

    /// Sets how excess vertical space is distributed.
    ///
    /// # Examples
    ///
    /// Center fixed-height rows in a taller available area:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Flex, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let plan = Column::new([Constraint::Length(1), Constraint::Length(1)])
    ///     .flex(Flex::Center)
    ///     .regions(Rect::new(0, 0, 10, 4));
    ///
    /// assert_eq!(plan.regions()[0].area.y, 1);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.layout = self.layout.flex(flex);
        self
    }

    /// Sets spacing or overlap between column items.
    ///
    /// # Examples
    ///
    /// Separate stacked form fields with one blank row:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let plan = Column::new([Constraint::Length(1), Constraint::Length(1)])
    ///     .spacing(1)
    ///     .regions(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(plan.regions()[1].area.y, 2);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn spacing<T>(mut self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.layout = self.layout.spacing(spacing);
        self
    }

    /// Solves the column into regions.
    ///
    /// The returned regions use ids in top-to-bottom order.
    ///
    /// # Examples
    ///
    /// Store vertical menu regions so the next pointer event can hit-test a row:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::linear::Column;
    ///
    /// let previous_frame =
    ///     Column::new([Constraint::Length(1), Constraint::Length(1)]).regions(Rect::new(0, 0, 10, 2));
    ///
    /// assert_eq!(previous_frame.hit_test((3, 1)).unwrap().id, 1);
    /// ```
    pub fn regions(&self, area: Rect) -> Regions<Id>
    where
        Id: Clone,
    {
        let regions = self
            .layout
            .split(area)
            .iter()
            .copied()
            .zip(self.ids.iter().cloned())
            .map(|(area, id)| Region::new(id, area))
            .collect::<Vec<_>>();
        Regions::from_regions(area, regions)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::{Constraint, Rect};

    use super::*;

    #[test]
    fn row_creates_indexed_regions() {
        let plan =
            Row::new([Constraint::Length(2), Constraint::Fill(1)]).regions(Rect::new(0, 0, 5, 1));

        assert_eq!(plan.regions()[0].area, Rect::new(0, 0, 2, 1));
        assert_eq!(plan.regions()[1].area, Rect::new(2, 0, 3, 1));
    }

    #[test]
    fn row_creates_named_regions() {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        enum SlotId {
            Left,
            Right,
        }

        let regions = [
            (SlotId::Left, Constraint::Length(2)),
            (SlotId::Right, Constraint::Fill(1)),
        ];
        let plan = Row::named(regions).regions(Rect::new(0, 0, 5, 1));

        assert_eq!(plan.area_for(SlotId::Left), Some(Rect::new(0, 0, 2, 1)));
        assert_eq!(plan.area_for(SlotId::Right), Some(Rect::new(2, 0, 3, 1)));
    }

    #[test]
    fn row_builder_matches_equivalent_layout() {
        let area = Rect::new(0, 0, 20, 3);
        let constraints = [Constraint::Length(4), Constraint::Length(4)];
        let plan = Row::new(constraints)
            .margin(1)
            .spacing(2)
            .flex(Flex::Start)
            .regions(area);
        let expected = Layout::horizontal(constraints)
            .margin(1)
            .spacing(2)
            .flex(Flex::Start)
            .split(area);

        assert_eq!(plan.regions()[0].area, expected[0]);
        assert_eq!(plan.regions()[1].area, expected[1]);
    }

    #[test]
    fn column_builder_matches_equivalent_layout() {
        let area = Rect::new(0, 0, 10, 12);
        let constraints = [Constraint::Length(2), Constraint::Length(3)];
        let plan = Column::new(constraints)
            .horizontal_margin(1)
            .vertical_margin(2)
            .spacing(1)
            .flex(Flex::Center)
            .regions(area);
        let expected = Layout::vertical(constraints)
            .horizontal_margin(1)
            .vertical_margin(2)
            .spacing(1)
            .flex(Flex::Center)
            .split(area);

        assert_eq!(plan.regions()[0].area, expected[0]);
        assert_eq!(plan.regions()[1].area, expected[1]);
    }

    #[test]
    fn column_creates_named_regions() {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        enum SlotId {
            Header,
            Body,
            Footer,
        }

        let regions = [
            (SlotId::Header, Constraint::Length(1)),
            (SlotId::Body, Constraint::Fill(1)),
            (SlotId::Footer, Constraint::Length(1)),
        ];
        let plan = Column::named(regions).regions(Rect::new(0, 0, 10, 5));

        assert_eq!(plan.area_for(SlotId::Header), Some(Rect::new(0, 0, 10, 1)));
        assert_eq!(plan.area_for(SlotId::Body), Some(Rect::new(0, 1, 10, 3)));
        assert_eq!(plan.area_for(SlotId::Footer), Some(Rect::new(0, 4, 10, 1)));
    }
}
