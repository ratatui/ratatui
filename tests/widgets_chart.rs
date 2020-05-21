use tui::{
    backend::TestBackend,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType::Line},
    Terminal,
};

#[test]
fn widgets_chart_can_have_axis_with_zero_length_bounds() {
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
fn widgets_chart_handles_overflows() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|mut f| {
            let datasets = [Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[
                    (1_588_298_471.0, 1.0),
                    (1_588_298_473.0, 0.0),
                    (1_588_298_496.0, 1.0),
                ])];
            let chart = Chart::default()
                .block(Block::default().title("Plot").borders(Borders::ALL))
                .x_axis(
                    Axis::default()
                        .bounds([1_588_298_471.0, 1_588_992_600.0])
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

#[test]
fn widgets_chart_can_have_empty_datasets() {
    let backend = TestBackend::new(100, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|mut f| {
            let datasets = [Dataset::default().data(&[]).graph_type(Line)];
            let chart = Chart::default()
                .block(
                    Block::default()
                        .title("Empty Dataset With Line")
                        .borders(Borders::ALL),
                )
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
