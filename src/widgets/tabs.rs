use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled},
    symbols,
    text::{Line, Span},
    widgets::{Block, Widget},
};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, Borders, Tabs};
/// # use ratatui::style::{Style, Color};
/// # use ratatui::text::{Line};
/// # use ratatui::symbols::{DOT};
/// let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Line::from).collect();
/// Tabs::new(titles)
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
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
}

impl<'a> Tabs<'a> {
    pub fn new<T>(titles: Vec<T>) -> Tabs<'a>
    where
        T: Into<Line<'a>>,
    {
        Tabs {
            block: None,
            titles: titles.into_iter().map(Into::into).collect(),
            selected: 0,
            style: Style::default(),
            highlight_style: Style::default(),
            divider: Span::raw(symbols::line::VERTICAL),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Tabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn select(mut self, selected: usize) -> Tabs<'a> {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> Tabs<'a> {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Tabs<'a> {
        self.highlight_style = style;
        self
    }

    pub fn divider<T>(mut self, divider: T) -> Tabs<'a>
    where
        T: Into<Span<'a>>,
    {
        self.divider = divider.into();
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
            x = x.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }
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
            x = pos.0.saturating_add(1);
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
                highlight_style: Style::default(),
                divider: Span::raw(symbols::line::VERTICAL),
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
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    ",])
        );
    }

    #[test]
    fn render_with_block() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .block(Block::default().title("Tabs").borders(Borders::ALL));
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 3)),
            Buffer::with_lines(vec![
                "┌Tabs────────────────────────┐",
                "│ Tab1 │ Tab2 │ Tab3 │ Tab4  │",
                "└────────────────────────────┘",
            ])
        );
    }

    #[test]
    fn render_style() {
        let tabs =
            Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).style(Style::default().fg(Color::Red));
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        expected.set_style(Rect::new(0, 0, 30, 1), Style::default().fg(Color::Red));
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 1)), expected);
    }

    #[test]
    fn render_select() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .highlight_style(Style::new().reversed());

        // first tab selected
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        expected.set_style(Rect::new(1, 0, 4, 1), Style::new().reversed());
        assert_buffer_eq!(
            render(tabs.clone().select(0), Rect::new(0, 0, 30, 1)),
            expected
        );

        // second tab selected
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        expected.set_style(Rect::new(8, 0, 4, 1), Style::new().reversed());
        assert_buffer_eq!(
            render(tabs.clone().select(1), Rect::new(0, 0, 30, 1)),
            expected
        );

        // last tab selected
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        expected.set_style(Rect::new(22, 0, 4, 1), Style::new().reversed());
        assert_buffer_eq!(
            render(tabs.clone().select(3), Rect::new(0, 0, 30, 1)),
            expected
        );

        // out of bounds selects no tab
        let expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        assert_buffer_eq!(
            render(tabs.clone().select(4), Rect::new(0, 0, 30, 1)),
            expected
        );
    }

    #[test]
    fn render_style_and_selected() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .style(Style::new().red())
            .highlight_style(Style::new().reversed())
            .select(0);
        let mut expected = Buffer::with_lines(vec![" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
        expected.set_style(Rect::new(0, 0, 30, 1), Style::new().red());
        expected.set_style(Rect::new(1, 0, 4, 1), Style::new().reversed());
        assert_buffer_eq!(render(tabs, Rect::new(0, 0, 30, 1)), expected);
    }

    #[test]
    fn render_divider() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).divider("--");
        assert_buffer_eq!(
            render(tabs, Rect::new(0, 0, 30, 1)),
            Buffer::with_lines(vec![" Tab1 -- Tab2 -- Tab3 -- Tab4 ",])
        );
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
