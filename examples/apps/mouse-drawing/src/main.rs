use color_eyre::Result;
/// A Ratatui example that demonstrates how to handle mouse events.
///
/// This example demonstrates how to handle mouse events in Ratatui. The example allows you to draw
/// lines by clicking and dragging the mouse.
///
/// This example runs with the version of Ratatui found in the branch that you are currently
/// reading. It may not work with the released version of Ratatui. See the `latest` branch for the
/// latest code which works with the released version of Ratatui.
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEvent, MouseEventKind},
    execute,
};
use ratatui::{
    layout::{Position, Rect, Size},
    symbols,
    text::Line,
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = MouseDrawingApp::default().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default)]
struct MouseDrawingApp {
    pub should_exit: bool,
    // The last known mouse position
    pub mouse_position: Option<Position>,
    // The points that have been clicked / drawn by dragging the mouse
    pub points: Vec<Position>,
}

impl MouseDrawingApp {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        execute!(std::io::stdout(), EnableMouseCapture)?;
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        execute!(std::io::stdout(), DisableMouseCapture)?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            event::Event::Mouse(event) => self.on_mouse_event(event),
            event::Event::Key(event) if matches!(event.code, KeyCode::Char('q') | KeyCode::Esc) => {
                self.should_exit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn on_mouse_event(&mut self, event: MouseEvent) {
        let position = Position::new(event.column, event.row);
        match event.kind {
            MouseEventKind::Down(_) => self.points.push(position),
            MouseEventKind::Drag(_) => {
                if let Some(start) = self.points.last() {
                    let (x0, y0) = (start.x as i32, start.y as i32);
                    let (x1, y1) = (position.x as i32, position.y as i32);
                    for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
                        self.points.push(Position::new(x as u16, y as u16));
                    }
                }
            }
            _ => {}
        }
        self.mouse_position = Some(position);
    }

    fn render(&self, frame: &mut Frame) {
        let title = Line::from("Mouse Example (Press 'q' or 'Esc' to quit. Click / drag to draw)")
            .centered();
        frame.render_widget(title, frame.area());
        self.render_mouse_cursor(frame);
        self.render_points(frame);
    }

    fn render_mouse_cursor(&self, frame: &mut Frame<'_>) {
        if let Some(position) = self.mouse_position {
            let area = Rect::from((position, Size::new(1, 1))).clamp(frame.area());
            frame.render_widget(symbols::block::FULL, area);
        }
    }

    fn render_points(&self, frame: &mut Frame<'_>) {
        for point in &self.points {
            let area = Rect::from((*point, Size::new(1, 1))).clamp(frame.area());
            frame.render_widget(symbols::block::FULL, area);
        }
    }
}
