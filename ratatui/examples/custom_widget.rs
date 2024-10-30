//! # [Ratatui] Custom Widget example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::{io::stdout, ops::ControlFlow, time::Duration};

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent,
            MouseEventKind,
        },
        execute,
    },
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let app_result = run(terminal);
    ratatui::restore();
    if let Err(err) = execute!(stdout(), DisableMouseCapture) {
        eprintln!("Error disabling mouse capture: {err}");
    }
    app_result
}

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

const GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight: Color::Rgb(64, 192, 64),
    shadow: Color::Rgb(32, 96, 32),
};

/// A button with a label that can be themed.
impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            theme: BLUE,
            state: State::Normal,
        }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub const fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

impl Button<'_> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            State::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
            State::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
            State::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
        }
    }
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut selected_button: usize = 0;
    let mut button_states = [State::Selected, State::Normal, State::Normal];
    loop {
        terminal.draw(|frame| draw(frame, button_states))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) => {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }
                if handle_key_event(key, &mut button_states, &mut selected_button).is_break() {
                    break;
                }
            }
            Event::Mouse(mouse) => {
                handle_mouse_event(mouse, &mut button_states, &mut selected_button);
            }
            _ => (),
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, states: [State; 3]) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(3),
        Constraint::Length(1),
        Constraint::Min(0), // ignore remaining space
    ]);
    let [title, buttons, help, _] = vertical.areas(frame.area());

    frame.render_widget(
        Paragraph::new("Custom Widget Example (mouse enabled)"),
        title,
    );
    render_buttons(frame, buttons, states);
    frame.render_widget(Paragraph::new("←/→: select, Space: toggle, q: quit"), help);
}

fn render_buttons(frame: &mut Frame<'_>, area: Rect, states: [State; 3]) {
    let horizontal = Layout::horizontal([
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Min(0), // ignore remaining space
    ]);
    let [red, green, blue, _] = horizontal.areas(area);

    frame.render_widget(Button::new("Red").theme(RED).state(states[0]), red);
    frame.render_widget(Button::new("Green").theme(GREEN).state(states[1]), green);
    frame.render_widget(Button::new("Blue").theme(BLUE).state(states[2]), blue);
}

fn handle_key_event(
    key: event::KeyEvent,
    button_states: &mut [State; 3],
    selected_button: &mut usize,
) -> ControlFlow<()> {
    match key.code {
        KeyCode::Char('q') => return ControlFlow::Break(()),
        KeyCode::Left | KeyCode::Char('h') => {
            button_states[*selected_button] = State::Normal;
            *selected_button = selected_button.saturating_sub(1);
            button_states[*selected_button] = State::Selected;
        }
        KeyCode::Right | KeyCode::Char('l') => {
            button_states[*selected_button] = State::Normal;
            *selected_button = selected_button.saturating_add(1).min(2);
            button_states[*selected_button] = State::Selected;
        }
        KeyCode::Char(' ') => {
            if button_states[*selected_button] == State::Active {
                button_states[*selected_button] = State::Normal;
            } else {
                button_states[*selected_button] = State::Active;
            }
        }
        _ => (),
    }
    ControlFlow::Continue(())
}

fn handle_mouse_event(
    mouse: MouseEvent,
    button_states: &mut [State; 3],
    selected_button: &mut usize,
) {
    match mouse.kind {
        MouseEventKind::Moved => {
            let old_selected_button = *selected_button;
            *selected_button = match mouse.column {
                x if x < 15 => 0,
                x if x < 30 => 1,
                _ => 2,
            };
            if old_selected_button != *selected_button {
                if button_states[old_selected_button] != State::Active {
                    button_states[old_selected_button] = State::Normal;
                }
                if button_states[*selected_button] != State::Active {
                    button_states[*selected_button] = State::Selected;
                }
            }
        }
        MouseEventKind::Down(MouseButton::Left) => {
            if button_states[*selected_button] == State::Active {
                button_states[*selected_button] = State::Normal;
            } else {
                button_states[*selected_button] = State::Active;
            }
        }
        _ => (),
    }
}
