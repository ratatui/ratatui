//! # [Ratatui] BarChart example
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

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct Company<'a> {
    revenue: [u64; 4],
    label: &'a str,
    bar_style: Style,
}

struct App<'a> {
    data: Vec<(&'a str, u64)>,
    months: [&'a str; 4],
    companies: [Company<'a>; 3],
}

const TOTAL_REVENUE: &str = "Total Revenue";

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            data: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
                ("B15", 4),
                ("B16", 6),
                ("B17", 4),
                ("B18", 7),
                ("B19", 13),
                ("B20", 8),
                ("B21", 11),
                ("B22", 9),
                ("B23", 3),
                ("B24", 5),
            ],
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

    fn on_tick(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
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
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);
    let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [top, bottom] = frame.size().split(&vertical);
    let [left, right] = bottom.split(&horizontal);

    let barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    frame.render_widget(barchart, top);
    draw_bar_with_group_labels(frame, app, left);
    draw_horizontal_bars(frame, app, right);
}

fn create_groups<'a>(app: &'a App, combine_values_and_labels: bool) -> Vec<BarGroup<'a>> {
    app.months
        .iter()
        .enumerate()
        .map(|(i, &month)| {
            let bars: Vec<Bar> = app
                .companies
                .iter()
                .map(|c| {
                    let mut bar = Bar::default()
                        .value(c.revenue[i])
                        .style(c.bar_style)
                        .value_style(
                            Style::default()
                                .bg(c.bar_style.fg.unwrap())
                                .fg(Color::Black),
                        );

                    if combine_values_and_labels {
                        bar = bar.text_value(format!(
                            "{} ({:.1} M)",
                            c.label,
                            (c.revenue[i] as f64) / 1000.
                        ));
                    } else {
                        bar = bar
                            .text_value(format!("{:.1}", (c.revenue[i] as f64) / 1000.))
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

fn draw_bar_with_group_labels(f: &mut Frame, app: &App, area: Rect) {
    let groups = create_groups(app, false);

    let mut barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .bar_width(7)
        .group_gap(3);

    for group in groups {
        barchart = barchart.data(group)
    }

    f.render_widget(barchart, area);

    const LEGEND_HEIGHT: u16 = 6;
    if area.height >= LEGEND_HEIGHT && area.width >= TOTAL_REVENUE.len() as u16 + 2 {
        let legend_width = TOTAL_REVENUE.len() as u16 + 2;
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: legend_width,
            y: area.y,
            x: area.right() - legend_width,
        };
        draw_legend(f, legend_area);
    }
}

fn draw_horizontal_bars(f: &mut Frame, app: &App, area: Rect) {
    let groups = create_groups(app, true);

    let mut barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .bar_width(1)
        .group_gap(1)
        .bar_gap(0)
        .direction(Direction::Horizontal);

    for group in groups {
        barchart = barchart.data(group)
    }

    f.render_widget(barchart, area);

    const LEGEND_HEIGHT: u16 = 6;
    if area.height >= LEGEND_HEIGHT && area.width >= TOTAL_REVENUE.len() as u16 + 2 {
        let legend_width = TOTAL_REVENUE.len() as u16 + 2;
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: legend_width,
            y: area.y,
            x: area.right() - legend_width,
        };
        draw_legend(f, legend_area);
    }
}

fn draw_legend(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(Span::styled(
            TOTAL_REVENUE,
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        )),
        Line::from(Span::styled(
            "- Company A",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "- Company B",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(vec![Span::styled(
            "- Company C",
            Style::default().fg(Color::White),
        )]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
