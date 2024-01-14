use std::{cell::RefCell, collections::HashMap, num::NonZeroUsize, rc::Rc, sync::OnceLock};

use cassowary::{
    strength::{MEDIUM, REQUIRED, STRONG, WEAK},
    AddConstraintError, Expression, Solver, Variable,
    WeightedRelation::{EQ, GE, LE},
};
use itertools::Itertools;
use lru::LruCache;

use super::{Flex, SegmentSize};
use crate::prelude::*;

type Cache = LruCache<(Rect, Layout), Rc<[Rect]>>;

thread_local! {
    static LAYOUT_CACHE: OnceLock<RefCell<Cache>> = OnceLock::new();
}

/// A layout is a set of constraints that can be applied to a given area to split it into smaller
/// ones.
///
/// A layout is composed of:
/// - a direction (horizontal or vertical)
/// - a set of constraints (length, ratio, percentage, min, max)
/// - a margin (horizontal and vertical), the space between the edge of the main area and the split
///   areas
/// - extra options for segment size preferences
///
/// The algorithm used to compute the layout is based on the [`cassowary-rs`] solver. It is a simple
/// linear solver that can be used to solve linear equations and inequalities. In our case, we
/// define a set of constraints that are applied to split the provided area into Rects aligned in a
/// single direction, and the solver computes the values of the position and sizes that satisfy as
/// many of the constraints as possible.
///
/// By default, the last chunk of the computed layout is expanded to fill the remaining space. To
/// avoid this behavior, add an unused `Constraint::Min(0)` as the last constraint. There is also
/// an unstable API to prefer equal chunks if other constraints are all satisfied, see
/// [`SegmentSize`] for more info.
///
/// When the layout is computed, the result is cached in a thread-local cache, so that subsequent
/// calls with the same parameters are faster. The cache is a simple HashMap, and grows
/// indefinitely. (See <https://github.com/ratatui-org/ratatui/issues/402> for more information)
///
/// # Constructors
///
/// There are four ways to create a new layout:
///
/// - [`Layout::default`]: create a new layout with default values
/// - [`Layout::new`]: create a new layout with a given direction and constraints
/// - [`Layout::vertical`]: create a new vertical layout with the given constraints
/// - [`Layout::horizontal`]: create a new horizontal layout with the given constraints
///
/// # Setters
///
/// There are several setters to modify the layout:
///
/// - [`Layout::direction`]: set the direction of the layout
/// - [`Layout::constraints`]: set the constraints of the layout
/// - [`Layout::margin`]: set the margin of the layout
/// - [`Layout::horizontal_margin`]: set the horizontal margin of the layout
/// - [`Layout::vertical_margin`]: set the vertical margin of the layout
/// - [`Layout::flex`]: set the way the space is distributed when the constraints are satisfied
///
/// # Example
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// fn render(frame: &mut Frame, area: Rect) {
///     let layout = Layout::new(
///         Direction::Vertical,
///         [Constraint::Length(5), Constraint::Min(0)],
///     )
///     .split(Rect::new(0, 0, 10, 10));
///     frame.render_widget(Paragraph::new("foo"), layout[0]);
///     frame.render_widget(Paragraph::new("bar"), layout[1]);
/// }
/// ```
///
/// The [`layout.rs` example](https://github.com/ratatui-org/ratatui/blob/main/examples/layout.rs)
/// shows the effect of combining constraints:
///
/// ![layout
/// example](https://camo.githubusercontent.com/77d22f3313b782a81e5e033ef82814bb48d786d2598699c27f8e757ccee62021/68747470733a2f2f7668732e636861726d2e73682f7668732d315a4e6f4e4c4e6c4c746b4a58706767396e435635652e676966)
///
/// [`cassowary-rs`]: https://crates.io/crates/cassowary
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: Margin,
    flex: Flex,
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
}

impl Layout {
    pub const DEFAULT_CACHE_SIZE: usize = 16;
    /// Creates a new layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// layout with fixed size chunks.
    ///
    /// Default values for the other fields are:
    ///
    /// - `margin`: 0, 0
    /// - `flex`: Flex::Fill
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// Layout::new(
    ///     Direction::Horizontal,
    ///     [Constraint::Length(5), Constraint::Min(0)],
    /// );
    ///
    /// Layout::new(
    ///     Direction::Vertical,
    ///     [1, 2, 3].iter().map(|&c| Constraint::Length(c)),
    /// );
    ///
    /// Layout::new(Direction::Horizontal, vec![1, 2]);
    /// ```
    pub fn new<I>(direction: Direction, constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        Layout {
            direction,
            margin: Margin::new(0, 0),
            constraints: constraints.into_iter().map(Into::into).collect(),
            flex: Flex::default(),
        }
    }

    /// Creates a new vertical layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);
    /// ```
    pub fn vertical<I>(constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        Layout::new(Direction::Vertical, constraints.into_iter().map(Into::into))
    }

    /// Creates a new horizontal layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::horizontal([Constraint::Length(5), Constraint::Min(0)]);
    /// ```
    pub fn horizontal<I>(constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        Layout::new(
            Direction::Horizontal,
            constraints.into_iter().map(Into::into),
        )
    }

    /// Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
    /// that subsequent calls with the same parameters are faster. The cache is a LruCache, and
    /// grows until `cache_size` is reached.
    ///
    /// Returns true if the cell's value was set by this call.
    /// Returns false if the cell's value was not set by this call, this means that another thread
    /// has set this value or that the cache size is already initialized.
    ///
    /// Note that a custom cache size will be set only if this function:
    /// * is called before [Layout::split()] otherwise, the cache size is
    ///   [`Self::DEFAULT_CACHE_SIZE`].
    /// * is called for the first time, subsequent calls do not modify the cache size.
    pub fn init_cache(cache_size: usize) -> bool {
        LAYOUT_CACHE
            .with(|c| {
                c.set(RefCell::new(LruCache::new(
                    NonZeroUsize::new(cache_size).unwrap(),
                )))
            })
            .is_ok()
    }

