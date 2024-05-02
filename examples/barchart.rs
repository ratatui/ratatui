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
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::{Frame, Terminal},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Paragraph},
};
use unicode_width::UnicodeWidthStr;

mod common;

struct Company<'a> {
    revenue: [u32; 4],
    label: &'a str,
    bar_style: Style,
}

struct App<'a> {
    data: Vec<Bar<'a>>,
    months: [&'a str; 4],
    companies: [Company<'a>; 3],
}

const TOTAL_REVENUE: &str = "Total Revenue";
const TICK_RATE: Duration = Duration::from_millis(250);

fn main() -> Result<(), Box<dyn Error>> {
    common::install_hooks()?;
    let mut terminal = common::init_terminal()?;
    App::new().run(&mut terminal)?;
    common::restore_terminal()?;
    Ok(())
}

impl<'a> App<'a> {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let data = (1..24)
            .map(|index| {
                Bar::default()
                    .label(format!("B{index}").into())
                    .value(rng.gen_range(1..15))
            })
            .collect();
        App {
            data,
            companies: [
                Company {
                    label: "Comp.A",
                    revenue: [9500, 12500, 5300, 8500],
                    bar_style: Style::default().fg(Color::Green),
                },
                Company {
                    label: "Comp.B",
                    revenue: [1500, 2500, 3000, 500],
                    bar_style: Style::default().fg(Color::Yellow),
                },
                Company {
                    label: "Comp.C",
                    revenue: [10500, 10600, 9000, 4200],
                    bar_style: Style::default().fg(Color::White),
                },
            ],
            months: ["Mars", "Apr", "May", "Jun"],
        }
    }

    fn run(mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.size()))?;

            let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= TICK_RATE {
                self.update_data();
                last_tick = Instant::now();
            }
        }
    }

    fn update_data(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top, bottom] = vertical.areas(area);
        let [left, right] = horizontal.areas(bottom);

        self.main_barchart().render(top, buf);
        self.group_labels_barchart().render(left, buf);
        App::legend().render(App::legend_area(left), buf);
        self.horizontal_barchart().render(right, buf);
        App::legend().render(App::legend_area(right), buf);
    }
}

impl App<'_> {
    fn main_barchart(&self) -> BarChart<'_> {
        BarChart::default()
            .block(Block::bordered().title("Data1"))
            .data(BarGroup::default().bars(&self.data))
            .bar_width(9)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow))
    }

    fn group_labels_barchart(&self) -> BarChart<'_> {
        let groups = self.create_groups(false);

        let mut barchart = BarChart::default()
            .block(Block::default().title("Data1").borders(Borders::ALL))
            .bar_width(7)
            .group_gap(3);
        for group in groups {
            barchart = barchart.data(group);
        }
        barchart
    }

    fn horizontal_barchart(&self) -> BarChart<'_> {
        let groups = self.create_groups(true);
        let mut barchart = BarChart::default()
            .block(Block::default().title("Data1").borders(Borders::ALL))
            .bar_width(1)
            .group_gap(1)
            .bar_gap(0)
            .direction(Direction::Horizontal);
        for group in groups {
            barchart = barchart.data(group);
        }
        barchart
    }

    fn legend_area(area: Rect) -> Rect {
        let height = 6;
        let width = TOTAL_REVENUE.width() as u16 + 2;
        Rect::new(area.right().saturating_sub(width), area.y, width, height).intersection(area)
    }

    fn legend() -> Paragraph<'static> {
        let text = vec![
            Line::styled(TOTAL_REVENUE, (Color::White, Modifier::BOLD)),
            Line::styled("- Company A", Color::Green),
            Line::styled("- Company B", Color::Yellow),
            Line::styled("- Company C", Color::White),
        ];
        Paragraph::new(text).block(Block::bordered().fg(Color::White))
    }

    fn create_groups(&self, combine_values_and_labels: bool) -> Vec<BarGroup<'_>> {
        self.months
            .iter()
            .enumerate()
            .map(|(i, &month)| {
                let bars: Vec<Bar> = self
                    .companies
                    .iter()
                    .map(|c| {
                        let mut bar = Bar::default()
                            .value(u64::from(c.revenue[i]))
                            .style(c.bar_style)
                            .value_style((Color::Black, c.bar_style.fg.unwrap_or(Color::White)));

                        if combine_values_and_labels {
                            bar = bar.text_value(format!(
                                "{} ({:.1} M)",
                                c.label,
                                f64::from(c.revenue[i]) / 1000.
                            ));
                        } else {
                            bar = bar
                                .text_value(format!("{:.1}", f64::from(c.revenue[i]) / 1000.))
                                .label(c.label.into());
                        }
                        bar
                    })
                    .collect();
                BarGroup::default()
                    .label(Line::from(month).centered())
                    .bars(&bars)
            })
            .collect()
    }
}
