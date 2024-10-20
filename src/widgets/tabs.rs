use itertools::Itertools;

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, Styled},
    symbols::{self},
    text::{Line, Span},
    widgets::{block::BlockExt, Block, Widget, WidgetRef},
};

const DEFAULT_HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);

/// A widget that displays a horizontal set of Tabs with a single tab selected.
///
/// Each tab title is stored as a [`Line`] which can be individually styled. The selected tab is set
/// using [`Tabs::select`] and styled using [`Tabs::highlight_style`]. The divider can be customized
/// with [`Tabs::divider`]. Padding can be set with [`Tabs::padding`] or [`Tabs::padding_left`] and
/// [`Tabs::padding_right`].
///
/// The divider defaults to |, and padding defaults to a singular space on each side.
///
/// # Example
///
/// ```
/// use ratatui::{
///     style::{Style, Stylize},
///     symbols,
///     widgets::{Block, Tabs},
/// };
///
/// Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
///     .block(Block::bordered().title("Tabs"))
///     .style(Style::default().white())
///     .highlight_style(Style::default().yellow())
///     .select(2)
///     .divider(symbols::DOT)
///     .padding("->", "<-");
/// ```
///
/// In addition to `Tabs::new`, any iterator whose element is convertible to `Line` can be collected
/// into `Tabs`.
///
/// ```
/// use ratatui::widgets::Tabs;
///
/// (0..5).map(|i| format!("Tab{i}")).collect::<Tabs>();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Tabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Line<'a>>,
    /// The index of the selected tabs
    selected: Option<usize>,
    /// The style used to draw the text
    style: Style,
    /// Style to apply to the selected item
    highlight_style: Style,
    /// Tab divider
    divider: Span<'a>,
    /// Tab Left Padding
    padding_left: Line<'a>,
    /// Tab Right Padding
    padding_right: Line<'a>,
}

impl Default for Tabs<'_> {
    /// Returns a default `Tabs` widget.
    ///
    /// The default widget has:
    /// - No tabs
    /// - No selected tab
    /// - The highlight style is set to reversed.
    /// - The divider is set to a pipe (`|`).
    /// - The padding on the left and right is set to a space.
    ///
    /// This is rarely useful on its own without calling [`Tabs::titles`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::default().titles(["Tab 1", "Tab 2"]);
    /// ```
    fn default() -> Self {
        Self::new(Vec::<Line>::new())
    }
}

impl<'a> Tabs<'a> {
    /// Creates new `Tabs` from their titles.
    ///
    /// `titles` can be a [`Vec`] of [`&str`], [`String`] or anything that can be converted into
    /// [`Line`]. As such, titles can be styled independently.
    ///
    /// The selected tab can be set with [`Tabs::select`]. The first tab has index 0 (this is also
    /// the default index).
    ///
    /// The selected tab can have a different style with [`Tabs::highlight_style`]. This defaults to
    /// a style with the [`Modifier::REVERSED`] modifier added.
    ///
    /// The default divider is a pipe (`|`), but it can be customized with [`Tabs::divider`].
    ///
    /// The entire widget can be styled with [`Tabs::style`].
    ///
    /// The widget can be wrapped in a [`Block`] using [`Tabs::block`].
    ///
    /// # Examples
    ///
    /// Basic titles.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]);
    /// ```
    ///
    /// Styled titles
    /// ```
    /// use ratatui::{style::Stylize, widgets::Tabs};
    ///
    /// let tabs = Tabs::new(vec!["Tab 1".red(), "Tab 2".blue()]);
    /// ```
    pub fn new<Iter>(titles: Iter) -> Self
    where
        Iter: IntoIterator,
        Iter::Item: Into<Line<'a>>,
    {
        let titles = titles.into_iter().map(Into::into).collect_vec();
        let selected = if titles.is_empty() { None } else { Some(0) };
        Self {
            block: None,
            titles,
            selected,
            style: Style::default(),
            highlight_style: DEFAULT_HIGHLIGHT_STYLE,
            divider: Span::raw(symbols::line::VERTICAL),
            padding_left: Line::from(" "),
            padding_right: Line::from(" "),
        }
    }

    /// Sets the titles of the tabs.
    ///
    /// `titles` is an iterator whose elements can be converted into `Line`.
    ///
    /// The selected tab can be set with [`Tabs::select`]. The first tab has index 0 (this is also
    /// the default index).
    ///
    /// # Examples
    ///
    /// Basic titles.
    ///
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::default().titles(vec!["Tab 1", "Tab 2"]);
    /// ```
    ///
    /// Styled titles.
    ///
    /// ```
    /// use ratatui::{style::Stylize, widgets::Tabs};
    ///
    /// let tabs = Tabs::default().titles(vec!["Tab 1".red(), "Tab 2".blue()]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn titles<Iter>(mut self, titles: Iter) -> Self
    where
        Iter: IntoIterator,
        Iter::Item: Into<Line<'a>>,
    {
        self.titles = titles.into_iter().map(Into::into).collect_vec();
        self.selected = if self.titles.is_empty() {
            None
        } else {
            // Ensure selected is within bounds, and default to 0 if no selected tab
            self.selected
                .map(|selected| selected.min(self.titles.len() - 1))
                .or(Some(0))
        };
        self
    }

