use std::fmt::{self, Display};

use itertools::Itertools;

/// A constraint that can be applied to a layout
///
/// Constraints are used to define the size of a layout. They can be used to define a fixed size, a
/// percentage of the available space, a ratio of the available space, or a minimum or maximum size.
///
/// Relative constraints (percentage, ratio) are calculated relative to the entire space being
/// split, not the space available after applying the more fixed constraints (min, max, length).
///
/// # Examples
///
/// `Constraint` has some helper methods to create lists of constraints from anything that can be
/// converted into an iterator of u16s ((u16, u16) for ratios).
///
/// ```rust
/// # use ratatui::prelude::*;
/// // a fixed layout
/// let constraints = Constraint::from_lengths([10, 20, 10]);
///
/// // a centered layout
/// let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
/// let constraints = Constraint::from_percentages([25, 50, 25]);
///
/// // a centered layout with a minimum size
/// let constraints = Constraint::from_mins([0, 100, 0]);
///
/// // a sidebar layout specifying maximum sizes of the columns
/// let constraints = Constraint::from_maxes([30, 170]);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Constraint {
    /// Apply a percentage to a given amount
    ///
    /// Converts the given percentage to a f32, and then converts it back, trimming off the decimal
    /// point (effectively rounding down)
    /// ```
    /// # use ratatui::prelude::*;
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
    /// # use ratatui::prelude::*;
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
    /// # use ratatui::prelude::*;
    /// assert_eq!(0, Constraint::Length(4).apply(0));
    /// assert_eq!(4, Constraint::Length(4).apply(4));
    /// assert_eq!(4, Constraint::Length(4).apply(10));
    /// ```
    Length(u16),
    /// Apply at most the given amount
    ///
    /// also see [std::cmp::min]
    /// ```
    /// # use ratatui::prelude::*;
    /// assert_eq!(0, Constraint::Max(4).apply(0));
    /// assert_eq!(4, Constraint::Max(4).apply(4));
    /// assert_eq!(4, Constraint::Max(4).apply(10));
    /// ```
    Max(u16),
    /// Apply at least the given amount
    ///
    /// also see [std::cmp::max]
    /// ```
    /// # use ratatui::prelude::*;
    /// assert_eq!(4, Constraint::Min(4).apply(0));
    /// assert_eq!(4, Constraint::Min(4).apply(4));
    /// assert_eq!(10, Constraint::Min(4).apply(10));
    /// ```
    Min(u16),
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

    /// Convert an iterator of lengths into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_lengths([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_lengths<T>(lengths: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        lengths.into_iter().map(Constraint::Length).collect_vec()
    }

    /// Convert an iterator of ratios into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_ratios<T>(ratios: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = (u32, u32)>,
    {
        ratios
            .into_iter()
            .map(|(n, d)| Constraint::Ratio(n, d))
            .collect_vec()
    }

    /// Convert an iterator of percentages into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_percentages([25, 50, 25]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_percentages<T>(percentages: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        percentages
            .into_iter()
            .map(Constraint::Percentage)
            .collect_vec()
    }

    /// Convert an iterator of maxes into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_maxes([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_maxes<T>(maxes: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        maxes.into_iter().map(Constraint::Max).collect_vec()
    }

    /// Convert an iterator of mins into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_mins([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_mins<T>(mins: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        mins.into_iter().map(Constraint::Min).collect_vec()
    }
}

impl AsRef<Constraint> for Constraint {
    fn as_ref(&self) -> &Constraint {
        self
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint::Percentage(100)
    }
}

impl Display for Constraint {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Constraint::default(), Constraint::Percentage(100));
    }

    #[test]
    fn to_string() {
        assert_eq!(Constraint::Percentage(50).to_string(), "Percentage(50)");
        assert_eq!(Constraint::Ratio(1, 2).to_string(), "Ratio(1, 2)");
        assert_eq!(Constraint::Length(10).to_string(), "Length(10)");
        assert_eq!(Constraint::Max(10).to_string(), "Max(10)");
        assert_eq!(Constraint::Min(10).to_string(), "Min(10)");
    }

    #[test]
    fn from_lengths() {
        let expected = [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(3),
        ];
        assert_eq!(Constraint::from_lengths([1, 2, 3]), expected);
        assert_eq!(Constraint::from_lengths(vec![1, 2, 3]), expected);
    }

    #[test]
    fn from_ratios() {
        let expected = [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 4),
        ];
        assert_eq!(Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]), expected);
        assert_eq!(
            Constraint::from_ratios(vec![(1, 4), (1, 2), (1, 4)]),
            expected
        );
    }

    #[test]
    fn from_percentages() {
        let expected = [
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ];
        assert_eq!(Constraint::from_percentages([25, 50, 25]), expected);
        assert_eq!(Constraint::from_percentages(vec![25, 50, 25]), expected);
    }

    #[test]
    fn from_maxes() {
        let expected = [Constraint::Max(1), Constraint::Max(2), Constraint::Max(3)];
        assert_eq!(Constraint::from_maxes([1, 2, 3]), expected);
        assert_eq!(Constraint::from_maxes(vec![1, 2, 3]), expected);
    }

    #[test]
    fn from_mins() {
        let expected = [Constraint::Min(1), Constraint::Min(2), Constraint::Min(3)];
        assert_eq!(Constraint::from_mins([1, 2, 3]), expected);
        assert_eq!(Constraint::from_mins(vec![1, 2, 3]), expected);
    }

    #[test]
    fn apply() {
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
}
