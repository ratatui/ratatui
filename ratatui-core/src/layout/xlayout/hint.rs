//! The [`Hint`] and [`HintRange`] types which are used to control how an [`XConstraint`] is
//! used to allocate space for a segment.

use core::ops::{
    Index, IndexMut, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};

use super::{Constraint, Debug, Display, EnumIs, Formatter, RangeLevel, Spacing, div_round};
use crate::layout::{Direction, XConstraint};

/// A hint represents a suggestion to the layout algorithm for how large some segment should be.
/// There are various ways to represent the suggested size of a segment, either in an absolute
/// measurement or relative to the size of the area that is to be split.
/// The default hint is `Hint::Percentage(100)`, meaning that the segment should fill the
/// entire area, 100%.
#[derive(Clone, Copy, Eq, PartialEq, Hash, EnumIs)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Hint {
    /// The absolute size of the segment, regardless of the size of the area to be split.
    Length(u16),
    /// The size of the segment as a percentage of the total size of the area to be split.
    Percentage(u16),
    /// The size of the segment as a ratio of the total size of the area to be split.
    /// For example, `Ratio(1,2)` means that the segment should be allocated one third of the
    /// total size of the area.
    Ratio(u32, u32),
    /// The size of the segment should be negative by the given absolute amount.
    /// If a segment is allocated a negative area, then its rectangle will have zero size
    /// and its neighboring segments will overlap each other, or even leave the bounds of
    /// the area that is being split.
    Overlap(u16),
    /// The size of the segment should be determed by the [`SizeRange`](super::SizeRange) of the
    /// constraint. It is as if it were `Hint::Length(x)` where `x` is the size of the
    /// constraint's area range at the given level along the layout axis.
    Size(RangeLevel),
}

impl Display for Hint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for Hint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Length(v) => Debug::fmt(v, f),
            Self::Percentage(v) => write!(f, "{v}%"),
            Self::Ratio(a, b) => write!(f, "{a}:{b}"),
            Self::Overlap(v) => write!(f, "-{v}"),
            Self::Size(level) => write!(f, "Area({level:?})"),
        }
    }
}

impl Hint {
    /// A hint indicating that a segment should have zero size.
    pub const ZERO: Self = Self::Length(0);
    /// A hint indicating that a segment should fill the entire area.
    pub const FULL: Self = Self::Percentage(100);
    /// Find the absolute size of the hint when splitting an area of the given size.
    /// For example, `Percentage(50).absolute_for(8)` would be 4, because 4 is 50% of 8.
    /// `Ratio(2,3).absolute_for(12)` would be 8, because 8 is 2/3 of 12.
    pub(super) fn absolute_for(
        self,
        constraint: &XConstraint,
        direction: Direction,
        area_size: u16,
    ) -> i32 {
        let size = u64::from(area_size);
        match self {
            Self::Length(v) => v.into(),
            Self::Percentage(100) => area_size.into(),
            Self::Percentage(v) => {
                i32::try_from(div_round(u64::from(v).saturating_mul(size), 100)).unwrap_or(i32::MAX)
            }
            //.max(1),
            Self::Ratio(a, 0) => i32::try_from(a).unwrap_or(i32::MAX),
            Self::Ratio(0, _) => 0,
            Self::Ratio(a, b) => {
                i32::try_from(div_round(u64::from(a).saturating_mul(size), u64::from(b)))
                    .unwrap_or(i32::MAX)
                //.max(1)
            }
            Self::Overlap(v) => -i32::from(v),
            Self::Size(level) => constraint.size.linear(direction)[level].into(),
        }
    }
}

impl Default for Hint {
    fn default() -> Self {
        Self::Percentage(100)
    }
}

impl From<Spacing> for Hint {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Space(v) => Self::Length(v),
            Spacing::Overlap(v) => Self::Overlap(v),
        }
    }
}

impl From<u16> for Hint {
    fn from(value: u16) -> Self {
        Self::Length(value)
    }
}

impl From<(u32, u32)> for Hint {
    fn from((a, b): (u32, u32)) -> Self {
        Self::Ratio(a, b)
    }
}

