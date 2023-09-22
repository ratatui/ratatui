use std::rc::Rc;

use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{tabs::*, AppContext, THEME};

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Root { context }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_title_bar(area[0], buf);
        self.render_selected_tab(area[1], buf);
        self.render_bottom_bar(area[2], buf);
    }
}

impl Root<'_> {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 45]);

        Paragraph::new(Span::styled("Ratatui", THEME.app_title)).render(area[0], buf);
        let titles = vec!["", " Recipe ", " Email ", " Traceroute ", " Weather "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.context.tab_index)
            .divider("")
            .render(area[1], buf);
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        let row_index = self.context.row_index;
        match self.context.tab_index {
            0 => AboutTab::new(row_index).render(area, buf),
            1 => RecipeTab::new(row_index).render(area, buf),
            2 => EmailTab::new(row_index).render(area, buf),
            3 => TracerouteTab::new(row_index).render(area, buf),
            4 => WeatherTab::new(row_index).render(area, buf),
            _ => unreachable!(),
        };
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [
            ("Q/Esc", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
        ];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", key), THEME.key_binding.key);
                let desc = Span::styled(format!(" {} ", desc), THEME.key_binding.description);
                [key, desc]
            })
            .collect_vec();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect_vec();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
