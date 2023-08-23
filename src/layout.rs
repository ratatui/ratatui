use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::HashMap,
    fmt,
    rc::Rc,
};

use cassowary::{
    strength::{MEDIUM, REQUIRED, STRONG, WEAK},
    AddConstraintError, Expression, Solver, Variable,
    WeightedRelation::{EQ, GE, LE},
};
use itertools::Itertools;
use strum::{Display, EnumString};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Corner {
    #[default]
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

/// Constraints to apply
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Constraint {
    /// Apply a percentage to a given amount
    ///
    /// Converts the given percentage to a f32, and then converts it back, trimming off the decimal
    /// point (effectively rounding down)
    /// ```
    /// # use ratatui::prelude::Constraint;
    /// assert_eq!(0, Constraint::Percentage(50).apply(0));
    /// assert_eq!(2, Constraint::Percentage(50).apply(4));
    /// assert_eq!(5, Constraint::Percentage(50).apply(10));
    /// assert_eq!(5, Constraint::Percentage(50).apply(11));
    /// ```
    Percentage(u16),
    /// Apply a ratio
    ///
    /// Converts the given numbers to a f32, and then converts it back, trimming off the decimal
    /// point (effectively rounding down)
    /// ```
    /// # use ratatui::prelude::Constraint;
    /// assert_eq!(0, Constraint::Ratio(4, 3).apply(0));
    /// assert_eq!(4, Constraint::Ratio(4, 3).apply(4));
    /// assert_eq!(10, Constraint::Ratio(4, 3).apply(10));
    /// assert_eq!(100, Constraint::Ratio(4, 3).apply(100));
    ///
    /// assert_eq!(0, Constraint::Ratio(3, 4).apply(0));
    /// assert_eq!(3, Constraint::Ratio(3, 4).apply(4));
    /// assert_eq!(7, Constraint::Ratio(3, 4).apply(10));
    /// assert_eq!(75, Constraint::Ratio(3, 4).apply(100));
    /// ```
    Ratio(u32, u32),
    /// Apply no more than the given amount (currently roughly equal to [Constraint::Max], but less
    /// consistent)
    /// ```
    /// # use ratatui::prelude::Constraint;
    /// assert_eq!(0, Constraint::Length(4).apply(0));
    /// assert_eq!(4, Constraint::Length(4).apply(4));
    /// assert_eq!(4, Constraint::Length(4).apply(10));
    /// ```
    Length(u16),
    /// Apply at most the given amount
    ///
    /// also see [std::cmp::min]
    /// ```
    /// # use ratatui::prelude::Constraint;
    /// assert_eq!(0, Constraint::Max(4).apply(0));
    /// assert_eq!(4, Constraint::Max(4).apply(4));
    /// assert_eq!(4, Constraint::Max(4).apply(10));
    /// ```
    Max(u16),
    /// Apply at least the given amount
    ///
    /// also see [std::cmp::max]
    /// ```
    /// # use ratatui::prelude::Constraint;
    /// assert_eq!(4, Constraint::Min(4).apply(0));
    /// assert_eq!(4, Constraint::Min(4).apply(4));
    /// assert_eq!(10, Constraint::Min(4).apply(10));
    /// ```
    Min(u16),
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint::Percentage(100)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Percentage(p) => write!(f, "Percentage({})", p),
            Constraint::Ratio(n, d) => write!(f, "Ratio({}, {})", n, d),
            Constraint::Length(l) => write!(f, "Length({})", l),
            Constraint::Max(m) => write!(f, "Max({})", m),
            Constraint::Min(m) => write!(f, "Min({})", m),
        }
    }
}

impl Constraint {
    pub fn apply(&self, length: u16) -> u16 {
        match *self {
            Constraint::Percentage(p) => {
                let p = p as f32 / 100.0;
                let length = length as f32;
                (p * length).min(length) as u16
            }
            Constraint::Ratio(numerator, denominator) => {
                // avoid division by zero by using 1 when denominator is 0
                // this results in 0/0 -> 0 and x/0 -> x for x != 0
                let percentage = numerator as f32 / denominator.max(1) as f32;
                let length = length as f32;
                (percentage * length).min(length) as u16
            }
            Constraint::Length(l) => length.min(l),
            Constraint::Max(m) => length.min(m),
            Constraint::Min(m) => length.max(m),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub const fn new(horizontal: u16, vertical: u16) -> Margin {
        Margin {
            horizontal,
            vertical,
        }
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.horizontal, self.vertical)
    }
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

/// A simple rectangle used in the computation of the layout and to give widgets a hint about the
/// area they are supposed to render to.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
    }
}

