use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use ratatui::layout::Rect;

fn rect_rows_benchmark(c: &mut Criterion) {
    let rect_sizes = vec![
        Rect::new(0, 0, 1, 16),
        Rect::new(0, 0, 1, 1024),
        Rect::new(0, 0, 1, 65535),
    ];
    let mut group = c.benchmark_group("rect_rows");
    for rect in rect_sizes {
        group.bench_with_input(BenchmarkId::new("rows", rect.height), &rect, |b, rect| {
            b.iter(|| {
                for row in rect.rows() {
                    // Perform any necessary operations on each row
                    black_box(row);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, rect_rows_benchmark);
