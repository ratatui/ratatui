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
use ratatui_widgets::borders::BorderType;
use ratatui_widgets::checkbox::Checkbox;

#[derive(Clone, Copy)]
struct CheckboxConfig<'a> {
    label: &'a str,
    checked: bool,
    checkbox_style: Option<Style>,
    label_style: Option<Style>,
    is_selected: bool,
}

impl<'a> CheckboxConfig<'a> {
    const fn new(label: &'a str, checked: bool, is_selected: bool) -> Self {
        Self {
            label,
            checked,
            checkbox_style: None,
            label_style: None,
            is_selected,
        }
    }

    const fn checkbox_style(mut self, style: Style) -> Self {
        self.checkbox_style = Some(style);
        self
    }

    const fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }
}

#[derive(Clone, Copy)]
struct CheckboxCustomConfig<'a> {
    label: &'a str,
    checked: bool,
    checked_symbol: &'a str,
    unchecked_symbol: &'a str,
    checkbox_style: Option<Style>,
    label_style: Option<Style>,
    is_selected: bool,
}

impl<'a> CheckboxCustomConfig<'a> {
    const fn new(
        label: &'a str,
        checked: bool,
        checked_symbol: &'a str,
        unchecked_symbol: &'a str,
        is_selected: bool,
    ) -> Self {
        Self {
            label,
            checked,
            checked_symbol,
            unchecked_symbol,
            checkbox_style: None,
            label_style: None,
            is_selected,
        }
    }
}

struct App {
    // 4 rows x 4 columns grid (first row only uses 1, emoji row uses 3)
    checkboxes: [[bool; 4]; 4],
    selected_row: usize,
    selected_col: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            checkboxes: [
                [true, false, false, false], // Basic row (only first used)
                [true, false, true, false],  // Styled row
                [true, false, true, false],  // Emoji row (only first 3 used)
                [true, false, true, false],  // Custom row
            ],
            selected_row: 0,
            selected_col: 0,
        }
    }
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
                        if app.selected_row > 0 {
                            app.selected_row -= 1;
                            // When moving to row 0, reset to column 0 (only one checkbox)
                            if app.selected_row == 0 {
                                app.selected_col = 0;
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected_row < 3 {
                            app.selected_row += 1;
                            // When moving to row 0, reset to column 0 (only one checkbox)
                            if app.selected_row == 0 {
                                app.selected_col = 0;
                            }
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if app.selected_col > 0 {
                            // Row 0 only has 1 checkbox, so don't allow left movement
                            if app.selected_row != 0 {
                                app.selected_col -= 1;
                            }
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        let max_col = match app.selected_row {
                            0 => 0, // Basic row has 1 checkbox
                            2 => 2, // Emoji row has 3 checkboxes
                            _ => 3, // Other rows have 4 checkboxes
                        };
                        if app.selected_col < max_col {
                            app.selected_col += 1;
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        app.checkboxes[app.selected_row][app.selected_col] =
                            !app.checkboxes[app.selected_row][app.selected_col];
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
        Constraint::Min(8),
        Constraint::Min(8),
        Constraint::Min(8),
        Constraint::Min(8),
    ])
    .spacing(1);

    let [title_area, basic_area, styled_area, emoji_area, custom_area] =
        frame.area().layout(&vertical);

    // Title
    let title = Line::from_iter([
        Span::from("Checkbox Widget Demo").bold(),
        Span::from(" (↑/↓/←/→: navigate, Space: toggle, q: quit)"),
    ]);
    frame.render_widget(title.centered(), title_area);

    // Basic checkboxes
    render_basic_checkboxes(frame, basic_area, app);

    // Styled checkboxes
    render_styled_checkboxes(frame, styled_area, app);

    // Emoji checkboxes
    render_emoji_checkboxes(frame, emoji_area, app);

    // Custom symbol checkboxes
    render_custom_checkboxes(frame, custom_area, app);
}

fn render_basic_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let outer_block = Block::bordered().title("Basic Checkbox");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Center the single checkbox horizontally
    let columns = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(25),
        Constraint::Fill(1),
    ])
    .split(inner);

    // Only one checkbox: Notifications
    let config = CheckboxConfig::new(
        "Notifications",
        app.checkboxes[0][0],
        app.selected_row == 0 && app.selected_col == 0,
    );
    render_checkbox_in_area(frame, columns[1], config);
}

