use std::fmt::{self, Display};

use itertools::Itertools;
use strum::EnumIs;

/// A constraint that defines the size of a layout element.
///
/// Constraints can be used to specify a fixed size, a percentage of the available space, a ratio of
/// the available space, a minimum or maximum size or a proportional value for a layout element.
///
/// Relative constraints (percentage, ratio) are calculated relative to the entire space being
/// divided, rather than the space available after applying more fixed constraints (min, max,
/// length).
///
/// Constraints are prioritized in the following order:
///
/// 1. [`Constraint::Fixed`]
/// 2. [`Constraint::Min`]
/// 3. [`Constraint::Max`]
/// 4. [`Constraint::Length`]
/// 5. [`Constraint::Percentage`]
/// 6. [`Constraint::Ratio`]
/// 7. [`Constraint::Proportional`]
///
/// # Examples
///
/// `Constraint` provides helper methods to create lists of constraints from various input formats.
///
/// ```rust
/// # use ratatui::prelude::*;
/// // Create a layout with specified lengths for each element
/// let constraints = Constraint::from_lengths([10, 20, 10]);
///
/// // Create a layout with specified fixed lengths for each element
/// let constraints = Constraint::from_fixed_lengths([10, 20, 10]);
///
/// // Create a centered layout using ratio or percentage constraints
/// let constraints = Constraint::from_ratios([(1, 4), (1, 2), (1, 4)]);
/// let constraints = Constraint::from_percentages([25, 50, 25]);
///
/// // Create a centered layout with a minimum size constraint for specific elements
/// let constraints = Constraint::from_mins([0, 100, 0]);
///
/// // Create a sidebar layout specifying maximum sizes for the columns
/// let constraints = Constraint::from_maxes([30, 170]);
///
/// // Create a layout with proportional sizes for each element
/// let constraints = Constraint::from_proportional_lengths([1, 2, 1]);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIs)]
pub enum Constraint {
    /// Applies a fixed size to the element
    ///
    /// The element size is set to the specified amount.
    /// [`Constraint::Fixed`] will take precedence over all other constraints.
    ///
    /// # Examples
    ///
    /// `[Fixed(40), Proportional(1)]`
    ///
    /// ```plain
    /// ┌──────────────────────────────────────┐┌────────┐
    /// │                 40 px                ││  10 px │
    /// └──────────────────────────────────────┘└────────┘
    /// ```
    ///
    /// `[Fixed(20), Fixed(20), Proportional(1)]`
    ///
    /// ```plain
    /// ┌──────────────────┐┌──────────────────┐┌────────┐
    /// │       20 px      ││       20 px      ││  10 px │
    /// └──────────────────┘└──────────────────┘└────────┘
    /// ```
    Fixed(u16),
    /// Applies a minimum size constraint to the element
    ///
    /// The element size is set to at least the specified amount.
    ///
    /// # Examples
    ///
    /// `[Percentage(100), Min(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Percentage(100), Min(10)]`
    ///
    /// ```plain
    /// ┌──────────────────────────────────────┐┌────────┐
    /// │                 40 px                ││  10 px │
    /// └──────────────────────────────────────┘└────────┘
    /// ```
    Min(u16),
    /// Applies a maximum size constraint to the element
    ///
    /// The element size is set to at most the specified amount.
    ///
    /// # Examples
    ///
    /// `[Percentage(100), Min(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Percentage(100), Min(10)]`
    ///
    /// ```plain
    /// ┌──────────────────────────────────────┐┌────────┐
    /// │                 40 px                ││  10 px │
    /// └──────────────────────────────────────┘└────────┘
    /// ```
    Max(u16),
    /// Applies a length constraint to the element
    ///
    /// The element size is set to the specified amount.
    ///
    /// # Examples
    ///
    /// `[Length(20), Fixed(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Length(20), Length(20)]`
    ///
    /// ```plain
    /// ┌──────────────────┐┌────────────────────────────┐
    /// │       20 px      ││            30 px           │
    /// └──────────────────┘└────────────────────────────┘
    /// ```
    Length(u16),
    /// Applies a percentage of the available space to the element
    ///
    /// Converts the given percentage to a floating-point value and multiplies that with area.
    /// This value is rounded back to a integer as part of the layout split calculation.
    ///
    /// # Examples
    ///
    /// `[Percentage(75), Proportional(1)]`
    ///
    /// ```plain
    /// ┌────────────────────────────────────┐┌──────────┐
    /// │                38 px               ││   12 px  │
    /// └────────────────────────────────────┘└──────────┘
    /// ```
    ///
    /// `[Percentage(50), Proportional(1)]`
    ///
    /// ```plain
    /// ┌───────────────────────┐┌───────────────────────┐
    /// │         25 px         ││         25 px         │
    /// └───────────────────────┘└───────────────────────┘
    /// ```
    Percentage(u16),
    /// Applies a ratio of the available space to the element
    ///
    /// Converts the given ratio to a floating-point value and multiplies that with area.
    /// This value is rounded back to a integer as part of the layout split calculation.
    ///
    /// # Examples
    ///
    /// `[Ratio(1, 2) ; 2]`
    ///
    /// ```plain
    /// ┌───────────────────────┐┌───────────────────────┐
    /// │         25 px         ││         25 px         │
    /// └───────────────────────┘└───────────────────────┘
    /// ```
    ///
    /// `[Ratio(1, 4) ; 4]`
    ///
    /// ```plain
    /// ┌───────────┐┌──────────┐┌───────────┐┌──────────┐
    /// │   13 px   ││   12 px  ││   13 px   ││   12 px  │
    /// └───────────┘└──────────┘└───────────┘└──────────┘
    /// ```
    Ratio(u32, u32),
    /// Applies the scaling factor proportional to all other [`Constraint::Proportional`] elements
    /// to fill excess space
    ///
    /// The element will only expand into excess available space, proportionally matching other
    /// [`Constraint::Proportional`] elements while satisfying all other constraints.
    ///
    /// # Examples
    ///
    ///
    /// `[Proportional(1), Proportional(2), Proportional(3)]`
    ///
    /// ```plain
    /// ┌──────┐┌───────────────┐┌───────────────────────┐
    /// │ 8 px ││     17 px     ││         25 px         │
    /// └──────┘└───────────────┘└───────────────────────┘
    /// ```
    ///
    /// `[Proportional(1), Percentage(50), Proportional(1)]`
    ///
    /// ```plain
    /// ┌───────────┐┌───────────────────────┐┌──────────┐
    /// │   13 px   ││         25 px         ││   12 px  │
    /// └───────────┘└───────────────────────┘└──────────┘
    /// ```
    Proportional(u16),
}

