use std::fmt;

use itertools::Itertools;
use strum::EnumIs;

/// A constraint that defines the size of a layout element.
///
/// Constraints can be used to specify a fixed size, a percentage of the available space, a ratio of
/// the available space, a minimum or maximum size or a fill proportional value for a layout
/// element.
///
/// Relative constraints (percentage, ratio) are calculated relative to the entire space being
/// divided, rather than the space available after applying more fixed constraints (min, max,
/// length).
///
/// Constraints are prioritized in the following order:
///
/// 1. [`Constraint::Min`]
/// 2. [`Constraint::Max`]
/// 3. [`Constraint::Length`]
/// 4. [`Constraint::Percentage`]
/// 5. [`Constraint::Ratio`]
/// 6. [`Constraint::Fill`]
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
/// // Create a layout with fill proportional sizes for each element
/// let constraints = Constraint::from_fills([1, 2, 1]);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIs)]
pub enum Constraint {
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
    /// `[Percentage(0), Max(20)]`
    ///
    /// ```plain
    /// ┌────────────────────────────┐┌──────────────────┐
    /// │            30 px           ││       20 px      │
    /// └────────────────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Percentage(0), Max(10)]`
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
    /// `[Length(20), Length(20)]`
    ///
    /// ```plain
    /// ┌──────────────────┐┌──────────────────┐
    /// │       20 px      ││       20 px      │
    /// └──────────────────┘└──────────────────┘
    /// ```
    ///
    /// `[Length(20), Length(30)]`
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
    /// `[Percentage(75), Fill(1)]`
    ///
    /// ```plain
    /// ┌────────────────────────────────────┐┌──────────┐
    /// │                38 px               ││   12 px  │
    /// └────────────────────────────────────┘└──────────┘
    /// ```
    ///
    /// `[Percentage(50), Fill(1)]`
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

    /// Applies the scaling factor proportional to all other [`Constraint::Fill`] elements
    /// to fill excess space
    ///
    /// The element will only expand or fill into excess available space, proportionally matching
    /// other [`Constraint::Fill`] elements while satisfying all other constraints.
    ///
    /// # Examples
    ///
    ///
    /// `[Fill(1), Fill(2), Fill(3)]`
    ///
    /// ```plain
    /// ┌──────┐┌───────────────┐┌───────────────────────┐
    /// │ 8 px ││     17 px     ││         25 px         │
    /// └──────┘└───────────────┘└───────────────────────┘
    /// ```
    ///
    /// `[Fill(1), Percentage(50), Fill(1)]`
    ///
    /// ```plain
    /// ┌───────────┐┌───────────────────────┐┌──────────┐
    /// │   13 px   ││         25 px         ││   12 px  │
    /// └───────────┘└───────────────────────┘└──────────┘
    /// ```
    Fill(u16),
}

impl Constraint {
    #[deprecated(
        since = "0.26.0",
        note = "This field will be hidden in the next minor version."
    )]
    pub fn apply(&self, length: u16) -> u16 {
        match *self {
            Self::Percentage(p) => {
                let p = f32::from(p) / 100.0;
                let length = f32::from(length);
                (p * length).min(length) as u16
            }
            Self::Ratio(numerator, denominator) => {
                // avoid division by zero by using 1 when denominator is 0
                // this results in 0/0 -> 0 and x/0 -> x for x != 0
                let percentage = numerator as f32 / denominator.max(1) as f32;
                let length = f32::from(length);
                (percentage * length).min(length) as u16
            }
            Self::Length(l) | Self::Fill(l) => length.min(l),
            Self::Max(m) => length.min(m),
            Self::Min(m) => length.max(m),
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
    pub fn from_lengths<T>(lengths: T) -> Vec<Self>
    where
        T: IntoIterator<Item = u16>,
    {
        lengths.into_iter().map(Self::Length).collect_vec()
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
    pub fn from_ratios<T>(ratios: T) -> Vec<Self>
    where
        T: IntoIterator<Item = (u32, u32)>,
    {
        ratios
            .into_iter()
            .map(|(n, d)| Self::Ratio(n, d))
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
    pub fn from_percentages<T>(percentages: T) -> Vec<Self>
    where
        T: IntoIterator<Item = u16>,
    {
        percentages.into_iter().map(Self::Percentage).collect_vec()
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
    pub fn from_maxes<T>(maxes: T) -> Vec<Self>
    where
        T: IntoIterator<Item = u16>,
    {
        maxes.into_iter().map(Self::Max).collect_vec()
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
    pub fn from_mins<T>(mins: T) -> Vec<Self>
    where
        T: IntoIterator<Item = u16>,
    {
        mins.into_iter().map(Self::Min).collect_vec()
    }

    /// Convert an iterator of proportional factors into a vector of constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let area = Rect::default();
    /// let constraints = Constraint::from_fills([1, 2, 3]);
    /// let layout = Layout::default().constraints(constraints).split(area);
    /// ```
    pub fn from_fills<T>(proportional_factors: T) -> Vec<Self>
    where
        T: IntoIterator<Item = u16>,
    {
        proportional_factors
            .into_iter()
            .map(Self::Fill)
            .collect_vec()
    }
}

impl From<u16> for Constraint {
    /// Convert a `u16` into a [`Constraint::Length`]
    ///
    /// This is useful when you want to specify a fixed size for a layout, but don't want to
    /// explicitly create a [`Constraint::Length`] yourself.
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
    fn from(length: u16) -> Self {
        Self::Length(length)
    }
}

impl From<&Self> for Constraint {
    fn from(constraint: &Self) -> Self {
        *constraint
    }
}

impl AsRef<Self> for Constraint {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Percentage(100)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percentage(p) => write!(f, "Percentage({p})"),
            Self::Ratio(n, d) => write!(f, "Ratio({n}, {d})"),
            Self::Length(l) => write!(f, "Length({l})"),
            Self::Fill(l) => write!(f, "Fill({l})"),
            Self::Max(m) => write!(f, "Max({m})"),
            Self::Min(m) => write!(f, "Min({m})"),
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
    fn from_fills() {
        let expected = [
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(3),
        ];
        assert_eq!(Constraint::from_fills([1, 2, 3]), expected);
        assert_eq!(Constraint::from_fills(vec![1, 2, 3]), expected);
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
