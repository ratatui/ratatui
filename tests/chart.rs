use tui::{
    backend::TestBackend,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Terminal,
};

#[test]
fn zero_axes_ok() {
    let backend = TestBackend::new(100, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|mut f| {
            let datasets = [Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(0.0, 0.0)])];
            let chart = Chart::default()
                .block(Block::default().title("Plot").borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 0.0]).labels(&["0.0", "1.0"]))
                .y_axis(Axis::default().bounds([0.0, 1.0]).labels(&["0.0", "1.0"]))
                .datasets(&datasets);
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                },
            );
        })
        .unwrap();
}

#[test]
fn handles_overflow() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|mut f| {
            let datasets = [Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[
                    (1588298471.0, 1.0),
                    (1588298473.0, 0.0),
                    (1588298496.0, 1.0),
                ])];
            let chart = Chart::default()
                .block(Block::default().title("Plot").borders(Borders::ALL))
                .x_axis(
                    Axis::default()
                        .bounds([1588298471.0, 1588992600.0])
                        .labels(&["1588298471.0", "1588992600.0"]),
                )
                .y_axis(Axis::default().bounds([0.0, 1.0]).labels(&["0.0", "1.0"]))
                .datasets(&datasets);
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 80,
                    height: 30,
                },
            );
        })
        .unwrap();
}
