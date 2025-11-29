#![warn(missing_docs)]

//! The [`XLayout`] type is an extended version of [`Layout`] that provides additional options for
//! layout flexibility.

mod constraint;
mod hint;
mod record;

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::array::TryFromSliceError;
use core::fmt::{Debug, Display, Formatter};
use core::ops::{Add, AddAssign, Index};

pub use constraint::{
    Align, ConstraintList, IntoConstraint, LinearMargin, LinearSizeRange, SizeRange, XConstraint,
    XMargin,
};
pub use hint::{Hint, HintRange};
use record::OptionFillRecord;
pub use record::{FillRecord, FillRecordSegment, FillRecordStep};
use strum::{Display, EnumIs, EnumString};

use crate::layout::layout::{Segments, Spacers};
use crate::layout::{Constraint, Direction, Flex, Layout, Rect, Size, Spacing};

/// During the layout algorithm the step size is multiplied by this factor in order
/// to allow the constraints to take less than a whole step.
/// A whole step would be [`HintRange::fill_scale`], and this factor allows
/// the constraint to take steps as small as `fill_scale / STEP_FACTOR`.
const STEP_FACTOR: u64 = 32;

/// In an [`XConstraint`] each range includes three values: min, preferred, and max.
/// This value dynamically specifies one particular value of those three.
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Hash, EnumIs, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RangeLevel {
    /// The smallest value represented by the range.
    Min,
    /// The ideal value represented by the range.
    Preferred,
    /// The largest value represented by the range.
    Max,
}

/// `XLayout` is an *extended* layout engine for dividing terminal space using constraints and
/// direction. It does what [`Layout`] does, plus offering more options to provide greater control
/// over layouts. `XLayout` tries to resemble `Layout` in every way possible, with the same methods
/// used in the same ways, so that one can easily switch between using `XLayout` and `Layout` within
/// a project, depending on your layout needs. `XLayout` uses [`XConstraint`], but `XConstraint`
/// implements `From<Constraint>` so that `XLayout` can be given the exact same constraints that
/// `Layout` accepts and produce similar results. `XLayout` also implements `From<Layout>` so one
/// can build a `Layout` and then convert it to an `XLayout` to add something requiring the extended
/// options before performing the split.
///
/// A layout is a set of constraints that can be applied to a given area to split it into smaller
/// rectangular areas. This is the core building block for creating structured user interfaces in
/// terminal applications.
///
/// A layout is composed of:
/// - a direction (horizontal or vertical)
/// - a set of constraints. See [`XConstraint`] for a discussion on the fields of an extended
///   constraint.
/// - a margin, the space between the edge of the main area and the split which can be set
///   independently along the top, bottom, left, and right sides of the area.
/// - spacing between segments is controlled by adding additional constraints which are flagged as
///   [`XConstraint::is_separator`], causing them to not produce segments but still take up space in
///   the area.
///
/// The algorithm used to compute the layout is based giving each constraint turns to allocate space
/// from the area for that constraint's segment. See [`SegmentTarget`] for details of how segments
/// grow in each phase. Layout ends once the entire area has been allocated.
///
/// If the reason why some constraint was allocated some portion of the area is unclear,
/// [`XLayout::split_for_debug`] can be used to create a diagnostic record of the allocation process
/// and how much space was allocated to each constraint in each phase and why. See [`FillRecord`]
/// for a step-by-step guide to how space is allocated for each constraint.
///
/// # Construction
///
/// - [`default`](Default::default) - Create a layout with default values (vertical direction, no
///   constraints, no margin)
/// - [`new`](Self::new) - Create a new layout with a given direction and constraints
/// - [`vertical`](Self::vertical) - Create a new vertical layout with the given constraints
/// - [`horizontal`](Self::horizontal) - Create a new horizontal layout with the given constraints
///
/// # Configuration
///
/// - [`direction`](Self::direction) - Set the direction of the layout
/// - [`constraints`](Self::constraints) - Set the constraints of the layout
/// - [`margin`](Self::margin) - Set uniform margin on all sides
/// - [`horizontal_margin`](Self::horizontal_margin) - Set the horizontal margin of the layout
/// - [`vertical_margin`](Self::vertical_margin) - Set the vertical margin of the layout
///
/// # Layout Operations
///
/// - [`areas`](Self::areas) - Split area into fixed number of rectangles (compile-time known)
/// - [`spacers`](Self::spacers) - Get spacer rectangles between layout areas
/// - [`split`](Self::split) - Split area into rectangles (runtime determined count)
/// - [`split_with_spacers`](Self::split_with_spacers) - Split area and return both areas and
///   spacers
///
/// # Example
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Constraint, Direction, Rect, XLayout};
/// use ratatui_core::text::Text;
/// use ratatui_core::widgets::Widget;
///
/// fn render(area: Rect, buf: &mut Buffer) {
///     let layout = XLayout::vertical([Constraint::Length(5), Constraint::Fill(1)]);
///     let [top, bottom] = layout.areas(area);
///     Text::from("foo").render(top, buf);
///     Text::from("bar").render(bottom, buf);
/// }
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XLayout {
    /// The axis of layout, determining whether the area is split into a
    /// horizontal row of segments or a vertical column of segments.
    direction: Direction,
    /// The constraints that control the number of segments and the size
    /// and position of each segment.
    constraints: Vec<XConstraint>,
    /// The amount to reduce the rectangle before layout begins.
    margin: XMargin,
}

#[inline]
const fn constraint_priority(constraint: Constraint) -> i16 {
    #[allow(clippy::match_same_arms)]
    match constraint {
        Constraint::Min(_) => 1,
        Constraint::Max(_) => 7,
        Constraint::Length(_) => 4,
        Constraint::Percentage(_) => 3,
        Constraint::Ratio(_, _) => 2,
        Constraint::Fill(_) => 1,
    }
}

/// A heuristic to try to replicate the behavior of [`Flex::Legacy`] by finding the
/// last constraint of lowest priority.
fn position_of_last_lowest<I: IntoIterator<Item = Constraint>>(list: I) -> usize {
    let mut max_p = i16::MAX;
    let mut max_i = 0;
    for (i, c) in list.into_iter().enumerate() {
        let p = constraint_priority(c);
        if p < max_p {
            max_p = p;
            max_i = i;
        }
        if p == max_p {
            max_i = i;
        }
    }
    max_i
}

/// A heuristic to try to replicate the behavior of [`Flex::Legacy`] by modifying
/// the last lowest constraint.
const fn legacy_effect(hint: HintRange, is_last_lowest: bool) -> HintRange {
    if is_last_lowest {
        HintRange {
            overfill: true,
            ..hint
        }
    } else {
        hint
    }
}

/// True if the given constraints contain at least one [`Constraint::Fill`]
/// with a value greater than zero.
fn has_nonzero_fill(constraints: &[Constraint]) -> bool {
    constraints.iter().any(|c| {
        if let Constraint::Fill(n) = c {
            *n > 0
        } else {
            false
        }
    })
}

/// Surprisingly, `Constraint::Fill(0)` does not always cause [`Layout`] to
/// produce a zero-sized segment. If *all* of the fills are zero, then they are
/// treated as if all of the fills were 1. To simplify the conversion,
/// turn every `Constraint::Fill(0)` into a `Constraint::Fill(1)` if all of the fills
/// are zero.
fn correct_zero_fills(constraints: &mut [Constraint]) {
    if has_nonzero_fill(constraints) {
        return;
    }
    for c in constraints {
        if let Constraint::Fill(n) = c {
            if *n == 0 {
                *n = 1;
            }
        }
    }
}

impl<I> From<I> for XLayout
where
    I: IntoIterator,
    I::Item: Into<XConstraint>,
{
    fn from(value: I) -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: value.into_iter().map(Into::into).collect(),
            margin: XMargin::ZERO,
        }
    }
}