fn render_styled_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let outer_block = Block::bordered().title("Styled Checkboxes");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let columns = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(inner);

    // Checkbox 0: Success (Green)
    let config = CheckboxConfig::new(
        "Success",
        app.checkboxes[1][0],
        app.selected_row == 1 && app.selected_col == 0,
    )
    .checkbox_style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )
    .label_style(Style::default().fg(Color::Green));
    render_checkbox_in_area(frame, columns[0], config);

    // Checkbox 1: Error (Red)
    let config = CheckboxConfig::new(
        "Error",
        app.checkboxes[1][1],
        app.selected_row == 1 && app.selected_col == 1,
    )
    .checkbox_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
    .label_style(Style::default().fg(Color::Red));
    render_checkbox_in_area(frame, columns[1], config);

    // Checkbox 2: Info (Blue)
    let config = CheckboxConfig::new(
        "Info",
        app.checkboxes[1][2],
        app.selected_row == 1 && app.selected_col == 2,
    )
    .checkbox_style(Style::default().fg(Color::Blue))
    .label_style(Style::default().fg(Color::Blue));
    render_checkbox_in_area(frame, columns[2], config);

    // Checkbox 3: Warning (Yellow)
    let config = CheckboxConfig::new(
        "Warning",
        app.checkboxes[1][3],
        app.selected_row == 1 && app.selected_col == 3,
    )
    .checkbox_style(Style::default().fg(Color::Yellow))
    .label_style(Style::default().fg(Color::Yellow));
    render_checkbox_in_area(frame, columns[3], config);
}

fn render_emoji_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let outer_block = Block::bordered().title("Emoji & Unicode Checkboxes");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let columns = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(inner);

    // Checkbox 0: Emoji style
    let config = CheckboxCustomConfig::new(
        "Emoji style",
        app.checkboxes[2][0],
        "✅ ",
        "⬜ ",
        app.selected_row == 2 && app.selected_col == 0,
    );
    render_checkbox_in_area_custom(frame, columns[0], config);

    // Checkbox 1: Circle
    let config = CheckboxCustomConfig::new(
        "Circle",
        app.checkboxes[2][1],
        "● ",
        "○ ",
        app.selected_row == 2 && app.selected_col == 1,
    );
    render_checkbox_in_area_custom(frame, columns[1], config);

    // Checkbox 2: Diamond
    let config = CheckboxCustomConfig::new(
        "Diamond",
        app.checkboxes[2][2],
        "◆ ",
        "◇ ",
        app.selected_row == 2 && app.selected_col == 2,
    );
    render_checkbox_in_area_custom(frame, columns[2], config);
}

fn render_custom_checkboxes(frame: &mut Frame, area: Rect, app: &App) {
    let outer_block = Block::bordered().title("Custom Symbols");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    let columns = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(inner);

    // Checkbox 0: ASCII style
    let config = CheckboxCustomConfig::new(
        "ASCII style",
        app.checkboxes[3][0],
        "[X]",
        "[ ]",
        app.selected_row == 3 && app.selected_col == 0,
    );
    render_checkbox_in_area_custom(frame, columns[0], config);

    // Checkbox 1: Asterisk
    let config = CheckboxCustomConfig::new(
        "Asterisk",
        app.checkboxes[3][1],
        "[*]",
        "[ ]",
        app.selected_row == 3 && app.selected_col == 1,
    );
    render_checkbox_in_area_custom(frame, columns[1], config);

    // Checkbox 2: Plus/Minus (dynamic label)
    let label = if app.checkboxes[3][2] {
        "Plus sign"
    } else {
        "Minus sign"
    };
    let config = CheckboxCustomConfig::new(
        label,
        app.checkboxes[3][2],
        "[+]",
        "[-]",
        app.selected_row == 3 && app.selected_col == 2,
    );
    render_checkbox_in_area_custom(frame, columns[2], config);

    // Checkbox 3: X/O style
    let config = CheckboxCustomConfig::new(
        "X/O style",
        app.checkboxes[3][3],
        "(X)",
        "(O)",
        app.selected_row == 3 && app.selected_col == 3,
    );
    render_checkbox_in_area_custom(frame, columns[3], config);
}

fn render_checkbox_in_area(frame: &mut Frame, area: Rect, config: CheckboxConfig) {
    let border_style = if config.is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let border_type = if config.is_selected {
        BorderType::Double
    } else {
        BorderType::Plain
    };

    let block = Block::bordered()
        .border_type(border_type)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Center the checkbox vertically
    let centered = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(inner);

    let mut checkbox = Checkbox::new(config.label, config.checked);

    if let Some(style) = config.checkbox_style {
        checkbox = checkbox.checkbox_style(style);
    }

    if let Some(style) = config.label_style {
        checkbox = checkbox.label_style(style);
    }

    frame.render_widget(checkbox, centered[1]);
}

fn render_checkbox_in_area_custom(frame: &mut Frame, area: Rect, config: CheckboxCustomConfig) {
    let border_style = if config.is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let border_type = if config.is_selected {
        BorderType::Double
    } else {
        BorderType::Plain
    };

    let block = Block::bordered()
        .border_type(border_type)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Center the checkbox vertically
    let centered = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(inner);

    let mut checkbox = Checkbox::new(config.label, config.checked)
        .checked_symbol(config.checked_symbol)
        .unchecked_symbol(config.unchecked_symbol);

    if let Some(style) = config.checkbox_style {
        checkbox = checkbox.checkbox_style(style);
    }

    if let Some(style) = config.label_style {
        checkbox = checkbox.label_style(style);
    }

    frame.render_widget(checkbox, centered[1]);
}
