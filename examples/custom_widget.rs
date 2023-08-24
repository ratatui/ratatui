use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
struct Label<'a> {
    text: &'a str,
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(mut self, text: &'a str) -> Label<'a> {
        self.text = text;
        self
    }
}

fn main() -> anyhow::Result<()> {
    let mut terminal = TerminalBuilder::crossterm_on_stdout().build()?;

    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let label = Label::default().text("Test");
    f.render_widget(label, size);
}
