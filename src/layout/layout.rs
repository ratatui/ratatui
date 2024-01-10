use std::{cell::RefCell, collections::HashMap, num::NonZeroUsize, rc::Rc, sync::OnceLock};

use cassowary::{
    strength::{MEDIUM, REQUIRED, STRONG, WEAK},
    AddConstraintError, Expression, Solver, Variable,
    WeightedRelation::{EQ, GE, LE},
};
use itertools::Itertools;
use lru::LruCache;

use super::SegmentSize;
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
/// - [`Layout::segment_size`]: set the way the space is distributed when the constraints are
///   satisfied
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
    /// option for segment size preferences
    segment_size: SegmentSize,
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
    /// - `segment_size`: SegmentSize::LastTakesRemainder
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
            segment_size: SegmentSize::LastTakesRemainder,
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

    /// Set whether chunks should be of equal size.
    ///
    /// This determines how the space is distributed when the constraints are satisfied. By default,
    /// the last chunk is expanded to fill the remaining space, but this can be changed to prefer
    /// equal chunks or to not distribute extra space at all (which is the default used for laying
    /// out the columns for [`Table`] widgets).
    ///
    /// Note: If you're using this feature please help us come up with a good name. See [Issue
    /// #536](https://github.com/ratatui-org/ratatui/issues/536) for more information.
    ///
    /// [`Table`]: crate::widgets::Table
    #[stability::unstable(
        feature = "segment-size",
        reason = "The name for this feature is not final and may change in the future",
        issue = "https://github.com/ratatui-org/ratatui/issues/536"
    )]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn segment_size(mut self, segment_size: SegmentSize) -> Layout {
        self.segment_size = segment_size;
        self
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
        let elements = layout
            .constraints
            .iter()
            .map(|_| Element::new())
            .collect::<Vec<Element>>();

        // ensure that all the elements are inside the area
        for element in &elements {
            solver.add_constraints(&[
                element.start | GE(REQUIRED) | area_start,
                element.end | LE(REQUIRED) | area_end,
                element.start | LE(REQUIRED) | element.end,
            ])?;
        }
        // ensure there are no gaps between the elements
        for pair in elements.windows(2) {
            solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
        }
        // ensure the first element touches the left/top edge of the area
        if let Some(first) = elements.first() {
            solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
        }
        if layout.segment_size != SegmentSize::None {
            // ensure the last element touches the right/bottom edge of the area
            if let Some(last) = elements.last() {
                solver.add_constraint(last.end | EQ(REQUIRED) | area_end)?;
            }
        }
        // apply the constraints
        for (&constraint, &element) in layout.constraints.iter().zip(elements.iter()) {
            match constraint {
                Constraint::Percentage(p) => {
                    let percent = f64::from(p) / 100.00;
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * percent))?;
                }
                Constraint::Ratio(n, d) => {
                    // avoid division by zero by using 1 when denominator is 0
                    let ratio = f64::from(n) / f64::from(d.max(1));
                    solver.add_constraint(element.size() | EQ(STRONG) | (area_size * ratio))?;
                }
                Constraint::Length(l) => {
                    solver.add_constraint(element.size() | EQ(STRONG) | f64::from(l))?
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
            }
        }
        // prefer equal chunks if other constraints are all satisfied
        if layout.segment_size == SegmentSize::EvenDistribution {
            for (left, right) in elements.iter().tuple_combinations() {
                solver.add_constraint(left.size() | EQ(WEAK) | right.size())?;
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
    fn new() -> Element {
        Element {
            start: Variable::new(),
            end: Variable::new(),
        }
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
                segment_size: SegmentSize::LastTakesRemainder,
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
                segment_size: SegmentSize::LastTakesRemainder,
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
                segment_size: SegmentSize::LastTakesRemainder,
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
    fn segment_size() {
        assert_eq!(
            Layout::default()
                .segment_size(SegmentSize::EvenDistribution)
                .segment_size,
            SegmentSize::EvenDistribution
        );
        assert_eq!(
            Layout::default()
                .segment_size(SegmentSize::LastTakesRemainder)
                .segment_size,
            SegmentSize::LastTakesRemainder
        );
        assert_eq!(
            Layout::default()
                .segment_size(SegmentSize::None)
                .segment_size,
            SegmentSize::None
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

        use crate::{
            assert_buffer_eq,
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
    }
}
