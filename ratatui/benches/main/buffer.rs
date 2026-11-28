use std::hint::black_box;

use hotpath::criterion::{BenchmarkId, Criterion};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::{Rect, Size};
use ratatui::style::Style;

hotpath::criterion::criterion_group!(
    benches,
    empty,
    filled,
    with_lines,
    set_string,
    diff_no_change,
    diff_full_change,
    diff_partial_change,
    diff_multi_width,
    diff_emoji
);

/// A reasonable terminal size, and a size that's probably too big to be useful
const SIZES: [Size; 2] = [Size::new(200, 50), Size::new(1000, 250)];

/// Allocating a buffer of default cells should grow linearly with the cell count.
fn empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/empty");
    for size in SIZES {
        let area = size.into();
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| b.iter(|| Buffer::empty(black_box(area))));
    }
    group.finish();
}

/// Filling with a non-default cell should cost about the same as `empty`; a gap would mean
/// cloning the cell got more expensive.
fn filled(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/filled");
    let cell = Cell::new("AAAA"); // a symbol longer than one character
    for size in SIZES {
        let area = size.into();
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| Buffer::filled(black_box(area), cell.clone()));
        });
    }
    group.finish();
}

/// Building a buffer from text should grow with the amount of text.
fn with_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/with_lines");
    for size in SIZES {
        let lines = sentences_filling(size);
        let rows = lines.iter().map(String::as_str);
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| Buffer::with_lines(black_box(rows.clone())));
        });
    }
    group.finish();
}

/// One sentence per row, each with roughly enough words to fill the width
fn sentences_filling(size: Size) -> Vec<String> {
    let words_per_row = i64::from(size.width / 6); // about 5 letters and a space per word
    (0..size.height)
        .map(|_| fakeit::words::sentence(words_per_row))
        .collect()
}

/// Diffing identical buffers is the floor: the cost of comparing every cell and emitting nothing.
fn diff_no_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_no_change");
    for size in SIZES {
        let buffer = Buffer::filled(size.into(), Cell::new("a"));
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| black_box(&buffer).diff(black_box(&buffer)));
        });
    }
    group.finish();
}

/// Changing every cell is the ceiling: every cell is compared and emitted as an update.
fn diff_full_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_full_change");
    for size in SIZES {
        let previous = Buffer::filled(size.into(), Cell::new("a"));
        let next = Buffer::filled(size.into(), Cell::new("b"));
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| black_box(&previous).diff(black_box(&next)));
        });
    }
    group.finish();
}

/// Changing every 10th cell, like a typical incremental redraw, should land between the floor
/// and the ceiling, in proportion to the number of updates.
fn diff_partial_change(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_partial_change");
    for size in SIZES {
        let previous = Buffer::filled(size.into(), Cell::new("a"));
        let next = with_every_tenth_cell_changed(&previous);
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| black_box(&previous).diff(black_box(&next)));
        });
    }
    group.finish();
}

fn with_every_tenth_cell_changed(buffer: &Buffer) -> Buffer {
    let mut changed = buffer.clone();
    for cell in changed.content.iter_mut().step_by(10) {
        cell.set_symbol("b");
    }
    changed
}

/// Replacing narrow text with wide (CJK) characters exercises the path that skips the cells a
/// wide character covers, so it should cost more per update than a full change.
fn diff_multi_width(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_multi_width");
    for size in SIZES {
        let previous = Buffer::filled(size.into(), Cell::new("a"));
        let next = wide_character_rows(size);
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| black_box(&previous).diff(black_box(&next)));
        });
    }
    group.finish();
}

/// Every row filled with `日本語中文한국어` repeated to the buffer width
fn wide_character_rows(size: Size) -> Buffer {
    let mut buffer = Buffer::filled(size.into(), Cell::new(" "));
    let row = "日本語中文한국어".repeat(size.width as usize);
    for y in 0..size.height {
        buffer.set_string(0, y, &row, Style::default());
    }
    buffer
}

/// Replacing narrow text with emoji sequences (VS16 and ZWJ) exercises the extra clearing the
/// diff does for emoji, so it should cost more per update than wide characters.
fn diff_emoji(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff_emoji");
    for size in SIZES {
        let previous = Buffer::filled(size.into(), Cell::new("a"));
        let next = emoji_rows(size);
        let id = BenchmarkId::from_parameter(size);
        group.bench_function(id, |b| {
            b.iter(|| black_box(&previous).diff(black_box(&next)));
        });
    }
    group.finish();
}

/// Every row filled with `⌨️👁️‍🗨️🐻‍❄️` repeated to the buffer width
fn emoji_rows(size: Size) -> Buffer {
    let mut buffer = Buffer::filled(size.into(), Cell::new(" "));
    let row = "⌨️👁️‍🗨️🐻‍❄️".repeat(size.width as usize);
    for y in 0..size.height {
        buffer.set_string(0, y, &row, Style::default());
    }
    buffer
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
