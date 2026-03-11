use crate::transient::Transient;

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
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) item_count: Transient<Option<usize>>,
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

    /// Returns the number of items in the list, if known.
    ///
    /// This value is set during rendering.
    /// Returns `None` if the list hasn't been rendered yet.
    pub const fn item_count(&self) -> Option<usize> {
        self.item_count.0
    }

    /// Sets the number of items in the list.
    ///
    /// This value is updated during rendering.
    /// You can update it manually to enable clamping before the first render,
    /// or change the upper bound of the clamp between renders.
    ///
    /// This will immediately clamp the `selected` to be less than `item_count`.
    pub(crate) const fn set_item_count(&mut self, count: Option<usize>) {
        self.item_count.0 = count;
        self.clamp_selected()
    }

    /// Sets the index of the selected item
    ///
    /// Set to `None` if no item is selected. This will also reset the offset to `0`.
    ///
    /// If the `item_count` is known (set during rendering), the
    /// `selected` index is clamped to valid bounds:
    /// indices past the last item are clamped to the last item.
    ///
    /// If the `item_count` is zero, any `selected` index results in `None`.
    ///
    /// # Examples
    ///
    /// Before `item_count` is set, no clamping occurs:
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListState, StatefulWidget};
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let mut state = ListState::default();
    /// state.select(Some(1));
    /// assert_eq!(state.selected(), Some(1));
    /// ```
    ///
    /// After rendering a list with 5 items, `item_count` becomes `Some(5)`
    /// and out-of-bounds indices are clamped to the last item:
    ///
    /// ```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # let mut state = ListState::default();
    /// let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(vec![""; 5]), r, &mut Buffer::empty(r), &mut state);
    /// state.select(Some(5)); // out of bounds (0-indexed)
    /// assert_eq!(state.selected(), Some(4));
    /// ```
    ///
    /// After rendering an empty list, `item_count` becomes `Some(0)`
    /// and any selection resolves to `None`:
    ///
    /// ```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # let mut state = ListState::default();
    /// # let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(Vec::<&str>::new()), r, &mut Buffer::empty(r), &mut state);
    /// assert_eq!(state.selected(), None);
    /// ```
    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
        self.clamp_selected()
    }

    /// Clamps the `selected` index to valid bounds if `item_count` is known.
    ///
    /// Returns `true` if the `selected` index was clamped.
    const fn clamp_selected(&mut self) {
        if let (Some(selected), Some(count)) = (self.selected, self.item_count.0) {
            if count == 0 {
                self.selected = None;
            }
            if selected >= count {
                self.selected = Some(count - 1);
            }
        }
    }

    /// Selects the next item or the first one if no item is selected
    ///
    /// After `item_count` is set, the `selected` index will be clamped to the
    /// last item in the list.
    ///
    /// If `item_count` is `Some(0)`, then `selected` will be clamped and set to `None`.
    ///
    /// # Examples
    ///
    /// Before rendering, item count is unknown:
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListState, StatefulWidget};
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let mut state = ListState::default();
    /// state.select_next();
    /// assert_eq!(state.selected(), Some(0));
    /// ```
    ///
    /// After rendering a list with 3 items, `select_next` clamps to the last item:
    ///
    /// ```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// #
    /// let mut state = ListState::default();
    /// let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(vec![""; 3]), r, &mut Buffer::empty(r), &mut state);
    /// state.select(Some(2));
    /// let moved = state.select_next(); // clamped to last item
    /// assert!(!moved);
    /// assert_eq!(state.selected(), Some(2));
    /// ```
    pub fn select_next(&mut self) {
        let next = self.selected.map_or(0, |i| i.saturating_add(1));
        self.select(Some(next))
    }

    /// Selects the previous item or the last one if no item is selected
    ///
    /// Note: until the list is rendered, the number of items is not known. In this case,
    /// the function will select `usize::MAX` as the last item.
    /// If `item_count` is known, the function will select `item_count - 1` as the last item.
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
    /// assert_eq!(state.selected(), Some(usize::MAX));
    ///
    /// state.select(Some(1));
    /// state.select_previous();
    /// assert_eq!(state.selected(), Some(0));
    ///
    /// let moved = state.select_previous(); // already at first item
    /// assert!(!moved);
    /// assert_eq!(state.selected(), Some(0));
    /// ```
    pub fn select_previous(&mut self) {
        let index_max = self
            .item_count
            .0
            .map_or(usize::MAX, |c| c.saturating_sub(1));
        let previous = self.selected.map_or(index_max, |i| i.saturating_sub(1));
        self.select(Some(previous))
    }

    /// Selects the first item or `None` if the `item_count is `0`.
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListState, StatefulWidget};
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let mut state = ListState::default();
    /// state.select_first();
    /// assert_eq!(state.selected(), Some(0));
    /// ```
    ///
    /// After rendering an empty list, `select_first` results in `None`:
    ///
    /// ```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// #
    /// let mut state = ListState::default();
    /// let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(Vec::<&str>::new()), r, &mut Buffer::empty(r), &mut state);
    /// state.select_first();
    /// assert_eq!(state.selected(), None);
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
    /// Before rendering, item count is unknown:
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListState, StatefulWidget};
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let mut state = ListState::default();
    /// state.select_last();
    /// assert_eq!(state.selected(), Some(usize::MAX));
    /// ```
    ///
    /// After rendering a list with 5 items, `select_last` selects the last valid index:
    ///
    /// ```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// #
    /// let mut state = ListState::default();
    /// let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(vec![""; 5]), r, &mut Buffer::empty(r), &mut state);
    /// state.select_last();
    /// assert_eq!(state.selected(), Some(4));
    /// ```
    pub fn select_last(&mut self) {
        let last = self
            .item_count
            .0
            .map_or(usize::MAX, |c| c.saturating_sub(1));
        self.select(Some(last));
    }

    /// Scrolls down by a specified `amount` in the list.
    ///
    /// This method updates the `selected` index by moving it down by the given `amount`.
    /// If the `amount` causes the index to go out of bounds (i.e., if the index is greater than
    /// the length of the list), the last item in the list will be selected.
    ///
    /// Returns `true` if the `selected` index was successfully moved, `false` if it was clamped.
    ///
    /// # Examples
    ///
    /// Before rendering, item count is unknown:
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListState, StatefulWidget};
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let mut state = ListState::default();
    /// state.scroll_down_by(3);
    /// assert_eq!(state.selected(), Some(3));
    /// ```
    ///
    /// After rendering a list with 5 items, `scroll_down_by` clamps to the last item:
    ///
    ///```rust
    /// # use ratatui::widgets::{List, ListState, StatefulWidget};
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// #
    /// let mut state = ListState::default();
    /// let r = Rect::new(0, 0, 20, 20);
    /// StatefulWidget::render(List::new(vec![""; 5]), r, &mut Buffer::empty(r), &mut state);
    /// state.select(Some(2));
    /// let moved = state.scroll_down_by(4); // clamped to last item
    /// assert!(!moved);
    /// assert_eq!(state.selected(), Some(4));
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
    /// Returns `true` if the `selected` index was successfully moved, `false` if it was clamped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.scroll_up_by(3);
    /// assert_eq!(state.selected(), Some(0));
    ///
    /// state.select(Some(6));
    /// state.scroll_up_by(4);
    /// assert_eq!(state.selected(), Some(2));
    ///
    /// let moved = state.scroll_up_by(4); // saturates at first item
    /// assert!(!moved);
    /// assert_eq!(state.selected(), Some(0));
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
    use crate::transient::Transient;

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
    fn select_next_should_not_exceed_item_count() {
        let mut state = ListState {
            item_count: Transient(Some(3)),
            ..Default::default()
        };
        state.select(Some(2));

        state.select_next();

        assert_eq!(state.selected(), Some(2), "should stay at last item");
    }

    #[test]
    fn scroll_down_by_should_not_exceed_item_count() {
        let mut state = ListState {
            item_count: Transient(Some(5)),
            ..Default::default()
        };
        state.select(Some(3));

        state.scroll_down_by(100);

        assert_eq!(
            state.selected(),
            Some(4),
            "should clamp to last valid index"
        );
    }

    #[test]
    fn select_last_should_use_item_count() {
        let mut state = ListState {
            item_count: Transient(Some(5)),
            ..Default::default()
        };

        state.select_last();

        assert_eq!(state.selected(), Some(4), "should go to last valid index");
    }

    #[test]
    fn select_previous_from_none_should_use_item_count() {
        let mut state = ListState {
            item_count: Transient(Some(5)),
            ..Default::default()
        };

        state.select_previous();

        assert_eq!(state.selected(), Some(4), "should go to last item");
    }

    #[test]
    fn item_count_returns_cached_value() {
        let state = ListState::default();
        assert_eq!(state.item_count(), None);

        let state = ListState {
            item_count: Transient(Some(42)),
            ..Default::default()
        };
        assert_eq!(state.item_count(), Some(42));
    }

    #[test]
    fn clamp_selected_with_zero_item_count_deselects() {
        let mut state = ListState {
            item_count: Transient(Some(0)),
            ..Default::default()
        };
        state.select(Some(5));

        state.select_next();

        assert_eq!(state.selected(), None, "empty list should deselect");
    }

    #[test]
    fn clamp_selected_does_nothing_when_within_bounds() {
        let mut state = ListState {
            item_count: Transient(Some(5)),
            ..Default::default()
        };
        state.select(Some(2));

        state.select_next();

        assert_eq!(
            state.selected(),
            Some(3),
            "should move normally when within bounds"
        );
    }
}
