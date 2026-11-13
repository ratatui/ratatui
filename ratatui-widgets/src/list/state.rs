/// State of the [`List`] widget
///
/// This state can be used to scroll through items and select one. When the list is rendered as a
/// stateful widget, the selected item will be highlighted and the list will be shifted to ensure
/// that the selected item is visible. This will modify the [`ListState`] object passed to the
/// `Frame::render_stateful_widget` method.
///
/// The state consists of three fields:
/// - [`offset`]: the index of the first item to be displayed
/// - [`selected`]: the index of the selected item, which can be `None` if no item is selected
/// - [`item_count`]: the number of items in the list, set during rendering
///
/// [`offset`]: ListState::offset()
/// [`selected`]: ListState::selected()
/// [`item_count`]: ListState::item_count()
///
/// See the list in the [Examples] directory for a more in-depth example of the various
/// configuration options and for how to handle state.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
///
/// # Example
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// use ratatui::widgets::{List, ListState};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = ["Item 1"];
/// let list = List::new(items);
///
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
///
/// *state.offset_mut() = 1; // display the second item and onwards
/// state.select(Some(3)); // select the fourth item (0-indexed)
///
/// frame.render_stateful_widget(list, area, &mut state);
/// # }
/// ```
///
/// [`List`]: super::List
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListState {
    pub(crate) offset: usize,
    pub(crate) selected: Option<usize>,
    pub(crate) item_count: Option<usize>,
}