impl Rect {
    /// Creates a new rect, with width and height limited to keep the area under max u16.
    /// If clipped, aspect ratio will be preserved.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        let max_area = u16::max_value();
        let (clipped_width, clipped_height) =
            if u32::from(width) * u32::from(height) > u32::from(max_area) {
                let aspect_ratio = f64::from(width) / f64::from(height);
                let max_area_f = f64::from(max_area);
                let height_f = (max_area_f / aspect_ratio).sqrt();
                let width_f = height_f * aspect_ratio;
                (width_f as u16, height_f as u16)
            } else {
                (width, height)
            };
        Rect {
            x,
            y,
            width: clipped_width,
            height: clipped_height,
        }
    }

    pub const fn area(self) -> u16 {
        self.width * self.height
    }

    pub const fn left(self) -> u16 {
        self.x
    }

    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub const fn top(self) -> u16 {
        self.y
    }

    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn inner(self, margin: &Margin) -> Rect {
        if self.width < 2 * margin.horizontal || self.height < 2 * margin.vertical {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin.horizontal,
                y: self.y + margin.vertical,
                width: self.width - 2 * margin.horizontal,
                height: self.height - 2 * margin.vertical,
            }
        }
    }

    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.x + self.width, other.x + other.width);
        let y2 = max(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.x + self.width, other.x + other.width);
        let y2 = min(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub const fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub(crate) enum SegmentSize {
    EvenDistribution,
    #[default]
    LastTakesRemainder,
    None,
}

/// A layout is a set of constraints that can be applied to a given area to split it into smaller
/// ones.
///
/// A layout is composed of:
/// - a direction (horizontal or vertical)
/// - a set of constraints (length, ratio, percentage, min, max)
/// - a margin (horizontal and vertical), the space between the edge of the main area and the split
///   areas
///
/// The algorithm used to compute the layout is based on the [`cassowary-rs`] solver. It is a simple
/// linear solver that can be used to solve linear equations and inequalities. In our case, we
/// define a set of constraints that are applied to split the provided area into Rects aligned in a
/// single direction, and the solver computes the values of the position and sizes that satisfy as
/// many of the constraints as possible.
///
/// By default, the last chunk of the computed layout is expanded to fill the remaining space. To
/// avoid this behavior, add an unused `Constraint::Min(0)` as the last constraint.
///
/// When the layout is computed, the result is cached in a thread-local cache, so that subsequent
/// calls with the same parameters are faster. The cache is a simple HashMap, and grows
/// indefinitely. (See <https://github.com/ratatui-org/ratatui/issues/402> for more information)
///
/// # Example
///
/// ```rust
/// # use ratatui::prelude::*;
/// # use ratatui::widgets::Paragraph;
/// fn render<B: Backend>(frame: &mut Frame<B>, area: Rect) {
///     let layout = Layout::default()
///         .direction(Direction::Vertical)
///         .constraints(vec![Constraint::Length(5), Constraint::Min(0)])
///         .split(Rect::new(0, 0, 10, 10));
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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Layout {
    direction: Direction,
    margin: Margin,
    constraints: Vec<Constraint>,
    /// option for segment size preferences
    segment_size: SegmentSize,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout::new()
    }
}

impl Layout {
    /// Creates a new layout with default values.
    ///
    /// - direction: [Direction::Vertical]
    /// - margin: 0, 0
    /// - constraints: empty
    /// - segment_size: SegmentSize::LastTakesRemainder
    pub const fn new() -> Layout {
        Layout {
            direction: Direction::Vertical,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
            constraints: Vec::new(),
            segment_size: SegmentSize::LastTakesRemainder,
        }
    }

    /// Builder method to set the constraints of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints(vec![
    ///         Constraint::Percentage(20),
    ///         Constraint::Ratio(1, 5),
    ///         Constraint::Length(2),
    ///         Constraint::Min(2),
    ///         Constraint::Max(2),
    ///     ])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [
    ///     Rect::new(0, 0, 10, 2),
    ///     Rect::new(0, 2, 10, 2),
    ///     Rect::new(0, 4, 10, 2),
    ///     Rect::new(0, 6, 10, 2),
    ///     Rect::new(0, 8, 10, 2),
    /// ]);
    /// ```
    pub fn constraints<C>(mut self, constraints: C) -> Layout
    where
        C: Into<Vec<Constraint>>,
    {
        self.constraints = constraints.into();
        self
    }

