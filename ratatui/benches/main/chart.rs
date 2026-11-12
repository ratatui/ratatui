use criterion::{Bencher, BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType, Widget};

/// Benchmarks for rendering a chart with three datasets.
fn chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart");

    for point_count in [100, 1_000, 10_000] {
        let series: Vec<Vec<(f64, f64)>> = (0..3).map(|i| wave(i, point_count)).collect();
        for graph_type in [GraphType::Line, GraphType::Scatter] {
            group.bench_with_input(
                BenchmarkId::new(format!("render/{graph_type}"), point_count),
                &(graph_type, &*series),
                |b, &(graph_type, series)| render(b, graph_type, series, Rect::new(0, 0, 200, 50)),
            );
        }
    }

    // A small chart like the ones on low-resolution embedded displays.
    let series: Vec<Vec<(f64, f64)>> = (0..3).map(|i| wave(i, 96)).collect();
    group.bench_with_input(
        BenchmarkId::new("render_small/Line", 96),
        &*series,
        |b, series| render(b, GraphType::Line, series, Rect::new(0, 0, 34, 11)),
    );

    group.finish();
}

/// A deterministic wave in `-1.0..=1.0` with `count` points, shifted in phase by `index`.
fn wave(index: usize, count: usize) -> Vec<(f64, f64)> {
    (0..count)
        .map(|i| {
            let x = i as f64;
            let y = (x / 7.0 + index as f64).sin() * (x / 53.0).cos();
            (x, y)
        })
        .collect()
}

fn render(bencher: &mut Bencher, graph_type: GraphType, series: &[Vec<(f64, f64)>], area: Rect) {
    let mut buffer = Buffer::empty(area);
    let colors = [Color::Red, Color::Green, Color::Blue];
    let max_x = series.first().map_or(0, Vec::len).saturating_sub(1) as f64;
    bencher.iter(|| {
        let datasets = series
            .iter()
            .zip(colors)
            .map(|(data, color)| {
                Dataset::default()
                    .marker(Marker::Braille)
                    .graph_type(graph_type)
                    .style(color)
                    .data(data)
            })
            .collect();
        Chart::new(datasets)
            .block(Block::bordered())
            .x_axis(Axis::default().bounds([0.0, max_x]))
            .y_axis(Axis::default().bounds([-1.5, 1.5]))
            .render(area, &mut buffer);
    });
}

criterion_group!(benches, chart);