impl ListState {
    /// Sets the index of the first item to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let state = ListState::default().with_offset(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the index of the selected item
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let state = ListState::default().with_selected(Some(1));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the number of items in the list
    ///
    /// Fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let state = ListState::default().with_item_count(Some(5));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_item_count(mut self, count: Option<usize>) -> Self {
        self.item_count = count;
        self
    }
    /// Index of the first item to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let state = ListState::default();
    /// assert_eq!(state.offset(), 0);
    /// ```
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first item to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// *state.offset_mut() = 1;
    /// ```
    pub const fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    /// Index of the selected item
    ///
    /// Bounded by `item_count` when known
    ///
    /// Returns `None` if no item is selected
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let state = ListState::default();
    /// assert_eq!(state.selected(), None);
    /// ```
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Mutable reference to the index of the selected item
    ///
    /// Returns `None` if no item is selected
    ///
    /// Note: this bypasses clamping. The `selected` value will be clamped on the next render
    /// or navigation method call.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// *state.selected_mut() = Some(1);
    /// ```
    pub const fn selected_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected
    }

    /// Sets the index of the selected item
    ///
    /// Set to `None` if no item is selected. This will also reset the offset to `0`.
    ///
    /// Clamped by `item_count` if known.
    ///
    /// If the `item_count` is zero, any `selected` index results in `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select(Some(1));
    /// # assert_eq!(state.selected(), Some(1));
    /// ```
    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
        self.clamp_selected();
    }

    /// Mutable reference of `item_count`.
    ///
    /// Returns `None` if `item_count` is not set.
    ///
    /// Note: this bypasses clamping. The `selected` value will be clamped on the next render
    /// or navigation method call.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// *state.item_count_mut() = Some(1);
    /// ```
    pub const fn item_count_mut(&mut self) -> &mut Option<usize> {
        &mut self.item_count
    }

    /// Returns the number of items in the list, if known.
    ///
    /// This value is set during rendering.
    /// Returns `None` if the list hasn't been rendered yet.
    pub const fn item_count(&self) -> Option<usize> {
        self.item_count
    }

    /// Updates the number of items in the list.
    ///
    /// This value is updated during rendering.
    /// You can update it manually to enable clamping before the first render,
    /// or change the upper bound of the clamp between renders.
    ///
    /// This will immediately clamp the `selected` to be less than `item_count`.
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.update_item_count(Some(5));
    ///
    /// // Clamping
    /// let mut state = ListState::default();
    /// state.select(Some(4));
    /// state.update_item_count(Some(2));
    /// assert_eq!(state.selected(), Some(1)); // (0-indexed)
    /// ```
    pub const fn update_item_count(&mut self, count: Option<usize>) {
        self.item_count = count;
        self.clamp_selected();
    }

    /// Clamps the `selected` index to valid bounds if `item_count` is known.
    const fn clamp_selected(&mut self) {
        if let (Some(selected), Some(count)) = (self.selected, self.item_count) {
            if count == 0 {
                self.selected = None;
            } else if selected >= count {
                self.selected = Some(count - 1);
            } else {
                // selected is already within bounds, nothing to do
            }
        }
    }

    /// Selects the next item or the first one if no item is selected
    ///
    /// Bounded by `item_count` when known, otherwise falls back to `usize::MAX`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select_next();
    /// ```
    pub fn select_next(&mut self) {
        let next = self.selected.map_or(0, |i| i.saturating_add(1));
        self.select(Some(next));
    }

    /// Selects the previous item or the last one if no item is selected
    ///
    /// If `item_count` is know, last item is `item_count - 1`, otherwise falls back to
    /// `usize::MAX`.
    ///
    /// If `item_count` is `Some(0)`, then `selected` will be clamped and set to `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select_previous();
    /// ```
    pub fn select_previous(&mut self) {
        let index_max = self.item_count.map_or(usize::MAX, |c| c.saturating_sub(1));
        let previous = self.selected.map_or(index_max, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

    /// Selects the first item or `None` if the `item_count` is `0`.
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select_first();
    /// ```
    pub const fn select_first(&mut self) {
        self.select(Some(0));
    }

    /// Selects the last item
    ///
    /// Note: until the list is rendered, the number of items is not known. In this case,
    /// the function will select `usize::MAX` as the last item.
    /// If `item_count` is known, the function will select `item_count - 1` as the last item.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select_last();
    /// ```
    pub fn select_last(&mut self) {
        let last = self.item_count.map_or(usize::MAX, |c| c.saturating_sub(1));
        self.select(Some(last));
    }

    /// Scrolls down by a specified `amount` in the list.
    ///
    /// This method updates the selected index by moving it down by the given `amount`.
    /// If the `amount` causes the index to go out of bounds (i.e., if the index is greater than
    /// the length of the list), the last item in the list will be selected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.scroll_down_by(4);
    /// ```
    pub fn scroll_down_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_add(amount as usize)));
    }

    /// Scrolls up by a specified `amount` in the list.
    ///
    /// This method updates the selected index by moving it up by the given `amount`.
    /// If the `amount` causes the index to go out of bounds (i.e., less than zero),
    /// the first item in the list will be selected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.scroll_up_by(4);
    /// ```
    pub fn scroll_up_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_sub(amount as usize)));
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::list::ListState;

    #[test]
    fn selected() {
        let mut state = ListState::default();
        assert_eq!(state.selected(), None);

        state.select(Some(1));
        assert_eq!(state.selected(), Some(1));

        state.select(None);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn selected_mut() {
        let mut state = ListState::default();
        *state.selected_mut() = Some(1);
        assert_eq!(state.selected(), Some(1));
    }

    #[test]
    fn select() {
        let mut state = ListState::default();
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);

        state.select(Some(2));
        assert_eq!(state.selected, Some(2));
        assert_eq!(state.offset, 0);

        state.select(None);
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn state_navigation() {
        let mut state = ListState::default();
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

        let mut state = ListState::default();
        state.select_next();
        assert_eq!(state.selected, Some(0));

        let mut state = ListState::default();
        state.select_previous();
        assert_eq!(state.selected, Some(usize::MAX));

        let mut state = ListState::default();
        state.select_last();
        state.scroll_down_by(4);
        assert_eq!(state.selected, Some(usize::MAX));

        state.select(Some(2));
        state.scroll_down_by(4);
        assert_eq!(state.selected, Some(6));

        let mut state = ListState::default();
        state.scroll_up_by(3);
        assert_eq!(state.selected, Some(0));

        state.select(Some(6));
        state.scroll_up_by(4);
        assert_eq!(state.selected, Some(2));

        state.scroll_up_by(4);
        assert_eq!(state.selected, Some(0));
    }

    #[test]
    fn item_count_returns_cached_value() {
        let state = ListState::default();
        assert_eq!(state.item_count(), None);

        let state = ListState {
            item_count: Some(42),
            ..Default::default()
        };
        assert_eq!(state.item_count(), Some(42));
    }
}
