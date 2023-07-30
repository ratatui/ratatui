use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
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
                    revenue: [1500, 2500, 3000, 4100],
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

fn main() -> Result<()> {
    let backend = CrosstermBackend::on_stdout()?;
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();

    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, chunks[0]);

    draw_bar_with_group_labels(f, app, chunks[1], false);
    draw_bar_with_group_labels(f, app, chunks[2], true);
}

fn draw_bar_with_group_labels<B>(f: &mut Frame<B>, app: &App, area: Rect, bar_labels: bool)
where
    B: Backend,
{
    let groups: Vec<BarGroup> = app
        .months
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
                        )
                        .text_value(format!("{:.1}", (c.revenue[i] as f64) / 1000.));
                    if bar_labels {
                        bar = bar.label(c.label.into());
                    }
                    bar
                })
                .collect();
            BarGroup::default().label(month.into()).bars(&bars)
        })
        .collect();

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
        let legend_area = Rect {
            height: LEGEND_HEIGHT,
            width: TOTAL_REVENUE.len() as u16 + 2,
            y: area.y,
            x: area.x,
        };
        draw_legend(f, legend_area);
    }
}

fn draw_legend<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
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