impl Constraint {
    #[deprecated(
        since = "0.26.0",
        note = "This field will be hidden in the next minor version."
    )]
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
            Constraint::Fixed(l) => length.min(l),
            Constraint::Proportional(l) => length.min(l),
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

    /// Convert an iterator of fixed lengths into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_fixed_lengths([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_fixed_lengths<T>(fixed_lengths: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        fixed_lengths
            .into_iter()
            .map(Constraint::Fixed)
            .collect_vec()
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

    /// Convert an iterator of proportional factors into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_mins([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_proportional_lengths<T>(proportional_lengths: T) -> Vec<Constraint>
    where
        T: IntoIterator<Item = u16>,
    {
        proportional_lengths
            .into_iter()
            .map(Constraint::Proportional)
            .collect_vec()
    }
}

impl From<u16> for Constraint {
    /// Convert a u16 into a [Constraint::Length]
    ///
    /// This is useful when you want to specify a fixed size for a layout, but don't want to
    /// explicitly create a [Constraint::Length] yourself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let layout = Layout::new(Direction::Vertical, [1, 2, 3]).split(area);
    /// let layout = Layout::horizontal([1, 2, 3]).split(area);
    /// let layout = Layout::vertical([1, 2, 3]).split(area);
    /// ````
    fn from(length: u16) -> Constraint {
        Constraint::Length(length)
    }
}

impl From<&Constraint> for Constraint {
    fn from(constraint: &Constraint) -> Self {
        *constraint
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
            Constraint::Fixed(l) => write!(f, "Fixed({})", l),
            Constraint::Proportional(l) => write!(f, "Proportional({})", l),
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
    fn from_fixed_lengths() {
        let expected = [
            Constraint::Fixed(1),
            Constraint::Fixed(2),
            Constraint::Fixed(3),
        ];
        assert_eq!(Constraint::from_fixed_lengths([1, 2, 3]), expected);
        assert_eq!(Constraint::from_fixed_lengths(vec![1, 2, 3]), expected);
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
    fn from_proportional_lengths() {
        let expected = [
            Constraint::Proportional(1),
            Constraint::Proportional(2),
            Constraint::Proportional(3),
        ];
        assert_eq!(Constraint::from_proportional_lengths([1, 2, 3]), expected);
        assert_eq!(
            Constraint::from_proportional_lengths(vec![1, 2, 3]),
            expected
        );
    }

    #[test]
    #[allow(deprecated)]
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