    /// Set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    /// Sets the constraints of the layout.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array or vec of u16 to this function to create a
    /// layout with fixed size chunks.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
    /// 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
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
    /// Layout::default().constraints([Constraint::Min(0)]);
    /// Layout::default().constraints(&[Constraint::Min(0)]);
    /// Layout::default().constraints(vec![Constraint::Min(0)]);
    /// Layout::default().constraints([Constraint::Min(0)].iter().filter(|_| true));
    /// Layout::default().constraints([1, 2, 3].iter().map(|&c| Constraint::Length(c)));
    /// Layout::default().constraints([1, 2, 3]);
    /// Layout::default().constraints(vec![1, 2, 3]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn constraints<I>(mut self, constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        self.constraints = constraints.into_iter().map(Into::into).collect();
        self
    }

    /// Set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    /// Set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .horizontal_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    /// Set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .vertical_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

    /// Sets flex options for justify content
    pub const fn flex(mut self, flex: Flex) -> Layout {
        self.flex = flex;
        self
    }

    /// Set whether chunks should be of equal size.
    ///
    /// This determines how the space is distributed when the constraints are satisfied. By default,
    /// the last chunk is expanded to fill the remaining space, but this can be changed to prefer
    /// equal chunks or to not distribute extra space at all (which is the default used for laying
    /// out the columns for [`Table`] widgets).
    ///
    /// This function exists for backwards compatibility reasons. Use [`Layout::flex`] instead.
    ///
    /// - `Flex::StretchLast` does now what `SegmentSize::LastTakesRemainder` did (default).
    /// - `Flex::Stretch` does now what `SegmentSize::EvenDistribution` did.
    /// - `Flex::Start` does now what `SegmentSize::None` did.
    #[stability::unstable(
        feature = "segment-size",
        reason = "The name for this feature is not final and may change in the future",
        issue = "https://github.com/ratatui-org/ratatui/issues/536"
    )]
    #[must_use = "method moves the value of self and returns the modified value"]
    #[deprecated(since = "0.26.0", note = "You should use `Layout::flex` instead.")]
    pub const fn segment_size(self, segment_size: SegmentSize) -> Layout {
        let flex = match segment_size {
            SegmentSize::None => Flex::Start,
            SegmentSize::LastTakesRemainder => Flex::StretchLast,
            SegmentSize::EvenDistribution => Flex::Stretch,
        };
        self.flex(flex)
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given area into
    /// smaller ones based on the preferred widths or heights and the direction.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in [20,
    /// 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).
    ///
    /// This method stores the result of the computation in a thread-local cache keyed on the layout
    /// and area, so that subsequent calls with the same parameters are faster. The cache is a
    /// LruCache, and grows until [`Self::DEFAULT_CACHE_SIZE`] is reached by default, if the cache
    /// is initialized with the [Layout::init_cache()] grows until the initialized cache size.
    ///
    /// There is a helper method on Rect that can be used to split the whole area into smaller ones
    /// based on the layout: [`Rect::split()`]. That method is a shortcut for calling this method.
    /// It allows you to destructure the result directly into variables, which is useful when you
    /// know at compile time the number of areas that will be created.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(2, 2, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    ///     .split(Rect::new(0, 0, 9, 2));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 3, 2), Rect::new(3, 0, 6, 2)]);
    /// ```
    pub fn split(&self, area: Rect) -> Rc<[Rect]> {
        LAYOUT_CACHE.with(|c| {
            c.get_or_init(|| {
                RefCell::new(LruCache::new(
                    NonZeroUsize::new(Self::DEFAULT_CACHE_SIZE).unwrap(),
                ))
            })
            .borrow_mut()
            .get_or_insert((area, self.clone()), || {
                Self::try_split(area, self).expect("failed to split")
            })
            .clone()
        })
    }

    fn try_split(area: Rect, layout: &Layout) -> Result<Rc<[Rect]>, AddConstraintError> {
        let mut solver = Solver::new();
        let inner = area.inner(&layout.margin);

        let (area_start, area_end) = match layout.direction {
            Direction::Horizontal => (f64::from(inner.x), f64::from(inner.right())),
            Direction::Vertical => (f64::from(inner.y), f64::from(inner.bottom())),
        };
        let area_size = area_end - area_start;

        // create an element for each constraint that needs to be applied. Each element defines the
        // variables that will be used to compute the layout.
        let elements: Vec<Element> = layout
            .constraints
            .iter()
            .map(|_| Element::constrain(&mut solver, (area_start, area_end)))
            .try_collect()?;

        // If there's just one constraint, it doesn't make sense to use `SpaceBetween`.
        // However, if the user chooses to use `SpaceBetween` we choose `Stretch` instead.
        //
        // Choosing `Stretch` will do this:
        //
        // <---~------80 px------~--->
        // ┌─~────────80 px────────~─┐
        // │         Max(20)         │
        // └─~─────────────────────~─┘
        //
        // In CSS the default when you use `flex` is justify to the start. So when there's just one
        // element that's what they do.
        //
        // For us, our default is `Stretch`.
        //
        // Additionally, there's two reasons I think `SpaceBetween` should be `Stretch`.
        //
        // 1. The way to think about it is that we are telling the solver that we want to add a
        //    spacer between adjacent elements but make the start of the first element at the start
        //    of the area and make the end of the last element at the end of the area. When there's
        //    just one element, there's no spacers added, and now the start and ends of the element
        //    should match the start and end of the area.
        // 2. This above point is exactly is what constraints are added in the `SpaceBetween` match
        //    but we are using `tuple_combinations` and `windows` so when there's just one element
        //    and no spacers, it doesn't do anything. If we make that code work for one element,
        //    it'll end up doing the same thing as `Stretch`.
        //
        // If we changed our default layout to use `Flex::Start`, there is a case to be made for
        // this to do `Flex::Start` as well.
        //
        // TODO: add test for this
        let flex = if layout.constraints.len() == 1 && layout.flex == Flex::SpaceBetween {
            Flex::Stretch
        } else {
            layout.flex
        };

        match flex {
            Flex::SpaceBetween => {
                let spacers: Vec<Element> = std::iter::repeat_with(|| {
                    Element::constrain(&mut solver, (area_start, area_end))
                })
                .take(elements.len().saturating_sub(1)) // one less than the number of elements
                .try_collect()?;
                // spacers growing should be the lowest priority
                for spacer in spacers.iter() {
                    solver.add_constraint(spacer.size() | EQ(WEAK) | area_size)?;
                }
                // Spacers should all be similar in size
                // these constraints should not be stronger than existing constraints
                // but if they are weaker `Min` and `Max` won't be pushed to their desired values
                // I found using `STRONG` gives the most desirable behavior
                for (left, right) in spacers.iter().tuple_combinations() {
                    solver.add_constraint(left.size() | EQ(STRONG) | right.size())?;
                }
                // interleave elements and spacers
                // for `SpaceBetween` we want the following
                // `[element, spacer, element, spacer, ..., element]`
                // this is why we use one less spacer than elements
                for pair in Itertools::interleave(elements.iter(), spacers.iter())
                    .collect::<Vec<&Element>>()
                    .windows(2)
                {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::SpaceAround => {
                let spacers: Vec<Element> = std::iter::repeat_with(|| {
                    Element::constrain(&mut solver, (area_start, area_end))
                })
                .take(elements.len().saturating_add(1)) // one more than number of elements
                .try_collect()?;
                // spacers growing should be the lowest priority
                for spacer in spacers.iter() {
                    solver.add_constraint(spacer.size() | EQ(WEAK) | area_size)?;
                }
                // Spacers should all be similar in size
                // these constraints should not be stronger than existing constraints
                // but if they are weaker `Min` and `Max` won't be pushed to their desired values
                // I found using `STRONG` gives the most desirable behavior
                for (left, right) in spacers.iter().tuple_combinations() {
                    solver.add_constraint(left.size() | EQ(STRONG) | right.size())?;
                }
                // interleave spacers and elements
                // for `SpaceAround` we want the following
                // `[spacer, element, spacer, element, ..., element, spacer]`
                // this is why we use one spacer than elements
                for pair in Itertools::interleave(spacers.iter(), elements.iter())
                    .collect::<Vec<&Element>>()
                    .windows(2)
                {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::StretchLast => {
                // this is the default behavior
                // within reason, cassowary tends to put excess into the last constraint
                if let Some(first) = elements.first() {
                    solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
                }
                if let Some(last) = elements.last() {
                    solver.add_constraint(last.end | EQ(REQUIRED) | area_end)?;
                }
                // ensure there are no gaps between the elements
                for pair in elements.windows(2) {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::Stretch => {
                if let Some(first) = elements.first() {
                    solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
                }
                if let Some(last) = elements.last() {
                    solver.add_constraint(last.end | EQ(REQUIRED) | area_end)?;
                }
                // prefer equal elements if other constraints are all satisfied
                for (left, right) in elements.iter().tuple_combinations() {
                    solver.add_constraint(left.size() | EQ(WEAK) | right.size())?;
                }
                // ensure there are no gaps between the elements
                for pair in elements.windows(2) {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::Center => {
                // for center, we add two flex elements, one at the beginning and one at the end.
                // this frees up inner constraints to be their true size
                let flex_start_element = Element::constrain(&mut solver, (area_start, area_end))?;
                let flex_end_element = Element::constrain(&mut solver, (area_start, area_end))?;
                // the start flex element must be before the users constraint
                if let Some(first) = elements.first() {
                    solver.add_constraints(&[
                        flex_start_element.start | EQ(REQUIRED) | area_start,
                        first.start | EQ(REQUIRED) | flex_start_element.end,
                    ])?;
                }
                // the end flex element must be after the users constraint
                if let Some(last) = elements.last() {
                    solver.add_constraints(&[
                        last.end | EQ(REQUIRED) | flex_end_element.start,
                        flex_end_element.end | EQ(REQUIRED) | area_end,
                    ])?;
                }
                // finally we ask for a strong preference to make the starting flex and ending flex
                // the same size, and this results in the remaining constraints being centered
                solver.add_constraint(
                    flex_start_element.size() | EQ(STRONG) | flex_end_element.size(),
                )?;
                // ensure there are no gaps between the elements
                for pair in elements.windows(2) {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::Start => {
                // for start, we add one flex element one at the end.
                // this frees up the end constraints and allows inner constraints to be aligned to
                // the start
                let flex_end_element = Element::constrain(&mut solver, (area_start, area_end))?;
                if let Some(first) = elements.first() {
                    solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
                }
                if let Some(last) = elements.last() {
                    solver.add_constraints(&[
                        last.end | EQ(REQUIRED) | flex_end_element.start,
                        flex_end_element.end | EQ(REQUIRED) | area_end,
                    ])?;
                }
                // ensure there are no gaps between the elements
                for pair in elements.windows(2) {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
            Flex::End => {
                // for end, we add one flex element one at the start.
                // this frees up the start constraints and allows inner constraints to be aligned to
                // the end
                let flex_start_element = Element::constrain(&mut solver, (area_start, area_end))?;
                if let Some(first) = elements.first() {
                    solver.add_constraints(&[
                        flex_start_element.start | EQ(REQUIRED) | area_start,
                        first.start | EQ(REQUIRED) | flex_start_element.end,
                    ])?;
                }
                if let Some(last) = elements.last() {
                    solver.add_constraint(last.end | EQ(REQUIRED) | area_end)?;
                }
                // ensure there are no gaps between the elements
                for pair in elements.windows(2) {
                    solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
                }
            }
        }

        // apply the constraints
        for (&constraint, &element) in layout.constraints.iter().zip(elements.iter()) {
            match constraint {
                Constraint::Fixed(l) => {
                    // when fixed is used, element size matching value provided will be the first
                    // priority. We use `REQUIRED - 1` instead `REQUIRED` because we don't want
                    // it to panic in cases when it cannot.
                    solver.add_constraint(element.size() | EQ(REQUIRED - 1.0) | f64::from(l))?
                }
                Constraint::Max(m) => {
                    solver.add_constraints(&[
                        element.size() | LE(STRONG) | f64::from(m),
                        element.size() | EQ(MEDIUM) | f64::from(m),
                    ])?;
                }
                Constraint::Min(m) => {
                    solver.add_constraints(&[
                        element.size() | GE(STRONG) | f64::from(m),
                        element.size() | EQ(MEDIUM) | f64::from(m),
                    ])?;
                }
                Constraint::Length(l) => {
                    solver.add_constraint(element.size() | EQ(STRONG) | f64::from(l))?
                }
                Constraint::Percentage(p) => {
                    let percent = f64::from(p) / 100.00;
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * percent))?;
                }
                Constraint::Ratio(n, d) => {
                    // avoid division by zero by using 1 when denominator is 0
                    let ratio = f64::from(n) / f64::from(d.max(1));
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * ratio))?;
                }
                Constraint::Proportional(_) => {
                    // given no other constraints, this segment will grow as much as possible.
                    //
                    // We want proportional constraints to behave the same as they do without
                    // spacers but we also want them to be fill excess space
                    // before a spacer fills excess space. This means we want
                    // Proportional to be stronger than a spacer constraint but weaker than all the
                    // other constraints.
                    // In my tests, I found choosing an order of magnitude weaker than a `MEDIUM`
                    // constraint did the trick.
                    solver.add_constraint(element.size() | EQ(MEDIUM / 10.0) | area_size)?;
                }
            }
        }
        // Make every `Proportional` constraint proportionally equal to each other
        // This will make it fill up empty spaces equally
        //
        // [Proportional(1), Proportional(1)]
        // ┌──────┐┌──────┐
        // │abcdef││abcdef│
        // └──────┘└──────┘
        //
        // [Proportional(1), Proportional(2)]
        // ┌──────┐┌────────────┐
        // │abcdef││abcdefabcdef│
        // └──────┘└────────────┘
        //
        // size == base_element * scaling_factor
        for ((&l_constraint, &l_element), (&r_constraint, &r_element)) in layout
            .constraints
            .iter()
            .zip(elements.iter())
            .filter(|(c, _)| matches!(c, Constraint::Proportional(_)))
            .tuple_combinations()
        {
            // `Proportional` will only expand into _excess_ available space. You can think of
            // `Proportional` element sizes as starting from `0` and incrementally
            // increasing while proportionally matching other `Proportional` spaces AND
            // also meeting all other constraints.
            if let (
                Constraint::Proportional(l_scaling_factor),
                Constraint::Proportional(r_scaling_factor),
            ) = (l_constraint, r_constraint)
            {
                // because of the way cassowary works, we need to use `*` instead of `/`
                // l_size / l_scaling_factor == l_size / l_scaling_factor
                // ≡
                // l_size * r_scaling_factor == r_size * r_scaling_factor
                //
                // we make `0` act as `1e-6`.
                // this gives us a numerically stable solution and more consistent behavior along
                // the number line
                //
                // I choose `1e-6` because we want a value that is as close to `0.0` as possible
                // without causing it to behave like `0.0`. `1e-9` for example gives the same
                // results as true `0.0`.
                // I found `1e-6` worked well in all the various combinations of constraints I
                // experimented with.
                let (l_scaling_factor, r_scaling_factor) = (
                    f64::from(l_scaling_factor).max(1e-6),
                    f64::from(r_scaling_factor).max(1e-6),
                );
                solver.add_constraint(
                    (r_scaling_factor * l_element.size())
                        | EQ(REQUIRED - 1.0)
                        | (l_scaling_factor * r_element.size()),
                )?;
            }
        }

        let changes: HashMap<Variable, f64> = solver.fetch_changes().iter().copied().collect();

        // please leave this comment here as it's useful for debugging unit tests when we make any
        // changes to layout code - we should replace this with tracing in the future.
        // let ends = format!(
        //     "{:?}",
        //     elements
        //         .iter()
        //         .map(|e| changes.get(&e.end).unwrap_or(&0.0))
        //         .collect::<Vec<&f64>>()
        // );
        // dbg!(ends);

        // convert to Rects
        let results = elements
            .iter()
            .map(|element| {
                let start = changes.get(&element.start).unwrap_or(&0.0).round() as u16;
                let end = changes.get(&element.end).unwrap_or(&0.0).round() as u16;
                let size = end - start;
                match layout.direction {
                    Direction::Horizontal => Rect {
                        x: start,
                        y: inner.y,
                        width: size,
                        height: inner.height,
                    },
                    Direction::Vertical => Rect {
                        x: inner.x,
                        y: start,
                        width: inner.width,
                        height: size,
                    },
                }
            })
            .collect::<Rc<[Rect]>>();
        Ok(results)
    }
}

impl Element {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            start: Variable::new(),
            end: Variable::new(),
        }
    }

    fn constrain(
        solver: &mut Solver,
        (area_start, area_end): (f64, f64),
    ) -> Result<Self, AddConstraintError> {
        let e = Element {
            start: Variable::new(),
            end: Variable::new(),
        };
        solver.add_constraints(&[
            e.start | GE(REQUIRED) | area_start,
            e.end | LE(REQUIRED) | area_end,
            e.start | LE(REQUIRED) | e.end,
        ])?;
        Ok(e)
    }

    fn size(&self) -> Expression {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn custom_cache_size() {
        assert!(Layout::init_cache(10));
        assert!(!Layout::init_cache(15));
        LAYOUT_CACHE.with(|c| {
            assert_eq!(c.get().unwrap().borrow().cap().get(), 10);
        })
    }

    #[test]
    fn default_cache_size() {
        let target = Rect {
            x: 2,
            y: 2,
            width: 10,
            height: 10,
        };

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Max(5),
                Constraint::Min(1),
            ])
            .split(target);
        assert!(!Layout::init_cache(15));
        LAYOUT_CACHE.with(|c| {
            assert_eq!(
                c.get().unwrap().borrow().cap().get(),
                Layout::DEFAULT_CACHE_SIZE
            );
        })
    }

    #[test]
    fn default() {
        assert_eq!(
            Layout::default(),
            Layout {
                direction: Direction::Vertical,
                margin: Margin::new(0, 0),
                constraints: vec![],
                flex: Flex::default(),
            }
        );
    }

    #[test]
    fn new() {
        // array
        let fixed_size_array = [Constraint::Min(0)];
        let layout = Layout::new(Direction::Horizontal, fixed_size_array);
        assert_eq!(layout.direction, Direction::Horizontal);
        assert_eq!(layout.constraints, [Constraint::Min(0)]);

        // array_ref
        #[allow(clippy::needless_borrows_for_generic_args)] // backwards compatibility test
        let layout = Layout::new(Direction::Horizontal, &[Constraint::Min(0)]);
        assert_eq!(layout.direction, Direction::Horizontal);
        assert_eq!(layout.constraints, [Constraint::Min(0)]);

        // vec
        let layout = Layout::new(Direction::Horizontal, vec![Constraint::Min(0)]);
        assert_eq!(layout.direction, Direction::Horizontal);
        assert_eq!(layout.constraints, [Constraint::Min(0)]);

        // vec_ref
        #[allow(clippy::needless_borrows_for_generic_args)] // backwards compatibility test
        let layout = Layout::new(Direction::Horizontal, &(vec![Constraint::Min(0)]));
        assert_eq!(layout.direction, Direction::Horizontal);
        assert_eq!(layout.constraints, [Constraint::Min(0)]);

        // iterator
        let layout = Layout::new(Direction::Horizontal, iter::once(Constraint::Min(0)));
        assert_eq!(layout.direction, Direction::Horizontal);
        assert_eq!(layout.constraints, [Constraint::Min(0)]);
    }

    #[test]
    fn vertical() {
        assert_eq!(
            Layout::vertical([Constraint::Min(0)]),
            Layout {
                direction: Direction::Vertical,
                margin: Margin::new(0, 0),
                constraints: vec![Constraint::Min(0)],
                flex: Flex::default(),
            }
        );
    }

    #[test]
    fn horizontal() {
        assert_eq!(
            Layout::horizontal([Constraint::Min(0)]),
            Layout {
                direction: Direction::Horizontal,
                margin: Margin::new(0, 0),
                constraints: vec![Constraint::Min(0)],
                flex: Flex::default(),
            }
        );
    }

    /// The purpose of this test is to ensure that layout can be constructed with any type that
    /// implements IntoIterator<Item = AsRef<Constraint>>.
    #[test]
    #[allow(
        clippy::needless_borrow,
        clippy::unnecessary_to_owned,
        clippy::useless_asref
    )]
    fn constraints() {
        const CONSTRAINTS: [Constraint; 2] = [Constraint::Min(0), Constraint::Max(10)];
        let fixed_size_array = CONSTRAINTS;
        assert_eq!(
            Layout::default().constraints(fixed_size_array).constraints,
            CONSTRAINTS,
            "constraints should be settable with an array"
        );

        let slice_of_fixed_size_array = &CONSTRAINTS;
        assert_eq!(
            Layout::default()
                .constraints(slice_of_fixed_size_array)
                .constraints,
            CONSTRAINTS,
            "constraints should be settable with a slice"
        );

        let vec = CONSTRAINTS.to_vec();
        let slice_of_vec = vec.as_slice();
        assert_eq!(
            Layout::default().constraints(slice_of_vec).constraints,
            CONSTRAINTS,
            "constraints should be settable with a slice"
        );

        assert_eq!(
            Layout::default().constraints(vec).constraints,
            CONSTRAINTS,
            "constraints should be settable with a Vec"
        );

        let iter = CONSTRAINTS.iter();
        assert_eq!(
            Layout::default().constraints(iter).constraints,
            CONSTRAINTS,
            "constraints should be settable with an iter"
        );

        let iterator = CONSTRAINTS.iter().map(|c| c.to_owned());
        assert_eq!(
            Layout::default().constraints(iterator).constraints,
            CONSTRAINTS,
            "constraints should be settable with an iterator"
        );

        let iterator_ref = CONSTRAINTS.iter().map(|c| c.as_ref());
        assert_eq!(
            Layout::default().constraints(iterator_ref).constraints,
            CONSTRAINTS,
            "constraints should be settable with an iterator of refs"
        );
    }

    #[test]
    fn direction() {
        assert_eq!(
            Layout::default().direction(Direction::Horizontal).direction,
            Direction::Horizontal
        );
        assert_eq!(
            Layout::default().direction(Direction::Vertical).direction,
            Direction::Vertical
        );
    }

    #[test]
    fn margins() {
        assert_eq!(Layout::default().margin(10).margin, Margin::new(10, 10));
        assert_eq!(
            Layout::default().horizontal_margin(10).margin,
            Margin::new(10, 0)
        );
        assert_eq!(
            Layout::default().vertical_margin(10).margin,
            Margin::new(0, 10)
        );
        assert_eq!(
            Layout::default()
                .horizontal_margin(10)
                .vertical_margin(20)
                .margin,
            Margin::new(10, 20)
        );
    }

    #[test]
    fn flex_default() {
        assert_eq!(Layout::default().flex, Flex::StretchLast);
    }

    #[test]
    #[allow(deprecated)]
    fn segment_size() {
        assert_eq!(
            Layout::default()
                .segment_size(SegmentSize::EvenDistribution)
                .flex,
            Flex::Stretch
        );
        assert_eq!(
            Layout::default()
                .segment_size(SegmentSize::LastTakesRemainder)
                .flex,
            Flex::StretchLast
        );
        assert_eq!(
            Layout::default().segment_size(SegmentSize::None).flex,
            Flex::Start
        );
    }

    /// Tests for the `Layout::split()` function.
    ///
    /// There are many tests in this as the number of edge cases that are caused by the interaction
    /// between the constraints is quite large. The tests are split into sections based on the type
    /// of constraints that are used.
    ///
    /// These tests are characterization tests. This means that they are testing the way the code
    /// currently works, and not the way it should work. This is because the current behavior is not
    /// well defined, and it is not clear what the correct behavior should be. This means that if
    /// the behavior changes, these tests should be updated to match the new behavior.
    ///
    ///  EOL comments in each test are intended to communicate the purpose of each test and to make
    ///  it easy to see that the tests are as exhaustive as feasible:
    /// - zero: constraint is zero
    /// - exact: constraint is equal to the space
    /// - underflow: constraint is for less than the full space
    /// - overflow: constraint is for more than the full space
    mod split {
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        use crate::{
            assert_buffer_eq,
            layout::flex::Flex,
            prelude::{Constraint::*, *},
            widgets::{Paragraph, Widget},
        };

        /// Test that the given constraints applied to the given area result in the expected layout.
        /// Each chunk is filled with a letter repeated as many times as the width of the chunk. The
        /// resulting buffer is compared to the expected string.
        ///
        /// This approach is used rather than testing the resulting rects directly because it is
        /// easier to visualize the result, and it leads to more concise tests that are easier to
        /// compare against each other. E.g. `"abc"` is much more concise than `[Rect::new(0, 0, 1,
        /// 1), Rect::new(1, 0, 1, 1), Rect::new(2, 0, 1, 1)]`.
        #[track_caller]
        fn test(area: Rect, constraints: &[Constraint], expected: &str) {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(area);
            let mut buffer = Buffer::empty(area);
            for (i, c) in ('a'..='z').take(constraints.len()).enumerate() {
                let s: String = c.to_string().repeat(area.width as usize);
                Paragraph::new(s).render(layout[i], &mut buffer);
            }
            let expected = Buffer::with_lines(vec![expected]);
            assert_buffer_eq!(buffer, expected);
        }

        #[test]
        fn length() {
            test(Rect::new(0, 0, 1, 1), &[Length(0)], "a"); // zero
            test(Rect::new(0, 0, 1, 1), &[Length(1)], "a"); // exact
            test(Rect::new(0, 0, 1, 1), &[Length(2)], "a"); // overflow

            test(Rect::new(0, 0, 2, 1), &[Length(0)], "aa"); // zero
            test(Rect::new(0, 0, 2, 1), &[Length(1)], "aa"); // underflow
            test(Rect::new(0, 0, 2, 1), &[Length(2)], "aa"); // exact
            test(Rect::new(0, 0, 2, 1), &[Length(3)], "aa"); // overflow

            test(Rect::new(0, 0, 1, 1), &[Length(0), Length(0)], "b"); // zero, zero
            test(Rect::new(0, 0, 1, 1), &[Length(0), Length(1)], "b"); // zero, exact
            test(Rect::new(0, 0, 1, 1), &[Length(0), Length(2)], "b"); // zero, overflow
            test(Rect::new(0, 0, 1, 1), &[Length(1), Length(0)], "a"); // exact, zero
            test(Rect::new(0, 0, 1, 1), &[Length(1), Length(1)], "a"); // exact, exact
            test(Rect::new(0, 0, 1, 1), &[Length(1), Length(2)], "a"); // exact, overflow
            test(Rect::new(0, 0, 1, 1), &[Length(2), Length(0)], "a"); // overflow, zero
            test(Rect::new(0, 0, 1, 1), &[Length(2), Length(1)], "a"); // overflow, exact
            test(Rect::new(0, 0, 1, 1), &[Length(2), Length(2)], "a"); // overflow, overflow

            test(Rect::new(0, 0, 2, 1), &[Length(0), Length(0)], "bb"); // zero, zero
            test(Rect::new(0, 0, 2, 1), &[Length(0), Length(1)], "bb"); // zero, underflow
            test(Rect::new(0, 0, 2, 1), &[Length(0), Length(2)], "bb"); // zero, exact
            test(Rect::new(0, 0, 2, 1), &[Length(0), Length(3)], "bb"); // zero, overflow
            test(Rect::new(0, 0, 2, 1), &[Length(1), Length(0)], "ab"); // underflow, zero
            test(Rect::new(0, 0, 2, 1), &[Length(1), Length(1)], "ab"); // underflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Length(1), Length(2)], "ab"); // underflow, exact
            test(Rect::new(0, 0, 2, 1), &[Length(1), Length(3)], "ab"); // underflow, overflow
            test(Rect::new(0, 0, 2, 1), &[Length(2), Length(0)], "aa"); // exact, zero
            test(Rect::new(0, 0, 2, 1), &[Length(2), Length(1)], "aa"); // exact, underflow
            test(Rect::new(0, 0, 2, 1), &[Length(2), Length(2)], "aa"); // exact, exact
            test(Rect::new(0, 0, 2, 1), &[Length(2), Length(3)], "aa"); // exact, overflow
            test(Rect::new(0, 0, 2, 1), &[Length(3), Length(0)], "aa"); // overflow, zero
            test(Rect::new(0, 0, 2, 1), &[Length(3), Length(1)], "aa"); // overflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Length(3), Length(2)], "aa"); // overflow, exact
            test(Rect::new(0, 0, 2, 1), &[Length(3), Length(3)], "aa"); // overflow, overflow

            test(Rect::new(0, 0, 3, 1), &[Length(2), Length(2)], "aab");
        }

        #[test]
        fn max() {
            test(Rect::new(0, 0, 1, 1), &[Max(0)], "a"); // zero
            test(Rect::new(0, 0, 1, 1), &[Max(1)], "a"); // exact
            test(Rect::new(0, 0, 1, 1), &[Max(2)], "a"); // overflow

            test(Rect::new(0, 0, 2, 1), &[Max(0)], "aa"); // zero
            test(Rect::new(0, 0, 2, 1), &[Max(1)], "aa"); // underflow
            test(Rect::new(0, 0, 2, 1), &[Max(2)], "aa"); // exact
            test(Rect::new(0, 0, 2, 1), &[Max(3)], "aa"); // overflow

            test(Rect::new(0, 0, 1, 1), &[Max(0), Max(0)], "b"); // zero, zero
            test(Rect::new(0, 0, 1, 1), &[Max(0), Max(1)], "b"); // zero, exact
            test(Rect::new(0, 0, 1, 1), &[Max(0), Max(2)], "b"); // zero, overflow
            test(Rect::new(0, 0, 1, 1), &[Max(1), Max(0)], "a"); // exact, zero
            test(Rect::new(0, 0, 1, 1), &[Max(1), Max(1)], "a"); // exact, exact
            test(Rect::new(0, 0, 1, 1), &[Max(1), Max(2)], "a"); // exact, overflow
            test(Rect::new(0, 0, 1, 1), &[Max(2), Max(0)], "a"); // overflow, zero
            test(Rect::new(0, 0, 1, 1), &[Max(2), Max(1)], "a"); // overflow, exact
            test(Rect::new(0, 0, 1, 1), &[Max(2), Max(2)], "a"); // overflow, overflow

            test(Rect::new(0, 0, 2, 1), &[Max(0), Max(0)], "bb"); // zero, zero
            test(Rect::new(0, 0, 2, 1), &[Max(0), Max(1)], "bb"); // zero, underflow
            test(Rect::new(0, 0, 2, 1), &[Max(0), Max(2)], "bb"); // zero, exact
            test(Rect::new(0, 0, 2, 1), &[Max(0), Max(3)], "bb"); // zero, overflow
            test(Rect::new(0, 0, 2, 1), &[Max(1), Max(0)], "ab"); // underflow, zero
            test(Rect::new(0, 0, 2, 1), &[Max(1), Max(1)], "ab"); // underflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Max(1), Max(2)], "ab"); // underflow, exact
            test(Rect::new(0, 0, 2, 1), &[Max(1), Max(3)], "ab"); // underflow, overflow
            test(Rect::new(0, 0, 2, 1), &[Max(2), Max(0)], "aa"); // exact, zero
            test(Rect::new(0, 0, 2, 1), &[Max(2), Max(1)], "aa"); // exact, underflow
            test(Rect::new(0, 0, 2, 1), &[Max(2), Max(2)], "aa"); // exact, exact
            test(Rect::new(0, 0, 2, 1), &[Max(2), Max(3)], "aa"); // exact, overflow
            test(Rect::new(0, 0, 2, 1), &[Max(3), Max(0)], "aa"); // overflow, zero
            test(Rect::new(0, 0, 2, 1), &[Max(3), Max(1)], "aa"); // overflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Max(3), Max(2)], "aa"); // overflow, exact
            test(Rect::new(0, 0, 2, 1), &[Max(3), Max(3)], "aa"); // overflow, overflow

            test(Rect::new(0, 0, 3, 1), &[Max(2), Max(2)], "aab");
        }

        #[test]
        fn min() {
            test(Rect::new(0, 0, 1, 1), &[Min(0), Min(0)], "b"); // zero, zero
            test(Rect::new(0, 0, 1, 1), &[Min(0), Min(1)], "b"); // zero, exact
            test(Rect::new(0, 0, 1, 1), &[Min(0), Min(2)], "b"); // zero, overflow
            test(Rect::new(0, 0, 1, 1), &[Min(1), Min(0)], "a"); // exact, zero
            test(Rect::new(0, 0, 1, 1), &[Min(1), Min(1)], "a"); // exact, exact
            test(Rect::new(0, 0, 1, 1), &[Min(1), Min(2)], "a"); // exact, overflow
            test(Rect::new(0, 0, 1, 1), &[Min(2), Min(0)], "a"); // overflow, zero
            test(Rect::new(0, 0, 1, 1), &[Min(2), Min(1)], "a"); // overflow, exact
            test(Rect::new(0, 0, 1, 1), &[Min(2), Min(2)], "a"); // overflow, overflow

            test(Rect::new(0, 0, 2, 1), &[Min(0), Min(0)], "bb"); // zero, zero
            test(Rect::new(0, 0, 2, 1), &[Min(0), Min(1)], "bb"); // zero, underflow
            test(Rect::new(0, 0, 2, 1), &[Min(0), Min(2)], "bb"); // zero, exact
            test(Rect::new(0, 0, 2, 1), &[Min(0), Min(3)], "bb"); // zero, overflow
            test(Rect::new(0, 0, 2, 1), &[Min(1), Min(0)], "ab"); // underflow, zero
            test(Rect::new(0, 0, 2, 1), &[Min(1), Min(1)], "ab"); // underflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Min(1), Min(2)], "ab"); // underflow, exact
            test(Rect::new(0, 0, 2, 1), &[Min(1), Min(3)], "ab"); // underflow, overflow
            test(Rect::new(0, 0, 2, 1), &[Min(2), Min(0)], "aa"); // exact, zero
            test(Rect::new(0, 0, 2, 1), &[Min(2), Min(1)], "aa"); // exact, underflow
            test(Rect::new(0, 0, 2, 1), &[Min(2), Min(2)], "aa"); // exact, exact
            test(Rect::new(0, 0, 2, 1), &[Min(2), Min(3)], "aa"); // exact, overflow
            test(Rect::new(0, 0, 2, 1), &[Min(3), Min(0)], "aa"); // overflow, zero
            test(Rect::new(0, 0, 2, 1), &[Min(3), Min(1)], "aa"); // overflow, underflow
            test(Rect::new(0, 0, 2, 1), &[Min(3), Min(2)], "aa"); // overflow, exact
            test(Rect::new(0, 0, 2, 1), &[Min(3), Min(3)], "aa"); // overflow, overflow

            test(Rect::new(0, 0, 3, 1), &[Min(2), Min(2)], "aab");
        }

        #[test]
        fn percentage() {
            // choose some percentages that will result in several different rounding behaviors
            // when applied to the given area. E.g. we want to test things that will end up exactly
            // integers, things that will round up, and things that will round down. We also want
            // to test when rounding occurs both in the position and the size.
            const ZERO: Constraint = Percentage(0);
            const TEN: Constraint = Percentage(10);
            const QUARTER: Constraint = Percentage(25);
            const THIRD: Constraint = Percentage(33);
            const HALF: Constraint = Percentage(50);
            const TWO_THIRDS: Constraint = Percentage(66);
            const NINETY: Constraint = Percentage(90);
            const FULL: Constraint = Percentage(100);
            const DOUBLE: Constraint = Percentage(200);

            test(Rect::new(0, 0, 1, 1), &[ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[QUARTER], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[DOUBLE], "a");

            test(Rect::new(0, 0, 2, 1), &[ZERO], "aa");
            test(Rect::new(0, 0, 2, 1), &[TEN], "aa");
            test(Rect::new(0, 0, 2, 1), &[QUARTER], "aa");
            test(Rect::new(0, 0, 2, 1), &[HALF], "aa");
            test(Rect::new(0, 0, 2, 1), &[TWO_THIRDS], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL], "aa");
            test(Rect::new(0, 0, 2, 1), &[DOUBLE], "aa");

            test(Rect::new(0, 0, 1, 1), &[ZERO, ZERO], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, TEN], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, HALF], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, NINETY], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, FULL], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, DOUBLE], "b");

            test(Rect::new(0, 0, 1, 1), &[TEN, ZERO], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, TEN], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, HALF], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, NINETY], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, FULL], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, DOUBLE], "b");

            test(Rect::new(0, 0, 1, 1), &[HALF, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, DOUBLE], "a");

            test(Rect::new(0, 0, 1, 1), &[NINETY, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, DOUBLE], "a");

            test(Rect::new(0, 0, 1, 1), &[FULL, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, DOUBLE], "a");

            test(Rect::new(0, 0, 2, 1), &[ZERO, ZERO], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, QUARTER], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, HALF], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, FULL], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, DOUBLE], "bb");

            test(Rect::new(0, 0, 2, 1), &[TEN, ZERO], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, QUARTER], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, HALF], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, FULL], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, DOUBLE], "bb");

            test(Rect::new(0, 0, 2, 1), &[QUARTER, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, QUARTER], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, DOUBLE], "ab");

            test(Rect::new(0, 0, 2, 1), &[THIRD, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, QUARTER], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, DOUBLE], "ab");

            test(Rect::new(0, 0, 2, 1), &[HALF, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[HALF, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[HALF, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[FULL, ZERO], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL, HALF], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL, FULL], "aa");

            test(Rect::new(0, 0, 3, 1), &[THIRD, THIRD], "abb");
            test(Rect::new(0, 0, 3, 1), &[THIRD, TWO_THIRDS], "abb");
        }

        #[test]
        fn ratio() {
            // choose some ratios that will result in several different rounding behaviors
            // when applied to the given area. E.g. we want to test things that will end up exactly
            // integers, things that will round up, and things that will round down. We also want
            // to test when rounding occurs both in the position and the size.
            const ZERO: Constraint = Ratio(0, 1);
            const TEN: Constraint = Ratio(1, 10);
            const QUARTER: Constraint = Ratio(1, 4);
            const THIRD: Constraint = Ratio(1, 3);
            const HALF: Constraint = Ratio(1, 2);
            const TWO_THIRDS: Constraint = Ratio(2, 3);
            const NINETY: Constraint = Ratio(9, 10);
            const FULL: Constraint = Ratio(1, 1);
            const DOUBLE: Constraint = Ratio(2, 1);

            test(Rect::new(0, 0, 1, 1), &[ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[QUARTER], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[DOUBLE], "a");

            test(Rect::new(0, 0, 2, 1), &[ZERO], "aa");
            test(Rect::new(0, 0, 2, 1), &[TEN], "aa");
            test(Rect::new(0, 0, 2, 1), &[QUARTER], "aa");
            test(Rect::new(0, 0, 2, 1), &[HALF], "aa");
            test(Rect::new(0, 0, 2, 1), &[TWO_THIRDS], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL], "aa");
            test(Rect::new(0, 0, 2, 1), &[DOUBLE], "aa");

            test(Rect::new(0, 0, 1, 1), &[ZERO, ZERO], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, TEN], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, HALF], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, NINETY], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, FULL], "b");
            test(Rect::new(0, 0, 1, 1), &[ZERO, DOUBLE], "b");

            test(Rect::new(0, 0, 1, 1), &[TEN, ZERO], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, TEN], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, HALF], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, NINETY], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, FULL], "b");
            test(Rect::new(0, 0, 1, 1), &[TEN, DOUBLE], "b");

            test(Rect::new(0, 0, 1, 1), &[HALF, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[HALF, DOUBLE], "a");

            test(Rect::new(0, 0, 1, 1), &[NINETY, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[NINETY, DOUBLE], "a");

            test(Rect::new(0, 0, 1, 1), &[FULL, ZERO], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, HALF], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, FULL], "a");
            test(Rect::new(0, 0, 1, 1), &[FULL, DOUBLE], "a");

            test(Rect::new(0, 0, 2, 1), &[ZERO, ZERO], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, QUARTER], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, HALF], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, FULL], "bb");
            test(Rect::new(0, 0, 2, 1), &[ZERO, DOUBLE], "bb");

            test(Rect::new(0, 0, 2, 1), &[TEN, ZERO], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, QUARTER], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, HALF], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, FULL], "bb");
            test(Rect::new(0, 0, 2, 1), &[TEN, DOUBLE], "bb");

            test(Rect::new(0, 0, 2, 1), &[QUARTER, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, QUARTER], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[QUARTER, DOUBLE], "ab");

            test(Rect::new(0, 0, 2, 1), &[THIRD, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, QUARTER], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[THIRD, DOUBLE], "ab");

            test(Rect::new(0, 0, 2, 1), &[HALF, ZERO], "ab");
            test(Rect::new(0, 0, 2, 1), &[HALF, HALF], "ab");
            test(Rect::new(0, 0, 2, 1), &[HALF, FULL], "ab");
            test(Rect::new(0, 0, 2, 1), &[FULL, ZERO], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL, HALF], "aa");
            test(Rect::new(0, 0, 2, 1), &[FULL, FULL], "aa");

            test(Rect::new(0, 0, 3, 1), &[THIRD, THIRD], "abb");
            test(Rect::new(0, 0, 3, 1), &[THIRD, TWO_THIRDS], "abb");
        }

        #[test]
        fn vertical_split_by_height() {
            let target = Rect {
                x: 2,
                y: 2,
                width: 10,
                height: 10,
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Max(5),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(target);

            assert_eq!(target.height, chunks.iter().map(|r| r.height).sum::<u16>());
            chunks.windows(2).for_each(|w| assert!(w[0].y <= w[1].y));
        }

        #[test]
        fn length_constraints() {
            // cassowary implementation tends to put excess in last variable
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Length(25),
                Length(25),
                Length(25),
            ]));
            assert_eq!([a.width, b.width, c.width], [25, 25, 50]);

            // Length is lower priority that breaking Min
            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Min(100)]));
            assert_eq!([a.width, b.width], [0, 100]);

            // Length is higher priority to non binding Min
            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Min(0)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is lower priority that breaking Max
            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Max(0)]));
            assert_eq!([a.width, b.width], [100, 0]);

            // Length is higher priority to non binding Min
            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Max(100)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is equal priority to Percentage
            // but cassowary modifies last constraint of equal weight to satisfy everything
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Percentage(25)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is equal priority to Percentage
            // but cassowary modifies last constraint of equal weight to satisfy everything
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Percentage(25), Length(25)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is equal priority to Ratio
            // but cassowary modifies last constraint of equal weight to satisfy everything
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Ratio(1, 4)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is equal priority to Ratio
            // but cassowary modifies last constraint of equal weight to satisfy everything
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Ratio(1, 4), Length(25)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is lower priority to Fixed
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Fixed(25), Length(25)]));
            assert_eq!([a.width, b.width], [25, 75]);

            // Length is lower priority to Fixed
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Length(25), Fixed(25)]));
            assert_eq!([a.width, b.width], [75, 25]);
        }

        // these are a few tests that document existing bugs in the layout algorithm
        #[test]
        fn edge_cases() {
            let layout = Layout::default()
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Min(0),
                ])
                .split(Rect::new(0, 0, 1, 1));
            assert_eq!(
                layout[..],
                [
                    Rect::new(0, 0, 1, 1),
                    Rect::new(0, 1, 1, 0),
                    Rect::new(0, 1, 1, 0)
                ]
            );

            let layout = Layout::default()
                .constraints([
                    Constraint::Max(1),
                    Constraint::Percentage(99),
                    Constraint::Min(0),
                ])
                .split(Rect::new(0, 0, 1, 1));
            assert_eq!(
                layout[..],
                [
                    Rect::new(0, 0, 1, 0),
                    Rect::new(0, 0, 1, 1),
                    Rect::new(0, 1, 1, 0)
                ]
            );

            // minimal bug from
            // https://github.com/ratatui-org/ratatui/pull/404#issuecomment-1681850644
            let layout = Layout::default()
                .constraints([Min(1), Length(0), Min(1)])
                .direction(Direction::Horizontal)
                .split(Rect::new(0, 0, 1, 1));
            assert_eq!(
                layout[..],
                [
                    Rect::new(0, 0, 1, 1),
                    Rect::new(1, 0, 0, 1),
                    Rect::new(1, 0, 0, 1),
                ]
            );

            let layout = Layout::default()
                .constraints([Length(3), Min(4), Length(1), Min(4)])
                .direction(Direction::Horizontal)
                .split(Rect::new(0, 0, 7, 1));
            assert_eq!(
                layout[..],
                [
                    Rect::new(0, 0, 0, 1),
                    Rect::new(0, 0, 4, 1),
                    Rect::new(4, 0, 0, 1),
                    Rect::new(4, 0, 3, 1),
                ]
            );
        }

        #[test]
        fn proportional_fixed() {
            // fixed doesn't panic when results don't match exactly
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Fixed(33),
                Fixed(33),
                Fixed(33),
            ]));
            assert_eq!([a.width, b.width, c.width], [33, 33, 34]);

            // cassowary implementation tends to put excess in last variable
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Fixed(25),
                Fixed(25),
                Fixed(25),
            ]));
            assert_eq!([a.width, b.width, c.width], [25, 25, 50]);

            // fixed with min and max
            let [a, b, c] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Min(25), Fixed(25), Max(25)]));
            assert_eq!([a.width, b.width, c.width], [50, 25, 25]);

            // fixed with percent and ratio
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Percentage(25),
                Fixed(25),
                Ratio(1, 4),
            ]));
            assert_eq!([a.width, b.width, c.width], [25, 25, 50]);

            // 3 lengths for reference
            // cassowary implementation tends to put excess in last variable
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Length(25),
                Length(25),
                Length(25),
            ]));
            assert_eq!([a.width, b.width, c.width], [25, 25, 50]);

            // fixed with length
            let [_a, _b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Length(25),
                Length(25),
                Fixed(25),
            ]));
            // this test gives different results when run over and over again
            //
            // assert_eq!([a.width, b.width], [25, 50]);
            // assert_eq!([a.width, b.width], [50, 25]);
            //
            // So we check just that the last value is exactly what we want it to be
            assert_eq!(c.width, 25);

            // ensure that middle width is 1
            // last length of 100 will not be met
            let [a, b, c] =
                Rect::new(0, 0, 50, 1).split(&Layout::horizontal([Min(20), Fixed(1), Length(100)]));

            assert_eq!([a.width, b.width, c.width], [20, 1, 29]);

            // ensure that middle width is 1
            // first length of 100 will not be met
            let [a, b, c] =
                Rect::new(0, 0, 50, 1).split(&Layout::horizontal([Length(100), Fixed(1), Min(20)]));
            assert_eq!([a.width, b.width, c.width], [29, 1, 20]);

            // middle fixed width is satisfied exactly
            // left and right are equal to each other
            let [a, b, c] = Rect::new(0, 0, 50, 1).split(&Layout::horizontal([
                Proportional(1),
                Fixed(10),
                Proportional(1),
            ]));
            assert_eq!([a.width, b.width, c.width], [20, 10, a.width]);

            // middle fixed width is satisfied exactly
            // ratio of left and right is 1 / 2
            let [a, b, c] = Rect::new(0, 0, 50, 1).split(&Layout::horizontal([
                Proportional(1),
                Fixed(10),
                Proportional(2),
            ]));
            assert_eq!([a.width, b.width, c.width], [13, 10, 27]);

            // second width is double all the others
            let [a, b, c, d] = Rect::new(0, 0, 50, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(2),
                Proportional(1),
                Proportional(1),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [10, 20, 10, 10]);

            // second width is still double all the others
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(2),
                Proportional(1),
                Proportional(1),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [20, 40, 20, 20]);

            // incremental proportions
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(2),
                Proportional(3),
                Proportional(4),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [10, 20, 30, 40]);

            // decremental proportions
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(4),
                Proportional(3),
                Proportional(2),
                Proportional(1),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [40, 30, 20, 10]);

            // randomly ordered proportions
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [10, 30, 20, 40]);

            // randomly ordered proportions with fixed
            let [a, b, c, d, e] = Rect::new(0, 0, 200, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Fixed(100),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [10, 30, 100, 20, 40]
            );

            // randomly ordered proportions with length
            let [a, b, c, d, e] = Rect::new(0, 0, 200, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Length(100),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [10, 30, 100, 20, 40]
            );

            // randomly ordered proportions with percentage
            let [a, b, c, d, e] = Rect::new(0, 0, 200, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Percentage(50),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [10, 30, 100, 20, 40]
            );

            // randomly ordered proportions with min
            let [a, b, c, d, e] = Rect::new(0, 0, 200, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Min(100),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [10, 30, 100, 20, 40]
            );

            // randomly ordered proportions with max
            let [a, b, c, d, e] = Rect::new(0, 0, 200, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(3),
                Max(100),
                Proportional(2),
                Proportional(4),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [10, 30, 100, 20, 40]
            );

            // first and third widths are zero
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Proportional(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [0, 100, 0]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Fixed(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Length(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Percentage(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Ratio(1, 100),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Min(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // Proportional always divides proportionally amongst zeros
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Max(1),
                Proportional(0),
            ]));
            assert_eq!([a.width, b.width, c.width], [50, 1, 49]);

            // first and third widths are zero
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Proportional(2),
                Proportional(0),
                Proportional(1),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [0, 67, 0, 33]);

            // 0 proportional will fill empty space
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Proportional(2),
                Percentage(20),
            ]));
            assert_eq!([a.width, b.width, c.width], [0, 80, 20]);

            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(0),
                Proportional(u16::MAX),
            ]));
            assert_eq!([a.width, b.width], [0, 100]);

            // 0 proportional will fill empty space
            let [a, b] = Rect::new(0, 0, 100, 1)
                .split(&Layout::horizontal([Proportional(0), Percentage(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // 1 proportional will fill empty spaces
            let [a, b] = Rect::new(0, 0, 100, 1)
                .split(&Layout::horizontal([Proportional(1), Percentage(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // proportional can be zero because of high scaling factor
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(u16::MAX),
                Proportional(1),
                Percentage(20),
            ]));
            assert_eq!([a.width, b.width, c.width], [80, 0, 20]);

            // single 0 proportional will fill empty space
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Proportional(0), Length(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // single 0 proportional will fill empty space
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Proportional(0), Max(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // 1 proportional will fill empty space
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Proportional(1), Max(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // 0 min still fills empty space
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Min(0), Percentage(20)]));
            assert_eq!([a.width, b.width], [80, 20]);

            // percentage constraint doesn't hold against defying `Max`
            let [a, b] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal([Max(0), Percentage(20)]));
            assert_eq!([a.width, b.width], [0, 100]);

            // specifying behavior of proportional with min and length
            let [a, b, c, d, e] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(1),
                Proportional(1),
                Min(30),
                Length(50),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [7, 6, 7, 30, 50]
            );

            // proportional is lower priority than length
            let [a, b, c, d, e] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(1),
                Proportional(1),
                Length(50),
                Length(50),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [0, 0, 0, 50, 50]
            );

            // proportional is lower priority than min and max
            let [a, b, c, d, e] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(1),
                Proportional(1),
                Min(50),
                Max(50),
            ]));
            assert_eq!(
                [a.width, b.width, c.width, d.width, e.width],
                [0, 0, 0, 50, 50]
            );

            // proportional is lower priority than percentage
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(1),
                Proportional(1),
                Percentage(100),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [0, 0, 0, 100]);

            // proportional is lower priority than ratio
            let [a, b, c, d] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([
                Proportional(1),
                Proportional(1),
                Proportional(1),
                Ratio(1, 1),
            ]));
            assert_eq!([a.width, b.width, c.width, d.width], [0, 0, 0, 100]);
        }

        #[rstest]
        #[case::length_stretches_to_end(Constraint::Length(50), Flex::StretchLast, (0, 100))]
        #[case::length_stretches(Constraint::Length(50), Flex::Stretch, (0, 100))]
        #[case::length_left_justified(Constraint::Length(50), Flex::Start, (0, 50))]
        #[case::length_right_justified(Length(50), Flex::End, (50, 50))]
        #[case::length_center_justified(Length(50), Flex::Center, (25, 50))]
        #[case::fixed_stretches_to_end(Fixed(50), Flex::StretchLast, (0, 100))]
        #[case::fixed_left_justified(Fixed(50), Flex::Start, (0, 50))]
        #[case::fixed_right_justified(Fixed(50), Flex::End, (50, 50))]
        #[case::fixed_center_justified(Fixed(50), Flex::Center, (25, 50))]
        #[case::ratio_stretches_to_end(Ratio(1, 2), Flex::StretchLast, (0, 100))]
        #[case::ratio_left_justified(Ratio(1, 2), Flex::Start, (0, 50))]
        #[case::ratio_right_justified(Ratio(1, 2), Flex::End, (50, 50))]
        #[case::ratio_center_justified(Ratio(1, 2), Flex::Center, (25, 50))]
        #[case::percent_stretches_to_end(Percentage(50), Flex::StretchLast, (0, 100))]
        #[case::percent_left_justified(Percentage(50), Flex::Start, (0, 50))]
        #[case::percent_right_justified(Percentage(50), Flex::End, (50, 50))]
        #[case::percent_center_justified(Percentage(50), Flex::Center, (25, 50))]
        #[case::min_stretches_to_end(Min(50), Flex::StretchLast, (0, 100))]
        #[case::min_left_justified(Min(50), Flex::Start, (0, 50))]
        #[case::min_right_justified(Min(50), Flex::End, (50, 50))]
        #[case::min_center_justified(Min(50), Flex::Center, (25, 50))]
        #[case::max_stretches_to_end(Max(50), Flex::StretchLast, (0, 100))]
        #[case::max_left_justified(Max(50), Flex::Start, (0, 50))]
        #[case::max_right_justified(Max(50), Flex::End, (50, 50))]
        #[case::max_center_justified(Max(50), Flex::Center, (25, 50))]
        fn flex_one_constraint(
            #[case] constraint: Constraint,
            #[case] flex: Flex,
            #[case] expected_widths: (u16, u16),
        ) {
            let [a] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal([constraint]).flex(flex));
            assert_eq!((a.x, a.width), expected_widths);
        }

        #[rstest]
        #[case::length_stretches_to_end([Length(25), Length(25)], Flex::StretchLast, [(0, 25), (25, 75)])]
        #[case::splits_equally_to_end([Length(25), Length(25)], Flex::Stretch, [(0, 50), (50, 50)])]
        #[case::lengths_justify_to_start([Length(25), Length(25)], Flex::Start, [(0, 25), (25, 25)])]
        #[case::length_justifies_to_center([Length(25), Length(25)], Flex::Center, [(25, 25), (50, 25)])]
        #[case::length_justifies_to_end([Length(25), Length(25)], Flex::End, [(50, 25), (75, 25)])]
        #[case::fixed_stretches_to_end_last([Fixed(25), Fixed(25)], Flex::StretchLast, [(0, 25), (25, 75)])]
        #[case::fixed_stretches_to_end([Fixed(25), Fixed(25)], Flex::Stretch, [(0, 50), (50, 50)])]
        #[case::fixed_justifies_to_start([Fixed(25), Fixed(25)], Flex::Start, [(0, 25), (25, 25)])]
        #[case::fixed_justifies_to_center([Fixed(25), Fixed(25)], Flex::Center, [(25, 25), (50, 25)])]
        #[case::fixed_justifies_to_end([Fixed(25), Fixed(25)], Flex::End, [(50, 25), (75, 25)])]
        #[case::percentage_stretches_to_end_last([Percentage(25), Percentage(25)], Flex::StretchLast, [(0, 25), (25, 75)])]
        #[case::percentage_stretches_to_end([Percentage(25), Percentage(25)], Flex::Stretch, [(0, 50), (50, 50)])]
        #[case::percentage_justifies_to_start([Percentage(25), Percentage(25)], Flex::Start, [(0, 25), (25, 25)])]
        #[case::percentage_justifies_to_center([Percentage(25), Percentage(25)], Flex::Center, [(25, 25), (50, 25)])]
        #[case::percentage_justifies_to_end([Percentage(25), Percentage(25)], Flex::End, [(50, 25), (75, 25)])]
        #[case::min_stretches_to_end([Min(25), Min(25)], Flex::StretchLast, [(0, 25), (25, 75)])]
        #[case::min_stretches_to_end([Min(25), Min(25)], Flex::Stretch, [(0, 50), (50, 50)])]
        #[case::min_justifies_to_start([Min(25), Min(25)], Flex::Start, [(0, 25), (25, 25)])]
        #[case::min_justifies_to_center([Min(25), Min(25)], Flex::Center, [(25, 25), (50, 25)])]
        #[case::min_justifies_to_end([Min(25), Min(25)], Flex::End, [(50, 25), (75, 25)])]
        #[case::length_spaced_between([Length(25), Length(25)], Flex::SpaceBetween, [(0, 25), (75, 25)])]
        #[case::length_spaced_around([Length(25), Length(25)], Flex::SpaceAround, [(17, 25), (58, 25)])]
        fn flex_two_constraints(
            #[case] constraints: [Constraint; 2],
            #[case] flex: Flex,
            #[case] expected_widths: [(u16, u16); 2],
        ) {
            let [a, b] = Rect::new(0, 0, 100, 1).split(&Layout::horizontal(constraints).flex(flex));
            assert_eq!([(a.x, a.width), (b.x, b.width)], expected_widths);
        }

        #[rstest]
        #[case::length_spaced_around([Length(25), Length(25), Length(25)], Flex::SpaceBetween, [(0, 25), (38, 25), (75, 25)])]
        fn flex_three_constraints(
            #[case] constraints: [Constraint; 3],
            #[case] flex: Flex,
            #[case] expected_widths: [(u16, u16); 3],
        ) {
            let [a, b, c] =
                Rect::new(0, 0, 100, 1).split(&Layout::horizontal(constraints).flex(flex));
            assert_eq!(
                [(a.x, a.width), (b.x, b.width), (c.x, c.width)],
                expected_widths
            );
        }

        #[test]
        fn flex() {
            // length should be spaced around
            let [a, b, c] = Rect::new(0, 0, 100, 1).split(
                &Layout::horizontal([Length(25), Length(25), Length(25)]).flex(Flex::SpaceAround),
            );
            assert!(b.x == 37 || b.x == 38);
            assert!(b.width == 26 || b.width == 25);
            assert_eq!([[a.x, a.width], [c.x, c.width]], [[6, 25], [69, 25]]);
        }
    }
}
