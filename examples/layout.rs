//! # [Ratatui] Layout example
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

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{layout::Constraint::*, prelude::*, widgets::*};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Length(4),  // text
        Length(50), // examples
        Min(0),     // fills remaining space
    ]);
    let [text_area, examples_area, _] = frame.size().split(&vertical);

    // title
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Horizontal Layout Example. Press q to quit".dark_gray()).centered(),
            Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),
            Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),
            Line::from("Note: constraint labels that don't fit are truncated"),
        ]),
        text_area,
    );

    let example_rows = Layout::vertical([
        Length(9),
        Length(9),
        Length(9),
        Length(9),
        Length(9),
        Min(0), // fills remaining space
    ])
    .split(examples_area);
    let example_areas = example_rows
        .iter()
        .flat_map(|area| {
            Layout::horizontal([
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Length(14),
                Constraint::Min(0), // fills remaining space
            ])
            .split(*area)
            .iter()
            .copied()
            .take(5) // ignore Min(0)
            .collect_vec()
        })
        .collect_vec();

    // the examples are a cartesian product of the following constraints
    // e.g. Len/Len, Len/Min, Len/Max, Len/Perc, Len/Ratio, Min/Len, Min/Min, ...
    let examples = [
        (
            "Len",
            vec![
                Length(0),
                Length(2),
                Length(3),
                Length(6),
                Length(10),
                Length(15),
            ],
        ),
        (
            "Min",
            vec![Min(0), Min(2), Min(3), Min(6), Min(10), Min(15)],
        ),
        (
            "Max",
            vec![Max(0), Max(2), Max(3), Max(6), Max(10), Max(15)],
        ),
        (
            "Perc",
            vec![
                Percentage(0),
                Percentage(25),
                Percentage(50),
                Percentage(75),
                Percentage(100),
                Percentage(150),
            ],
        ),
        (
            "Ratio",
            vec![
                Ratio(0, 4),
                Ratio(1, 4),
                Ratio(2, 4),
                Ratio(3, 4),
                Ratio(4, 4),
                Ratio(6, 4),
            ],
        ),
    ];

    for (i, (a, b)) in examples
        .iter()
        .cartesian_product(examples.iter())
        .enumerate()
    {
        let (name_a, examples_a) = a;
        let (name_b, examples_b) = b;
        let constraints = examples_a
            .iter()
            .copied()
            .zip(examples_b.iter().copied())
            .collect_vec();
        render_example_combination(
            frame,
            example_areas[i],
            &format!("{name_a}/{name_b}"),
            constraints,
        );
    }
}

/// Renders a single example box
fn render_example_combination(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    constraints: Vec<(Constraint, Constraint)>,
) {
    let block = Block::default()
        .title(title.gray())
        .style(Style::reset())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let layout = Layout::vertical(vec![Length(1); constraints.len() + 1]).split(inner);
    for (i, (a, b)) in constraints.iter().enumerate() {
        render_single_example(frame, layout[i], vec![*a, *b, Min(0)]);
    }
    // This is to make it easy to visually see the alignment of the examples
    // with the constraints.
    frame.render_widget(Paragraph::new("123456789012"), layout[6]);
}

/// Renders a single example line
fn render_single_example(frame: &mut Frame, area: Rect, constraints: Vec<Constraint>) {
    let red = Paragraph::new(constraint_label(constraints[0])).on_red();
    let blue = Paragraph::new(constraint_label(constraints[1])).on_blue();
    let green = Paragraph::new("·".repeat(12)).on_green();
    let horizontal = Layout::horizontal(constraints);
    let [r, b, g] = area.split(&horizontal);
    frame.render_widget(red, r);
    frame.render_widget(blue, b);
    frame.render_widget(green, g);
}

fn constraint_label(constraint: Constraint) -> String {
    match constraint {
        Length(n) => format!("{n}"),
        Min(n) => format!("{n}"),
        Max(n) => format!("{n}"),
        Percentage(n) => format!("{n}"),
        Proportional(n) => format!("{n}"),
        Fixed(n) => format!("{n}"),
        Ratio(a, b) => format!("{a}:{b}"),
    }
}
