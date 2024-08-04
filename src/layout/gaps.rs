/// Gaps around or within something in all 4 directions.
///
/// This is inspired by the [CSS margin](https://developer.mozilla.org/en-US/docs/Web/CSS/margin) / [CSS padding](https://developer.mozilla.org/en-US/docs/Web/CSS/padding).
///
/// # Examples
///
/// ```rust
/// # use ratatui::layout::Gaps;
/// let some = Gaps {
///     top: 1,
///     right: 2,
///     bottom: 3,
///     left: 4,
/// };
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gaps {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Gaps {
    /// `Gaps` with zero gap in all directions
    pub const ZERO: Self = Self::all(0);

    /// Create with the same gaps in all directions.
    /// This is similar to the css with one argument: `margin: 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::layout::Gaps;
    /// assert_eq!(
    ///     Gaps::all(1),
    ///     Gaps {
    ///         top: 1,
    ///         right: 1,
    ///         bottom: 1,
    ///         left: 1
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn all(all: u16) -> Self {
        Self {
            top: all,
            right: all,
            bottom: all,
            left: all,
        }
    }

    /// Create on the horizontal and vertical axis.
    ///
    /// This is similar to the css with two arguments: `margin: horizontal vertical`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::layout::Gaps;
    /// assert_eq!(
    ///     Gaps::horizontal_vertical(1, 2),
    ///     Gaps {
    ///         top: 2,
    ///         right: 1,
    ///         bottom: 2,
    ///         left: 1
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn horizontal_vertical(horizontal: u16, vertical: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create with the same value for `left` and `right` while the rest stays zero.
    #[must_use]
    pub const fn horizontal(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    /// Create with the same value for `top` and `bottom` while the rest stays zero.
    #[must_use]
    pub const fn vertical(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    /// Create with only the `top` value set.
    #[must_use]
    pub const fn top(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: 0,
        }
    }

    /// Create with only the `right` value set.
    #[must_use]
    pub const fn right(value: u16) -> Self {
        Self {
            left: 0,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    /// Create with only the `bottom` value set.
    #[must_use]
    pub const fn bottom(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: value,
        }
    }

    /// Create with only the `left` value set.
    #[must_use]
    pub const fn left(value: u16) -> Self {
        Self {
            left: value,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    /// Returns the total horizontal space taken on the left and right.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::layout::Gaps;
    /// let some = Gaps {
    ///     top: 1,
    ///     right: 2,
    ///     bottom: 3,
    ///     left: 4,
    /// };
    /// assert_eq!(some.total_horizontal(), 6);
    /// ```
    #[must_use]
    pub const fn total_horizontal(self) -> u16 {
        self.left.saturating_add(self.right)
    }

    /// Returns the total vertical space taken on the top and bottom.
    ///     ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::layout::Gaps;
    /// let some = Gaps {
    ///     top: 1,
    ///     right: 2,
    ///     bottom: 3,
    ///     left: 4,
    /// };
    /// assert_eq!(some.total_vertical(), 4);
    /// ```
    #[must_use]
    pub const fn total_vertical(self) -> u16 {
        self.top.saturating_add(self.bottom)
    }
}
