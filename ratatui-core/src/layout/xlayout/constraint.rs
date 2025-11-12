//! The [`XConstraint`] type is the fundamental building block of [`XLayout`], and it is supported
//! by [`ConstraintList`], [`Align`], [`SizeRange`], and [`XMargin`].

use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Write};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Index, IndexMut, Range, RangeBounds, RangeFrom, RangeFull,
    RangeInclusive, RangeTo, RangeToInclusive, Sub, SubAssign,
};

use strum::{Display, EnumIs, EnumString};

use crate::layout::{
    Constraint, Direction, Hint, HintRange, HorizontalAlignment, Margin, RangeLevel, Rect, Size,
    VerticalAlignment, XLayout,
};

/// Trait for types that can be converted into [`XConstraint`]. These methods all take
/// self and automatically convert it into a constraint before performing their operation
/// and returning the resulting value, usually a constraint with some modifications.
/// This allows non-constraint values to be treated as if they are constraint values when
/// constructing an [`XLayout`].
pub trait IntoConstraint: Into<XConstraint> {
    /// Create an [`XConstraint`] from this value.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn constraint(self) -> XConstraint {
        <Self as Into<XConstraint>>::into(self)
    }
    /// Create a [`ConstraintList`] from this value.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn list(self) -> ConstraintList {
        ConstraintList::single(self)
    }
    /// Create a separator constraint by setting the [`XConstraint::is_separator`] flag to true,
    /// thereby preventing the constraint from creating a segment when it is used in an
    /// [`XLayout`] to split an area. Separator constraints only serve to help position
    /// other segments.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn separator(self) -> XConstraint {
        self.constraint().separator()
    }
    /// Create a constraint with the given alignment on the x axis.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn x_align<A: Into<Align>>(self, align: A) -> XConstraint {
        self.constraint().x_align(align)
    }
    /// Create a constraint with the given alignment on the y axis.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn y_align<A: Into<Align>>(self, align: A) -> XConstraint {
        self.constraint().y_align(align)
    }
    /// Create a constraint with the given margin.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn margin<M: Into<XMargin>>(self, margin: M) -> XConstraint {
        self.constraint().margin(margin)
    }
    /// Create a constraint with for a segment with sizes in the given range.
    /// When a [`Rect`] is split into segments, the segments do not necessarily fill the
    /// portion of `Rect` that was allocated to them. Setting the area range of a constraint
    /// controls how large the segment should be within the allocated area. See [`SizeRange`]
    /// for details.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn size<A: Into<SizeRange>>(self, size: A) -> XConstraint {
        self.constraint().size(size)
    }
    /// Create a constraint with the given minimum value for the [`SizeRange`].
    /// Use [`IntoConstraint::size`] to set all three levels of the range at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn size_min(self, size: Size) -> XConstraint {
        self.constraint().size_min(size)
    }
    /// Create a constraint with the given preferred value for the [`SizeRange`].
    /// Use [`IntoConstraint::size`] to set all three levels of the range at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn size_preferred(self, size: Size) -> XConstraint {
        self.constraint().size_preferred(size)
    }
    /// Create a constraint with the given maximum value for the [`SizeRange`].
    /// Use [`IntoConstraint::size`] to set all three levels of the range at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn size_max(self, size: Size) -> XConstraint {
        self.constraint().size_max(size)
    }
    /// Create a constraint with the given hint to control its layout.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn hint(self, hint: HintRange) -> XConstraint {
        self.constraint().hint(hint)
    }
    /// Create a constraint with the given value for [`HintRange::min`].
    #[must_use = "method moves the value of self and returns the modified value"]
    fn with_min<H: Into<Hint>>(self, hint: H) -> XConstraint {
        self.constraint().with_min(hint)
    }
    /// Create a constraint with the given value for [`HintRange::preferred`].
    #[must_use = "method moves the value of self and returns the modified value"]
    fn preferred<H: Into<Hint>>(self, hint: H) -> XConstraint {
        self.constraint().preferred(hint)
    }
    /// Create a constraint with the given value for [`HintRange::max`].
    #[must_use = "method moves the value of self and returns the modified value"]
    fn with_max<H: Into<Hint>>(self, hint: H) -> XConstraint {
        self.constraint().with_max(hint)
    }
    /// Create a constraint with no maximum hint by setting its max hint to [`Hint::FULL`].
    #[must_use = "method moves the value of self and returns the modified value"]
    fn without_max(self) -> XConstraint {
        self.constraint().with_max(Hint::FULL)
    }
    /// Create a constraint with the given [`HintRange::fill_scale`].
    #[must_use = "method moves the value of self and returns the modified value"]
    fn scale(self, fill_scale: u16) -> XConstraint {
        self.constraint().scale(fill_scale)
    }
    /// Create a constraint with the given [`HintRange::priority`] to control in what
    /// order the constraint gets to allocate space relative to the other constraints.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn priority(self, priority: i16) -> XConstraint {
        self.constraint().priority(priority)
    }
    /// Create a constraint with the given [`HintRange::overfill`] to control whether
    /// this constraint allocates space beyond its maximum.
    #[must_use = "method moves the value of self and returns the modified value"]
    fn overfill(self, overfill: bool) -> XConstraint {
        self.constraint().overfill(overfill)
    }
}