/// Depending on the value of [`Layout::flex`] and [`Layout::spacing`], construct separator
/// constraints to be inserted: (at the start, between each pair, at the end). For example,
/// [`Flex::Center`] would have two equal-sized separators at the start and the end.
/// [`Flex::SpaceBetween`] would have repeat the same separator between each pair of constraints.
fn separators_for_flex(
    value: &Layout,
) -> (
    Option<XConstraint>,
    Option<XConstraint>,
    Option<XConstraint>,
) {
    // Depending on `flex` certain separators may be only necessary if we do not have a constraint
    // that fills space.
    let not_filling = !value.constraints.iter().any(|c| c.is_fill() || c.is_min());
    let start_space;
    let end_space;
    match value.flex {
        Flex::Start => {
            start_space = None;
            end_space = not_filling.then_some(HintRange::filler(1).separator());
        }
        Flex::End => {
            start_space = not_filling.then_some(HintRange::filler(1).separator());
            end_space = None;
        }
        Flex::Center => {
            start_space = not_filling.then_some(HintRange::filler(1).separator());
            end_space = start_space.clone();
        }
        Flex::SpaceAround | Flex::SpaceEvenly => {
            start_space = match value.spacing {
                Spacing::Space(0) | Spacing::Overlap(_) => {
                    not_filling.then_some(HintRange::filler(1).separator())
                }
                Spacing::Space(n) => Some(HintRange::filler(1).with_min(n).separator()),
            };
            end_space = start_space.clone();
        }
        Flex::SpaceBetween | Flex::Legacy => {
            start_space = None;
            end_space = None;
        }
    }
    let inner_space = match value.flex {
        Flex::Legacy | Flex::Start | Flex::End | Flex::Center => match value.spacing {
            Spacing::Space(0) | Spacing::Overlap(0) => None,
            Spacing::Space(n) => Some(n.separator()),
            Spacing::Overlap(n) => {
                Some(HintRange::from_hint(Hint::Overlap(n)).scale(0).separator())
            }
        },
        Flex::SpaceBetween => match value.spacing {
            Spacing::Space(0) | Spacing::Overlap(0) => {
                not_filling.then_some(HintRange::filler(1).separator())
            }
            Spacing::Space(n) => Some(HintRange::filler(1).with_min(n).separator()),
            Spacing::Overlap(n) => Some(XConstraint::overlap(n)),
        },
        Flex::SpaceEvenly => match value.spacing {
            Spacing::Space(0) | Spacing::Overlap(_) => {
                not_filling.then_some(HintRange::filler(1).separator())
            }
            Spacing::Space(n) => Some(HintRange::filler(1).with_min(n).separator()),
        },
        Flex::SpaceAround => match value.spacing {
            Spacing::Space(0) | Spacing::Overlap(_) => {
                not_filling.then_some(HintRange::filler(2).separator())
            }
            Spacing::Space(n) => Some(
                HintRange::filler(2)
                    .with_min(n.saturating_mul(2))
                    .separator(),
            ),
        },
    };
    (start_space, inner_space, end_space)
}

impl From<Layout> for XLayout {
    /// Generate an `XLayout` from the given `Layout` with the goal of perfectly replicating any
    /// split that would be produced by the given `Layout`.
    fn from(mut value: Layout) -> Self {
        correct_zero_fills(&mut value.constraints);
        let has_fills = value.constraints.iter().any(Constraint::is_fill) || value.flex.is_legacy();
        // Attempt to replicated the special behaviour `Flex::Legacy` has for the last lowest
        // priority constraint.
        let legacy_target = if value.flex == Flex::Legacy {
            Some(position_of_last_lowest(value.constraints.iter().copied()))
        } else {
            None
        };
        // Construct separators to go around and between the constraints, based on the value of
        // [`Layout::flex`] and [`Layout::spacing`].
        let (start_space, inner_space, end_space) = separators_for_flex(&value);
        let mut priority_offset: i16 = 0;
        let mut iter = value.constraints.into_iter().enumerate().map(|(i, c)| {
            let mut xc = XConstraint::from(c);
            // For `Flex::Legacy`, increase the priority to the right, causing each constraint to be
            // handled in order from left-to-right.
            if value.flex.is_legacy() {
                xc.hint.priority = xc.hint.priority.saturating_add(priority_offset);
                priority_offset = priority_offset.saturating_add(1);
            }
            // If there are fill constraints, then put a max on `Min` constraints to prevent them
            // from growing.
            if let Constraint::Min(_) = c {
                if has_fills {
                    xc.hint.max = xc.hint.min;
                    xc.hint.preferred = xc.hint.min;
                } else {
                    xc.hint.overfill = true;
                }
            }
            (i, xc)
        });
        let mut constraints = Vec::new();
        if let Some(space) = start_space {
            constraints.push(space);
        }
        if let Some((_, mut xc)) = iter.next() {
            xc.hint = legacy_effect(xc.hint, legacy_target == Some(0));
            constraints.push(xc);
        }
        for (i, mut xc) in iter {
            if let Some(space) = inner_space.clone() {
                constraints.push(space);
            }
            xc.hint = legacy_effect(xc.hint, legacy_target == Some(i));
            constraints.push(xc);
        }
        if let Some(space) = end_space {
            constraints.push(space);
        }
        Self {
            direction: value.direction,
            constraints,
            margin: value.margin.into(),
        }
    }
}

impl<C: Into<XConstraint>> Add<C> for XLayout {
    type Output = Self;

    fn add(mut self, rhs: C) -> Self::Output {
        self.constraints.push(rhs.into());
        self
    }
}

impl<C: Into<XConstraint>> AddAssign<C> for XLayout {
    fn add_assign(&mut self, rhs: C) {
        self.constraints.push(rhs.into());
    }
}

impl Add<ConstraintList> for XLayout {
    type Output = Self;
    fn add(mut self, rhs: ConstraintList) -> Self::Output {
        self.constraints.append(&mut rhs.into());
        self
    }
}

impl AddAssign<ConstraintList> for XLayout {
    fn add_assign(&mut self, rhs: ConstraintList) {
        self.constraints.append(&mut rhs.into());
    }
}

impl Add<XLayout> for XConstraint {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self);
        rhs
    }
}

impl Add<XLayout> for HintRange {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self.into());
        rhs
    }
}

impl Add<XLayout> for Size {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self.into());
        rhs
    }
}

impl Add<XLayout> for SizeRange {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self.into());
        rhs
    }
}

impl Add<XLayout> for Hint {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self.into());
        rhs
    }
}

impl Add<XLayout> for u16 {
    type Output = XLayout;

    fn add(self, mut rhs: XLayout) -> Self::Output {
        rhs.constraints.insert(0, self.into());
        rhs
    }
}

