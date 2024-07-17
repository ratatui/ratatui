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

use std::iter::zip;

use color_eyre::Result;
use rand::{thread_rng, Rng};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
    Frame,
};

use self::terminal::Terminal;

const COMPANY_COUNT: usize = 3;
const PERIOD_COUNT: usize = 4;

struct App {
    should_exit: bool,
    data: Vec<Bar<'static>>,
    companies: [Company; COMPANY_COUNT],
    revenues: [Revenues; PERIOD_COUNT],
}

struct Revenues {
    period: &'static str,
    revenues: [u32; COMPANY_COUNT],
}

struct Company {
    short_name: &'static str,
    name: &'static str,
    color: Color,
}

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let app = App::new();
    app.run(&mut terminal)?;
    terminal::restore_terminal()?;
    Ok(())
}

impl App {
    fn new() -> Self {
        Self {
            should_exit: false,
            data: random_barchart_data(),
            companies: fake_companies(),
            revenues: fake_revenues(),
        }
    }

    fn run(mut self, terminal: &mut Terminal) -> Result<()> {
        while !self.should_exit {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal) -> Result<()> {
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

    fn render(&self, frame: &mut Frame) {
        use Constraint::{Fill, Min};
        let [top, mid, bottom] = Layout::vertical([Fill(1), Fill(2), Min(17)])
            .spacing(1)
            .areas(frame.size());

        frame.render_widget(self.vertical_ungrouped_barchart(), top);
        frame.render_widget(self.vertical_revenue_barchart(), mid);
        frame.render_widget(self.horizontal_revenue_barchart(), bottom);
    }

    /// Create a bar chart with the data from the `data` field.
    fn vertical_ungrouped_barchart(&self) -> BarChart<'_> {
        BarChart::default()
            .block(Block::new().borders(Borders::TOP).title("Ungrouped"))
            .data(BarGroup::default().bars(&self.data))
            .bar_width(5)
            .bar_style(Style::new().fg(Color::LightYellow))
            .value_style(Style::new().fg(Color::Black).bg(Color::LightYellow))
    }

    /// Create a vertical revenue bar chart with the data from the `revenues` field.
    fn vertical_revenue_barchart(&self) -> BarChart<'_> {
        let mut barchart = BarChart::default()
            .block(Block::new().borders(Borders::TOP).title("Vertical Grouped"))
            .bar_gap(0)
            .bar_width(6)
            .group_gap(2);
        for group in self
            .revenues
            .iter()
            .map(|revenue| revenue.to_vertical_bar_group(&self.companies))
        {
            barchart = barchart.data(group);
        }
        barchart
    }

    /// Create a horizontal revenue bar chart with the data from the `revenues` field.
    fn horizontal_revenue_barchart(&self) -> BarChart<'_> {
        let mut barchart = BarChart::default()
            .block(
                Block::new()
                    .borders(Borders::TOP)
                    .title("Horizontal Grouped"),
            )
            .bar_width(1)
            .group_gap(1)
            .bar_gap(0)
            .direction(Direction::Horizontal);
        for group in self
            .revenues
            .iter()
            .map(|revenue| revenue.to_horizontal_bar_group(&self.companies))
        {
            barchart = barchart.data(group);
        }
        barchart
    }
}

/// Generate some random data for the main bar chart
fn random_barchart_data() -> Vec<Bar<'static>> {
    let mut rng = thread_rng();
    (1..50)
        .map(|index| {
            let value = rng.gen_range(60..80);
            let label = format!("{index:>02}:00").into();
            let text = format!("{value:>3}°");
            Bar::default().label(label).value(value).text_value(text)
        })
        .collect()
}

/// Generate fake company data
const fn fake_companies() -> [Company; COMPANY_COUNT] {
    [
        Company::new("BAKE", "Bake my day", Color::LightRed),
        Company::new("BITE", "Bits and Bites", Color::Blue),
        Company::new("TART", "Tart of the Table", Color::White),
    ]
}