    /// Builder method to set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints(vec![Constraint::Min(0)])
    ///     .margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
    /// ```
    pub const fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    /// Builder method to set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints(vec![Constraint::Min(0)])
    ///     .horizontal_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
    /// ```
    pub const fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    /// Builder method to set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints(vec![Constraint::Min(0)])
    ///     .vertical_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
    /// ```
    pub const fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

    /// Builder method to set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints(vec![Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints(vec![Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
    /// ```
    pub const fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    /// Builder method to set whether chunks should be of equal size.
    pub(crate) const fn segment_size(mut self, segment_size: SegmentSize) -> Layout {
        self.segment_size = segment_size;
        self
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given area into
    /// smaller ones based on the preferred widths or heights and the direction.
    ///
    /// This method stores the result of the computation in a thread-local cache keyed on the layout
    /// and area, so that subsequent calls with the same parameters are faster. The cache is a
    /// simple HashMap, and grows indefinitely (<https://github.com/ratatui-org/ratatui/issues/402>).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints(vec![Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(2, 2, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 10, 5), Rect::new(2, 7, 10, 5)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints(vec![Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    ///     .split(Rect::new(0, 0, 9, 2));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 3, 2), Rect::new(3, 0, 6, 2)]);
    /// ```
    pub fn split(&self, area: Rect) -> Rc<[Rect]> {
        LAYOUT_CACHE.with(|c| {
            c.borrow_mut()
                .entry((area, self.clone()))
                .or_insert_with(|| split(area, self))
                .clone()
        })
    }
}

type Cache = HashMap<(Rect, Layout), Rc<[Rect]>>;
thread_local! {
    // TODO: Maybe use a fixed size cache https://github.com/ratatui-org/ratatui/issues/402
    static LAYOUT_CACHE: RefCell<Cache> = RefCell::new(HashMap::new());
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
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

fn split(area: Rect, layout: &Layout) -> Rc<[Rect]> {
    try_split(area, layout).expect("failed to split")
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
    // ensure the last element touches the right/bottom edge of the area
    if layout.segment_size != SegmentSize::None {
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

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::{SegmentSize::*, *};
    use crate::prelude::Constraint::*;

    #[test]
    fn corner_to_string() {
        assert_eq!(Corner::BottomLeft.to_string(), "BottomLeft");
        assert_eq!(Corner::BottomRight.to_string(), "BottomRight");
        assert_eq!(Corner::TopLeft.to_string(), "TopLeft");
        assert_eq!(Corner::TopRight.to_string(), "TopRight");
    }

    #[test]
    fn corner_from_str() {
        assert_eq!("BottomLeft".parse::<Corner>(), Ok(Corner::BottomLeft));
        assert_eq!("BottomRight".parse::<Corner>(), Ok(Corner::BottomRight));
        assert_eq!("TopLeft".parse::<Corner>(), Ok(Corner::TopLeft));
        assert_eq!("TopRight".parse::<Corner>(), Ok(Corner::TopRight));
        assert_eq!("".parse::<Corner>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn direction_to_string() {
        assert_eq!(Direction::Horizontal.to_string(), "Horizontal");
        assert_eq!(Direction::Vertical.to_string(), "Vertical");
    }

    #[test]
    fn direction_from_str() {
        assert_eq!("Horizontal".parse::<Direction>(), Ok(Direction::Horizontal));
        assert_eq!("Vertical".parse::<Direction>(), Ok(Direction::Vertical));
        assert_eq!("".parse::<Direction>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn constraint_to_string() {
        assert_eq!(Constraint::Percentage(50).to_string(), "Percentage(50)");
        assert_eq!(Constraint::Ratio(1, 2).to_string(), "Ratio(1, 2)");
        assert_eq!(Constraint::Length(10).to_string(), "Length(10)");
        assert_eq!(Constraint::Max(10).to_string(), "Max(10)");
        assert_eq!(Constraint::Min(10).to_string(), "Min(10)");
    }

    #[test]
    fn margin_to_string() {
        assert_eq!(Margin::new(1, 2).to_string(), "1x2");
    }

    #[test]
    fn margin_new() {
        assert_eq!(
            Margin::new(1, 2),
            Margin {
                horizontal: 1,
                vertical: 2
            }
        );
    }

    #[test]
    fn alignment_to_string() {
        assert_eq!(Alignment::Left.to_string(), "Left");
        assert_eq!(Alignment::Center.to_string(), "Center");
        assert_eq!(Alignment::Right.to_string(), "Right");
    }

    #[test]
    fn alignment_from_str() {
        assert_eq!("Left".parse::<Alignment>(), Ok(Alignment::Left));
        assert_eq!("Center".parse::<Alignment>(), Ok(Alignment::Center));
        assert_eq!("Right".parse::<Alignment>(), Ok(Alignment::Right));
        assert_eq!("".parse::<Alignment>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn rect_to_string() {
        assert_eq!(Rect::new(1, 2, 3, 4).to_string(), "3x4+1+2");
    }

    #[test]
    fn rect_new() {
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn rect_area() {
        assert_eq!(Rect::new(1, 2, 3, 4).area(), 12);
    }

    #[test]
    fn rect_left() {
        assert_eq!(Rect::new(1, 2, 3, 4).left(), 1);
    }

    #[test]
    fn rect_right() {
        assert_eq!(Rect::new(1, 2, 3, 4).right(), 4);
    }

    #[test]
    fn rect_top() {
        assert_eq!(Rect::new(1, 2, 3, 4).top(), 2);
    }

    #[test]
    fn rect_bottom() {
        assert_eq!(Rect::new(1, 2, 3, 4).bottom(), 6);
    }

    #[test]
    fn rect_inner() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).inner(&Margin::new(1, 2)),
            Rect::new(2, 4, 1, 0)
        );
    }

    #[test]
    fn rect_union() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).union(Rect::new(2, 3, 4, 5)),
            Rect::new(1, 2, 5, 6)
        );
    }

    #[test]
    fn rect_intersection() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).intersection(Rect::new(2, 3, 4, 5)),
            Rect::new(2, 3, 2, 3)
        );
    }

    #[test]
    fn rect_intersects() {
        assert!(Rect::new(1, 2, 3, 4).intersects(Rect::new(2, 3, 4, 5)));
        assert!(!Rect::new(1, 2, 3, 4).intersects(Rect::new(5, 6, 7, 8)));
    }

    #[test]
    fn segment_size_to_string() {
        assert_eq!(
            SegmentSize::EvenDistribution.to_string(),
            "EvenDistribution"
        );
        assert_eq!(
            SegmentSize::LastTakesRemainder.to_string(),
            "LastTakesRemainder"
        );
        assert_eq!(SegmentSize::None.to_string(), "None");
    }

    #[test]
    fn segment_size_from_string() {
        assert_eq!(
            "EvenDistribution".parse::<SegmentSize>(),
            Ok(EvenDistribution)
        );
        assert_eq!(
            "LastTakesRemainder".parse::<SegmentSize>(),
            Ok(LastTakesRemainder)
        );
        assert_eq!("None".parse::<SegmentSize>(), Ok(None));
        assert_eq!("".parse::<SegmentSize>(), Err(ParseError::VariantNotFound));
    }

    fn get_x_width_with_segment_size(
        segment_size: SegmentSize,
        constraints: Vec<Constraint>,
        target: Rect,
    ) -> Vec<(u16, u16)> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .segment_size(segment_size);
        let chunks = layout.split(target);
        chunks.iter().map(|r| (r.x, r.width)).collect()
    }

