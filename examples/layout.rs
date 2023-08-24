use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use ratatui::{layout::Constraint::*, prelude::*, widgets::*};

fn main() -> Result<()> {
    let mut terminal = TerminalBuilder::crossterm_on_stdout().build()?;

    loop {
        terminal.draw(|f| ui(f))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Length(4),  // text
            Length(50), // examples
            Min(0),     // fills remaining space
        ])
        .split(frame.size());

    // title
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Horizontal Layout Example. Press q to quit".dark_gray())
                .alignment(Alignment::Center),
            Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),
            Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),
            Line::from("Note: constraint labels that don't fit are truncated"),
        ]),
        main_layout[0],
    );

    let example_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Length(9),
            Length(9),
            Length(9),
            Length(9),
            Length(9),
            Min(0), // fills remaining space
        ])
        .split(main_layout[1]);
    let example_areas = example_rows
        .iter()
        .flat_map(|area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Length(14),
                    Length(14),
                    Length(14),
                    Length(14),
                    Length(14),
                    Min(0), // fills remaining space
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
fn render_example_combination<B: Backend>(
    frame: &mut Frame<B>,
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
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(1); constraints.len() + 1])
        .split(inner);
    for (i, (a, b)) in constraints.iter().enumerate() {
        render_single_example(frame, layout[i], vec![*a, *b, Min(0)]);
    }
    // This is to make it easy to visually see the alignment of the examples
    // with the constraints.
    frame.render_widget(Paragraph::new("123456789012"), layout[6]);
}

/// Renders a single example line
fn render_single_example<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    constraints: Vec<Constraint>,
) {
    let red = Paragraph::new(constraint_label(constraints[0])).on_red();
    let blue = Paragraph::new(constraint_label(constraints[1])).on_blue();
    let green = Paragraph::new("·".repeat(12)).on_green();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);
    frame.render_widget(red, layout[0]);
    frame.render_widget(blue, layout[1]);
    frame.render_widget(green, layout[2]);
}

fn constraint_label(constraint: Constraint) -> String {
    match constraint {
        Length(n) => format!("{n}"),
        Min(n) => format!("{n}"),
        Max(n) => format!("{n}"),
        Percentage(n) => format!("{n}"),
        Ratio(a, b) => format!("{a}:{b}"),
    }
}
