use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

const TABS_COUNT: usize = 4;

const PALETTES: [tailwind::Palette; TABS_COUNT] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const BORDER_TYPES: [BorderType; TABS_COUNT] = [
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
    fn tab_title(value: SelectedTab) -> Line<'static> {
        let text = format!("  {value}  ");
        let palette = &PALETTES[value as usize];
        text.fg(tailwind::SLATE.c200).bg(palette.c900).into()
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
    let titles = SelectedTab::iter().map(SelectedTab::tab_title);
    let block = Block::new()
        .title("Tabs Example".bold())
        .title("Use h l or ◄ ► to change tab")
        .title_alignment(Alignment::Center)
        .padding(Padding::top(1));

    let selected_tab_index = app.selected_tab as usize;
    let tabs = Tabs::new(titles)
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
