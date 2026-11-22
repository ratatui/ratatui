/// State of the [`List`] widget
///
/// This state can be used to scroll through items and select one. When the list is rendered as a
/// stateful widget, the selected item will be highlighted and the list will be shifted to ensure
/// that the selected item is visible. This will modify the [`ListState`] object passed to the
/// `Frame::render_stateful_widget` method.
///
/// The state consists of four fields:
/// - [`offset`]: the index of the first item to be displayed
/// - [`selected`]: the index of the selected item, which can be `None` if no item is selected
/// - [`item_count`]: an optional item count that enables bounded navigation
/// - [`selection_mode`]: whether navigation clamps at the ends or wraps around
///
/// [`offset`]: ListState::offset()
/// [`selected`]: ListState::selected()
/// [`item_count`]: ListState::item_count()
/// [`selection_mode`]: ListState::selection_mode()
///
/// Rendering records the number of items, and navigation methods such as
/// [`select_next`](Self::select_next) stay within that count, so [`selected`] is a valid index
/// into the items that were last rendered. By default navigation stops at the first and last
/// items; choose [`SelectionMode::Wrap`] with [`with_selection_mode`](Self::with_selection_mode)
/// to continue from the other end.
///
/// The state only knows about the items as of the last render. Selecting the last item and then
/// adding more items leaves the selection where it was; it does not follow the end of the list.
/// If items change before the next render, update [`item_count_mut`](Self::item_count_mut) before
/// navigating. Before the first render the count is unknown unless supplied with
/// [`with_item_count`](Self::with_item_count), and navigation is unbounded until rendering clamps
/// it. Direct selection assignments are only clamped during rendering, so you can select an item
/// that will be added on the next render.
///
/// Equality and hashing compare all four fields, so a state compares unequal to an otherwise
/// identical one after rendering records the item count.
///
/// Serialization preserves the offset, selection and selection mode. The item count is not
/// serialized; rendering or [`item_count_mut`](Self::item_count_mut) restores it.
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
    item_count: Option<usize>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "SelectionMode::is_clamp")
    )]
    selection_mode: SelectionMode,
}

/// How list navigation behaves at the first and last items.
///
/// This applies to [`ListState::select_next`], [`ListState::select_previous`],
/// [`ListState::scroll_down_by`] and [`ListState::scroll_up_by`] when the item count is known.
/// Direct selection assignments are clamped during rendering instead of wrapping.
///
/// The mode takes effect once the item count is known: after the first render, or immediately
/// when supplied with [`ListState::with_item_count`]. Before that, navigation is unbounded.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectionMode {
    /// Stop at the first or last item.
    #[default]
    Clamp,
    /// Continue from the other end, including when moving by more than the list length.
    Wrap,
}