impl<V: Into<XConstraint>> IntoConstraint for V {}

/// The layout process for [`XLayout`] is controlled by a list of constraints,
/// much like [`Layout`](super::Layout), but these are extended constraints that
/// contain far more settings to fine-tune the layout. Instead of needing to choose
/// whether a constraint has a maximum or a minimum, every `XConstraint` is allowed to have
/// both, as well as a preferred size, a margin, and an alignment.
#[derive(Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XConstraint {
    /// The margin between the segment and the segment's allocated area.
    /// This is a fixed amount of space that will be subtracted from the sides
    /// of the allocated area before the size and position of the segment are calculated.
    pub margin: XMargin,
    /// The horizontal alignment of the segment within the space allocated for the segment.
    /// If more space is allocated by the layout than is needed for the segment, then the segment
    /// can be at the left, at the right, horizontally centered, or stretched to fill the
    /// allocated space.
    pub x_align: Align,
    /// The vertical alignment of the segment within the space allocated for the segment.
    /// If more space is allocated by the layout than is needed for the segment, then the segment
    /// can be at the top, at the bottom, vertically centered, or stretched to fill the
    /// allocated space.
    pub y_align: Align,
    /// The size range needed for the resulting segment, including minimum size, preferred size,
    /// and maximum size in both the horizontal and the vertical. The layout direction
    /// determines which axis of the size range will be used. Ideally this should be supplied
    /// by the widget that will be rendered in the segment, and calculated based upon the
    /// widget's content.
    pub size: SizeRange,
    /// A hint to control the layout process, such in which order segments should have space
    /// allocated, how much space a segment needs at minimum, and so on. See [`HintRange`] for
    /// details.
    pub hint: HintRange,
    /// A separator constraint does not cause [`XLayout`] to produce a segment when splitting an
    /// area. Separator constraints only serve to help position other segments by taking up
    /// space.
    pub is_separator: bool,
}

impl Debug for XConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("XConstraint(")?;
        if self.is_separator {
            f.write_str("separator, ")?;
        }
        write!(
            f,
            "({:?}, {:?}), {:?}, {:?})",
            self.x_align, self.y_align, self.size, self.hint
        )
    }
}

impl From<&Constraint> for XConstraint {
    fn from(value: &Constraint) -> Self {
        HintRange::from(*value).into_constraint()
    }
}

impl From<&u16> for XConstraint {
    fn from(value: &u16) -> Self {
        HintRange::from(*value).into_constraint()
    }
}

impl From<&Hint> for XConstraint {
    fn from(value: &Hint) -> Self {
        HintRange::from(*value).into_constraint()
    }
}

impl<R: Into<HintRange>> From<R> for XConstraint {
    fn from(value: R) -> Self {
        value.into().into_constraint()
    }
}

impl From<&HintRange> for XConstraint {
    fn from(value: &HintRange) -> Self {
        value.constraint()
    }
}

impl From<SizeRange> for XConstraint {
    fn from(value: SizeRange) -> Self {
        value.into_constraint()
    }
}

impl<R: Into<LinearSizeRange>, S: Into<LinearSizeRange>> From<(R, S)> for XConstraint {
    fn from((width, height): (R, S)) -> Self {
        SizeRange::new(width, height).into()
    }
}

impl From<Size> for XConstraint {
    fn from(value: Size) -> Self {
        SizeRange::new(value.width, value.height).into()
    }
}

impl From<&Size> for XConstraint {
    fn from(value: &Size) -> Self {
        SizeRange::new(value.width, value.height).into()
    }
}

impl From<&SizeRange> for XConstraint {
    fn from(value: &SizeRange) -> Self {
        value.constraint()
    }
}

/// A margin along a single axis, either horizontal or vertical.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearMargin {
    /// The top or left margin.
    pub start: u16,
    /// The bottom or right margin.
    pub end: u16,
}

impl From<u16> for LinearMargin {
    fn from(value: u16) -> Self {
        Self {
            start: value,
            end: value,
        }
    }
}

impl From<(u16, u16)> for LinearMargin {
    fn from((start, end): (u16, u16)) -> Self {
        Self { start, end }
    }
}