impl XLayout {
    /// Creates a new layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<XConstraint>>`. This includes arrays, slices, vectors, iterators. `Into<XConstraint>`
    /// is implemented on `u16` and other types, so you can pass an array, `Vec`, etc. of `u16`
    /// to this function to create a layout with fixed size chunks.
    ///
    /// See [`XLayout::constraints`] for various ways to construct an `XConstraint`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Direction, Hint, XConstraint, XLayout};
    ///
    /// XLayout::new(
    ///     Direction::Horizontal,
    ///     [Hint::Length(5), Hint::Percentage(25)],
    /// ) + XConstraint::SPACER;
    ///
    /// XLayout::new(
    ///     Direction::Vertical,
    ///     [1, 2, 3].iter().map(|&c| Hint::Length(c)),
    /// ) + XConstraint::SPACER;
    ///
    /// XLayout::new(Direction::Horizontal, vec![1, 2]);
    /// ```
    pub fn new<I>(direction: Direction, constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        Self {
            direction,
            constraints: constraints.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
    /// Creates a new vertical layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<XConstraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Hint, XLayout};
    ///
    /// let layout = XLayout::vertical([Hint::Length(5), Hint::Percentage(50)]);
    /// ```
    pub fn vertical<I>(constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        Self::new(Direction::Vertical, constraints.into_iter().map(Into::into))
    }
    /// Creates a new horizontal layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Hint, XLayout};
    ///
    /// let layout = XLayout::horizontal([Hint::Length(5), Hint::Percentage(50)]);
    /// ```
    pub fn horizontal<I>(constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        Self::new(
            Direction::Horizontal,
            constraints.into_iter().map(Into::into),
        )
    }

    /// Set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Direction, Rect, XLayout};
    ///
    /// let layout = XLayout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Length(5), Constraint::Fill(1)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);
    ///
    /// let layout = XLayout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Fill(1)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the constraints of the layout.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<XConstraint>>`. This includes arrays, slices, vectors, iterators. `Into<XConstraint>`
    /// is implemented on several types to provide shortcuts for specifying useful constraints.
    /// * `u16`: Becomes a constraint with the values of [`XConstraint::FULL`] except that min and
    ///   preferred of [`HintRange`] are set to the value of the `u16` using [`Hint::Length`].
    /// * `Range<u16>`, `RangeInclusive<u16>`, `RangeTo<u16>`, etc.: Ranges such as `1..=5` become a
    ///   [`XConstraint::FULL`] except [`HintRange`] has min set to the low end of the range, while
    ///   preferred and max are set to the high end of the range using [`Hint::Length`].
    /// * `(u16, u16)`: Becomes a `Size` with the given width and height, which is then converted to
    ///   an [`XConstraint`].
    /// * `(Range<u16>, Range<u16>)`: Becomes a `SizeRange` with its horizontal range matching the
    ///   first value and its vertical range matching the second value, which is then converted to
    ///   `XConstraint`. The preferred value on each axis is set to the high end of the given
    ///   ranges.
    /// * [`Size`]: Becomes a `SizeRange` with min, preferred, and max all set to the given `Size`,
    ///   which is then converted to an `XConstraint`.
    /// * [`SizeRange`]: Becomes an `XConstraint` with the values of [`XConstraint::FULL`] except
    ///   with the given value as its `size` and with the hint range set to [`HintRange::SIZE`],
    ///   which suggests that the layout should confirm closely to the size range of this
    ///   constraint.
    /// * [`HintRange`]: Becomes an `XConstraint` with the values of [`XConstraint::FULL`] except
    ///   that the hint is the given value.
    /// * [`Constraint::Length`]: Becomes an [`HintRange::FULL`] with `preferred` and `max` set to
    ///   [`Hint::Length`] with the same value.
    /// * [`Constraint::Percentage`]: Becomes a [`HintRange::FULL`] with `preferred` and `max` set
    ///   to [`Hint::Percentage`] with the same value.
    /// * [`Constraint::Ratio`]: Becomes a [`HintRange::FULL`] with `preferred` and `max` set to
    ///   [`Hint::Ratio`] with the same value.
    /// * [`Constraint::Min`]: Becomes a [`HintRange::FULL`] with `min` set to [`Hint::Length`] with
    ///   the same value and with `priority` set to 100.
    /// * [`Constraint::Max`]: Becomes a [`HintRange::FULL`] with `preferred` and `max` set to
    ///   [`Hint::Length`] with the same value and with `priority` set to 100. It is exactly like
    ///   `Constraint::Length` except its higher priority value causes it to wait before allocating
    ///   its preferred space. See [`HintRange::priority`] for details.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
    /// 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let layout = XLayout::default()
    ///     .constraints([
    ///         Constraint::Percentage(20),
    ///         Constraint::Ratio(1, 5),
    ///         Constraint::Length(2),
    ///         Constraint::Min(2),
    ///         Constraint::Max(2),
    ///     ])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(
    ///     layout[..],
    ///     [
    ///         Rect::new(0, 0, 10, 2),
    ///         Rect::new(0, 2, 10, 2),
    ///         Rect::new(0, 4, 10, 2),
    ///         Rect::new(0, 6, 10, 2),
    ///         Rect::new(0, 8, 10, 2),
    ///     ]
    /// );
    ///
    /// XLayout::default().constraints([Constraint::Fill(1)]);
    /// XLayout::default().constraints(&[Constraint::Fill(1)]);
    /// XLayout::default().constraints(vec![Constraint::Fill(1)]);
    /// XLayout::default().constraints([Constraint::Fill(1)].iter().filter(|_| true));
    /// XLayout::default().constraints([1, 2, 3].iter().map(|&c| Constraint::Length(c)));
    /// XLayout::default().constraints([1, 2, 3]);
    /// XLayout::default().constraints(vec![1, 2, 3]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn constraints<I>(mut self, constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        self.constraints = constraints.into_iter().map(Into::into).collect();
        self
    }

    /// Set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let layout = XLayout::default()
    ///     .constraints([Constraint::Fill(1)])
    ///     .margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn margin<M: Into<XMargin>>(mut self, margin: M) -> Self {
        self.margin = margin.into();
        self
    }

