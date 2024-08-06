//! # [Ratatui] `BarChart` example
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

use color_eyre::Result;
use rand::{thread_rng, Rng};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    prelude::{Color, Line, Style, Stylize},
    widgets::{Bar, BarChart, BarGroup, Block},
};

use self::terminal::Terminal;

struct App {
    should_exit: bool,
    temperatures: Vec<u8>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = terminal::init()?;
    let app = App::new();
    app.run(&mut terminal)?;
    terminal::restore()?;
    Ok(())
}

impl App {
    fn new() -> Self {
        let mut rng = thread_rng();
        let temperatures = (0..24).map(|_| rng.gen_range(50..90)).collect();
        Self {
            should_exit: false,
            temperatures,
        }
    }

    fn run(mut self, terminal: &mut Terminal) -> Result<()> {
        while !self.should_exit {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal) -> Result<()> {
        terminal.draw(|frame| self.render(frame))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        let [title, vertical, horizontal] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(frame.area());
        frame.render_widget("Barchart".bold().into_centered_line(), title);
        frame.render_widget(vertical_barchart(&self.temperatures), vertical);
        frame.render_widget(horizontal_barchart(&self.temperatures), horizontal);
    }
}

/// Create a vertical bar chart from the temperatures data.
fn vertical_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .map(|v| u64::from(*v))
        .enumerate()
        .map(|(i, value)| {
            Bar::default()
                .value(value)
                .label(Line::from(format!("{i:>02}:00")))
                .text_value(format!("{value:>3}°"))
                .style(temperature_style(value))
                .value_style(temperature_style(value).reversed())
        })
        .collect();
    let title = Line::from("Weather (Vertical)").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(5)
}

/// Create a horizontal bar chart from the temperatures data.
fn horizontal_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .map(|v| u64::from(*v))
        .enumerate()
        .map(|(i, value)| {
            let style = temperature_style(value);
            Bar::default()
                .value(value)
                .label(Line::from(format!("{i:>02}:00")))
                .text_value(format!("{value:>3}°"))
                .style(style)
                .value_style(style.reversed())
        })
        .collect();
    let title = Line::from("Weather (Horizontal)").centered();
    BarChart::default()
        .block(Block::new().title(title))
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(0)
        .direction(Direction::Horizontal)
}

/// create a yellow to red value based on the value (50-90)
fn temperature_style(value: u64) -> Style {
    let green = (255.0 * (1.0 - (value - 50) as f64 / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().fg(color)
}

/// Contains functions common to all examples
mod terminal {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use ratatui::{
        backend::CrosstermBackend,
        crossterm::{
            execute,
            terminal::{
                disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
                LeaveAlternateScreen,
            },
        },
    };

    // A type alias to simplify the usage of the terminal and make it easier to change the backend
    // or choice of writer.
    pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

    /// Initialize the terminal by enabling raw mode and entering the alternate screen.
    ///
    /// This function should be called before the program starts to ensure that the terminal is in
    /// the correct state for the application.
    pub fn init() -> io::Result<Terminal> {
        install_panic_hook();
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    /// Restore the terminal by leaving the alternate screen and disabling raw mode.
    ///
    /// This function should be called before the program exits to ensure that the terminal is
    /// restored to its original state.
    pub fn restore() -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::FromCursorDown),
        )
    }

    /// Install a panic hook that restores the terminal before printing the panic.
    ///
    /// This prevents error messages from being messed up by the terminal state.
    fn install_panic_hook() {
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = restore();
            panic_hook(panic_info);
        }));
    }
}