impl SelectionMode {
    #[cfg(feature = "serde")]
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's skip_serializing_if passes a reference"
    )]
    const fn is_clamp(&self) -> bool {
        matches!(self, Self::Clamp)
    }
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
    /// `Some(count)` enables bounded navigation before the first render. Rendering replaces the
    /// count with the number of items rendered. This does not change the selection. The count is
    /// not serialized.
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

    /// Sets whether navigation clamps at the ends or wraps around.
    ///
    /// Defaults to [`SelectionMode::Clamp`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{ListState, SelectionMode};
    ///
    /// let mut state = ListState::default()
    ///     .with_item_count(Some(3))
    ///     .with_selection_mode(SelectionMode::Wrap);
    /// state.select_first();
    /// state.select_previous();
    /// assert_eq!(state.selected(), Some(2));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Returns how navigation behaves at the ends of the list. Defaults to
    /// [`SelectionMode::Clamp`].
    pub const fn selection_mode(&self) -> SelectionMode {
        self.selection_mode
    }

    /// Mutable reference to how navigation behaves at the ends of the list.
    pub const fn selection_mode_mut(&mut self) -> &mut SelectionMode {
        &mut self.selection_mode
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
    /// Navigation is bounded by the item count when known. Direct selection assignments may be
    /// out of bounds until rendering clamps them.
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
    /// This bypasses bounds checks. Rendering or navigation with a known item count corrects
    /// out-of-bounds selections.
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
    /// Bounds are checked during rendering, allowing selection of items added on the next render.
    /// Rendering an empty list clears the selection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select(Some(1));
    /// ```
    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    /// Mutable reference to the number of items in the list.
    ///
    /// Set this before navigating when items change between renders, for example after adding an
    /// item and before calling [`select_last`](Self::select_last). This does not change the
    /// selection; rendering or bounded navigation corrects out-of-bounds selections. Rendering
    /// replaces the count with the number of items rendered. The count is not serialized.
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
    /// Returns `None` until the list is rendered or a count is supplied. Rendering records the
    /// number of items rendered.
    pub const fn item_count(&self) -> Option<usize> {
        self.item_count
    }

    /// Records the rendered item count and keeps the selection in bounds.
    pub(super) const fn update_selection_for_render(&mut self, item_count: usize) {
        self.item_count = Some(item_count);
        self.clamp_selected();
    }

    /// Clamps the `selected` index to valid bounds if `item_count` is known.
    const fn clamp_selected(&mut self) {
        match (self.selected, self.item_count) {
            (_, Some(0)) => self.select(None),
            (Some(selected), Some(count)) if selected >= count => {
                self.selected = Some(count - 1);
            }
            _ => {}
        }
    }

    /// Selects the next item or the first one if no item is selected
    ///
    /// Clamps or wraps according to [`selection_mode`](Self::selection_mode) when the item count
    /// is known. An empty list selects `None`. With an unknown count, saturates at `usize::MAX`.
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
        if self.selected.is_none() {
            self.select_first();
        } else {
            self.scroll_down_by(1);
        }
    }

    /// Selects the previous item or the last one if no item is selected
    ///
    /// Clamps or wraps according to [`selection_mode`](Self::selection_mode) when the item count
    /// is known. An empty list selects `None`. With an unknown count, the last index is
    /// `usize::MAX`.
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
        if self.selected.is_none() {
            self.select_last();
        } else {
            self.scroll_up_by(1);
        }
    }

    /// Selects the first item or `None` if the `item_count` is `0`.
    ///
    /// With an unknown item count, selects `0`, which is corrected during rendering.
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
        self.clamp_selected();
    }

    /// Selects the last item
    ///
    /// Selects `item_count - 1` when known, or `None` for an empty list. With an unknown count,
    /// selects `usize::MAX`, which is clamped to the last item during rendering.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::ListState;
    ///
    /// let mut state = ListState::default();
    /// state.select_last();
    /// ```
    pub const fn select_last(&mut self) {
        let last = match self.item_count {
            Some(count) => count.saturating_sub(1),
            None => usize::MAX,
        };
        self.select(Some(last));
        self.clamp_selected();
    }

    /// Scrolls down by a specified `amount` in the list.
    ///
    /// Moves the selection down, starting at index `0` if nothing is selected. With a known count,
    /// clamps or wraps according to [`selection_mode`](Self::selection_mode); an empty list selects
    /// `None`. With an unknown count, saturates at `usize::MAX`.
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
        self.clamp_selected();
        let selected = self.selected.unwrap_or_default();
        let next = match (self.selection_mode(), self.item_count) {
            (SelectionMode::Wrap, Some(count)) if count > 0 => {
                let amount = usize::from(amount) % count;
                let remaining = count - selected;
                if amount >= remaining {
                    amount - remaining
                } else {
                    selected + amount
                }
            }
            _ => selected.saturating_add(usize::from(amount)),
        };
        self.select(Some(next));
        self.clamp_selected();
    }

    /// Scrolls up by a specified `amount` in the list.
    ///
    /// Moves the selection up, starting at index `0` if nothing is selected. With a known count,
    /// clamps or wraps according to [`selection_mode`](Self::selection_mode); an empty list selects
    /// `None`. With an unknown count, saturates at `0`.
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
        self.clamp_selected();
        let selected = self.selected.unwrap_or_default();
        let previous = match (self.selection_mode(), self.item_count) {
            (SelectionMode::Wrap, Some(count)) if count > 0 => {
                let amount = usize::from(amount) % count;
                if amount > selected {
                    count - (amount - selected)
                } else {
                    selected - amount
                }
            }
            _ => selected.saturating_sub(usize::from(amount)),
        };
        self.select(Some(previous));
        self.clamp_selected();
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::list::{ListState, SelectionMode};

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

    #[rstest]
    #[case::clamp_end(SelectionMode::Clamp, Some(3), Some(2), Some(2), Some(1))]
    #[case::clamp_start(SelectionMode::Clamp, Some(3), Some(0), Some(1), Some(0))]
    #[case::wrap_end(SelectionMode::Wrap, Some(3), Some(2), Some(0), Some(1))]
    #[case::wrap_start(SelectionMode::Wrap, Some(3), Some(0), Some(1), Some(2))]
    #[case::no_selection(SelectionMode::Wrap, Some(3), None, Some(0), Some(2))]
    #[case::unknown_count(SelectionMode::Wrap, None, None, Some(0), Some(usize::MAX))]
    #[case::unknown_end(SelectionMode::Wrap, None, Some(usize::MAX), Some(usize::MAX), Some(usize::MAX - 1))]
    #[case::empty(SelectionMode::Wrap, Some(0), Some(2), None, None)]
    #[case::single_item(SelectionMode::Wrap, Some(1), Some(0), Some(0), Some(0))]
    #[case::stale_selection(SelectionMode::Clamp, Some(3), Some(10), Some(2), Some(1))]
    fn bounded_navigation(
        #[case] mode: SelectionMode,
        #[case] count: Option<usize>,
        #[case] selected: Option<usize>,
        #[case] next: Option<usize>,
        #[case] previous: Option<usize>,
    ) {
        let state = ListState::default()
            .with_item_count(count)
            .with_selected(selected)
            .with_selection_mode(mode);
        let mut next_state = state;
        next_state.select_next();
        assert_eq!(next_state.selected(), next);
        let mut previous_state = state;
        previous_state.select_previous();
        assert_eq!(previous_state.selected(), previous);
    }

    #[rstest]
    fn scrolling_matches_repeated_steps(
        #[values(SelectionMode::Clamp, SelectionMode::Wrap)] mode: SelectionMode,
        #[values(None, Some(0), Some(1), Some(3), Some(usize::MAX))] count: Option<usize>,
        #[values(None, Some(0), Some(2), Some(usize::MAX))] selected: Option<usize>,
    ) {
        let state = ListState::default()
            .with_item_count(count)
            .with_selected(selected)
            .with_selection_mode(mode);
        for (scroll, step) in [
            (
                ListState::scroll_down_by as fn(&mut ListState, u16),
                ListState::select_next as fn(&mut ListState),
            ),
            (
                ListState::scroll_up_by as fn(&mut ListState, u16),
                ListState::select_previous as fn(&mut ListState),
            ),
        ] {
            let mut once = state;
            scroll(&mut once, 0); // Resolve an initial selection using scroll_by semantics.
            let mut repeated = once;
            scroll(&mut once, 17);
            for _ in 0..17 {
                step(&mut repeated);
            }
            assert_eq!(once, repeated);
        }
    }

    #[rstest]
    #[case(5, 1, 100, 1, 1)]
    #[case(5, 2, 3, 0, 4)]
    #[case(usize::MAX, usize::MAX - 1, u16::MAX, usize::from(u16::MAX) - 1, usize::MAX - 1 - usize::from(u16::MAX))]
    fn wrapping_scroll(
        #[case] count: usize,
        #[case] selected: usize,
        #[case] amount: u16,
        #[case] down: usize,
        #[case] up: usize,
    ) {
        let state = ListState::default()
            .with_item_count(Some(count))
            .with_selected(Some(selected))
            .with_selection_mode(SelectionMode::Wrap);
        let mut down_state = state;
        down_state.scroll_down_by(amount);
        assert_eq!(down_state.selected(), Some(down));
        let mut up_state = state;
        up_state.scroll_up_by(amount);
        assert_eq!(up_state.selected(), Some(up));
    }

    #[rstest]
    #[case(None, Some(0), Some(usize::MAX))]
    #[case(Some(0), None, None)]
    #[case(Some(3), Some(0), Some(2))]
    fn first_and_last_are_bounded_and_const(
        #[case] count: Option<usize>,
        #[case] first: Option<usize>,
        #[case] last: Option<usize>,
        #[values(SelectionMode::Clamp, SelectionMode::Wrap)] mode: SelectionMode,
    ) {
        const fn first_and_last(mut state: ListState) -> (Option<usize>, Option<usize>) {
            state.select_first();
            let first = state.selected();
            state.select_last();
            (first, state.selected())
        }
        let state = ListState::default()
            .with_item_count(count)
            .with_selection_mode(mode);
        assert_eq!(first_and_last(state), (first, last));
    }

    #[test]
    fn update_count_before_navigation() {
        let mut state = ListState::default()
            .with_item_count(Some(1))
            .with_selected(Some(0));
        *state.item_count_mut() = Some(3);
        state.select_next();
        assert_eq!(state.selected(), Some(1));
        *state.selection_mode_mut() = SelectionMode::Wrap;
        state.select_first();
        state.select_previous();
        assert_eq!(state.selected(), Some(2));
    }
}