impl LinearMargin {
    /// Create a new linear margin from start and end values.
    pub const fn new(start: u16, end: u16) -> Self {
        Self { start, end }
    }
    /// The sum of the two sides of the margin.
    pub const fn total(self) -> u16 {
        self.start.saturating_add(self.end)
    }
    /// Create a full [`XMargin`] by specifying an axis for this margin.
    /// The other axis will have a margin of 0.
    pub const fn into_margin(self, direction: Direction) -> XMargin {
        match direction {
            Direction::Horizontal => XMargin {
                left: self.start,
                right: self.end,
                top: 0,
                bottom: 0,
            },
            Direction::Vertical => XMargin {
                top: self.start,
                bottom: self.end,
                left: 0,
                right: 0,
            },
        }
    }
}

impl XConstraint {
    /// The default constraint. It has no margin, it is aligned to [`Align::Full`] both
    /// horizontally and vertically, its `size` is [`SizeRange::FULL`], and
    /// its hint range is [`HintRange::FULL`]. It creates a segment with no minimum or maximum
    /// size and it prefers to fill whatever space is available.
    pub const FULL: Self = Self {
        margin: XMargin::ZERO,
        x_align: Align::Full,
        y_align: Align::Full,
        size: SizeRange::FULL,
        hint: HintRange::FULL,
        is_separator: false,
    };

    /// A separator constraint with a preferred size of zero which can grow to [`Hint::FULL`]
    /// at maximum to allow segments to not need to stretch.
    pub const SPACER: Self = Self {
        hint: HintRange {
            preferred: Hint::ZERO,
            priority: 1,
            ..HintRange::FULL
        },
        is_separator: true,
        ..Self::FULL
    };

    /// A separator constraint that has negative size for its min and preferred,
    /// creating the given amount of overlap. Its max is set to [`Hint::ZERO`] so that
    /// the overlap is removed if the layout is struggling to fill the available space,
    /// and `priority` is set to 100 to encourage the overlap to be removed before any
    /// segments get stretched.
    pub const fn overlap(amount: u16) -> Self {
        Self {
            hint: HintRange {
                min: Hint::Overlap(amount),
                preferred: Hint::Overlap(amount),
                max: Hint::ZERO,
                priority: 100,
                ..HintRange::FULL
            },
            is_separator: true,
            ..Self::FULL
        }
    }

    /// The alignment of this constraint along the given axis.
    pub const fn get_align_for(&self, direction: Direction) -> Align {
        match direction {
            Direction::Horizontal => self.x_align,
            Direction::Vertical => self.y_align,
        }
    }
    /// Convert the constraint into a separator by setting its separator flag to true,
    /// thereby preventing it from creating a segment when it is used in an
    /// [`XLayout`] to split an area. Separator constraints only serve to help position
    /// other segments.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn separator(self) -> Self {
        Self {
            is_separator: true,
            ..self
        }
    }
    /// Set the horizontal alignment of the constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn x_align<A: Into<Align>>(self, x_align: A) -> Self {
        Self {
            x_align: x_align.into(),
            ..self
        }
    }
    /// Set the vertical alignment of the constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn y_align<A: Into<Align>>(self, y_align: A) -> Self {
        Self {
            y_align: y_align.into(),
            ..self
        }
    }
    /// Set the margin of the constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn margin<M: Into<XMargin>>(self, margin: M) -> Self {
        Self {
            margin: margin.into(),
            ..self
        }
    }
    /// Set the hint range of the constraint to modify the layout process.
    /// See [`HintRange`] for details.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn hint(self, hint: HintRange) -> Self {
        Self { hint, ..self }
    }
    /// Set the minimum hint for the constraint to ensure that the resulting segment
    /// is at least this big, if possible.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_min<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            hint: self.hint.with_min(hint),
            ..self
        }
    }
    /// Set the preferred hint for the constraint to tell layout to aim for this size,
    /// if possible.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn preferred<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            hint: self.hint.preferred(hint),
            ..self
        }
    }
    /// Set the maximum hint for the constraint to prevent the segment from growing
    /// large than this, unless required to fill the area.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_max<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            hint: self.hint.with_max(hint),
            ..self
        }
    }
    /// Set the [`HintRange::fill_scale`] for this constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn scale(self, scale: u16) -> Self {
        Self {
            hint: self.hint.scale(scale),
            ..self
        }
    }
    /// Set the [`HintRange::priority`] for this constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn priority(self, priority: i16) -> Self {
        Self {
            hint: self.hint.priority(priority),
            ..self
        }
    }
    /// Set the [`SizeRange`] for this constraint.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn size<A: Into<SizeRange>>(self, range: A) -> Self {
        Self {
            size: range.into(),
            ..self
        }
    }
    /// Set the min value for the [`SizeRange`] of this constraint.
    /// Use [`XConstraint::size`] to set all three range levels at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_min(self, size: Size) -> Self {
        Self {
            size: self.size.size_min(size),
            ..self
        }
    }
    /// Set the preferred value for the [`SizeRange`] of this constraint.
    /// Use [`XConstraint::size`] to set all three range levels at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_preferred(self, size: Size) -> Self {
        Self {
            size: self.size.size_preferred(size),
            ..self
        }
    }
    /// Set the max value for the [`SizeRange`] of this constraint.
    /// Use [`XConstraint::size`] to set all three range levels at once.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_max(self, size: Size) -> Self {
        Self {
            size: self.size.size_max(size),
            ..self
        }
    }
    /// Given an allocated area for this constraint, find the segment [`Rect`] within the allocated
    /// `Rect`, based on margin, the [`Align`], and preferred size of this constraint.
    /// This is used for the final step in the layout process.
    pub fn inner_rect(&self, area: Rect) -> Rect {
        let area = area - self.margin;
        let x_align = self
            .x_align
            .align(self.size.horizontal.preferred, area.width);
        let y_align = self
            .y_align
            .align(self.size.vertical.preferred, area.height);
        Rect {
            x: area.x.saturating_add(x_align.position),
            y: area.y.saturating_add(y_align.position),
            width: x_align.size,
            height: y_align.size,
        }
    }
}