    #[test]
    fn test_split_equally_in_underspecified_case() {
        let target = Rect::new(100, 200, 10, 10);
        assert_eq!(
            get_x_width_with_segment_size(LastTakesRemainder, vec![Min(2), Min(2), Min(0)], target),
            [(100, 2), (102, 2), (104, 6)]
        );
        assert_eq!(
            get_x_width_with_segment_size(EvenDistribution, vec![Min(2), Min(2), Min(0)], target),
            [(100, 3), (103, 4), (107, 3)]
        );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_min() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(50), Min(10), Percentage(50)],
                target
            ),
            [(100, 50), (150, 10), (160, 40)]
        );
        assert_eq!(
            get_x_width_with_segment_size(
                EvenDistribution,
                vec![Percentage(50), Min(10), Percentage(50)],
                target
            ),
            [(100, 45), (145, 10), (155, 45)]
        );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_max() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(30), Max(10), Percentage(30)],
                target
            ),
            [(100, 30), (130, 10), (140, 60)]
        );
        assert_eq!(
            get_x_width_with_segment_size(
                EvenDistribution,
                vec![Percentage(30), Max(10), Percentage(30)],
                target
            ),
            [(100, 45), (145, 10), (155, 45)]
        );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_length() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(50), Length(10), Percentage(50)],
                target
            ),
            [(100, 50), (150, 10), (160, 40)]
        );
        assert_eq!(
            get_x_width_with_segment_size(
                EvenDistribution,
                vec![Percentage(50), Length(10), Percentage(50)],
                target
            ),
            [(100, 45), (145, 10), (155, 45)]
        );
    }

    #[test]
    fn test_rect_size_truncation() {
        for width in 256u16..300u16 {
            for height in 256u16..300u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert!(rect.width < width || rect.height < height);
                // The target dimensions are rounded down so the math will not be too precise
                // but let's make sure the ratios don't diverge crazily.
                assert!(
                    (f64::from(rect.width) / f64::from(rect.height)
                        - f64::from(width) / f64::from(height))
                    .abs()
                        < 1.0
                );
            }
        }

        // One dimension below 255, one above. Area above max u16.
        let width = 900;
        let height = 100;
        let rect = Rect::new(0, 0, width, height);
        assert_ne!(rect.width, 900);
        assert_ne!(rect.height, 100);
        assert!(rect.width < width || rect.height < height);
    }

    #[test]
    fn test_rect_size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        // One dimension below 255, one above. Area below max u16.
        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }

    #[test]
    fn test_constraint_apply() {
        assert_eq!(Constraint::Percentage(0).apply(100), 0);
        assert_eq!(Constraint::Percentage(50).apply(100), 50);
        assert_eq!(Constraint::Percentage(100).apply(100), 100);
        assert_eq!(Constraint::Percentage(200).apply(100), 100);
        assert_eq!(Constraint::Percentage(u16::MAX).apply(100), 100);

        // 0/0 intentionally avoids a panic by returning 0.
        assert_eq!(Constraint::Ratio(0, 0).apply(100), 0);
        // 1/0 intentionally avoids a panic by returning 100% of the length.
        assert_eq!(Constraint::Ratio(1, 0).apply(100), 100);
        assert_eq!(Constraint::Ratio(0, 1).apply(100), 0);
        assert_eq!(Constraint::Ratio(1, 2).apply(100), 50);
        assert_eq!(Constraint::Ratio(2, 2).apply(100), 100);
        assert_eq!(Constraint::Ratio(3, 2).apply(100), 100);
        assert_eq!(Constraint::Ratio(u32::MAX, 2).apply(100), 100);

        assert_eq!(Constraint::Length(0).apply(100), 0);
        assert_eq!(Constraint::Length(50).apply(100), 50);
        assert_eq!(Constraint::Length(100).apply(100), 100);
        assert_eq!(Constraint::Length(200).apply(100), 100);
        assert_eq!(Constraint::Length(u16::MAX).apply(100), 100);

        assert_eq!(Constraint::Max(0).apply(100), 0);
        assert_eq!(Constraint::Max(50).apply(100), 50);
        assert_eq!(Constraint::Max(100).apply(100), 100);
        assert_eq!(Constraint::Max(200).apply(100), 100);
        assert_eq!(Constraint::Max(u16::MAX).apply(100), 100);

        assert_eq!(Constraint::Min(0).apply(100), 100);
        assert_eq!(Constraint::Min(50).apply(100), 100);
        assert_eq!(Constraint::Min(100).apply(100), 100);
        assert_eq!(Constraint::Min(200).apply(100), 200);
        assert_eq!(Constraint::Min(u16::MAX).apply(100), u16::MAX);
    }

    #[test]
    fn rect_can_be_const() {
        const RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        const _AREA: u16 = RECT.area();
        const _LEFT: u16 = RECT.left();
        const _RIGHT: u16 = RECT.right();
        const _TOP: u16 = RECT.top();
        const _BOTTOM: u16 = RECT.bottom();
        assert!(RECT.intersects(RECT));
    }

    #[test]
    fn layout_can_be_const() {
        const _LAYOUT: Layout = Layout::new();
        const _DEFAULT_LAYOUT: Layout = Layout::new()
            .direction(Direction::Horizontal)
            .margin(1)
            .segment_size(SegmentSize::LastTakesRemainder);
        const _HORIZONTAL_LAYOUT: Layout = Layout::new().horizontal_margin(1);
        const _VERTICAL_LAYOUT: Layout = Layout::new().vertical_margin(1);
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
    mod layout_split {
        use pretty_assertions::assert_eq;

        use crate::{
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
            assert_eq!(buffer.content, Buffer::with_lines(vec![expected]).content);
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
                .constraints(vec![
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
                .constraints(vec![
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
                .constraints(vec![Min(1), Length(0), Min(1)])
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
                .constraints(vec![Length(3), Min(4), Length(1), Min(4)])
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
