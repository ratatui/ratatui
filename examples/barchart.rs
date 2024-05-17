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

use std::{
    io,
    time::{Duration, Instant},
};

use common::Terminal;
use crossterm::event::{self, Event, KeyCode};
use itertools::{izip, Itertools};
use rand::{thread_rng, Rng};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

const COMPANY_COUNT: usize = 3;
const PERIOD_COUNT: usize = 4;

struct App<'a> {
    exit: bool,
    data: Vec<Bar<'a>>,
    last_update: Instant,
    companies: [Company; COMPANY_COUNT],
    revenues: [Revenues; PERIOD_COUNT],
}

struct Revenues {
    period: &'static str,
    revenues: [u32; COMPANY_COUNT],
}

struct Company {
    name: &'static str,
    label: &'static str,
    color: Color,
}

fn main() -> color_eyre::Result<()> {
    common::install_hooks()?;
    let mut terminal = common::init_terminal()?;
    let app = App::new();
    app.run(&mut terminal)?;
    common::restore_terminal()?;
    Ok(())
}

impl<'a> App<'a> {
    // update the data every 250ms
    const UPDATE_RATE: Duration = Duration::from_millis(250);

    /// Create a new instance of the application
    fn new() -> Self {
        App {
            exit: false,
            data: generate_main_barchart_data(),
            last_update: Instant::now(),
            companies: Company::fake_companies(),
            revenues: Revenues::fake_revenues(),
        }
    }

    /// Run the application
    fn run(mut self, terminal: &mut Terminal) -> io::Result<()> {
        while !self.exit {
            self.draw(terminal)?;
            self.handle_events()?;
            self.update_data();
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Self::UPDATE_RATE.saturating_sub(self.last_update.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
            }
        }
        Ok(())
    }

    // Rotate the data to simulate a real-time update
    fn update_data(&mut self) {
        if self.last_update.elapsed() >= Self::UPDATE_RATE {
            let value = self.data.pop().unwrap();
            self.data.insert(0, value);
            self.last_update = Instant::now();
        }
    }
}

/// Generate some random data for the main bar chart
fn generate_main_barchart_data() -> Vec<Bar<'static>> {
    let mut rng = thread_rng();
    (1..50)
        .map(|index| {
            Bar::default()
                .label(format!("B{index:>02}").into())
                .value(rng.gen_range(1..15))
        })
        .collect()
}

impl Widget for &App<'_> {
    /// Render the application
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Percentage, Ratio};
        let [top, bottom] = Layout::vertical([Ratio(1, 3), Ratio(2, 3)]).areas(area);
        let [left, right] = Layout::horizontal([Percentage(50), Percentage(50)]).areas(bottom);

        let left_legend_area = App::legend_area(left);
        let right_legend_area = App::legend_area(right);

        self.main_barchart().render(top, buf);
        self.vertical_revenue_barchart().render(left, buf);
        self.horizontal_revenue_barchart().render(right, buf);
        self.legend().render(left_legend_area, buf);
        self.legend().render(right_legend_area, buf);
    }
}

impl App<'_> {
    const TOTAL_REVENUE_LABEL: &'static str = "Total Revenue";

    /// Create a bar chart with the data from the `data` field.
    fn main_barchart(&self) -> BarChart<'_> {
        BarChart::default()
            .block(Block::bordered().title("Vertical Grouped"))
            .data(BarGroup::default().bars(&self.data))
            .bar_width(5)
            .bar_style(Style::new().fg(Color::Yellow))
            .value_style(Style::new().fg(Color::Black).bg(Color::Yellow))
    }

    /// Create a vertical revenue bar chart with the data from the `revenues` field.
    fn vertical_revenue_barchart(&self) -> BarChart<'_> {
        let mut barchart = BarChart::default()
            .block(Block::bordered().title("Vertical Grouped"))
            .bar_width(7)
            .group_gap(3);
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
            .block(Block::bordered().title("Horizontal Grouped"))
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

    /// Calculate the area for the legend based on the width of the revenue bar chart.
    fn legend_area(area: Rect) -> Rect {
        let height = 6;
        let width = Self::TOTAL_REVENUE_LABEL.width() as u16 + 2;
        Rect::new(area.right().saturating_sub(width), area.y, width, height).intersection(area)
    }

    /// Create a `Paragraph` widget with the legend for the revenue bar charts.
    fn legend(&self) -> Paragraph<'static> {
        let mut text = vec![Line::styled(
            Self::TOTAL_REVENUE_LABEL,
            (Color::White, Modifier::BOLD),
        )];
        for company in &self.companies {
            text.push(Line::styled(format!("- {}", company.name), company.color));
        }
        Paragraph::new(text).block(Block::bordered().white())
    }
}

