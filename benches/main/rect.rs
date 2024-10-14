use criterion::{black_box, criterion_group, BatchSize, Bencher, BenchmarkId, Criterion};
use ratatui::layout::Rect;

fn rect_iters_benchmark(c: &mut Criterion) {
    let rect_sizes = vec![
        Rect::new(0, 0, 16, 16),
        Rect::new(0, 0, 128, 128),
        Rect::new(0, 0, 256, 256),
    ];
    let mut group = c.benchmark_group("rect");
    for rect in rect_sizes {
        group.bench_with_input(
            BenchmarkId::new("rect_rows_iter", rect.height),
            &rect,
            |b, rect| rect_rows_iter(b, *rect),
        );
        group.bench_with_input(
            BenchmarkId::new("rect_rows_collect", rect.height),
            &rect,
            |b, rect| rect_rows_collect(b, *rect),
        );
        group.bench_with_input(
            BenchmarkId::new("rect_columns_iter", rect.width),
            &rect,
            |b, rect| rect_columns_iter(b, *rect),
        );
        group.bench_with_input(
            BenchmarkId::new("rect_columns_collect", rect.width),
            &rect,
            |b, rect| rect_columns_collect(b, *rect),
        );
        group.bench_with_input(
            BenchmarkId::new(
                "rect_positions_iter",
                format!("{}x{}", rect.width, rect.height),
            ),
            &rect,
            |b, rect| rect_positions_iter(b, *rect),
        );
        group.bench_with_input(
            BenchmarkId::new(
                "rect_positions_collect",
                format!("{}x{}", rect.width, rect.height),
            ),
            &rect,
            |b, rect| rect_positions_collect(b, *rect),
        );
    }
    group.finish();
}

fn rect_rows_iter(c: &mut Bencher, rect: Rect) {
    c.iter_batched(
        || black_box(rect),
        |rect| {
            for row in rect.rows() {
                black_box(row);
            }
        },
        BatchSize::LargeInput,
    );
}

fn rect_rows_collect(c: &mut Bencher, rect: Rect) {
    c.iter_batched(
        || black_box(rect),
        |rect| rect.rows().collect::<Vec<_>>(),
        BatchSize::LargeInput,
    );
}

fn rect_columns_iter(c: &mut Bencher, rect: Rect) {
    c.iter_batched(
        || black_box(rect),
        |rect| {
            for col in rect.columns() {
                black_box(col);
            }
        },
        BatchSize::LargeInput,
    );
}

fn rect_columns_collect(c: &mut Bencher, rect: Rect) {
    c.iter_batched(
        || black_box(rect),
        |rect| rect.columns().collect::<Vec<_>>(),
        BatchSize::LargeInput,
    );
}

fn rect_positions_iter(c: &mut Bencher, rect: Rect) {
    c.iter_batched(
        || black_box(rect),
        |rect| {
            for pos in rect.positions() {
                black_box(pos);
            }
        },
        BatchSize::LargeInput,
    );
}

fn rect_positions_collect(b: &mut Bencher, rect: Rect) {
    b.iter_batched(
        || black_box(rect),
        |rect| rect.positions().collect::<Vec<_>>(),
        BatchSize::LargeInput,
    );
}

criterion_group!(benches, rect_iters_benchmark);
