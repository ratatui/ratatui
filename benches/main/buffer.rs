use std::iter::zip;

use criterion::{black_box, BenchmarkId, Criterion};
use ratatui::{
    buffer::{Buffer, Cell},
    layout::Rect,
    text::Line,
    widgets::Widget,
};

criterion::criterion_group!(benches, empty, filled, with_lines, diff);

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

fn diff(c: &mut Criterion) {
    const AREA: Rect = Rect {
        x: 0,
        y: 0,
        width: 200,
        height: 50,
    };
    c.bench_function("buffer/diff", |b| {
        let buffer_1 = create_random_buffer(AREA);
        let buffer_2 = create_random_buffer(AREA);
        b.iter(|| {
            let _ = black_box(&buffer_1).diff(black_box(&buffer_2));
        });
    });
}

fn create_random_buffer(area: Rect) -> Buffer {
    const PARAGRAPH_COUNT: i64 = 15;
    const SENTENCE_COUNT: i64 = 5;
    const WORD_COUNT: i64 = 20;
    const SEPARATOR: &str = "\n\n";
    let paragraphs = fakeit::words::paragraph(
        PARAGRAPH_COUNT,
        SENTENCE_COUNT,
        WORD_COUNT,
        SEPARATOR.to_string(),
    );
    let mut buffer = Buffer::empty(area);
    for (line, row) in zip(paragraphs.lines(), area.rows()) {
        Line::from(line).render(row, &mut buffer);
    }
    buffer
}
