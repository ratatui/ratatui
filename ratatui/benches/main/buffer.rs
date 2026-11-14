use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::{Rect, Size};
use ratatui::style::Style;
use ratatui::text::Line;

criterion::criterion_group!(
    benches,
    empty,
    filled,
    with_lines,
    diff_no_change,
    diff_full_change,
    diff_partial_change,
    diff_multi_width,
    diff_emoji
);

const fn rect(size: Size) -> Rect {
    Rect {
        x: 0,
        y: 0,
        width: size.width,
        height: size.height,
    }
}

const SIZES: [Size; 2] = [Size::new(200, 50), Size::new(1000, 250)];

fn empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/empty");
    for size in SIZES {
        let area = rect(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &area, |b, &area| {
            b.iter(|| {
                let _buffer = Buffer::empty(black_box(area));
            });
        });
    }
    group.finish();
}

/// This likely should have the same performance as `empty`, but it's here for completeness
/// and to catch any potential performance regressions.
fn filled(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/filled");
    for size in SIZES {
        let area = rect(size);
        let cell = Cell::new("AAAA"); // simulate a multi-byte character
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(area, cell),
            |b, (area, cell)| {
                b.iter(|| {
                    let _buffer = Buffer::filled(black_box(*area), cell.clone());
                });
            },
        );
    }
    group.finish();
}

fn with_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/with_lines");
    for size in SIZES {
        let word_count = 50;
        let lines = fakeit::words::sentence(word_count);
        let lines = lines.lines().map(Line::from);
        group.bench_with_input(BenchmarkId::from_parameter(size), &lines, |b, lines| {
            b.iter(|| {
                let _buffer = Buffer::with_lines(black_box(lines.clone()));
            });
        });
    }
    group.finish();
}

fn diff_no_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_no_change");
    for size in SIZES {
        let area = rect(size);
        let buffer = Buffer::filled(area, Cell::new("x"));
        group.bench_with_input(BenchmarkId::from_parameter(size), &buffer, |b, buffer| {
            b.iter(|| {
                let diff = black_box(buffer).diff(black_box(buffer));
                black_box(diff);
            });
        });
    }
    group.finish();
}

/// This tests maximum update cost with every cell needing an update
fn diff_full_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_full_change");
    for size in SIZES {
        let area = rect(size);
        let prev = Buffer::filled(area, Cell::new("a"));
        let next = Buffer::filled(area, Cell::new("b"));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| {
                    let diff = black_box(prev).diff(black_box(next));
                    black_box(diff);
                });
            },
        );
    }
    group.finish();
}

/// This simulates typical incremental updates in a TUI
fn diff_partial_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_partial_change");
    for size in SIZES {
        let area = rect(size);
        let prev = Buffer::filled(area, Cell::new("a"));
        let mut next = Buffer::filled(area, Cell::new("a"));

        let total_cells = (size.width as usize) * (size.height as usize);
        for i in (0..total_cells).step_by(10) {
            if i < next.content.len() {
                next.content[i].set_symbol("b");
            }
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| {
                    let diff = black_box(prev).diff(black_box(next));
                    black_box(diff);
                });
            },
        );
    }
    group.finish();
}

fn diff_multi_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_multi_width");
    for size in SIZES {
        let area = rect(size);
        let prev = Buffer::with_lines(vec!["a".repeat(size.width as usize); size.height as usize]);
        let mut next = Buffer::filled(area, Cell::new(" "));

        for y in 0..size.height {
            next.set_string(
                0,
                y,
                "日本語中文한국어".repeat(size.width as usize / 6),
                Style::default(),
            );
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| {
                    let diff = black_box(prev).diff(black_box(next));
                    black_box(diff);
                });
            },
        );
    }
    group.finish();
}

/// Tests the emoji-specific VS16 clearing path for complex emoji sequences
fn diff_emoji(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_emoji");
    for size in SIZES {
        let area = rect(size);
        let prev = Buffer::filled(area, Cell::new("a"));
        let mut next = Buffer::filled(area, Cell::new(" "));

        for y in 0..size.height {
            next.set_string(
                0,
                y,
                "⌨️👁️‍🗨️🐻‍❄️".repeat(size.width as usize / 6),
                Style::default(),
            );
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(prev, next),
            |b, (prev, next)| {
                b.iter(|| {
                    let diff = black_box(prev).diff(black_box(next));
                    black_box(diff);
                });
            },
        );
    }
    group.finish();
}
