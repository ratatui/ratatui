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

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            SelectedTab::Tab0 => self.render_tab0(area, buf),
            SelectedTab::Tab1 => self.render_tab1(area, buf),
            SelectedTab::Tab2 => self.render_tab2(area, buf),
            SelectedTab::Tab3 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    fn render_tab0(&self, area: Rect, buf: &mut Buffer) {
        SelectedTab::inner_block(0).render(area, buf)
    }

    fn render_tab1(&self, area: Rect, buf: &mut Buffer) {
        SelectedTab::inner_block(1).render(area, buf)
    }

    fn render_tab2(&self, area: Rect, buf: &mut Buffer) {
        SelectedTab::inner_block(2).render(area, buf)
    }

    fn render_tab3(&self, area: Rect, buf: &mut Buffer) {
        SelectedTab::inner_block(3).render(area, buf)
    }

    /// Get the generated inner for the current tab.
    fn inner_block(index: usize) -> Block<'static> {
        let inner_block = Block::default()
            .title(format!("Inner {index}"))
            .borders(Borders::ALL)
            .border_type(BORDER_TYPES[index])
            .border_style(Style::new().fg(PALETTES[index].c600));
        inner_block
    }
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(4), Constraint::Min(3)]);
        let [tabs_area, inner_area] = area.split(&vertical);

        self.render_tabs(tabs_area, buf);
        self.render_inner(inner_area, buf);
    }
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

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Tabs Example".bold())
            .title("Use h l or ◄ ► to change tab")
            .title_alignment(Alignment::Center)
            .padding(Padding::top(1)); // padding to separate tabs from block title.

        let selected_tab_index = self.selected_tab as usize;
        // Gets tab titles from `SelectedTab::iter()`
        Tabs::new(SelectedTab::iter())
            .block(block)
            .highlight_style(
                Style::new()
                    .bg(PALETTES[selected_tab_index].c600)
                    .underlined(),
            )
            .select(selected_tab_index)
            .padding("", "")
            .divider(" | ")
            .render(area, buf);
    }

    fn render_inner(&self, area: Rect, buf: &mut Buffer) {
        self.selected_tab.render(area, buf);
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
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
        app.draw(terminal)?;

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