/// A margin specifies the empty space that should surround a layout segment.
/// An extended margin includes separate values for each side of the [`Rect`],
/// allowing the top margin to be different from the bottom margin, and the left
/// to be different from the right.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XMargin {
    /// The amount of empty space above the segment.
    pub top: u16,
    /// The amount of empty space below the segment.
    pub bottom: u16,
    /// The amount of empty space left of the segment.
    pub left: u16,
    /// The amount of empty space right of the segment.
    pub right: u16,
}

impl Debug for XMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.top == self.bottom && self.left == self.right {
            if self.top == self.left {
                write!(f, "XMargin::uniform({})", self.top)
            } else {
                write!(f, "XMargin::new({}, {})", self.left, self.top)
            }
        } else {
            write!(
                f,
                "XMargin{{top:{}, bottom:{}, left:{}, right:{}}}",
                self.top, self.bottom, self.left, self.right
            )
        }
    }
}

impl From<u16> for XMargin {
    fn from(value: u16) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

impl From<Margin> for XMargin {
    fn from(value: Margin) -> Self {
        Self::new(value.horizontal, value.vertical)
    }
}

impl Sub<XMargin> for Rect {
    type Output = Self;

    fn sub(self, rhs: XMargin) -> Self::Output {
        rhs.inner_rect(self)
    }
}

impl SubAssign<XMargin> for Rect {
    fn sub_assign(&mut self, rhs: XMargin) {
        *self = rhs.inner_rect(*self);
    }
}

impl Sub<Margin> for Rect {
    type Output = Self;