    /// Surrounds the `Tabs` with a [`Block`].
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the selected tab.
    ///
    /// The first tab has index 0 (this is also the default index).
    /// The selected tab can have a different style with [`Tabs::highlight_style`].
    ///
    /// # Examples
    ///
    /// Select the second tab.
    ///
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).select(1);
    /// ```
    ///
    /// Deselect the selected tab.
    ///
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).select(None);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn select<T: Into<Option<usize>>>(mut self, selected: T) -> Self {
        self.selected = selected.into();
        self
    }

    /// Sets the style of the tabs.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This will set the given style on the entire render area.
    /// More precise style can be applied to the titles by styling the ones given to [`Tabs::new`].
    /// The selected tab can be styled differently using [`Tabs::highlight_style`].
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the style for the highlighted tab.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// Highlighted tab can be selected with [`Tabs::select`].
    #[must_use = "method moves the value of self and returns the modified value"]
    ///
    /// [`Color`]: crate::style::Color
    pub fn highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.highlight_style = style.into();
        self
    }

    /// Sets the string to use as tab divider.
    ///
    /// By default, the divider is a pipe (`|`).
    ///
    /// # Examples
    ///
    /// Use a dot (`•`) as separator.
    /// ```
    /// use ratatui::{symbols, widgets::Tabs};
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider(symbols::DOT);
    /// ```
    /// Use dash (`-`) as separator.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider("-");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn divider<T>(mut self, divider: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.divider = divider.into();
        self
    }

    /// Sets the padding between tabs.
    ///
    /// Both default to space.
    ///
    /// # Examples
    ///
    /// A space on either side of the tabs.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding(" ", " ");
    /// ```
    /// Nothing on either side of the tabs.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding("", "");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn padding<T, U>(mut self, left: T, right: U) -> Self
    where
        T: Into<Line<'a>>,
        U: Into<Line<'a>>,
    {
        self.padding_left = left.into();
        self.padding_right = right.into();
        self
    }

    /// Sets the left side padding between tabs.
    ///
    /// Defaults to a space.
    ///
    /// # Example
    ///
    /// An arrow on the left of tabs.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_left("->");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn padding_left<T>(mut self, padding: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.padding_left = padding.into();
        self
    }

    /// Sets the right side padding between tabs.
    ///
    /// Defaults to a space.
    ///
    /// # Example
    ///
    /// An arrow on the right of tabs.
    /// ```
    /// use ratatui::widgets::Tabs;
    ///
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_right("<-");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn padding_right<T>(mut self, padding: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.padding_left = padding.into();
        self
    }
}

impl<'a> Styled for Tabs<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for Tabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Tabs<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        self.block.render_ref(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_tabs(inner, buf);
    }
}

impl Tabs<'_> {
    fn render_tabs(&self, tabs_area: Rect, buf: &mut Buffer) {
        if tabs_area.is_empty() {
            return;
        }

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.iter().enumerate() {
            let last_title = titles_length - 1 == i;
            let remaining_width = tabs_area.right().saturating_sub(x);

            if remaining_width == 0 {
                break;
            }

            // Left Padding
            let pos = buf.set_line(x, tabs_area.top(), &self.padding_left, remaining_width);
            x = pos.0;
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }

            // Title
            let pos = buf.set_line(x, tabs_area.top(), title, remaining_width);
            if Some(i) == self.selected {
                buf.set_style(
                    Rect {
                        x,
                        y: tabs_area.top(),
                        width: pos.0.saturating_sub(x),
                        height: 1,
                    },
                    self.highlight_style,
                );
            }
            x = pos.0;
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }

            // Right Padding
            let pos = buf.set_line(x, tabs_area.top(), &self.padding_right, remaining_width);
            x = pos.0;
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_title {
                break;
            }

            let pos = buf.set_span(x, tabs_area.top(), &self.divider, remaining_width);
            x = pos.0;
        }
    }
}

