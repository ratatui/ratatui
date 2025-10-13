//! # [Ratatui] `Checkbox` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui_widgets::block::Block;
use ratatui_widgets::checkbox::Checkbox;

struct App {
    checkboxes: [bool; 4],
    selected: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            checkboxes: [true, false, true, false],
            selected: 0,
        }
    }
}

impl App {
    const FEATURE_ENABLED: usize = 0;
    const NOTIFICATIONS: usize = 1;
    const AUTO_SAVE: usize = 2;
    const DARK_MODE: usize = 3;
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::default();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &app))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected < 3 {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if app.selected < app.checkboxes.len() {
                            app.checkboxes[app.selected] = !app.checkboxes[app.selected];
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}

/// Render the UI with various checkbox styles.
fn render(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .spacing(1);

    let [title_area, basic_area, styled_area, custom_area] = frame.area().layout(&vertical);

    // Title
    let title = Line::from_iter([
        Span::from("Checkbox Widget Demo").bold(),
        Span::from(" (↑/↓: navigate, Space: toggle, q: quit)"),
    ]);
    frame.render_widget(title.centered(), title_area);

    // Basic checkboxes
    render_basic_checkboxes(frame, basic_area, app);

    // Styled checkboxes
    render_styled_checkboxes(frame, styled_area, app);

    // Custom symbol checkboxes
    render_custom_checkboxes(frame, custom_area, app);
}

fn render_basic_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().title("Basic Checkboxes");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([Constraint::Length(1); 4]).split(inner);

    let highlight_style = if app.selected == 0 {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };
    let checkbox1 = Checkbox::new(
        "Enable experimental features",
        app.checkboxes[App::FEATURE_ENABLED],
    )
    .style(highlight_style);
    frame.render_widget(checkbox1, layout[0]);

    let highlight_style = if app.selected == 1 {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };
    let checkbox2 = Checkbox::new("Receive notifications", app.checkboxes[App::NOTIFICATIONS])
        .style(highlight_style);
    frame.render_widget(checkbox2, layout[1]);

    let highlight_style = if app.selected == 2 {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };
    let checkbox3 =
        Checkbox::new("Auto-save enabled", app.checkboxes[App::AUTO_SAVE]).style(highlight_style);
    frame.render_widget(checkbox3, layout[2]);

    let highlight_style = if app.selected == 3 {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };
    let checkbox4 =
        Checkbox::new("Dark mode", app.checkboxes[App::DARK_MODE]).style(highlight_style);
    frame.render_widget(checkbox4, layout[3]);
}

fn render_styled_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().title("Styled Checkboxes");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let horizontal =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(inner);

    let left =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(horizontal[0]);

    let right =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(horizontal[1]);

    // Green checkbox
    let checkbox1 = Checkbox::new("Success".green(), app.checkboxes[App::FEATURE_ENABLED])
        .checkbox_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(checkbox1, left[0]);

    // Red checkbox
    let checkbox2 = Checkbox::new("Error".red(), app.checkboxes[App::NOTIFICATIONS])
        .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    frame.render_widget(checkbox2, left[1]);

    // Blue checkbox
    let checkbox3 = Checkbox::new("Info".blue(), app.checkboxes[App::AUTO_SAVE])
        .checkbox_style(Style::default().fg(Color::Blue));
    frame.render_widget(checkbox3, right[0]);

    // Yellow checkbox
    let checkbox4 = Checkbox::new("Warning".yellow(), app.checkboxes[App::DARK_MODE])
        .checkbox_style(Style::default().fg(Color::Yellow));
    frame.render_widget(checkbox4, right[1]);
}

fn render_custom_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().title("Custom Symbols");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(inner);

    // ASCII style
    let checkbox1 = Checkbox::new("ASCII style", app.checkboxes[App::FEATURE_ENABLED])
        .checked_symbol("[X]")
        .unchecked_symbol("[ ]");
    frame.render_widget(checkbox1, layout[0]);

    // Asterisk style
    let checkbox2 = Checkbox::new("Asterisk", app.checkboxes[App::NOTIFICATIONS])
        .checked_symbol("[*]")
        .unchecked_symbol("[ ]");
    frame.render_widget(checkbox2, layout[1]);

    // Plus style
    let checkbox3 = Checkbox::new("Plus sign", app.checkboxes[App::AUTO_SAVE])
        .checked_symbol("[+]")
        .unchecked_symbol("[-]");
    frame.render_widget(checkbox3, layout[2]);
}