    fn sub(self, rhs: Margin) -> Self::Output {
        self.inner(rhs)
    }
}

impl SubAssign<Margin> for Rect {
    fn sub_assign(&mut self, rhs: Margin) {
        *self = self.inner(rhs);
    }
}

impl XMargin {
    /// A margin of zero size, specifying no empty space.
    pub const ZERO: Self = Self::uniform(0);
    /// Create a margin with the space horizontally and vertically,
    /// with equal space left and right, and equal space top and bottom.
    pub const fn new(horizontal: u16, vertical: u16) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
    /// Create a margin with equal space on all sides.
    pub const fn uniform(thickness: u16) -> Self {
        Self {
            top: thickness,
            bottom: thickness,
            left: thickness,
            right: thickness,
        }
    }
    /// Returns a new `Rect` inside the current one, with the given margin on each side.
    ///
    /// If the margin is larger than the given `Rect`, the [`Rect::ZERO`] will be returned.
    #[must_use = "method returns the modified value"]
    pub const fn inner_rect(self, rect: Rect) -> Rect {
        let doubled_margin_horizontal = self.left.saturating_add(self.right);
        let doubled_margin_vertical = self.top.saturating_add(self.bottom);

        if rect.width < doubled_margin_horizontal || rect.height < doubled_margin_vertical {
            Rect::ZERO
        } else {
            Rect {
                x: rect.x.saturating_add(self.left),
                y: rect.y.saturating_add(self.top),
                width: rect.width.saturating_sub(doubled_margin_horizontal),
                height: rect.height.saturating_sub(doubled_margin_vertical),
            }
        }
    }
    /// Set the empty space in this margin along the given axis.
    pub fn set_linear<M: Into<LinearMargin>>(&mut self, direction: Direction, value: M) {
        let margin = value.into();
        match direction {
            Direction::Horizontal => {
                self.left = margin.start;
                self.right = margin.end;
            }
            Direction::Vertical => {
                self.top = margin.start;
                self.bottom = margin.end;
            }
        }
    }
    /// The empty space in this margin along the given axis.
    pub const fn linear(self, direction: Direction) -> LinearMargin {
        match direction {
            Direction::Horizontal => LinearMargin {
                start: self.left,
                end: self.right,
            },
            Direction::Vertical => LinearMargin {
                start: self.top,
                end: self.bottom,
            },
        }
    }
}

/// A position and size of a segment within an allocated area along one axis.
struct AlignmentPos {
    /// The distance from the left side or the top of the allocated area, depending on axis.
    pub position: u16,
    /// The width or height of the segment, depending on axis.
    pub size: u16,
}

impl AlignmentPos {
    /// Construct an alignment position from the given position and size.
    const fn new(position: u16, size: u16) -> Self {
        Self { position, size }
    }
}

/// The alignment of an `XConstraint`, much like [`HorizontalAlignment`] and [`VerticalAlignment`],
/// but it also includes a [`Align::Full`] variant for stretching the segment to fill its allocated
/// space. `Full` is the default value.
#[derive(Debug, Display, Default, Clone, Copy, Eq, PartialEq, Hash, EnumIs, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Align {
    /// Causes the segment to fill the entire allocated area, and reduce the hint
    /// for the max size of the allocated area to be no more than the max size of the segment.
    /// If the allocated area still exceeds the max size of the segment despite the hint, then
    /// the segment will still fill the entire allocated area.
    #[default]
    Full,
    /// Causes the segment to be at the left side or the top of the allocated area, depending on
    /// whether the alignment is horizontal or vertical.
    Start,
    /// Causes the segment to be centered in the allocated area.
    Center,
    /// Causes the segment to be at the right side or the bottom of the allocated area, depending on
    /// whether the alignment is horizontal or vertical.
    End,
}

impl From<HorizontalAlignment> for Align {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::Left => Self::Start,
            HorizontalAlignment::Center => Self::Center,
            HorizontalAlignment::Right => Self::End,
        }
    }
}

impl From<VerticalAlignment> for Align {
    fn from(value: VerticalAlignment) -> Self {
        match value {
            VerticalAlignment::Top => Self::Start,
            VerticalAlignment::Center => Self::Center,
            VerticalAlignment::Bottom => Self::End,
        }
    }
}

impl Align {
    /// Determines the position of a segment with a given preferred size within an allocated area
    /// with the given available space along one axis according to this alignment.
    fn align(self, preferred_size: u16, available_space: u16) -> AlignmentPos {
        let size = preferred_size.min(available_space);
        match self {
            Self::Full => AlignmentPos::new(0, available_space),
            Self::Start => AlignmentPos::new(0, size),
            Self::End => AlignmentPos::new(available_space.saturating_sub(size), size),
            Self::Center => {
                let x = available_space.saturating_sub(size) / 2;
                AlignmentPos::new(x, size)
            }
        }
    }
}

/// Specifies a range of sizes for a segment to control the behavior of an [`XConstraint`].
/// along one axis. See [`SizeRange`] for details of how it is used in a constraint.
/// The three values in this range can be indexed by [`RangeLevel`].
///
/// A `LinearSizeRange` can be converted from various types:
/// * `u16`: `n` becomes `LinearSizeRange {min: n, preferred: n, max: u16::MAX}`
/// * `(u16,u16)`: `(a, b)` becomes `LinearSizeRange {min: a, preferred: b, max: u16::MAX}`
/// * `(u16,u16,u16)`: `(a,b,c)` becomes `LinearSizeRange {min: a, preferred: b, max: c}`
/// * `a..=b` becomes `LinearSizeRange {min: a, preferred: b, max: b}`
///
/// In general if the max value is not specified then it is assumed to be `u16::MAX`,
/// because usually things do not have a maximum size.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearSizeRange {
    /// The segment should be at least this large, if possible.
    /// The `min` hint for the constraint will be increased at least this large.
    pub min: u16,
    /// Specifies the size of the segment if the allocated area is large
    /// enough to allow it and the alignment is not set to [`Align::Full`] along this axis.
    /// Regardless of how much larger an area is allocated to the segment, the segment will be this
    /// size.
    pub preferred: u16,
    /// Specifies how large the segment should be at most. This has no effect
    /// unless the constraint's alignment is set to [`Align::Full`], which forces the segment
    /// to fill the entire allocated area, and reduces the `max` hint for the segment to be
    /// no more than this large.
    pub max: u16,
}