/// A range of layout hints including min, preferred, and max, along with other
/// modifiers to how the segment should be allocated space within an area.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HintRange {
    /// A hint for the minimum amount to allocate. During layout this may be increased
    /// if constraint has an [`XConstraint::size`] that requires a certain minimum size.
    /// During layout, segments with lower `priority` are allocated their minimum size before
    /// segments with greater `priority`.
    pub min: Hint,
    /// A hint for the preferred amount to allocate to the segment.
    /// During layout, segments with lower `priority` are allocated their preferred size before
    /// segments with greater `priority`.
    pub preferred: Hint,
    /// A hint for the maximum acceptable size for the segment's allocated area. This hint may be
    /// exceeded if all segments reach their maximum and yet there is still area to be filled.
    /// Segments with greater `priority` are allocated their preferred size before segments with
    /// lower `priority`. This is reversed from how priority is handled for `min` and
    /// `preferred`, meaning that low `priority` values give a segment greater chance to get to
    /// its preferred size and stay there during the layout process.
    pub max: Hint,
    /// During layout, the amount allocated to the segment will be multiplied by its `fill_scale`,
    /// causing some segments to grow faster than others. This has no effect if all segments
    /// reach the sizes they are growing toward, but if the area is not large enough to give
    /// every segment its desired size, then `fill_scale` allows some segments to get an
    /// unequal share of what is available.
    ///
    /// This is akin to `priority` in that it allows some segments to have a better chance to be
    /// allocated space, but priority causes segments to take turns in allocation, so for
    /// example a segment with a low priority value would be allocated its full minimum size
    /// before any other segments even start allocating their minimum sizes. While `fill_scale`
    /// causes segments to share the allocation unequally, `priority` causes segments to not share
    /// at all.
    ///
    /// If `fill_scale` is zero, then the segment cannot ever grow, bypassing the layout process.
    /// The segment is given its minimum hint size indiscriminantly, even if that is larger
    /// than the `Rect` that is being split.
    pub fill_scale: u16,
    /// After the stage where all segments grow to their `max` hint size, if there is still space
    /// in the area to be filled, that space is required to be filled. If `overfill` is true,
    /// then this segment is offering to be among the segments that will be filled beyond its
    /// maximum. Any segment with `overfill` set to false will be ignored for the remainder of
    /// the layout process, unless all segments have `overfill` set to false, in which case all
    /// segments will grow. The default value is false.
    pub overfill: bool,
    /// By default all segments will simultaneously allocate space within the area for each stage
    /// of their growth, first growing to their minimum size, then all growing to their
    /// preferred size, then all to their maximum size, but `priority` makes it possible to
    /// choose the order in which allocation happens, so some segments take their share
    /// before other segments, leaving the other segments to allocate only what is left over.
    ///
    /// During the min stage and the preferred stage, lower priority segments allocate first to
    /// give them a better chance of reaching their min/preferred size. During later stages the
    /// priority is reversed, so that lower priority segments allocate last, giving them the
    /// best chance to remain at their preferred size and not be forced to grow.
    ///
    /// This is akin to `fill_scale` in that it allows some segments to have a better chance to be
    /// allocated space, but fill scale causes segments to allocate faster within their turn,
    /// thereby taking more of the available space even among other segments of the same
    /// priority. Neither `fill_scale` nor `priority` has any effect if all the segments are
    /// able to reach their preferred size and none are force to grow to fill the area.
    pub priority: i16,
}

impl Debug for HintRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?}/{:?}/{:?} scale:{}",
            self.min, self.preferred, self.max, self.fill_scale
        )?;
        if self.overfill {
            f.write_str(" overfill")?;
        }
        write!(f, " priority:{}", self.priority)
    }
}

impl Index<RangeLevel> for HintRange {
    type Output = Hint;

    fn index(&self, index: RangeLevel) -> &Self::Output {
        match index {
            RangeLevel::Min => &self.min,
            RangeLevel::Preferred => &self.preferred,
            RangeLevel::Max => &self.max,
        }
    }
}

impl IndexMut<RangeLevel> for HintRange {
    fn index_mut(&mut self, index: RangeLevel) -> &mut Self::Output {
        match index {
            RangeLevel::Min => &mut self.min,
            RangeLevel::Preferred => &mut self.preferred,
            RangeLevel::Max => &mut self.max,
        }
    }
}

impl Default for HintRange {
    fn default() -> Self {
        Self::FULL
    }
}

/// Generate `From` implementations for [`HintRange`] from types that
/// implement [`RangeBounds<u16>`].
macro_rules! hint_from_bounds {
    ($($t:ty),*) => {
        $(impl From<$t> for HintRange {
            fn from(value: $t) -> Self {
                Self::from_bounds(value)
            }
        })*
    }
}