    /// Set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let layout = XLayout::default()
    ///     .constraints([Constraint::Fill(1)])
    ///     .horizontal_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn horizontal_margin<M: Into<LinearMargin>>(mut self, horizontal: M) -> Self {
        self.margin.set_linear(Direction::Horizontal, horizontal);
        self
    }

    /// Set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let layout = XLayout::default()
    ///     .constraints([Constraint::Fill(1)])
    ///     .vertical_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn vertical_margin<M: Into<LinearMargin>>(mut self, vertical: M) -> Self {
        self.margin.set_linear(Direction::Vertical, vertical);
        self
    }

    /// Split the rect into a number of sub-rects according to the constraints.
    ///
    /// An ergonomic wrapper around [`XLayout::split`] that returns an array of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// This method requires the number of constraints to be known at compile time. If you don't
    /// know the number of constraints at compile time, use [`XLayout::split`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the number of constraints is not equal to the length of the returned array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = XLayout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    /// let [top, main] = layout.areas(area);
    ///
    /// // or explicitly specify the number of constraints:
    /// let areas = layout.areas::<2>(area);
    /// ```
    pub fn areas<const N: usize>(&self, area: Rect) -> [Rect; N] {
        let areas = self.split(area);
        areas.as_ref().try_into().unwrap_or_else(|_| {
            panic!(
                "invalid number of rects: expected {N}, found {}",
                areas.len()
            )
        })
    }

    /// Split the rect into a number of sub-rects according to the given [`Layout`].
    ///
    /// An ergonomic wrapper around [`XLayout::split`] that returns an array of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// This method requires the number of constraints to be known at compile time. If you don't
    /// know the number of constraints at compile time, use [`XLayout::split`] instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of constraints is not equal to the length of the returned
    /// array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = XLayout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    /// let [top, main] = layout.try_areas(area)?;
    ///
    /// // or explicitly specify the number of constraints:
    /// let areas = layout.try_areas::<2>(area)?;
    /// # Ok::<(), core::array::TryFromSliceError>(())
    /// ```
    pub fn try_areas<const N: usize>(&self, area: Rect) -> Result<[Rect; N], TryFromSliceError> {
        self.split(area).as_ref().try_into()
    }

    /// Split the rect into a number of sub-rects according to the constraints and return just
    /// the spacers between the areas.
    ///
    /// The number of areas is determined by the number of non-separator constraints. Constraints
    /// with the [`XConstraint::is_separator`] flag set do not add areas to the layout, nor do
    /// they add spacers to the list returned by this method.
    ///
    /// The number of spacers will always be one greater than the number of areas, with one spacer
    /// before the first area, between each area, and after the last area. If there is no space
    /// between two areas, or the areas overlap, then the spacer `Rect` will still be in the
    /// list, but it will have zero size.
    ///
    /// This method requires the number of spacers to be known at compile time. If you don't
    /// know the number of spacers at compile time, use [`XLayout::split_with_spacers`] instead.
    ///
    /// This method is similar to [`XLayout::areas`], and can be called with the same parameters,
    /// but it returns just the spacers between the areas.
    ///
    /// # Panics
    ///
    /// Panics if the number of constraints + 1 is not equal to the length of the returned array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect, XLayout};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = XLayout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    /// let [top, main] = layout.areas(area);
    /// let [before, inbetween, after] = layout.spacers(area);
    ///
    /// // or explicitly specify the number of constraints:
    /// let spacers = layout.spacers::<3>(area);
    /// ```
    pub fn spacers<const N: usize>(&self, area: Rect) -> [Rect; N] {
        let (_, spacers) = self.split_with_spacers(area);
        spacers
            .as_ref()
            .try_into()
            .expect("invalid number of rects")
    }
    /// Split the given area into smaller ones based on the constraints and the direction.
    /// This method is similar to `split`, but it returns two sets of rectangles: one for the areas
    /// and one for the spaces before, after, and between the areas.
    ///
    /// The number of areas is determined by the number of non-separator constraints. Constraints
    /// with the [`XConstraint::is_separator`] flag set do not add areas to the list returned by
    /// this method.
    ///
    /// The number of spacers will always be one greater than the number of areas, with one spacer
    /// before the first area, between each area, and after the last area. If there is no space
    /// between two areas, or the areas overlap, then the spacer `Rect` will still be in the
    /// list, but it will have zero size.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Direction, IntoConstraint, Rect, XLayout};
    ///
    /// let (areas, spacers) = XLayout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Fill(1)])
    ///     .split_with_spacers(Rect::new(2, 2, 10, 10));
    /// assert_eq!(areas[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);
    /// assert_eq!(
    ///     spacers[..],
    ///     [
    ///         Rect::new(2, 2, 10, 0),
    ///         Rect::new(2, 7, 10, 0),
    ///         Rect::new(2, 12, 10, 0)
    ///     ]
    /// );
    ///
    /// let (areas, spacers) = XLayout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([
    ///         Constraint::Ratio(1, 3).into(),
    ///         1.separator(),
    ///         Constraint::Ratio(2, 3).into(),
    ///     ])
    ///     .split_with_spacers(Rect::new(0, 0, 10, 2));
    /// assert_eq!(areas[..], [Rect::new(0, 0, 3, 2), Rect::new(4, 0, 6, 2)]);
    /// assert_eq!(
    ///     spacers[..],
    ///     [
    ///         Rect::new(0, 0, 0, 2),
    ///         Rect::new(3, 0, 1, 2),
    ///         Rect::new(10, 0, 0, 2)
    ///     ]
    /// );
    /// ```
    pub fn split_with_spacers(&self, area: Rect) -> (Segments, Spacers) {
        let area = area - self.margin;
        let rules = self.build_segment_set(area, None);
        let segments = rules.build_segments(self.direction, area, &self.constraints);
        let spacers = build_spacers(self.direction, area, &segments);
        (segments, spacers)
    }
    /// Split a given area into smaller ones based on the constraints and the direction.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
    /// 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]) as
    /// the two 50% each fight to claim half of the 100 when only 80 is actually available.
    ///
    /// Here is the actual result:
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Direction, Rect, XLayout};
    /// let layout = XLayout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([
    ///         Constraint::Min(20),
    ///         Constraint::Percentage(50),
    ///         Constraint::Percentage(50),
    ///     ])
    ///     .split(Rect::new(2, 2, 100, 10));
    /// assert_eq!(
    ///     layout[..],
    ///     [
    ///         Rect::new(2, 2, 20, 10),
    ///         Rect::new(22, 2, 40, 10),
    ///         Rect::new(62, 2, 40, 10)
    ///     ]
    /// );
    /// ```
    ///
    /// In this case it actually did result in [20, 40, 40] because the two `Percentage(50)`
    /// constraints have equal priority and so they shared what was available equally. They were
    /// each trying to take 50 out of 100, and space was allocated to them at the same rate
    /// until all the space was used up, leaving each with 40.
    ///
    /// There is a helper method that can be used to split the whole area into smaller ones based on
    /// the layout: [`Layout::areas()`]. That method is a shortcut for calling this method. It
    /// allows you to destructure the result directly into variables, which is useful when you know
    /// at compile time the number of areas that will be created.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Direction, Layout, Rect};
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Fill(1)])
    ///     .split(Rect::new(2, 2, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    ///     .split(Rect::new(0, 0, 9, 2));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 3, 2), Rect::new(3, 0, 6, 2)]);
    /// ```
    pub fn split(&self, area: Rect) -> Segments {
        let area = area - self.margin;
        let rules = self.build_segment_set(area, None);
        rules.build_segments(self.direction, area, &self.constraints)
    }

    /// This method performs the same split as [`XLayout::split`] and similar methods,
    /// but it throws away the result of that split and instead returns as record of the steps
    /// that it took in order to allocate space to each constraint. This can help debug a layout
    /// if the segments are not where they should be. See [`FillRecord`] for more information.
    pub fn split_for_debug(&self, area: Rect) -> FillRecord {
        let area = area - self.margin;
        let record = FillRecord::default();
        let rules = self.build_segment_set(area, Some(record));
        rules.record.0.unwrap()
    }

    /// Calculate the size range of this layout based on the `size` of and `margin` of each
    /// constraint. The [`XConstraint::size`] represents the space needed by the content of that
    /// segment, and this method calculates the size needed by the entire layout if it were to be
    /// nested within another layout. The result of this method can be converted into an
    /// `XConstraint` and used to build the layout that will contain this layout.
    ///
    /// If there are any constraints that include hints that request an overlap, the size of this
    /// layout is reduced according to the overlap. Otherwise, all hints are ignored when
    /// calculating the size.
    ///
    /// # Example
    /// ```
    /// use ratatui_core::layout::{Rect, Size, SizeRange, XLayout};
    /// use ratatui_core::text::Text;
    /// let area = Rect::new(0, 0, 25, 10);
    /// let a = Text::from("Alpha\nAlpha"); // 5x2
    /// let b = Text::from("Bravo Charlie"); // 13x1
    /// let c = Text::from("Chilly\nChaps\nChuckles"); // 8x3
    /// let d = Text::from("Delta Fox"); // 9x1
    /// let inner = XLayout::vertical([a.size(), b.size(), d.size()]);
    /// assert_eq!(inner.size(), SizeRange::from((13, 4)));
    /// let outer = XLayout::horizontal([c.size().into(), inner.size()]);
    /// let [segC, segInner] = outer.areas(area);
    /// let [segA, segB, segD] = inner.areas(segInner);
    /// assert_eq!(segC, Rect::new(1, 3, 8, 3));
    /// assert_eq!(segInner, Rect::new(11, 3, 13, 4));
    /// assert_eq!(segA, Rect::new(15, 3, 5, 2));
    /// assert_eq!(segB, Rect::new(11, 5, 13, 1));
    /// assert_eq!(segD, Rect::new(13, 6, 9, 1));
    /// ```
    pub fn size(&self) -> SizeRange {
        SizeRange {
            horizontal: self.linear_size(Direction::Horizontal),
            vertical: self.linear_size(Direction::Vertical),
        }
    }
    /// The same as [`XLayout::size`] but it calculates the size along only one axis.
    pub fn linear_size(&self, direction: Direction) -> LinearSizeRange {
        if self.direction == direction {
            LinearSizeRange::new(
                self.sum(RangeLevel::Min),
                self.sum(RangeLevel::Preferred),
                self.sum(RangeLevel::Max),
            )
        } else {
            LinearSizeRange::new(
                self.max(RangeLevel::Min),
                self.max(RangeLevel::Preferred),
                self.min(RangeLevel::Max),
            )
        }
    }
    /// Iterate through the priorities of the constraints of a layout to return each distinct
    /// priority level from lowest to highest. This returns the value of the priority, not the
    /// constraint, and each priority value is returned only once, even if multiple constraints
    /// have the same value.
    const fn priorities(&self) -> PriorityIter<'_> {
        PriorityIter {
            source: self,
            front: None,
            back: None,
        }
    }
    /// The minimum value of the [`SizeRange`] of each constraint at the given level
    /// and measured perpendicular to the layout direction. Return `u16::MAX` if the constraint
    /// list is empty.
    fn min(&self, level: RangeLevel) -> u16 {
        self.fold(level, u16::MAX, u16::min)
    }
    /// The maximum value of the [`SizeRange`] of each constraint at the given level
    /// and measured perpendicular to the layout direction. Return zero if the constraint list
    /// is empty.
    fn max(&self, level: RangeLevel) -> u16 {
        self.fold(level, 0, u16::max)
    }
    /// Applies the given function to fold the values of [`SizeRange`] of each constraint
    /// at the given level and measured perpendicular to the layout direction.
    /// Returns `initial` if the constraint list is empty. Otherwise, `func` is applied
    /// to the running total and the value of the current constraint to produce the
    /// new running total.
    fn fold<F>(&self, level: RangeLevel, initial: u16, func: F) -> u16
    where
        F: Fn(u16, u16) -> u16,
    {
        let mut total = initial;
        let dir = self.direction.perpendicular();
        for c in &self.constraints {
            let size = c
                .margin
                .linear(dir)
                .total()
                .saturating_add(c.size.linear(dir)[level]);
            total = func(total, size);
        }
        total.saturating_add(self.margin.linear(dir).total())
    }
    /// Sums the values of [`SizeRange`] of each constraint at the given level
    /// measured parallel to the layout direction.
    fn sum(&self, level: RangeLevel) -> u16 {
        let mut sum = self.margin.linear(self.direction).total();
        let mut overlap: u16 = 0;
        for c in &self.constraints {
            sum = sum.saturating_add(c.size.linear(self.direction)[level]);
            sum = sum.saturating_add(c.margin.linear(self.direction).total());
            if let Hint::Overlap(v) = c.hint[level] {
                overlap = overlap.saturating_add(v);
            }
        }
        sum.saturating_sub(overlap)
    }
    /// The core of the layout algorithm that takes an area to be split and produces a
    /// [`SegmentSet`] that specifies the size of each segment. It does everything except
    /// translate the segment sizes into rectangles. The creation of rectangles is the
    /// responsibility of [`SegmentSet::build_segments`].
    fn build_segment_set(&self, area: Rect, record: Option<FillRecord>) -> SegmentSet {
        use FillState::Finished;
        use RangeLevel::{Max, Min, Preferred};
        use SegmentTarget::{Forced, Overfill, Range};
        let area_length = match self.direction {
            Direction::Horizontal => area.width,
            Direction::Vertical => area.height,
        };
        let rules = self
            .constraints
            .iter()
            .map(|c| SegmentRule::new(c, self.direction, area_length))
            .collect::<Vec<_>>();
        let area_length = i32::from(area_length);
        let min_total = rules.iter().map(|r| r.min).sum::<i32>();
        let mut set = SegmentSet::new(rules, area_length, record);
        // Try skipping the min phase, since a min phase can give some segments a head-start
        // in the preferred phase that changes the outcome.
        // Only do the min phase now if we know it will finish the layout.
        let skip_min = min_total < area_length;
        if !skip_min && set.fill_by_priorities(Range(Min), self.priorities()) == Finished {
            return set;
        }
        let result = set.fill_by_priorities(Range(Preferred), self.priorities());
        if skip_min && !set.are_all_at_min() {
            // Skipping the min phase was a mistake, since some did not reach min in the
            // preferred phase, so reset back to initial and do the min and preferred phases.
            set.reset();
            if set.fill_by_priorities(Range(Min), self.priorities()) == Finished {
                return set;
            }
            if set.fill_by_priorities(Range(Preferred), self.priorities()) == Finished {
                return set;
            }
        }
        if result == Finished {
            return set;
        }
        if set.fill_by_priorities(Range(Max), self.priorities().rev()) == Finished {
            return set;
        }
        if set.fill_by_priorities(Overfill, self.priorities().rev()) == Finished {
            return set;
        }
        _ = set.fill_by_priorities(Forced, self.priorities().rev());
        set
    }
}

