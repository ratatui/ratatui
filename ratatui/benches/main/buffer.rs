use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;

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

/// The kinds of content used to fill buffers in the diff benchmarks.
#[derive(Clone, Copy, Debug)]
enum Content {
    Ascii,
    Cjk,
    Emoji,
}

impl Content {
    const fn symbols(self) -> &'static [&'static str] {
        match self {
            Self::Ascii => &["a", "b", "c", "d"],
            Self::Cjk => &["あ", "い", "う", "え"],
            Self::Emoji => &["😀", "👍🏽", "🇺🇸", "👨\u{200d}👩\u{200d}👧"],
        }
    }
}

/// Fills a buffer with `content`, offsetting the symbols by `shift` so that two buffers built with
/// different shifts differ in every non-blank cell.
fn content_buffer(area: Rect, content: Content, shift: usize) -> Buffer {
    let mut buffer = Buffer::empty(area);
    let symbols = content.symbols();
    for y in area.top()..area.bottom() {
        let mut x = area.left();
        let mut i = shift;
        while x < area.right() {
            let symbol = symbols[i % symbols.len()];
            let (next_x, _) =
                buffer.set_stringn(x, y, symbol, usize::from(area.right() - x), Style::new());
            if next_x == x {
                break;
            }
            x = next_x;
            i += 1;
        }
    }
    buffer
}

/// Returns a copy of `buffer` where every `stride`-th row has been replaced by the same row of
/// `changed`.
fn change_rows(buffer: &Buffer, changed: &Buffer, stride: u16) -> Buffer {
    let mut result = buffer.clone();
    let area = buffer.area;
    for y in (area.top()..area.bottom()).step_by(usize::from(stride)) {
        for x in area.left()..area.right() {
            result[(x, y)] = changed[(x, y)].clone();
        }
    }
    result
}

/// Benchmarks `Buffer::diff_iter`, which runs on every frame.
///
/// Each case diffs a 200x50 buffer against an identical buffer, one with 10% of its rows changed
/// and one where every cell changed, for ASCII, CJK (double-width) and emoji content.
fn diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer/diff");
    let area = Rect::new(0, 0, 200, 50);
    for content in [Content::Ascii, Content::Cjk, Content::Emoji] {
        let prev = content_buffer(area, content, 0);
        let all_changed = content_buffer(area, content, 1);
        let cases = [
            ("identical", prev.clone()),
            ("10pct", change_rows(&prev, &all_changed, 10)),
            ("100pct", all_changed),
        ];
        for (name, next) in cases {
            let id = BenchmarkId::new(format!("{content:?}"), name);
            group.bench_with_input(id, &(&prev, &next), |b, (prev, next)| {
                b.iter(|| black_box(prev).diff_iter(black_box(next)).count());
            });
        }
    }
    group.finish();
}
