//! # [Ratatui] Line Gauge example
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

use std::{io::stdout, time::Duration};

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    terminal::Terminal,
    widgets::{block::Title, Block, Borders, LineGauge, Padding, Paragraph, Widget},
};

const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress_columns: u16,
    progress: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Started,
    Quitting,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.state != AppState::Quitting {
            self.draw(&mut terminal)?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn update(&mut self, terminal_width: u16) {
        if self.state != AppState::Started {
            return;
        }

        self.progress_columns = (self.progress_columns + 1).clamp(0, terminal_width);
        self.progress = f64::from(self.progress_columns) / f64::from(terminal_width);
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') | KeyCode::Enter => self.start(),
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn start(&mut self) {
        self.state = AppState::Started;
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min, Ratio};
        let layout = Layout::vertical([Length(2), Min(0), Length(1)]);
        let [header_area, main_area, footer_area] = layout.areas(area);

        let layout = Layout::vertical([Ratio(1, 3); 3]);
        let [gauge1_area, gauge2_area, gauge3_area] = layout.areas(main_area);

        header().render(header_area, buf);
        footer().render(footer_area, buf);

        self.render_gauge1(gauge1_area, buf);
        self.render_gauge2(gauge2_area, buf);
        self.render_gauge3(gauge3_area, buf);
    }
}

fn header() -> impl Widget {
    Paragraph::new("Ratatui Line Gauge Example")
        .bold()
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
}

fn footer() -> impl Widget {
    Paragraph::new("Press ENTER / SPACE to start")
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .bold()
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Blue / red only foreground");
        LineGauge::default()
            .block(title)
            .filled_style(Style::default().fg(Color::Blue))
            .unfilled_style(Style::default().fg(Color::Red))
            .label("Foreground:")
            .ratio(self.progress)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Blue / red only background");
        LineGauge::default()
            .block(title)
            .filled_style(Style::default().fg(Color::Blue).bg(Color::Blue))
            .unfilled_style(Style::default().fg(Color::Red).bg(Color::Red))
            .label("Background:")
            .ratio(self.progress)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Fully styled with background");
        LineGauge::default()
            .block(title)
            .filled_style(
                Style::default()
                    .fg(tailwind::BLUE.c400)
                    .bg(tailwind::BLUE.c600),
            )
            .unfilled_style(
                Style::default()
                    .fg(tailwind::RED.c400)
                    .bg(tailwind::RED.c800),
            )
            .label("Both:")
            .ratio(self.progress)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::default()
        .title(title)
        .borders(Borders::NONE)
        .fg(CUSTOM_LABEL_COLOR)
        .padding(Padding::vertical(1))
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
        panic(info);
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