/// Iterate through the priorities of the constraints of a layout to return each distinct priority
/// level from lowest to highest. This returns the value of the priority, not the constraint,
/// and each priority value is returned only once, even if multiple constraints have the same value.
struct PriorityIter<'a> {
    /// The constraints containing the priorities to iterate over.
    source: &'a XLayout,
    /// The lowest constraint priority that we have returned so far.
    front: Option<i16>,
    /// The highest constraint priority that we have returned so far.
    back: Option<i16>,
}

impl PriorityIter<'_> {
    /// True if the given priority may be returned from `next` due to being within the range
    /// between [`front`](Self::front) and [`back`](Self::back).
    fn is_in_range(&self, priority: i16) -> bool {
        self.front.is_none_or(|p| p < priority) && self.back.is_none_or(|p| priority < p)
    }
}

impl Iterator for PriorityIter<'_> {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .source
            .constraints
            .iter()
            .map(|p| p.hint.priority)
            .filter(|p| self.is_in_range(*p))
            .min();
        if result.is_some() {
            self.front = result;
        }
        result
    }
}

impl DoubleEndedIterator for PriorityIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = self
            .source
            .constraints
            .iter()
            .map(|p| p.hint.priority)
            .filter(|p| self.is_in_range(*p))
            .max();
        if result.is_some() {
            self.back = result;
        }
        result
    }
}

/// Is the layout algorithm done yet?
#[derive(Eq, PartialEq, Clone, Copy)]
enum FillState {
    /// The algorithm has finished due to filling the entire area.
    Finished,
    /// The algorithm has work yet to do.
    Continue,
}

/// The collection of segments that the layout algorithm operates on.
struct SegmentSet {
    /// The segments that the algorithm gradually grows.
    rules: Vec<SegmentRule>,
    /// The amount of space that has already been allocated.
    rules_length: i32,
    /// The total amount of space available to allocate.
    area_length: i32,
    /// A record of the steps taken by the algorithm.
    record: OptionFillRecord,
}

/// Integer division that rounds up.
const fn div_ceil(a: u64, b: u64) -> u64 {
    if a == 0 {
        return 0;
    }
    a.saturating_sub(1).saturating_div(b).saturating_add(1)
}

/// Integer division that rounds up or down as appropriate.
const fn div_round(a: u64, b: u64) -> u64 {
    if a == 0 {
        return 0;
    }
    let q = a / b;
    let r = a % b;
    q.saturating_add((r * 2 >= b) as u64)
}

