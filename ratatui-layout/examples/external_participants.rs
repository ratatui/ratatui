//! Render app-owned items from a plan without storing widgets in the container.
//!
//! A normal `Layout` call can produce the same rectangles. The extra piece here is that each region
//! id is passed back to an app-owned participant, so the container never stores card widgets.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect, Size};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::linear::Column;
use ratatui_layout::measure::{MeasureConstraint, SizeHint};
use ratatui_layout::participant::{LayoutParticipant, MeasureContext, RenderContext};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event()
                && !app.handle_key(key.code)
            {
                break Ok(());
            }
        }
    })
}

const CARDS: [(&str, &str); 3] = [
    ("Inbox", "14 unread"),
    ("Builds", "2 running"),
    ("Deploys", "production healthy"),
];

#[derive(Default)]
struct App {
    selected: usize,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let plan = Column::new([Constraint::Length(3); CARDS.len()]).regions(frame.area());
        let mut cards = Cards;

        for region in plan.regions() {
            cards.render(
                region.id,
                region.area,
                frame.buffer_mut(),
                RenderContext::selected(region.id == self.selected),
            );
        }
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.select_next_card(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous_card(),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            _ => {}
        }
        true
    }

    const fn select_next_card(&mut self) {
        self.selected = (self.selected + 1) % CARDS.len();
    }

    const fn select_previous_card(&mut self) {
        self.selected = (self.selected + CARDS.len() - 1) % CARDS.len();
    }

    const fn quit() -> bool {
        false
    }
}

struct Cards;

impl LayoutParticipant for Cards {
    fn measure(&self, id: usize, _: MeasureConstraint, _: MeasureContext) -> SizeHint {
        let (title, detail) = CARDS[id];
        SizeHint::exact(Size::new((title.len() + detail.len() + 3) as u16, 3))
    }

    fn render(&mut self, id: usize, area: Rect, buf: &mut Buffer, ctx: RenderContext) {
        let (title, detail) = CARDS[id];
        let style = if ctx.state.selected {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };

        Paragraph::new(format!("{title}\n  {detail}"))
            .style(style)
            .render(area, buf);
    }
}