impl Debug for LinearSizeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.min, f)?;
        f.write_char('/')?;
        if self.preferred == u16::MAX {
            f.write_str("max")?;
        } else {
            Debug::fmt(&self.preferred, f)?;
        }
        if self.max < u16::MAX {
            f.write_char('/')?;
            Debug::fmt(&self.max, f)?;
        }
        Ok(())
    }
}

impl Default for LinearSizeRange {
    fn default() -> Self {
        Self::FULL
    }
}

impl From<u16> for LinearSizeRange {
    fn from(value: u16) -> Self {
        Self::new(value, value, u16::MAX)
    }
}
impl From<(u16, u16)> for LinearSizeRange {
    fn from((min, pref): (u16, u16)) -> Self {
        Self::new(min, pref, u16::MAX)
    }
}
impl From<(u16, u16, u16)> for LinearSizeRange {
    fn from((min, pref, max): (u16, u16, u16)) -> Self {
        Self::new(min, pref, max)
    }
}
impl From<i32> for LinearSizeRange {
    fn from(value: i32) -> Self {
        let value = u16::try_from(value).unwrap_or(0);
        Self::new(value, value, u16::MAX)
    }
}
impl From<(i32, i32)> for LinearSizeRange {
    fn from((min, pref): (i32, i32)) -> Self {
        let min = u16::try_from(min).unwrap_or(0);
        let pref = u16::try_from(pref).unwrap_or(0);
        Self::new(min, pref, u16::MAX)
    }
}
impl From<(i32, i32, i32)> for LinearSizeRange {
    fn from((min, pref, max): (i32, i32, i32)) -> Self {
        let min = u16::try_from(min).unwrap_or(0);
        let pref = u16::try_from(pref).unwrap_or(0);
        let max = u16::try_from(max).unwrap_or(0);
        Self::new(min, pref, max)
    }
}

/// Generate `From` implementations for [`LinearSizeRange`] from types
/// that implement `RangeBounds<u16>`.
macro_rules! range_from_bounds {
    ($($t:ty),*) => {
        $(impl From<$t> for LinearSizeRange {
            fn from(value: $t) -> Self {
                Self::from_bounds(value)
            }
        })*
    }
}

range_from_bounds!(
    Range<u16>,
    RangeFrom<u16>,
    RangeTo<u16>,
    RangeInclusive<u16>,
    RangeToInclusive<u16>,
    RangeFull
);

impl Index<RangeLevel> for LinearSizeRange {
    type Output = u16;

    fn index(&self, index: RangeLevel) -> &Self::Output {
        match index {
            RangeLevel::Min => &self.min,
            RangeLevel::Preferred => &self.preferred,
            RangeLevel::Max => &self.max,
        }
    }
}

impl IndexMut<RangeLevel> for LinearSizeRange {
    fn index_mut(&mut self, index: RangeLevel) -> &mut Self::Output {
        match index {
            RangeLevel::Min => &mut self.min,
            RangeLevel::Preferred => &mut self.preferred,
            RangeLevel::Max => &mut self.max,
        }
    }
}

impl LinearSizeRange {
    /// A default size range for segments where all sizes are acceptable.
    /// It has a `min` of 0, a `preferred` of `MAX` and a `max` of `MAX` so that
    /// the size range will not affect how much area is allocated to the segment
    /// and the segment will fill whatever area is allocated to it.
    pub const FULL: Self = Self {
        min: 0,
        preferred: u16::MAX,
        max: u16::MAX,
    };
    /// Create a size range with the given min, preferred size, and max size.
    pub const fn new(min: u16, preferred: u16, max: u16) -> Self {
        Self {
            min,
            preferred,
            max,
        }
    }
    /// Creates a range from the given bounds, with the preferred size defaulting
    /// to the maximum size.
    pub fn from_bounds<B: RangeBounds<u16>>(bounds: B) -> Self {
        let min = match bounds.start_bound() {
            core::ops::Bound::Included(x) => *x,
            core::ops::Bound::Excluded(x) => x.saturating_add(1),
            core::ops::Bound::Unbounded => 0,
        };
        let max = match bounds.end_bound() {
            core::ops::Bound::Included(x) => *x,
            core::ops::Bound::Excluded(x) => x.saturating_sub(1),
            core::ops::Bound::Unbounded => u16::MAX,
        };
        Self::new(min, max, max)
    }
    /// Modify the minimum value of the range.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_min(self, min: u16) -> Self {
        Self { min, ..self }
    }
    /// Modify the preferred value of the range.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn preferred(self, preferred: u16) -> Self {
        Self { preferred, ..self }
    }
    /// Modify the maximum value of the range.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_max(self, max: u16) -> Self {
        Self { max, ..self }
    }
}