impl SegmentSet {
    /// Construct a new set from a list of rules and an area size.
    /// The `record` is optional and is normally `None` unless the algorithm is being run
    /// to debug its constraints.
    fn new(rules: Vec<SegmentRule>, area_length: i32, record: Option<FillRecord>) -> Self {
        let rules_length = rules.iter().map(|r| r.size).sum();
        let mut record = OptionFillRecord(record);
        record.set_segments(area_length, &rules);
        Self {
            rules,
            rules_length,
            area_length,
            record,
        }
    }
    /// Reset the segments to the state at the start of the algorithm, thereby allowing
    /// the algorithm to try an alternative strategy in case the first approach failed.
    fn reset(&mut self) {
        self.record.reset();
        for rule in &mut self.rules {
            rule.size = rule.initial_size;
        }
        self.rules_length = self.rules.iter().map(|r| r.size).sum();
    }
    /// Checks that every segment has reached its minimum size.
    fn are_all_at_min(&self) -> bool {
        self.rules.iter().all(|r| r.min <= r.size)
    }
    /// Iterate through the given iterator of priorities, for each priority,
    /// activate the segments with that priority and let them fill to the given
    /// target. Return `Finished` if the layout is complete.
    fn fill_by_priorities<I: Iterator<Item = i16>>(
        &mut self,
        target: SegmentTarget,
        priorities: I,
    ) -> FillState {
        for p in priorities {
            if self.fill_to(p, target) == FillState::Finished {
                return FillState::Finished;
            }
        }
        FillState::Continue
    }
    /// Activate segments with the given priority if they can be activated, and
    /// then iterate through those segments repeatedly until they have all reached
    /// the given target or the layout is finished. Return `Finished` if the layout
    /// is finished.
    fn fill_to(&mut self, priority: i16, target: SegmentTarget) -> FillState {
        loop {
            if self.area_length <= self.rules_length {
                return FillState::Finished;
            }
            // The speed is how much all the active segments might allocate this round
            // per STEP_FACTOR units of step size.
            let speed: u64 = self
                .rules
                .iter()
                .filter(|r| r.can_fill(priority, target))
                .map(|r| r.fill_scale)
                .sum();
            // Speed == 0 means there are no active segments, so stop and move on to the
            // next round.
            if speed == 0 {
                return FillState::Continue;
            }
            // Determine how much space remains to be allocated.
            let remainder =
                u64::try_from(self.area_length.saturating_sub(self.rules_length)).unwrap_or(0);
            // Determine how big a step size we can allow each segment this round without
            // possibility of overflowing the area.
            let step_size = div_ceil(remainder.saturating_mul(STEP_FACTOR), speed);
            // Iterate through all the active segments in spiral order (for symmetry), and give them
            // all the same step size so they each get a fair chance to reach their target.
            for (index, s) in
                SpiralIter::new(&mut self.rules).filter(|(_, r)| r.can_fill(priority, target))
            {
                let limit = self.area_length.saturating_sub(self.rules_length);
                let before = s.size;
                // Grow the segment without exceeding the target or overflowing the area.
                s.fill_steps_to(step_size, target, limit);
                let grow = s.size.saturating_sub(before);
                s.size = s.size.max(s.size);
                // Record what just happened, if we are in debug mode.
                self.record
                    .add_step(index, before, s.size, step_size, target);
                self.rules_length = self.rules_length.saturating_add(grow);
                if self.area_length <= self.rules_length {
                    return FillState::Finished;
                }
            }
        }
    }
    /// Generate rectangles for the segments at their current sizes.
    fn build_segments(
        &self,
        direction: Direction,
        area: Rect,
        constraints: &[XConstraint],
    ) -> Segments {
        match direction {
            Direction::Horizontal => self.build_segments_x(area, constraints),
            Direction::Vertical => self.build_segments_y(area, constraints),
        }
    }
    /// Generate rectangles for the segments assuming that layout is horizontal.
    fn build_segments_x(&self, area: Rect, constraints: &[XConstraint]) -> Segments {
        let mut result = Vec::new();
        let mut x = area.left();
        let y = area.y;
        let height = area.height;
        for (c, rule) in constraints.iter().zip(&self.rules) {
            let start = x;
            x = x.saturating_add_signed(i16::try_from(rule.size).unwrap_or(0));
            if c.is_separator {
                continue;
            }
            let width = x.saturating_sub(start);
            let r = Rect::new(start, y, width, height);
            result.push(c.inner_rect(r));
        }
        Rc::from(&*result)
    }
    /// Generate rectangles for the segments assuming that layout is vertical.
    fn build_segments_y(&self, area: Rect, constraints: &[XConstraint]) -> Segments {
        let mut result = Vec::new();
        let mut y = area.top();
        let x = area.x;
        let width = area.width;
        for (c, rule) in constraints.iter().zip(&self.rules) {
            let start = y;
            y = y.saturating_add_signed(i16::try_from(rule.size).unwrap_or(0));
            if c.is_separator {
                continue;
            }
            let height = y.saturating_sub(start);
            let r = Rect::new(x, start, width, height);
            result.push(c.inner_rect(r));
        }
        Rc::from(&*result)
    }
}

/// Improve symmetry by iterating inward from both ends toward the center
/// of the list. If the list were `[a,b,c,d,e]`, the iterator would produce
/// (1,a), (5,e), (2,b), (4,d), (3,c). It produces `(usize, &mut T)`.
struct SpiralIter<'a, T> {
    /// The slice to iterate over.
    items: &'a mut [T],
    /// The index of the first element of the slice.
    front_index: usize,
    /// True if the next element should be from the front of the slice.
    /// Otherwise, the next element should be the last element of the slice.
    front_turn: bool,
}

impl<'a, T> SpiralIter<'a, T> {
    /// Construct a new iterator over the given slice.
    const fn new(items: &'a mut [T]) -> Self {
        Self {
            items,
            front_index: 0,
            front_turn: true,
        }
    }
}

impl<'a, T> Iterator for SpiralIter<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let items = core::mem::take(&mut self.items);
        if let Some((item, next)) = if self.front_turn {
            items.split_first_mut()
        } else {
            items.split_last_mut()
        } {
            let index = if self.front_turn {
                self.front_index
            } else {
                self.front_index + next.len()
            };
            if self.front_turn {
                self.front_index += 1;
            }
            self.items = next;
            self.front_turn = !self.front_turn;
            Some((index, item))
        } else {
            None
        }
    }
}

/// Transpose the x and y axis of a `Rect`, so horizontal becomes vertical and x becomes y,
/// allowing us to pretend all `Rect`s are arranged along the x-axis and then flip them
/// if necessary to the y-axis at the end.
const fn transpose(rect: Rect) -> Rect {
    Rect::new(rect.y, rect.x, rect.height, rect.width)
}

/// Construct a list of spacers from an area and a list of segments within that area.
fn build_spacers(direction: Direction, area: Rect, segments: &[Rect]) -> Spacers {
    // Pretend that direction is horizontal by transposing the area's x and y.
    let area = if direction == Direction::Vertical {
        transpose(area)
    } else {
        area
    };
    let mut result = Vec::new();
    let mut x = area.left();
    let y = area.y;
    let height = area.height;
    for seg in segments {
        let seg = if direction == Direction::Vertical {
            transpose(*seg)
        } else {
            *seg
        };
        let width = seg.x.saturating_sub(x);
        result.push(Rect::new(x, y, width, height));
        x = if seg.width > 0 {
            seg.right()
        } else {
            // If seg.width == 0 then we cannot trust seg.right()
            // to be accurate.
            x.saturating_add(width)
        };
    }
    let width = area.right().saturating_sub(x);
    result.push(Rect::new(x, y, width, height));
    if direction == Direction::Vertical {
        for r in &mut result {
            *r = transpose(*r);
        }
    }
    Rc::from(&*result)
}

/// These represent the stages of layout. Each stage involves allocating
/// some portion of the area to each constraint base on that constraint's hints.
/// At each stage each constraint has some target that it wants to grow to, and
/// the layout algorithm checks how much space is available and divides it among
/// the active constraints to allow them to grow toward their targets.
///
/// A constraint is active if it has not yet reached its target size for the phase
/// and if its [`HintRange::priority`] value indicates that it is that constraint's
/// turn to grow. As active constraints reach their target and become inactive, the
/// remaining available area is again divide between the remaining constraints
/// until either all active constraints have reached their targets or there is no
/// more remaining space.
///
/// Assuming that there is some space remaining to allocate, layout proceeds to the
/// next phase in order:
///
/// 1. Range(Min): The target of each constraint comes from [`HintRange::min`].
/// 2. Range(Preferred): The target of each constraint comes from [`HintRange::preferred`].
/// 3. Range(Max): The target of each constraint comes from [`HintRange::max`].
/// 4. Overfill: Only constraints with [`HintRange::overfill`] are active during this phase, and
///    constraints grow until the whole area is filled.
/// 5. Forced: All constraints grow until the whole areas is filled.
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIs)]
pub enum SegmentTarget {
    /// There are three `Range` stages: min, preferred, and max, each corresponding to
    /// a `RangeLevel` within a [`HintRange`]. The actual target size for each constraint
    /// is determined based on the [`Hint`], the constraint's [`SizeRange`], and the size
    /// of the area to fill.
    ///
    /// Regardless of the min hint's value, the min phase target cannot be smaller than
    /// the min of the `SizeRange` in the layout direction. And if the alignment in the
    /// layout direction is [`Align::Full`] then the target of the max phase cannot be larger
    /// than the max of the `SizeRange`. If the hint is [`Hint::Size`] then the target takes
    /// its value directly from the `SizeRange`.
    Range(RangeLevel),
    /// The overfill phase is where constraints fill beyond their max target if necessary to
    /// completely fill the area, so the target is infinite, but only constraints with the
    /// [`HintRange::overfill`] flag set are active.
    Overfill,
    /// This phase is only possible if no constraint has the [`HintRange::overfill`] flag set,
    /// since otherwise the [`SegmentTarget::Overfill`] phase would have inevitably allocated the
    /// entire area. Therefore, in order to ensure that the area is filled, the forced phase
    /// causes all constraints to grow even without the overfill flag, thereby ending the layout
    /// process.
    Forced,
}

impl Display for SegmentTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Range(r) => Debug::fmt(r, f),
            _ => Debug::fmt(&self, f),
        }
    }
}

