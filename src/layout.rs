use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Corner {
    #[default]
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

/// Constraints to apply  
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Constraint {
    /// Apply a percentage to a given amount  
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
    pub vertical: u16,
    pub horizontal: u16,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Layout {
    direction: Direction,
    margin: Margin,
    constraints: Vec<Constraint>,
    /// Whether the last chunk of the computed layout should be expanded to fill the available
    /// space.
    expand_to_fill: bool,
}

type Cache = HashMap<(Rect, Layout), Rc<[Rect]>>;
thread_local! {
    static LAYOUT_CACHE: RefCell<Cache> = RefCell::new(HashMap::new());
}

impl Default for Layout {
    fn default() -> Layout {
        Layout::new()
    }
}

use log::*;

impl Layout {
    pub const fn new() -> Layout {
        Layout {
            direction: Direction::Vertical,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
            constraints: Vec::new(),
            expand_to_fill: true,
        }
    }

    pub fn constraints<C>(mut self, constraints: C) -> Layout
    where
        C: Into<Vec<Constraint>>,
    {
        self.constraints = constraints.into();
        self
    }

    pub const fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    pub const fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    pub const fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

    pub const fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    pub(crate) const fn expand_to_fill(mut self, expand_to_fill: bool) -> Layout {
        self.expand_to_fill = expand_to_fill;
        self
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given
    /// area into smaller ones based on the preferred widths or heights and the direction.
    ///
    /// # Examples
    /// ```
    /// # use ratatui::layout::{Rect, Constraint, Direction, Layout};
    /// let chunks = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
    ///     .split(Rect {
    ///         x: 2,
    ///         y: 2,
    ///         width: 10,
    ///         height: 10,
    ///     });
    /// assert_eq!(
    ///     chunks[..],
    ///     [
    ///         Rect {
    ///             x: 2,
    ///             y: 2,
    ///             width: 10,
    ///             height: 5
    ///         },
    ///         Rect {
    ///             x: 2,
    ///             y: 7,
    ///             width: 10,
    ///             height: 5
    ///         }
    ///     ]
    /// );
    ///
    /// let chunks = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
    ///     .split(Rect {
    ///         x: 0,
    ///         y: 0,
    ///         width: 9,
    ///         height: 2,
    ///     });
    /// assert_eq!(
    ///     chunks[..],
    ///     [
    ///         Rect {
    ///             x: 0,
    ///             y: 0,
    ///             width: 3,
    ///             height: 2
    ///         },
    ///         Rect {
    ///             x: 3,
    ///             y: 0,
    ///             width: 6,
    ///             height: 2
    ///         }
    ///     ]
    /// );
    /// ```
    pub fn split(&self, area: Rect) -> Rc<[Rect]> {
        // TODO: Maybe use a fixed size cache ?
        LAYOUT_CACHE.with(|c| {
            c.borrow_mut()
                .entry((area, self.clone()))
                .or_insert_with(|| split(area, self))
                .clone()
        })
    }
}

fn split(area: Rect, layout: &Layout) -> Rc<[Rect]> {
    let mut res = layout
        .constraints
        .iter()
        .map(|_| Rect::default())
        .collect::<Rc<[Rect]>>();

    let results = Rc::get_mut(&mut res).expect("newly created Rc should have no shared refs");

    let dest_area = area.inner(&layout.margin);
    {
        use taffy::prelude::*;
        let mut taffy = Taffy::with_capacity(layout.constraints.len());
        let mut taffy_keys = Vec::with_capacity(layout.constraints.len());

        // disabling rounding, because otherwise the following tests fail:
        // widgets_table_columns_widths_can_use_mixed_constraints
        // widgets_table_column_spacing_can_be_changed
        //
        // but disabling rounding makes the following tests fail:
        // layout::tests::test_vertical_split_by_height
        taffy.disable_rounding();

        for elt in &layout.constraints {
            // alias for dest_area width or height, depending on layout.direction
            let dest_area_var = match layout.direction {
                Direction::Horizontal => dest_area.width,
                Direction::Vertical => dest_area.height,
            };

            // value to use for the dimension (special case for percent)
            let elt_applied = match elt {
                // special case for percent, because it could be directly supported by taffy
                // Constraint::Percentage(v) => (*v as f32) / 100.0,
                _ => elt.apply(dest_area_var) as f32,
            };

            // minimal size for a constraint
            let min_size = match elt {
                Constraint::Length(v) => *v as f32,
                Constraint::Min(v) => *v as f32,
                _ => 0 as f32,
            } as f32;
            // maximal size for a constraint
            let max_size = match elt {
                // need to re-use the same calculation as elt_applied
                // Constraint::Percentage(v) => (*v as f32) / 100.0,
                Constraint::Length(v) => *v as f32,
                Constraint::Max(v) => *v as f32,
                _ => u16::MAX as f32,
            } as f32;

            // which dimension type to use
            let dim = match elt {
                // see comment on "elt_applied"
                // Constraint::Percentage(_) => Dimension::Percent,
                _ => Dimension::Points,
            };

            match layout.direction {
                Direction::Horizontal => taffy_keys.push(
                    taffy
                        .new_leaf(Style {
                            // add node only with "width" set, because of layout.direction
                            flex_basis: dim(elt_applied),
                            min_size: Size {
                                width: dim(min_size),
                                height: zero(),
                            },
                            max_size: Size {
                                width: dim(max_size),
                                height: auto(),
                            },
                            ..Default::default()
                        })
                        .unwrap(),
                ),
                Direction::Vertical => taffy_keys.push(
                    taffy
                        .new_leaf(Style {
                            // add node only with "height" set, because of layout.direction
                            flex_basis: dim(elt_applied),
                            min_size: Size {
                                width: zero(),
                                height: dim(min_size),
                            },
                            max_size: Size {
                                width: auto(),
                                height: dim(max_size),
                            },
                            ..Default::default()
                        })
                        .unwrap(),
                ),
            };
        }

        let root_node = taffy
            .new_with_children(
                Style {
                    // setting flex_direction, because otherwise "Column" would be used for
                    // "Horizontal", which makes it basically render 0 length
                    flex_direction: match layout.direction {
                        Direction::Horizontal => FlexDirection::Row,
                        Direction::Vertical => FlexDirection::Column,
                    },
                    // root node needs to take up full space of available_space later, otherwise the
                    // child elements are all length 0
                    size: Size {
                        width: percent(1.0),
                        height: percent(1.0),
                    },
                    ..Default::default()
                },
                &taffy_keys,
            )
            .unwrap();

        taffy
            .compute_layout(
                root_node,
                Size {
                    width: AvailableSpace::Definite(dest_area.width as f32),
                    height: AvailableSpace::Definite(dest_area.height as f32),
                },
            )
            .unwrap();

        // assign computed values back to the result array
        for (i, key) in taffy_keys.iter().enumerate() {
            let computed = taffy.layout(*key).unwrap();

            results[i].width = computed.size.width as u16;
            results[i].height = computed.size.height as u16;
            results[i].x = computed.location.x as u16 + dest_area.x;
            results[i].y = computed.location.y as u16 + dest_area.y;
        }
    }

    return res;
}

/// A simple rectangle used in the computation of the layout and to give widgets a hint about the
/// area they are supposed to render to.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_split_by_height() {
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
            .expand_to_fill(false);
        const _HORIZONTAL_LAYOUT: Layout = Layout::new().horizontal_margin(1);
        const _VERTICAL_LAYOUT: Layout = Layout::new().vertical_margin(1);
    }
}