/// An `SizeRange` represents a range of sizes for a layout segment,
/// with `min`, `preferred`, and `max` sizes on both the horizontal and vertical
/// axis.
/// * `min`: says that the segment should be at least this large, if possible. If `min` is greater
///   than zero, the min hint for the constraint will be increased at least this large.
/// * `preferred`: specifies the size of the segment if the allocated area is large enough to allow
///   it and the alignment is not set to [`Align::Full`]. Regardless of how much larger an area is
///   allocated to the segment, the segment will be this size.
/// * `max`: specifies how large the segment should be at most. This has no effect unless the
///   constraint's alignment is set to [`Align::Full`], which forces the segment to fill the entire
///   allocated area, and reduces the `max` hint for the segment to be no more than this large.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizeRange {
    /// The range of sizes along the horizontal axis.
    pub horizontal: LinearSizeRange,
    /// The range of sizes along the vertical axis.
    pub vertical: LinearSizeRange,
}

impl Debug for SizeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}, {:?})", self.horizontal, self.vertical)
    }
}

impl<R: Into<LinearSizeRange>, S: Into<LinearSizeRange>> From<(R, S)> for SizeRange {
    fn from((width, height): (R, S)) -> Self {
        Self::new(width, height)
    }
}

impl From<Size> for SizeRange {
    fn from(value: Size) -> Self {
        Self::new(value.width, value.height)
    }
}

impl From<&Size> for SizeRange {
    fn from(value: &Size) -> Self {
        Self::new(value.width, value.height)
    }
}

impl SizeRange {
    /// Size range that covers sizes from 0 x 0 to `u16::MAX` x `u16::MAX`.
    /// The preferred size is set to `u16::MAX` x `u16::MAX`.
    pub const FULL: Self = Self {
        horizontal: LinearSizeRange::FULL,
        vertical: LinearSizeRange::FULL,
    };
    /// Create a size range from the given horizontal and vertical ranges.
    ///
    /// # Example Usage
    /// ```
    /// use ratatui_core::layout::{SizeRange, XLayout};
    /// // A layout that centers a 10x6 rect within its given area.
    /// let layout = XLayout::from([SizeRange::new(10, 6)]);
    /// // It can also be written as:
    /// let layout = XLayout::from([(10, 6)]);
    /// ```
    pub fn new<R: Into<LinearSizeRange>, S: Into<LinearSizeRange>>(
        horizontal: R,
        vertical: S,
    ) -> Self {
        Self {
            horizontal: horizontal.into(),
            vertical: vertical.into(),
        }
    }
    /// The size at the given range level.
    pub fn get(&self, level: RangeLevel) -> Size {
        Size::new(self.horizontal[level], self.vertical[level])
    }
    /// Modify the range to have the given horizontal dimension.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn horizontal<R: Into<LinearSizeRange>>(self, range: R) -> Self {
        Self {
            horizontal: range.into(),
            ..self
        }
    }
    /// Modify the range to have the given vertical dimension.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn vertical<R: Into<LinearSizeRange>>(self, range: R) -> Self {
        Self {
            vertical: range.into(),
            ..self
        }
    }
    /// Modify the range to have the given minimum size. Use [`Size::ZERO`] if there
    /// should be no minimum size.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_min(self, size: Size) -> Self {
        Self {
            horizontal: self.horizontal.with_min(size.width),
            vertical: self.vertical.with_min(size.height),
        }
    }
    /// Modify the range to have the given preferred size.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_preferred(self, size: Size) -> Self {
        Self {
            horizontal: self.horizontal.preferred(size.width),
            vertical: self.vertical.preferred(size.height),
        }
    }
    /// Modify the range to have the given maximum size. Use [`Size::MAX`]
    /// if there should be no maximum size.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn size_max(self, size: Size) -> Self {
        Self {
            horizontal: self.horizontal.with_max(size.width),
            vertical: self.vertical.with_max(size.height),
        }
    }
    /// The size range along the given axis.
    pub const fn linear(&self, direction: Direction) -> LinearSizeRange {
        match direction {
            Direction::Horizontal => self.horizontal,
            Direction::Vertical => self.vertical,
        }
    }
    /// Create a constraint based on this size, with horizontal and vertical
    /// alignment set to [`Align::Center`], no margin, and [`HintRange::SIZE`].
    pub const fn into_constraint(self) -> XConstraint {
        XConstraint {
            margin: XMargin::ZERO,
            x_align: Align::Center,
            y_align: Align::Center,
            size: self,
            hint: HintRange::SIZE,
            is_separator: false,
        }
    }
}

