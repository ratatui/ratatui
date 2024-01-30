use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Chart, Dataset, Widget},
};
use ringbuf::{LocalRb, Rb};

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

        // Render a live chart with a ring buffer
        group.bench_with_input(
            BenchmarkId::new("render_live_ringbuf", data_count),
            &data_count,
            render_live_ringbuf,
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
        for i in 0..1000 {
            // Very bad because it has the time complexity O(n) where n is the length of the data vector.
            // Therefore, the usage of a ring buffer is recommended.
            data.remove(0);
            data.push((i as f64, i as f64));
        }
        Chart::new(vec![Dataset::default().data(&data)]).render(buffer.area, &mut buffer);
    });
}

fn render_live_ringbuf(bencher: &mut Bencher, data_count: &usize) {
    let mut data = LocalRb::new(*data_count);
    data.push_iter(&mut (0..*data_count).map(|i| (i as f64, i as f64)));
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));

    bencher.iter(|| {
        for i in 0..1000 {
            data.push_overwrite((i as f64, i as f64));
        }
        Chart::new(vec![Dataset::default().data(data.iter())]).render(buffer.area, &mut buffer);
    });
}

criterion_group!(benches, chart);
criterion_main!(benches);
