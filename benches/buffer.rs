use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ratatui::{buffer::Buffer, layout::Rect, style::Style};

pub fn set_string(c: &mut Criterion) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 100));
    c.bench_function("Buffer::set_string", |b| {
        b.iter(|| {
            buffer.set_string(0, 0, black_box("Hello, world!"), Style::default());
        })
    });
}

/// Note that diffing is not generally a bottleneck, but this bench provides a reference number for
/// a critical part of rendering.
pub fn diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("Buffer::diff");
    for &count in [1, 10, 255].iter() {
        let buffer_a = Buffer::with_lines(vec![str::repeat("a", count); count]);
        let buffer_b = Buffer::with_lines(vec![str::repeat("b", count); count]);

        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &black_box((buffer_a, buffer_b)),
            |b, bufs| b.iter(|| bufs.0.diff(&bufs.1)),
        );
    }
    group.finish();
}

criterion_group!(benches, set_string, diff);
criterion_main!(benches);