/// A list of constraints that can be used to help build an [`XLayout`].
/// The `ConstraintList` can be extended using the + operator which allows
/// values of multiple types to be combined into a single list and automatically
/// converted into [`XConstraint`].
#[derive(Default)]
pub struct ConstraintList(Vec<XConstraint>);

impl From<ConstraintList> for Vec<XConstraint> {
    fn from(value: ConstraintList) -> Self {
        value.0
    }
}

impl IntoIterator for ConstraintList {
    type Item = XConstraint;
    type IntoIter = alloc::vec::IntoIter<XConstraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for ConstraintList {
    type Target = [XConstraint];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConstraintList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Generate `Add` implementations for types that can be added to the
/// beginning of a [`ConstraintList`] and implement `Into<XConstraint>`.
macro_rules! add_constraint_list_impl {
    ($($t:ty),*) => {
        $(impl Add<ConstraintList> for $t {
            type Output = ConstraintList;
            fn add(self, mut rhs: ConstraintList) -> ConstraintList {
                rhs.insert(0, self);
                rhs
            }
        })*
    }
}

add_constraint_list_impl!(u16, Hint, HintRange, SizeRange, Size, Constraint);

impl<C: Into<XConstraint>> Add<C> for ConstraintList {
    type Output = Self;
    fn add(mut self, rhs: C) -> Self {
        self.push(rhs);
        self
    }
}
impl<C: Into<XConstraint>> AddAssign<C> for ConstraintList {
    fn add_assign(&mut self, rhs: C) {
        self.push(rhs);
    }
}

impl<C: Into<XConstraint>> FromIterator<C> for ConstraintList {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl Add for XConstraint {
    type Output = ConstraintList;
    fn add(self, rhs: Self) -> Self::Output {
        [self, rhs].into_iter().collect()
    }
}

impl Add for ConstraintList {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.append(&mut rhs);
        self
    }
}

impl Add<XLayout> for ConstraintList {
    type Output = XLayout;
    fn add(mut self, mut rhs: XLayout) -> Self::Output {
        core::mem::swap(&mut rhs.constraints, &mut self.0);
        rhs.constraints.append(&mut self.0);
        rhs
    }
}

impl ConstraintList {
    /// Create an empty list.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    /// Create a list from an iterable collection of values.
    pub fn from<I>(constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        Self(constraints.into_iter().map(Into::into).collect())
    }
    /// Create a list containing a single constraint.
    pub fn single<C: Into<XConstraint>>(constraint: C) -> Self {
        let mut list = Self::new();
        list.push(constraint.into());
        list
    }
    /// Add a constraint to the end of the list.
    pub fn push<C: Into<XConstraint>>(&mut self, constraint: C) {
        self.0.push(constraint.into());
    }
    /// Insert a constraint into the list at the given position.
    pub fn insert<C: Into<XConstraint>>(&mut self, index: usize, constraint: C) {
        self.0.insert(index, constraint.into());
    }
    /// Add multiple constraints to the end of the list.
    pub fn extend<I>(&mut self, constraints: I)
    where
        I: IntoIterator,
        I::Item: Into<XConstraint>,
    {
        self.0.extend(constraints.into_iter().map(Into::into));
    }
    /// Combine two constraint lists together. This constraint list is extended,
    /// and the given constraint list is drained and left empty.
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(
            LinearSizeRange::from(1),
            LinearSizeRange::new(1, 1, u16::MAX)
        );
        assert_eq!(LinearSizeRange::from(1..=1), LinearSizeRange::new(1, 1, 1));
        assert_eq!(LinearSizeRange::from(1..2), LinearSizeRange::new(1, 1, 1));
        assert_eq!(
            LinearSizeRange::from((3, 14)),
            LinearSizeRange::new(3, 14, u16::MAX)
        );
        assert_eq!(
            LinearSizeRange::from((3, 14, 17)),
            LinearSizeRange::new(3, 14, 17)
        );
        assert_eq!(
            SizeRange::from((1, 2)),
            SizeRange::new(
                LinearSizeRange::new(1, 1, u16::MAX),
                LinearSizeRange::new(2, 2, u16::MAX)
            )
        );
        assert_eq!(
            SizeRange::from(Size::new(1, 2)),
            SizeRange::new(
                LinearSizeRange::new(1, 1, u16::MAX),
                LinearSizeRange::new(2, 2, u16::MAX)
            )
        );
        assert_eq!(
            SizeRange::from((1..=3, 2..=5)),
            SizeRange::new(LinearSizeRange::new(1, 3, 3), LinearSizeRange::new(2, 5, 5))
        );
    }
}