impl<'a, Item> FromIterator<Item> for Tabs<'a>
where
    Item: Into<Line<'a>>,
{
    fn from_iter<Iter: IntoIterator<Item = Item>>(iter: Iter) -> Self {
        Self::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Stylize};

    #[test]
    fn new() {
        let titles = vec!["Tab1", "Tab2", "Tab3", "Tab4"];
        let tabs = Tabs::new(titles.clone());
        assert_eq!(
            tabs,
            Tabs {
                block: None,
                titles: vec![
                    Line::from("Tab1"),
                    Line::from("Tab2"),
                    Line::from("Tab3"),
                    Line::from("Tab4"),
                ],
                selected: Some(0),
                style: Style::default(),
                highlight_style: DEFAULT_HIGHLIGHT_STYLE,
                divider: Span::raw(symbols::line::VERTICAL),
                padding_right: Line::from(" "),
                padding_left: Line::from(" "),
            }
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Tabs::default(),
            Tabs {
                block: None,
                titles: vec![],
                selected: None,
                style: Style::default(),
                highlight_style: DEFAULT_HIGHLIGHT_STYLE,
                divider: Span::raw(symbols::line::VERTICAL),
                padding_right: Line::from(" "),
                padding_left: Line::from(" "),
            }
        );
    }

    #[test]
    fn select_into() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);
        assert_eq!(tabs.clone().select(2).selected, Some(2));
        assert_eq!(tabs.clone().select(None).selected, None);
        assert_eq!(tabs.clone().select(1u8 as usize).selected, Some(1));
    }

    #[test]
    fn select_before_titles() {
        let tabs = Tabs::default().select(1).titles(["Tab1", "Tab2"]);
        assert_eq!(tabs.selected, Some(1));
    }

    #[test]
    fn new_from_vec_of_str() {
        Tabs::new(vec!["a", "b"]);
    }

    #[test]
    fn collect() {
        let tabs: Tabs = (0..5).map(|i| format!("Tab{i}")).collect();
        assert_eq!(
            tabs.titles,
            vec![
                Line::from("Tab0"),
                Line::from("Tab1"),
                Line::from("Tab2"),
                Line::from("Tab3"),
                Line::from("Tab4"),
            ],
        );
    }

    #[track_caller]
    fn test_case(tabs: Tabs, area: Rect, expected: &Buffer) {
        let mut buffer = Buffer::empty(area);
        tabs.render(area, &mut buffer);
        assert_eq!(&buffer, expected);
    }

    #[test]
    fn render_new() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);
        let mut expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        // first tab selected
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_no_padding() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("", "");
        let mut expected = Buffer::with_lines(["Tab1│Tab2│Tab3│Tab4           "]);
        // first tab selected
        expected.set_style(Rect::new(0, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_more_padding() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("---", "++");
        let mut expected = Buffer::with_lines(["---Tab1++│---Tab2++│---Tab3++│"]);
        // first tab selected
        expected.set_style(Rect::new(3, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_with_block() {
        let tabs =
            Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).block(Block::bordered().title("Tabs"));
        let mut expected = Buffer::with_lines([
            "┌Tabs────────────────────────┐",
            "│ Tab1 │ Tab2 │ Tab3 │ Tab4  │",
            "└────────────────────────────┘",
        ]);
        // first tab selected
        expected.set_style(Rect::new(2, 1, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        test_case(tabs, Rect::new(0, 0, 30, 3), &expected);
    }

    #[test]
    fn render_style() {
        let tabs =
            Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).style(Style::default().fg(Color::Red));
        let mut expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    ".red()]);
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE.red());
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_select() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);

        // first tab selected
        let expected = Buffer::with_lines([Line::from(vec![
            " ".into(),
            "Tab1".reversed(),
            " │ Tab2 │ Tab3 │ Tab4    ".into(),
        ])]);
        test_case(tabs.clone().select(0), Rect::new(0, 0, 30, 1), &expected);

        // second tab selected
        let expected = Buffer::with_lines([Line::from(vec![
            " Tab1 │ ".into(),
            "Tab2".reversed(),
            " │ Tab3 │ Tab4    ".into(),
        ])]);
        test_case(tabs.clone().select(1), Rect::new(0, 0, 30, 1), &expected);

        // last tab selected
        let expected = Buffer::with_lines([Line::from(vec![
            " Tab1 │ Tab2 │ Tab3 │ ".into(),
            "Tab4".reversed(),
            "    ".into(),
        ])]);
        test_case(tabs.clone().select(3), Rect::new(0, 0, 30, 1), &expected);

        // out of bounds selects no tab
        let expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        test_case(tabs.clone().select(4), Rect::new(0, 0, 30, 1), &expected);

        // deselect
        let expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        test_case(tabs.clone().select(None), Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_style_and_selected() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .style(Style::new().red())
            .highlight_style(Style::new().underlined())
            .select(0);
        let expected = Buffer::with_lines([Line::from(vec![
            " ".red(),
            "Tab1".red().underlined(),
            " │ Tab2 │ Tab3 │ Tab4    ".red(),
        ])]);
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn render_divider() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).divider("--");
        let mut expected = Buffer::with_lines([" Tab1 -- Tab2 -- Tab3 -- Tab4 "]);
        // first tab selected
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            Tabs::new(vec![""])
                .black()
                .on_white()
                .bold()
                .not_italic()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        );
    }
}
