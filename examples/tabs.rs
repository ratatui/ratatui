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

use std::{error::Error, io, io::stdout};

use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
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

struct App {
    pub selected_tab: SelectedTab,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // create app and run it
    App::new().run(terminal)?;

    restore_terminal()?;

    Ok(())
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
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
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(2),
        ]);
        let [header_area, tabs_area, inner_area, footer_area] = area.split(&vertical);

        self.render_header(header_area, buf);
        self.render_tabs(tabs_area, buf);
        self.render_inner(inner_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('l') | KeyCode::Right => self.next(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous(),
                        _ => {}
                    }
                }
            }
        }
    }
    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Tabs Example".bold())
            .alignment(Alignment::Center)
            .render(area, buf);
    }
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let selected_tab_index = self.selected_tab as usize;
        // Gets tab titles from `SelectedTab::iter()`
        Tabs::new(SelectedTab::iter())
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

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("\nUse h l or ◄ ► to change tab")
            .alignment(Alignment::Center)
            .render(area, buf);
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
        Paragraph::new("Hello, World!")
            .block(SelectedTab::inner_block(0))
            .render(area, buf)
    }

    fn render_tab1(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(SelectedTab::inner_block(1))
            .render(area, buf)
    }

    fn render_tab2(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(SelectedTab::inner_block(2))
            .render(area, buf)
    }

    fn render_tab3(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(SelectedTab::inner_block(3))
            .render(area, buf)
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

impl From<SelectedTab> for Line<'_> {
    /// Return enum name as a styled `Line` with two spaces both left and right.
    fn from(value: SelectedTab) -> Self {
        format!("  {value}  ")
            .fg(tailwind::SLATE.c200)
            .bg(PALETTES[value as usize].c900)
            .into()
    }
}
