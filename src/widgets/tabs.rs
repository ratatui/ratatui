#![deny(missing_docs)]
use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, Styled},
    symbols,
    text::{Line, Span},
    widgets::{Block, Widget},
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
/// use ratatui::{prelude::*, widgets::*};
///
/// Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .style(Style::default().white())
///     .highlight_style(Style::default().yellow())
///     .select(2)
///     .divider(symbols::DOT)
///     .padding("->", "<-");
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Tabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Line<'a>>,
    /// The index of the selected tabs
    selected: usize,
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
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]);
    /// ```
    ///
    /// Styled titles
    /// ```
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1".red(), "Tab 2".blue()]);
    /// ```
    pub fn new<T>(titles: Vec<T>) -> Tabs<'a>
    where
        T: Into<Line<'a>>,
    {
        Tabs {
            block: None,
            titles: titles.into_iter().map(Into::into).collect(),
            selected: 0,
            style: Style::default(),
            highlight_style: DEFAULT_HIGHLIGHT_STYLE,
            divider: Span::raw(symbols::line::VERTICAL),
            padding_left: Line::from(" "),
            padding_right: Line::from(" "),
        }
    }

    /// Surrounds the `Tabs` with a [`Block`].
    pub fn block(mut self, block: Block<'a>) -> Tabs<'a> {
        self.block = Some(block);
        self
    }

    /// Sets the selected tab.
    ///
    /// The first tab has index 0 (this is also the default index).
    /// The selected tab can have a different style with [`Tabs::highlight_style`].
    pub fn select(mut self, selected: usize) -> Tabs<'a> {
        self.selected = selected;
        self
    }

    /// Sets the style of the tabs.
    ///
    /// This will set the given style on the entire render area.
    /// More precise style can be applied to the titles by styling the ones given to [`Tabs::new`].
    /// The selected tab can be styled differently using [`Tabs::highlight_style`].
    pub fn style(mut self, style: Style) -> Tabs<'a> {
        self.style = style;
        self
    }

    /// Sets the style for the highlighted tab.
    ///
    /// Highlighted tab can be selected with [`Tabs::select`].
    pub fn highlight_style(mut self, style: Style) -> Tabs<'a> {
        self.highlight_style = style;
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
    /// # use ratatui::{prelude::*, widgets::Tabs, symbols};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider(symbols::DOT);
    /// ```
    /// Use dash (`-`) as separator.
    /// ```
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider("-");
    /// ```
    pub fn divider<T>(mut self, divider: T) -> Tabs<'a>
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
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding(" ", " ");
    /// ```
    /// Nothing on either side of the tabs.
    /// ```
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding("", "");
    /// ```
    pub fn padding<T, U>(mut self, left: T, right: U) -> Tabs<'a>
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
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_left("->");
    /// ```
    pub fn padding_left<T>(mut self, padding: T) -> Tabs<'a>
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
    /// # use ratatui::{prelude::*, widgets::Tabs};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).padding_right("<-");
    /// ```
    pub fn padding_right<T>(mut self, padding: T) -> Tabs<'a>
    where
        T: Into<Line<'a>>,
    {
        self.padding_left = padding.into();
        self
    }
}

impl<'a> Styled for Tabs<'a> {
    type Item = Tabs<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Widget for Tabs<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.into_iter().enumerate() {
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
            let pos = buf.set_line(x, tabs_area.top(), &title, remaining_width);
            if i == self.selected {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_buffer_eq, prelude::*, widgets::Borders};

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
                selected: 0,
                style: Style::default(),
                highlight_style: DEFAULT_HIGHLIGHT_STYLE,
                divider: Span::raw(symbols::line::VERTICAL),
                padding_right: Line::from(" "),
                padding_left: Line::from(" "),
            }
        );
    }

    fn render(tabs: Tabs, area: Rect) -> Buffer {
        let mut buffer = Buffer::empty(area);
        tabs.render(area, &mut buffer);
        buffer
    }

    #[test]
    fn render_default() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        // first tab selected
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 1)), expected);
    }

    #[test]
    fn render_no_padding() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("", "");
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec!["Tab1│Tab2│Tab3│Tab4           "])
        );
    }

    #[test]
    fn render_more_padding() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("   ", "  ");
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec!["   Tab1  │   Tab2  │   Tab3  │"])
        );
    }

    #[test]
    fn render_with_block() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .block(Block::default().title("Tabs").borders(Borders::ALL));
        let mut expected = Buffer::with_lines(vec![
            "┌Tabs────────────────────────┐",
            "│ Tab1 │ Tab2 │ Tab3 │ Tab4  │",
            "└────────────────────────────┘",
        ]);
        // first tab selected
        expected.set_style(Rect::new(2, 1, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 3)), expected);
    }

    #[test]
    fn render_style() {
        let tabs =
            Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).style(Style::default().fg(Color::Red));
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    ".red()]);
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE.red());
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 1)), expected);
    }

    #[test]
    fn render_select() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);

        // first tab selected
        assert_buffer_eq!(
            render(tabs.clone().select(0), Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![Line::from(vec![
                " ".into(),
                "Tab1".reversed(),
                " │ Tab2 │ Tab3 │ Tab4    ".into(),
            ])])
        );

        // second tab selected
        assert_buffer_eq!(
            render(tabs.clone().select(1), Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![Line::from(vec![
                " Tab1 │ ".into(),
                "Tab2".reversed(),
                " │ Tab3 │ Tab4    ".into(),
            ])])
        );

        // last tab selected
        assert_buffer_eq!(
            render(tabs.clone().select(3), Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![Line::from(vec![
                " Tab1 │ Tab2 │ Tab3 │ ".into(),
                "Tab4".reversed(),
                "    ".into(),
            ])])
        );

        // out of bounds selects no tab
        assert_buffer_eq!(
            render(tabs.clone().select(4), Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "])
        );
    }

    #[test]
    fn render_style_and_selected() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .style(Style::new().red())
            .highlight_style(Style::new().underlined())
            .select(0);
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![Line::from(vec![
                " ".red(),
                "Tab1".red().underlined(),
                " │ Tab2 │ Tab3 │ Tab4    ".red(),
            ])])
        );
    }

    #[test]
    fn render_divider() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).divider("--");
        let mut expected = Buffer::with_lines(vec![" Tab1 -- Tab2 -- Tab3 -- Tab4 "]);
        // first tab selected
        expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 1)), expected);
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
        )
    }
}
