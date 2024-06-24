/// State of a [`Table`] widget
///
/// This state can be used to scroll through the rows and select one of them. When the table is
/// rendered as a stateful widget, the selected row will be highlighted and the table will be
/// shifted to ensure that the selected row is visible. This will modify the [`TableState`] object
/// passed to the [`Frame::render_stateful_widget`] method.
///
/// The state consists of two fields:
/// - [`offset`]: the index of the first row to be displayed
/// - [`selected`]: the index of the selected row, which can be `None` if no row is selected
///
/// [`offset`]: TableState::offset()
/// [`selected`]: TableState::selected()
///
/// See the `table`` example and the `recipe`` and `traceroute`` tabs in the demo2 example in the
/// [Examples] directory for a more in depth example of the various configuration options and for
/// how to handle state.
///
/// [Examples]: https://github.com/ratatui-org/ratatui/blob/master/examples/README.md
///
/// # Example
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
/// # let widths = [Constraint::Length(5), Constraint::Length(5)];
/// let table = Table::new(rows, widths).widths(widths);
///
/// // Note: TableState should be stored in your application state (not constructed in your render
/// // method) so that the selected row is preserved across renders
/// let mut table_state = TableState::default();
/// *table_state.offset_mut() = 1; // display the second row and onwards
/// table_state.select(Some(3)); // select the forth row (0-indexed)
///
/// frame.render_stateful_widget(table, area, &mut table_state);
/// # }
/// ```
///
/// Note that if [`Table::widths`] is not called before rendering, the rendered columns will have
/// equal width.
///
/// [`Table`]: crate::widgets::Table
/// [`Table::widths`]: crate::widgets::Table::widths
/// [`Frame::render_stateful_widget`]: crate::Frame::render_stateful_widget
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableState {
    pub(crate) offset: usize,
    pub(crate) selected: Option<usize>,
}

impl TableState {
    /// Creates a new [`TableState`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            offset: 0,
            selected: None,
        }
    }

    /// Sets the index of the first row to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::new().with_offset(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the index of the selected row
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::new().with_selected(Some(1));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selected<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selected = selected.into();
        self
    }

    /// Index of the first row to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::new();
    /// assert_eq!(state.offset(), 0);
    /// ```
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first row to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// *state.offset_mut() = 1;
    /// ```
    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    /// Index of the selected row
    ///
    /// Returns `None` if no row is selected
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::new();
    /// assert_eq!(state.selected(), None);
    /// ```
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Mutable reference to the index of the selected row
    ///
    /// Returns `None` if no row is selected
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// *state.selected_mut() = Some(1);
    /// ```
    pub fn selected_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected
    }

    /// Sets the index of the selected row
    ///
    /// Set to `None` if no row is selected. This will also reset the offset to `0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// state.select(Some(1));
    /// ```
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    /// Selects the next item or the first one if no item is selected
    ///
    /// Note: until the table is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the table is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// state.select_next();
    /// ```
    pub fn select_next(&mut self) {
        let next = self.selected.map_or(0, |i| i.saturating_add(1));
        self.select(Some(next));
    }

    /// Selects the previous item or the last one if no item is selected
    ///
    /// Note: until the table is rendered, the number of items is not known, so the index is set to
    /// `usize::MAX` and will be corrected when the table is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// state.select_previous();
    /// ```
    pub fn select_previous(&mut self) {
        let previous = self.selected.map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

    /// Selects the first item
    ///
    /// Note: until the table is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the table is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// state.select_first();
    /// ```
    pub fn select_first(&mut self) {
        self.select(Some(0));
    }

    /// Selects the last item
    ///
    /// Note: until the table is rendered, the number of items is not known, so the index is set to
    /// `usize::MAX` and will be corrected when the table is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = TableState::default();
    /// state.select_last();
    /// ```
    pub fn select_last(&mut self) {
        self.select(Some(usize::MAX));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let state = TableState::new();
        assert_eq!(state.offset, 0);
        assert_eq!(state.selected, None);
    }

    #[test]
    fn with_offset() {
        let state = TableState::new().with_offset(1);
        assert_eq!(state.offset, 1);
    }

    #[test]
    fn with_selected() {
        let state = TableState::new().with_selected(Some(1));
        assert_eq!(state.selected, Some(1));
    }

    #[test]
    fn offset() {
        let state = TableState::new();
        assert_eq!(state.offset(), 0);
    }

    #[test]
    fn offset_mut() {
        let mut state = TableState::new();
        *state.offset_mut() = 1;
        assert_eq!(state.offset, 1);
    }

    #[test]
    fn selected() {
        let state = TableState::new();
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn selected_mut() {
        let mut state = TableState::new();
        *state.selected_mut() = Some(1);
        assert_eq!(state.selected, Some(1));
    }

    #[test]
    fn select() {
        let mut state = TableState::new();
        state.select(Some(1));
        assert_eq!(state.selected, Some(1));
    }

    #[test]
    fn select_none() {
        let mut state = TableState::new().with_selected(Some(1));
        state.select(None);
        assert_eq!(state.selected, None);
    }

    #[test]
    fn test_table_state_navigation() {
        let mut state = TableState::default();
        state.select_first();
        assert_eq!(state.selected, Some(0));

        state.select_previous(); // should not go below 0
        assert_eq!(state.selected, Some(0));

        state.select_next();
        assert_eq!(state.selected, Some(1));

        state.select_previous();
        assert_eq!(state.selected, Some(0));

        state.select_last();
        assert_eq!(state.selected, Some(usize::MAX));

        state.select_next(); // should not go above usize::MAX
        assert_eq!(state.selected, Some(usize::MAX));

        state.select_previous();
        assert_eq!(state.selected, Some(usize::MAX - 1));

        state.select_next();
        assert_eq!(state.selected, Some(usize::MAX));

        let mut state = TableState::default();
        state.select_next();
        assert_eq!(state.selected, Some(0));

        let mut state = TableState::default();
        state.select_previous();
        assert_eq!(state.selected, Some(usize::MAX));
    }
}
