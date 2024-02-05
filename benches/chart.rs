use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Chart, Dataset, Widget},
};

/// Benchmark for rendering a chart.
pub fn chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart");

    for data_count in [64, 256, 2048] {
        // Render a basic chart
        group.bench_with_input(BenchmarkId::new("render", data_count), &data_count, render);

        // Render a live chart
        group.bench_with_input(
            BenchmarkId::new("render_live", data_count),
            &data_count,
            render_live,
        );
    }

    group.finish();
}

/// Render the widget in a classical size buffer
fn render(bencher: &mut Bencher, data_count: &usize) {
    let data = (0..*data_count)
        .map(|i| (i as f64, i as f64))
        .collect::<Vec<_>>();
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));

    bencher.iter(|| {
        Chart::new(vec![Dataset::default().data(&data)]).render(buffer.area, &mut buffer);
    });
}

fn render_live(bencher: &mut Bencher, data_count: &usize) {
    let mut data = (0..*data_count)
        .map(|i| (i as f64, i as f64))
        .collect::<Vec<_>>();
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));

    bencher.iter(|| {
        data.drain(0..5);
        for i in 0..5 {
            data.push((i as f64, i as f64));
        }
        Chart::new(vec![Dataset::default().data(&data)]).render(buffer.area, &mut buffer);
    });
}

criterion_group!(benches, chart);
criterion_main!(benches);