/// Some fake revenue data
const fn fake_revenues() -> [Revenues; PERIOD_COUNT] {
    [
        Revenues::new("Jan", [8500, 6500, 7000]),
        Revenues::new("Feb", [9000, 7500, 8500]),
        Revenues::new("Mar", [12500, 4500, 8200]),
        Revenues::new("Apr", [6300, 4000, 5000]),
    ]
}

impl Revenues {
    /// Create a new instance of `Revenues`
    const fn new(period: &'static str, revenues: [u32; COMPANY_COUNT]) -> Self {
        Self { period, revenues }
    }

    /// Create a `BarGroup` with vertical bars for each company
    fn to_vertical_bar_group<'a>(&self, companies: &'a [Company]) -> BarGroup<'a> {
        let bars: Vec<Bar> = zip(companies, self.revenues)
            .map(|(company, revenue)| company.vertical_revenue_bar(revenue))
            .collect();
        BarGroup::default()
            .label(Line::from(self.period).centered())
            .bars(&bars)
    }

    /// Create a `BarGroup` with horizontal bars for each company
    fn to_horizontal_bar_group<'a>(&'a self, companies: &'a [Company]) -> BarGroup<'a> {
        let bars: Vec<Bar> = zip(companies, self.revenues)
            .map(|(company, revenue)| company.horizontal_revenue_bar(revenue))
            .collect();
        BarGroup::default()
            .label(Line::from(self.period).centered())
            .bars(&bars)
    }
}

impl Company {
    /// Create a new instance of `Company`
    const fn new(short_name: &'static str, name: &'static str, color: Color) -> Self {
        Self {
            short_name,
            name,
            color,
        }
    }

    /// Create a vertical revenue bar for the company
    ///
    /// The label is the short name of the company, and will be displayed under the bar
    fn vertical_revenue_bar(&self, revenue: u32) -> Bar {
        let text_value = format!("{:.1}M", f64::from(revenue) / 1000.);
        Bar::default()
            .label(self.short_name.into())
            .value(u64::from(revenue))
            .text_value(text_value)
            .style(self.color)
            .value_style(Style::new().fg(Color::Black).bg(self.color))
    }

    /// Create a horizontal revenue bar for the company
    ///
    /// The label is the long name of the company combined with the revenue and will be displayed
    /// on the bar
    fn horizontal_revenue_bar(&self, revenue: u32) -> Bar {
        let text_value = format!("{} ({:.1} M)", self.name, f64::from(revenue) / 1000.);
        Bar::default()
            .value(u64::from(revenue))
            .text_value(text_value)
            .style(self.color)
            .value_style(Style::new().fg(Color::Black).bg(self.color))
    }
}

/// Contains functions common to all examples
mod terminal {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{
        config::{EyreHook, HookBuilder, PanicHook},
        eyre::{self},
        Result,
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
    pub fn init() -> Result<Terminal> {
        install_hooks()?;
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
    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::FromCursorDown),
        )
    }

    /// Installs hooks for panic and error handling.
    ///
    /// Makes the app resilient to panics and errors by restoring the terminal before printing the
    /// panic or error message. This prevents error messages from being messed up by the terminal
    /// state.
    fn install_hooks() -> Result<()> {
        let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
        install_panic_hook(panic_hook);
        install_error_hook(eyre_hook)?;
        Ok(())
    }

    /// Install a panic hook that restores the terminal before printing the panic.
    fn install_panic_hook(panic_hook: PanicHook) {
        let panic_hook = panic_hook.into_panic_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = restore_terminal();
            panic_hook(panic_info);
        }));
    }

    /// Install an error hook that restores the terminal before printing the error.
    fn install_error_hook(eyre_hook: EyreHook) -> Result<()> {
        let eyre_hook = eyre_hook.into_eyre_hook();
        eyre::set_hook(Box::new(move |error| {
            let _ = restore_terminal();
            eyre_hook(error)
        }))?;
        Ok(())
    }
}