impl Revenues {
    /// Create a new instance of `Revenues`
    const fn new(period: &'static str, revenues: [u32; COMPANY_COUNT]) -> Self {
        Self { period, revenues }
    }

    /// Some fake revenue data
    const fn fake_revenues() -> [Self; PERIOD_COUNT] {
        [
            Self::new("Jan", [9500, 1500, 10500]),
            Self::new("Feb", [12500, 2500, 10600]),
            Self::new("Mar", [5300, 3000, 9000]),
            Self::new("Apr", [8500, 500, 4200]),
        ]
    }

    /// Create a `BarGroup` with vertical bars for each company
    fn to_vertical_bar_group<'a>(&'a self, companies: &'a [Company]) -> BarGroup<'a> {
        let bars = izip!(companies, self.revenues)
            .map(|(company, revenue)| company.vertical_revenue_bar(revenue))
            .collect_vec();
        BarGroup::default()
            .label(Line::from(self.period).centered())
            .bars(&bars)
    }

    /// Create a `BarGroup` with horizontal bars for each company
    fn to_horizontal_bar_group<'a>(&'a self, companies: &'a [Company]) -> BarGroup<'a> {
        let bars = izip!(companies, self.revenues)
            .map(|(company, revenue)| company.horizontal_revenue_bar(revenue))
            .collect_vec();
        BarGroup::default()
            .label(Line::from(self.period).centered())
            .bars(&bars)
    }
}

impl Company {
    /// Create a new instance of `Company`
    const fn new(name: &'static str, label: &'static str, color: Color) -> Self {
        Self { name, label, color }
    }

    /// Generate fake company data
    const fn fake_companies() -> [Self; COMPANY_COUNT] {
        [
            Self::new("Company A", "Comp.A", Color::Green),
            Self::new("Company B", "Comp.B", Color::Yellow),
            Self::new("Company C", "Comp.C", Color::White),
        ]
    }

    /// Create a vertical revenue bar for the company
    ///
    /// The label is the short name of the company, and will be displayed under the bar
    fn vertical_revenue_bar(&self, revenue: u32) -> Bar {
        let text_value = format!("{:.1}", f64::from(revenue) / 1000.);
        Bar::default()
            .label(self.label.into())
            .value(u64::from(revenue))
            .text_value(text_value)
            .style(self.color)
            .value_style(Style::new().fg(Color::Black).bg(self.color))
    }

    /// Create a horizontal revenue bar for the company
    ///
    /// The label is the short name of the company combined with the revenue and will be displayed
    /// on the bar
    fn horizontal_revenue_bar(&self, revenue: u32) -> Bar {
        let text_value = format!("{} ({:.1} M)", self.label, f64::from(revenue) / 1000.);
        Bar::default()
            .value(u64::from(revenue))
            .text_value(text_value)
            .style(self.color)
            .value_style(Style::new().fg(Color::Black).bg(self.color))
    }
}

/// Contains functions common to all examples
mod common {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{
        config::{EyreHook, HookBuilder, PanicHook},
        eyre::{self},
    };
    use crossterm::{
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    };
    use ratatui::backend::CrosstermBackend;

    // A type alias to simplify the usage of the terminal and make it easier to change the backend
    // or choice of writer.
    pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

    /// Initialize the terminal by enabling raw mode and entering the alternate screen.
    ///
    /// This function should be called before the program starts to ensure that the terminal is in
    /// the correct state for the application.
    pub fn init_terminal() -> io::Result<Terminal> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend)
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
    pub fn install_hooks() -> color_eyre::Result<()> {
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
    fn install_error_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
        let eyre_hook = eyre_hook.into_eyre_hook();
        eyre::set_hook(Box::new(move |error| {
            let _ = restore_terminal();
            eyre_hook(error)
        }))?;
        Ok(())
    }
}