impl SegmentTarget {
    /// True if this is the [`SegmentTarget::Range`] variant,
    /// and the given predicate returns true.
    pub fn is_range_and<P>(self, pred: P) -> bool
    where
        P: FnOnce(RangeLevel) -> bool,
    {
        match self {
            Self::Range(r) => pred(r),
            _ => false,
        }
    }
}

/// This represents a segment's constraint within a given area to be split.
/// The hint values that were relative to an unknown area size in the [`XConstraint`]
/// have been converted to absolute values, so we are ready to begin allocating
/// space within the area.
#[derive(Debug)]
struct SegmentRule {
    /// The initial size of the segment, for the purposes of resetting the algorithm.
    initial_size: i32,
    /// The minimum size of the area to allocate for the segment, if possible.
    /// It may be negative to indicate an overlap.
    min: i32,
    /// The preferred size of the area to allocate for the segment, if possible.
    /// A high value for `priority` helps to ensure that the preferred size is respected.
    /// It may be negative to indicate an overlap.
    preferred: i32,
    /// The maximum size of the area to allocate for the segment, though the segment
    /// may be allocated more if necessary to fill the area. To prevent overfilling,
    /// ensure that `overfill` is false and `priority` is a low value.
    /// It may be negative to indicate an overlap.
    max: i32,
    /// The rate at which to allocate space for the segment.
    /// If this is zero, then the segment is never allocated any space, even if that means
    /// that the area fails to be filled, even if `overflow` is true.
    /// Segments with a fill scale of 2 are allocated space twice as quickly as segments
    /// with a fill scale of 1, and so on. The default is 1.
    fill_scale: u64,
    /// If true, then the segment should be allocated space beyond its max when this is necessary
    /// in order to fill the area. If false, then the segment will never be allocated space beyond
    /// its max unless no segment has `overfill` set to true. The default is `false`.
    overfill: bool,
    /// A number that determines the order in which segments are allocated space.
    /// Segments with lower values are allocated first when allocating the min space
    /// for each segment, and when allocating the preferred space.
    /// When allocating the maximum space and the overfill space, segments with higher
    /// values allocate first. This means that lower values are more likely to be allocated
    /// close to their preferred size, while higher values are more likely to be shrunk or
    /// stretched as needed.
    priority: i16,
    /// The current space allocated to the segment. This value changes during the allocation
    /// process.
    size: i32,
}

impl Index<RangeLevel> for SegmentRule {
    type Output = i32;

    fn index(&self, index: RangeLevel) -> &Self::Output {
        match index {
            RangeLevel::Min => &self.min,
            RangeLevel::Preferred => &self.preferred,
            RangeLevel::Max => &self.max,
        }
    }
}

