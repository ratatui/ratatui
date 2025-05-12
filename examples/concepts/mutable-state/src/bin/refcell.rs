//! This example shows how to mutate state while rendering, by storing a `RefCell`.
use std::cell::Cell;
use std::io;
use std::rc::Rc;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let counter = CounterWidget::default();
    loop {
        terminal.draw(|frame| render(frame, counter.clone()))?;
        if esc_pressed()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, counter: CounterWidget) {
    frame.render_widget(counter, frame.area());
}

#[derive(Default, Clone)]
struct CounterWidget {
    count: Rc<Cell<usize>>,
}

impl Widget for CounterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut count = self.count.get();
        count += 1;
        self.count.set(count);
        let text = format!("Counter: {count}");
        text.render(area, buf);
    }
}

fn esc_pressed() -> io::Result<bool> {
    Ok(matches!(
        event::read()?,
        Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            code: KeyCode::Esc,
            ..
        })
    ))
}
