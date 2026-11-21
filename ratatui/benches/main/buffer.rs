use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;

criterion::criterion_group!(benches, empty, filled, with_lines, set_string);

const fn rect(size: u16) -> Rect {
    Rect::new(0, 0, size, size)
}

fn empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/empty");
    for size in [16, 64, 255] {
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
    for size in [16, 64, 255] {
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
    for size in [16, 64, 255] {
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

/// Fills every row of a 200x50 buffer with one string, as a screen of text would.
fn set_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/set_string");
    let ascii = |len| {
        "the quick brown fox jumps over the lazy dog "
            .chars()
            .cycle()
            .take(len)
    };
    let cases = [
        ("ascii", ascii(200).collect()),
        ("ascii_short", "ok".to_string()),
        ("ascii_then_emoji", ascii(198).chain(['😀']).collect()),
        ("cjk", "你好世界".repeat(25)),
    ];
    let area = Rect::new(0, 0, 200, 50);
    for (name, string) in &cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), string, |b, string| {
            let mut buffer = Buffer::empty(area);
            b.iter(|| {
                for y in 0..area.height {
                    buffer.set_string(0, y, black_box(string), Style::new());
                }
            });
        });
    }
    group.finish();
}
