use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, StyleDiff},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Widget},
};

/// A widget to display available tabs in a multiple panels context.
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Tabs};
/// # use tui::style::{Style, Color};
/// # use tui::text::{Spans};
/// # use tui::symbols::{DOT};
/// let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(Spans::from).collect();
/// Tabs::new(titles)
///     .block(Block::default().title("Tabs").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow))
///     .divider(DOT);
/// ```
#[derive(Debug, Clone)]
pub struct Tabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Spans<'a>>,
    /// The index of the selected tabs
    selected: usize,
    /// The style used to draw the text
    style: Style,
    /// Style diff to apply to the selected item
    highlight_style_diff: StyleDiff,
    /// Tab divider
    divider: Span<'a>,
}

impl<'a> Tabs<'a> {
    pub fn new(titles: Vec<Spans<'a>>) -> Tabs<'a> {
        Tabs {
            block: None,
            titles,
            selected: 0,
            style: Default::default(),
            highlight_style_diff: Default::default(),
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

    #[deprecated(since = "0.10.0", note = "You should use `Tabs::highlight_style_diff`")]
    pub fn highlight_style(mut self, style: Style) -> Tabs<'a> {
        self.highlight_style_diff = StyleDiff::from(style);
        self
    }

    pub fn highlight_style_diff(mut self, diff: StyleDiff) -> Tabs<'a> {
        self.highlight_style_diff = diff;
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

impl<'a> Widget for Tabs<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
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

        buf.set_background(tabs_area, self.style.bg);

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, mut title) in self.titles.into_iter().enumerate() {
            let last_title = titles_length - 1 == i;
            if i == self.selected {
                for span in &mut title.0 {
                    span.style_diff = span.style_diff.patch(self.highlight_style_diff);
                }
            }
            x = x.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }
            let pos = buf.set_spans(x, tabs_area.top(), &title, remaining_width, self.style);
            x = pos.0.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_title {
                break;
            }
            let pos = buf.set_span(
                x,
                tabs_area.top(),
                &self.divider,
                remaining_width,
                self.style,
            );
            x = pos.0;
        }
    }
}
