//! # [Ratatui] Tabs example
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
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

const PALETTES: &[tailwind::Palette] = &[
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const BORDER_TYPES: &[BorderType] = &[
    BorderType::Rounded,
    BorderType::Plain,
    BorderType::Double,
    BorderType::Thick,
];

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    Tab0,
    Tab1,
    Tab2,
    Tab3,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(&self) -> Self {
        let current_index = *self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(*self)
    }
}

impl From<SelectedTab> for Line<'_> {
    /// Return enum name as a styled `Line` with two spaces both left and right.
    fn from(value: SelectedTab) -> Self {
        format!("  {value}  ")
            .fg(tailwind::SLATE.c200)
            .bg(PALETTES[value as usize].c900)
            .into()
    }
}

struct App {
    pub selected_tab: SelectedTab,
}

impl App {
    fn new() -> App {
        App {
            selected_tab: SelectedTab::default(),
        }
    }

    pub fn next(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('l') | KeyCode::Right => app.next(),
                    KeyCode::Char('h') | KeyCode::Left => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.size();
    let vertical = Layout::vertical([Constraint::Length(4), Constraint::Min(3)]);
    let [tabs_area, inner_area] = area.split(&vertical);

    render_tabs(f, app, tabs_area);

    render_inner(f, app, inner_area);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::new()
        .title("Tabs Example".bold())
        .title("Use h l or ◄ ► to change tab")
        .title_alignment(Alignment::Center)
        .padding(Padding::top(1)); // padding to separate tabs from block title.

    let selected_tab_index = app.selected_tab as usize;
    // Gets tab titles from `SelectedTab::iter()`
    let tabs = Tabs::new(SelectedTab::iter())
        .block(block)
        .highlight_style(
            Style::new()
                .bg(PALETTES[selected_tab_index].c600)
                .underlined(),
        )
        .select(selected_tab_index)
        .padding("", "")
        .divider(" | ");

    f.render_widget(tabs, area);
}

fn render_inner(f: &mut Frame, app: &App, area: Rect) {
    let index = app.selected_tab as usize;
    let inner_block = Block::default()
        .title(format!("Inner {index}"))
        .borders(Borders::ALL)
        .border_type(BORDER_TYPES[index])
        .border_style(Style::new().fg(PALETTES[index].c600));

    f.render_widget(inner_block, area);
}