hint_from_bounds!(
    Range<u16>,
    RangeFrom<u16>,
    RangeTo<u16>,
    RangeInclusive<u16>,
    RangeToInclusive<u16>,
    RangeFull
);

impl HintRange {
    /// The default hint range that allows the segment to have zero size,
    /// or grow to fill the entire area. `preferred` is `Hint::FULL`,
    /// `fill_scale` is 1, `overfill` is false, and `priority` is 0.
    pub const FULL: Self = Self {
        min: Hint::ZERO,
        preferred: Hint::FULL,
        max: Hint::FULL,
        fill_scale: 1,
        overfill: false,
        priority: 0,
    };
    /// A hint range that has zero for min, preferred, and max.
    /// All the rest is the same as [`HintRange::FULL`].
    pub const ZERO: Self = Self {
        min: Hint::ZERO,
        preferred: Hint::ZERO,
        max: Hint::ZERO,
        ..Self::FULL
    };
    /// The default hint range for constraints that converted from sizes,
    /// using [`Hint::Size`] to constrain the segment based on the size along the layout axis.
    /// `min`, `preferred`, `max` come from the size.
    /// `fill_scale` is 1, `overfill` is false, and `priority` is 0.
    pub const SIZE: Self = Self {
        min: Hint::Size(RangeLevel::Min),
        preferred: Hint::Size(RangeLevel::Preferred),
        max: Hint::Size(RangeLevel::Max),
        ..Self::FULL
    };
    /// Create an [`XConstraint`] from this hint range by taking
    /// [`XConstraint::FULL`] and replacing its hint range with this hint range.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn into_constraint(self) -> XConstraint {
        XConstraint {
            hint: self,
            ..XConstraint::FULL
        }
    }
    /// Create a [`HintRange::FULL`] except min, preferred, and max are set to the given hint.
    pub const fn from_hint(hint: Hint) -> Self {
        Self {
            min: hint,
            preferred: hint,
            max: hint,
            ..Self::FULL
        }
    }
    /// Construct a hint range from the given bounds.
    /// The low end of the bounds become a [`Hint::Length`] value for min.
    /// The high end of the bounds become a [`Hint::Length`] value for both
    /// preferred and max.
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
        Self {
            min: Hint::Length(min),
            preferred: Hint::Length(max),
            max: Hint::Length(max),
            ..Self::FULL
        }
    }
    /// Create a `HintRange` that directs the constraint to fill whatever space
    /// is leftover after other segments have reached their preferred sizes.
    /// Its `fill_scale` is set to the given value to determine how large it will grow
    /// relative to other filler constraints.
    /// The min, preferred, are set to [`Hint::ZERO`] so that it will
    /// not grow until max phase of layout, and max is set to [`Hint::FULL`] so it will
    /// keep growing until no space is left. `priority` is set to 100 to encourage it to
    /// grow before any other segments are stretched toward their own max.
    pub const fn filler(fill_scale: u16) -> Self {
        Self::from_hint(Hint::ZERO)
            .without_max()
            .scale(fill_scale)
            .priority(100)
    }
    /// Set the `min`, `preferred`, and `max` to be `Hint::Area(RangeLevel::Min)`,
    /// `Hint::Area(RangeLevel::Preferred)` and `Hint::Area(RangeLevel::Max)`, so that
    /// this hint range directs its constraint to try to satisfy the requirements of its
    /// area. These are the default values when an [`SizeRange`](super::SizeRange) is converted
    /// into an [`XConstraint`].
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn hint_size(self) -> Self {
        self.with_min(Hint::Size(RangeLevel::Min))
            .preferred(Hint::Size(RangeLevel::Preferred))
            .with_max(Hint::Size(RangeLevel::Max))
    }
    /// Set the hint for the minimum amount to allocate. During layout this may be increased
    /// if constraint has an [`XConstraint::size`] that requires a certain minimum size.
    /// During layout, segments with lower `priority` are allocated their minimum size before
    /// segments with greater `priority`.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_min<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            min: hint.into(),
            ..self
        }
    }
    /// Set the hint for the preferred amount to allocate to the segment.
    /// During layout, segments with lower `priority` are allocated their preferred size before
    /// segments with greater `priority`.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn preferred<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            preferred: hint.into(),
            ..self
        }
    }
    /// Set the hint for the maximum acceptable size for the segment's allocated area. This hint may
    /// be exceeded if all segments reach their maximum and yet there is still area to be
    /// filled. Segments with greater `priority` are allocated their preferred size before
    /// segments with lower `priority`. This is reversed from how priority is handled for `min`
    /// and `preferred`, meaning that low `priority` values give a segment greater chance to get
    /// to its preferred size and stay there during the layout process.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_max<H: Into<Hint>>(self, hint: H) -> Self {
        Self {
            max: hint.into(),
            ..self
        }
    }
    /// Set the `max` hint to [`Hint::FULL`] which is the full size of the area to be split,
    /// which effectively means that there is no maximum.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn without_max(self) -> Self {
        Self {
            max: Hint::FULL,
            ..self
        }
    }
    /// During layout, the amount allocated to the segment will be multiplied by its `fill_scale`,
    /// causing some segments to grow faster than others. This has no effect if all segments
    /// reach the sizes they are growing toward, but if the area is not large enough to give
    /// every segment its desired size, then `fill_scale` allows some segments to get an unequal
    /// share of what is available.
    ///
    /// This is akin to `priority` in that it allows some segments to have a better chance to be
    /// allocated space, but priority causes segments to take turns in allocation, so for
    /// example a segment with a low priority value would be allocated its full minimum size
    /// before any other segments even start allocating their minimum sizes. While `fill_scale`
    /// causes segments to share the allocation unequally, `priority` causes segments to not share
    /// at all.
    ///
    /// If `fill_scale` is zero, then the segment cannot ever grow, bypassing the layout process.
    /// The segment is given its minimum hint size indiscriminantly, even if that is larger than
    /// the `Rect` that is being split.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn scale(self, fill_scale: u16) -> Self {
        Self { fill_scale, ..self }
    }
    /// By default all segments will simultaneously allocate space within the area for each stage of
    /// their growth, first growing to their minimum size, then all growing to their preferred
    /// size, then all to their maximum size, but `priority` makes it possible to choose the
    /// order in which allocation happens, so some segments take their share before other
    /// segments, leaving the other segments to allocate only what is left over.
    ///
    /// During the min stage and the preferred stage, lower priority segments allocate first to give
    /// them a better chance of reaching their min/preferred size. During later stages the
    /// priority is reversed, so that lower priority segments allocate last, giving them the
    /// best chance to remain at their preferred size and not be forced to grow.
    ///
    /// This is akin to `fill_scale` in that it allows some segments to have a better chance to be
    /// allocated space, but fill scale causes segments to allocate faster within their turn,
    /// thereby taking more of the available space even among other segments of the same
    /// priority. Neither `fill_scale` nor `priority` has any effect if all the segments are
    /// able to reach their preferred size and none are force to grow to fill the area.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn priority(self, priority: i16) -> Self {
        Self { priority, ..self }
    }
    /// After the stage where all segments grow to their `max` hint size, if there is still space in
    /// the area to be filled, that space is required to be filled. If `overfill` is true, then
    /// this segment is offering to be among the segments that will be filled beyond its
    /// maximum. Any segment with `overfill` set to false will be ignored for the remainder of
    /// the layout process, unless all segments have `overfill` set to false, in which case all
    /// segments will grow. The default value is false.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn overfill(self, overfill: bool) -> Self {
        Self { overfill, ..self }
    }
}

impl<V: Into<Hint>> From<V> for HintRange {
    fn from(value: V) -> Self {
        Self::from_hint(value.into())
    }
}

impl From<Constraint> for HintRange {
    fn from(value: Constraint) -> Self {
        match value {
            Constraint::Min(v) => Self {
                min: v.into(),
                //preferred: v.into(),
                priority: 100,
                ..Self::FULL
            },
            Constraint::Max(v) => Self {
                preferred: v.into(),
                max: v.into(),
                priority: 100,
                ..Self::FULL
            },
            Constraint::Length(v) => Self {
                preferred: v.into(),
                max: v.into(),
                ..Self::FULL
            },
            Constraint::Percentage(v) => Self {
                preferred: Hint::Percentage(v),
                max: Hint::Percentage(v),
                ..Self::FULL
            },
            Constraint::Ratio(a, b) => Self {
                preferred: Hint::Ratio(a, b),
                max: Hint::Ratio(a, b),
                ..Self::FULL
            },
            Constraint::Fill(v) => Self {
                fill_scale: v,
                priority: i16::MAX,
                ..Self::FULL
            },
        }
    }
}
