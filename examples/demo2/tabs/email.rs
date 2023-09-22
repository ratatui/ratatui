use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use unicode_width::UnicodeWidthStr;

use crate::{layout, RgbSwatch, THEME};

#[derive(Debug, Default)]
pub struct Email {
    from: &'static str,
    subject: &'static str,
    body: &'static str,
}

const EMAILS: &[Email] = &[
    Email {
        from: "Alice <alice@example.com>",
        subject: "Hello",
        body: "Hi Bob,\nHow are you?\n\nAlice",
    },
    Email {
        from: "Bob <bob@example.com>",
        subject: "Re: Hello",
        body: "Hi Alice,\nI'm fine, thanks!\n\nBob",
    },
    Email {
        from: "Charlie <charlie@example.com>",
        subject: "Re: Hello",
        body: "Hi Alice,\nI'm fine, thanks!\n\nCharlie",
    },
    Email {
        from: "Dave <dave@example.com>",
        subject: "Re: Hello (STOP REPLYING TO ALL)",
        body: "Hi Everyone,\nPlease stop replying to all.\n\nDave",
    },
    Email {
        from: "Eve <eve@example.com>",
        subject: "Re: Hello (STOP REPLYING TO ALL)",
        body: "Hi Everyone,\nI'm reading all your emails.\n\nEve",
    },
];

#[derive(Debug, Default)]
pub struct EmailTab {
    selected_index: usize,
}

impl EmailTab {
    pub fn new(selected_index: usize) -> Self {
        Self {
            selected_index: selected_index % EMAILS.len(),
        }
    }
}

impl Widget for EmailTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        let area = layout(area, Direction::Vertical, vec![5, 0]);
        render_inbox(self.selected_index, area[0], buf);
        render_email(self.selected_index, area[1], buf);
    }
}
fn render_inbox(selected_index: usize, area: Rect, buf: &mut Buffer) {
    let area = layout(area, Direction::Vertical, vec![1, 0]);
    let theme = THEME.email;
    Tabs::new(vec![" Inbox ", " Sent ", " Drafts "])
        .style(theme.tabs)
        .highlight_style(theme.tabs_selected)
        .select(0)
        .divider("")
        .render(area[0], buf);

    let highlight_symbol = ">>";
    let from_width = EMAILS
        .iter()
        .map(|e| e.from.width())
        .max()
        .unwrap_or_default();
    let items = EMAILS
        .iter()
        .map(|e| {
            let from = format!("{:width$}", e.from, width = from_width).into();
            ListItem::new(Line::from(vec![from, " ".into(), e.subject.into()]))
        })
        .collect_vec();
    let mut state = ListState::default().with_selected(Some(selected_index));
    StatefulWidget::render(
        List::new(items)
            .style(theme.inbox)
            .highlight_style(theme.selected_item)
            .highlight_symbol(highlight_symbol),
        area[1],
        buf,
        &mut state,
    );
    let mut scrollbar_state = ScrollbarState::default()
        .content_length(EMAILS.len())
        .position(selected_index);
    Scrollbar::default()
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(None)
        .thumb_symbol("▐")
        .render(area[1], buf, &mut scrollbar_state);
}

fn render_email(selected_index: usize, area: Rect, buf: &mut Buffer) {
    let theme = THEME.email;
    let email = EMAILS.get(selected_index);
    let block = Block::new()
        .style(theme.body)
        .padding(Padding::new(2, 2, 0, 0))
        .borders(Borders::TOP)
        .border_type(BorderType::Thick);
    let inner = block.inner(area);
    block.render(area, buf);
    if let Some(email) = email {
        let area = layout(inner, Direction::Vertical, vec![3, 0]);
        let headers = vec![
            Line::from(vec![
                "From: ".set_style(theme.header),
                email.from.set_style(theme.header_value),
            ]),
            Line::from(vec![
                "Subject: ".set_style(theme.header),
                email.subject.set_style(theme.header_value),
            ]),
            "-".repeat(inner.width as usize).dim().into(),
        ];
        Paragraph::new(headers)
            .style(theme.body)
            .render(area[0], buf);
        let body = email.body.lines().map(Line::from).collect_vec();
        Paragraph::new(body).style(theme.body).render(area[1], buf);
    } else {
        Paragraph::new("No email selected").render(inner, buf);
    }
}
