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
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::iter::zip;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

const COMPANY_COUNT: usize = 3;
const PERIOD_COUNT: usize = 4;

struct App {
    should_exit: bool,
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

impl App {
    const fn new() -> Self {
        Self {
            should_exit: false,
            companies: fake_companies(),
            revenues: fake_revenues(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};
        let vertical = Layout::vertical([Length(1), Fill(1), Min(20)]).spacing(1);
        let [title, top, bottom] = vertical.areas(frame.area());

        frame.render_widget("Grouped Barchart".bold().into_centered_line(), title);
        frame.render_widget(self.vertical_revenue_barchart(), top);
        frame.render_widget(self.horizontal_revenue_barchart(), bottom);
    }

    /// Create a vertical revenue bar chart with the data from the `revenues` field.
    fn vertical_revenue_barchart(&self) -> BarChart<'_> {
        let mut barchart = BarChart::default()
            .block(Block::new().title(Line::from("Company revenues (Vertical)").centered()))
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
        let title = Line::from("Company Revenues (Horizontal)").centered();
        let mut barchart = BarChart::default()
            .block(Block::new().title(title))
            .bar_width(1)
            .group_gap(2)
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
        Revenues::new("Mar", [9500, 4500, 8200]),
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