impl SegmentRule {
    /// Construct a `SegmentRule` using the state of the layout and the size of the area.
    /// This transforms a constraint from an abstract notion with no knowledge of what area
    /// it will be laying out, into a set of concrete specific goals.
    fn new(constraint: &XConstraint, direction: Direction, area_length: u16) -> Self {
        let range = constraint.size.linear(direction);
        let margin = constraint.margin.linear(direction).total();
        // Convert the three hints from Hint values to i32, now that we know
        // the size of the area.
        let mut min = constraint
            .hint
            .min
            .absolute_for(constraint, direction, area_length);
        let mut preferred =
            constraint
                .hint
                .preferred
                .absolute_for(constraint, direction, area_length);
        let mut max = constraint
            .hint
            .max
            .absolute_for(constraint, direction, area_length);
        // If the alignment is Full, then max cannot be larger than the max of the size.
        if constraint.get_align_for(direction) == Align::Full {
            let range_max = i32::from(range.max.saturating_add(margin));
            max = max.min(range_max);
        }
        // If the minimum size is greater than 0, this segment will contain some
        // important content, therefore increase min to be at least that minimum size.
        let range_min = i32::from(range.min.saturating_add(margin));
        if range_min > 0 {
            min = min.max(range_min);
        }
        preferred = preferred.max(min);
        // Preferred must not be larger than max, since that would allow the segment
        // to accidentally exceed max during the preferred phase.
        preferred = preferred.min(max);
        // Normally the initial size of a segment would be 0, but if the fill_scale is 0
        // then this segment cannot grow, so make a special exception and set its initial
        // size to its min size.
        let size = if constraint.hint.fill_scale == 0 {
            min
        } else {
            // If min is less than 0, then this segment is creating overlap,
            // so start its size as a negative number. Otherwise, let the initial size be 0.
            min.min(0)
        };
        Self {
            initial_size: size,
            min,
            preferred,
            max,
            fill_scale: u64::from(constraint.hint.fill_scale),
            priority: constraint.hint.priority,
            overfill: constraint.hint.overfill,
            size,
        }
    }
    /// Construct a [`FillRecordSegment`] for this segment, to record it for debugging purposes.
    fn record(&self) -> FillRecordSegment {
        FillRecordSegment {
            min: self.min,
            preferred: self.preferred,
            max: self.max,
            fill_scale: i32::try_from(self.fill_scale).unwrap_or(i32::MAX),
            priority: self.priority,
            overfill: self.overfill,
        }
    }
    /// Is this segment active for the given priority and target?
    fn can_fill(&self, priority: i16, target: SegmentTarget) -> bool {
        self.priority == priority
            && self.fill_scale > 0
            && (target.is_range_and(|t| self.dist(t) > 0)
                || target.is_overfill() && self.overfill
                || target.is_forced())
    }
    /// The distance from the current size to the given target.
    #[inline]
    fn dist(&self, target: RangeLevel) -> i32 {
        self[target] - self.size
    }
    /// Update the size of this segment as much as possible within the limits of
    /// `step_size`, the target, and the limit which represents the remaining area.
    fn fill_steps_to(&mut self, step_size: u64, target: SegmentTarget, limit: i32) {
        let max = self.size.saturating_add(limit);
        // We are only allowed to allocate fill_scale units for each STEP_FACTOR of step_size,
        // So `grow_fac` represents how much we can grow this step times STEP_FACTOR.
        let grow_fac = self.fill_scale.saturating_mul(step_size);
        // Divide by STEP_FACTOR and round up if appropriate to calculate the actual limit
        // of how much we can grow this step.
        let grow_mod = grow_fac % STEP_FACTOR;
        let grow = (grow_fac / STEP_FACTOR)
            .saturating_add(u64::from(grow_mod * 2 > STEP_FACTOR))
            .max(1);
        // Calculate the new size by adding `grow` and clamp it to `max`.
        let mut size = self
            .size
            .saturating_add(i32::try_from(grow).unwrap_or(1))
            .min(max);
        // If we have a target we are aiming for, clamp our size to no greater than the target.
        if let SegmentTarget::Range(target) = target {
            size = size.min(self[target]);
        }
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::layout::{
        ConstraintList, Direction, Hint, HintRange, IntoConstraint, Rect, SizeRange,
        VerticalAlignment, XConstraint, XLayout, XMargin,
    };
    #[test]
    fn empty_layout() {
        let segments = XLayout::default().split(Rect::new(0, 0, 12, 4));
        assert!(segments.is_empty());
    }
    #[test]
    fn layout_too_large() {
        let [area] = XLayout::horizontal([HintRange::FULL.with_min(15).scale(0)])
            .areas(Rect::new(0, 0, 12, 4));
        assert_eq!(area, Rect::new(0, 0, 15, 4));
        let [area] = XLayout::vertical([HintRange::FULL.with_min(15).scale(0)])
            .areas(Rect::new(0, 0, 12, 4));
        assert_eq!(area, Rect::new(0, 0, 12, 15));
    }
    #[test]
    fn overlap_beyond_area() {
        let [area] = XLayout::horizontal(XConstraint::overlap(5).scale(0).list() + HintRange::FULL)
            .areas(Rect::new(10, 10, 12, 4));
        assert_eq!(area, Rect::new(5, 10, 17, 4));
        let [area] = XLayout::vertical(XConstraint::overlap(5).scale(0).list() + HintRange::FULL)
            .areas(Rect::new(10, 10, 12, 4));
        assert_eq!(area, Rect::new(10, 5, 12, 9));
    }
    #[test]
    fn centered_text() {
        use crate::text::Text;
        let text = Text::from("1234\n123456");
        let [area] = XLayout::from([text.size()]).areas(Rect::new(0, 0, 12, 4));
        assert_eq!(area, Rect::new(3, 1, 6, 2));
    }
    #[test]
    fn centered_size() {
        let [area] = XLayout::from([(10, 6)]).areas(Rect::new(0, 0, 20, 10));
        assert_eq!(area, Rect::new(5, 2, 10, 6));
    }
    #[test]
    fn vertical_stack_top() {
        let [a, b] =
            (XLayout::from([(10, 6), (4, 4)]) + XConstraint::SPACER).areas(Rect::new(0, 0, 20, 20));
        assert_eq!([a, b], [Rect::new(5, 0, 10, 6), Rect::new(8, 6, 4, 4)]);
    }
    #[test]
    fn vertical_stack_bottom() {
        let [a, b] =
            (XConstraint::SPACER + XLayout::from([(10, 6), (4, 4)])).areas(Rect::new(0, 0, 20, 20));
        assert_eq!([a, b], [Rect::new(5, 10, 10, 6), Rect::new(8, 16, 4, 4)]);
    }
    #[test]
    fn horizontal_stack_left() {
        let [a, b] =
            XLayout::horizontal(ConstraintList::from([(10, 6), (4, 4)]) + XConstraint::SPACER)
                .areas(Rect::new(0, 0, 20, 20));
        assert_eq!([a, b], [Rect::new(0, 7, 10, 6), Rect::new(10, 8, 4, 4)]);
    }
    #[test]
    fn horizontal_stack_right() {
        let [a, b] = XLayout::horizontal(XConstraint::SPACER.list() + (10, 6) + (4, 4))
            .areas(Rect::new(0, 0, 20, 20));
        assert_eq!([a, b], [Rect::new(6, 7, 10, 6), Rect::new(16, 8, 4, 4)]);
    }
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn horizontal_stack_separated() {
        let layout = XLayout::horizontal((10, 6).list() + XConstraint::SPACER + (4, 4));
        let area = Rect::new(0, 0, 20, 20);
        let [a, b] = layout.areas(area);
        let [x, y, z] = layout.spacers(area);
        assert_eq!(
            [x, a, y, b, z],
            [
                Rect::new(0, 0, 0, 20),
                Rect::new(0, 7, 10, 6),
                Rect::new(10, 0, 6, 20),
                Rect::new(16, 8, 4, 4),
                Rect::new(20, 0, 0, 20)
            ]
        );
    }
    #[test]
    fn horizontal_top_bottom() {
        let layout = XLayout::horizontal(
            (10, 6).y_align(VerticalAlignment::Top).list()
                + XConstraint::SPACER
                + (4, 4).y_align(VerticalAlignment::Bottom),
        );
        let area = Rect::new(0, 0, 20, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 10, 6), Rect::new(16, 16, 4, 4),]);
    }
    #[test]
    fn horizontal_bottom_top() {
        let layout = XLayout::horizontal(
            (10, 6).y_align(VerticalAlignment::Bottom).list()
                + XConstraint::SPACER
                + (4, 4).y_align(VerticalAlignment::Top),
        );
        let area = Rect::new(0, 0, 20, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 14, 10, 6), Rect::new(16, 0, 4, 4),]);
    }
    #[test]
    fn horizontal_divided() {
        let layout = XLayout::horizontal((.., ..).list() + (3, ..).separator() + (.., ..));
        let area = Rect::new(0, 0, 21, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 9, 20), Rect::new(12, 0, 9, 20),]);
    }
    #[test]
    fn vertical_divide_and_squeeze() {
        let layout = XLayout::vertical((.., 8).list() + (.., 2).separator().priority(-1) + (.., 8));
        let area = Rect::new(0, 0, 20, 10);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 20, 4), Rect::new(0, 6, 20, 4),]);
    }
    #[test]
    fn uneven_squeeze() {
        let layout = XLayout::vertical(
            (.., 8).scale(1).list() + (.., 2).separator().scale(0) + (.., 8).scale(2),
        );
        let area = Rect::new(0, 0, 20, 8);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 20, 2), Rect::new(0, 4, 20, 4),]);
    }
    #[test]
    fn stretch() {
        use crate::layout::Align::Full;
        let layout = XLayout::vertical(
            (.., 8).y_align(Full).list() + (.., 2).separator().scale(0) + (.., 4).y_align(Full),
        );
        let area = Rect::new(0, 0, 20, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 20, 11), Rect::new(0, 13, 20, 7),],);
    }
    #[test]
    fn percent_left_right() {
        let layout = XLayout::horizontal(
            Hint::Percentage(25).list() + XConstraint::SPACER + Hint::Percentage(25),
        );
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 3, 20), Rect::new(7, 0, 3, 20),],);
    }
    #[test]
    fn ratio_left_right1() {
        let layout =
            XLayout::horizontal(Hint::Ratio(1, 4).list() + XConstraint::SPACER + Hint::Ratio(1, 4));
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 3, 20), Rect::new(7, 0, 3, 20),],);
    }
    #[test]
    fn ratio_left_right2() {
        let layout =
            XLayout::horizontal(Hint::Ratio(1, 3).list() + XConstraint::SPACER + Hint::Ratio(2, 4));
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 3, 20), Rect::new(5, 0, 5, 20),],);
    }
    #[test]
    fn wide_narrow() {
        let layout = XLayout::horizontal((4..=6).list() + (1..=10));
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(0, 0, 5, 20), Rect::new(5, 0, 5, 20),]);
    }
    #[test]
    fn layout_margin() {
        let layout = XLayout::horizontal((4..=6).list() + (1..=10)).margin(XMargin {
            top: 1,
            bottom: 2,
            left: 3,
            right: 4,
        });
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        assert_eq!([a, b], [Rect::new(3, 1, 2, 17), Rect::new(5, 1, 1, 17),],);
    }
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn segment_margin_horizontal() {
        let margin = XMargin {
            top: 1,
            bottom: 2,
            left: 3,
            right: 4,
        };
        let layout = XLayout::horizontal((4..=6).list() + (1..=10).margin(margin));
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        let [x, y, z] = layout.spacers(area);
        assert_eq!(
            [x, a, y, b, z],
            [
                Rect::new(0, 0, 0, 20),
                Rect::new(0, 0, 4, 20),
                Rect::new(4, 0, 0, 20),
                Rect::new(0, 0, 0, 0),
                Rect::new(4, 0, 6, 20),
            ]
        );
    }
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn segment_margin_vertical() {
        let margin = XMargin {
            top: 1,
            bottom: 2,
            left: 3,
            right: 4,
        };
        let layout = XLayout::vertical((4..=6).list() + (1..=10).margin(margin));
        let area = Rect::new(0, 0, 10, 20);
        let [a, b] = layout.areas(area);
        let [x, y, z] = layout.spacers(area);
        assert_eq!(
            [x, a, y, b, z],
            [
                Rect::new(0, 0, 10, 0),
                Rect::new(0, 0, 10, 8),
                Rect::new(0, 8, 10, 1),
                Rect::new(3, 9, 3, 9),
                Rect::new(0, 18, 10, 2),
            ]
        );
    }
    #[rstest]
    #[case::size1(SizeRange::new(25,7), [(3,4),(21,7),(1,1)], Direction::Horizontal)]
    #[case::size2(SizeRange::new(21,12), [(3,4),(21,7),(1,1)], Direction::Vertical)]
    #[case::size3(SizeRange::new(10..=21,12), (3,4).list() + (10..=21,7) + (1,1), Direction::Vertical)]
    #[case::size4(SizeRange::new(3..=21,(12,13)), (3,4..6).list() + (..=21,7) + (1,1), Direction::Vertical)]
    #[case::size5(SizeRange::new((4,25),(7,7,5)), (3,4..6).list() + (..=21,7) + (1,1), Direction::Horizontal)]
    fn size<I>(#[case] expected: SizeRange, #[case] constraints: I, #[case] direction: Direction)
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        let layout = XLayout::new(direction, constraints);
        assert_eq!(layout.size(), expected);
    }
}
