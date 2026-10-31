use std::hint::black_box;

use criterion::{Bencher, BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Widget, Wrap};

const WIDTH: u16 = 200;
const HEIGHT: u16 = 50;
const AREA: Rect = Rect::new(0, 0, WIDTH, HEIGHT);
const WRAP: Wrap = Wrap { trim: false };

/// Every span is a fixed width word plus a trailing space (e.g. `"w00042-07 "`), so the number of
/// rows each line wraps to is known up front.
const SPAN_WIDTH: usize = 10;

/// Benchmark for rendering owned text that is stored in application state.
///
/// Applications often keep a long, multi-span `Text<'static>` (log viewers, text built by a
/// scripting layer, widget property maps) and render it every frame. Because `Paragraph` takes
/// ownership of its text, the stored text has to be cloned on each frame. The clone is part of the
/// per-frame cost, so unlike the `paragraph` and `text` benchmarks, it is measured inside the timed
/// closure here.
fn stored_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("stored_text");
    for (line_count, span_count) in [(100, 5), (100, 50), (10_000, 5), (10_000, 50)] {
        let input = StoredText::new(line_count, span_count);
        let id = format!("{line_count}x{span_count}");

        // Criterion runs at least one iteration per sample, so slow cases take
        // `sample_size * iteration time` regardless of the target measurement time.
        group.sample_size(match line_count * span_count {
            0..1_000 => 100,
            1_000..10_000 => 50,
            _ => 20,
        });

        let clone_id = BenchmarkId::new("clone", &id);
        group.bench_with_input(clone_id, &input, clone);
        let borrow_id = BenchmarkId::new("borrow", &id);
        group.bench_with_input(borrow_id, &input, borrow);
        let clone_render_id = BenchmarkId::new("clone_render", &id);
        group.bench_with_input(clone_render_id, &input, clone_render);
        let borrow_render_id = BenchmarkId::new("borrow_render", &id);
        group.bench_with_input(borrow_render_id, &input, borrow_render);
        let clone_render_end_id = BenchmarkId::new("clone_render_end", &id);
        group.bench_with_input(clone_render_end_id, &input, clone_render_end);
        let borrow_render_end_id = BenchmarkId::new("borrow_render_end", &id);
        group.bench_with_input(borrow_render_end_id, &input, borrow_render_end);
        let slice_render_id = BenchmarkId::new("slice_render", &id);
        group.bench_with_input(slice_render_id, &input, slice_render);
    }
    group.finish();
}

/// The cost of cloning the stored text by itself.
fn clone(bencher: &mut Bencher, input: &StoredText) {
    bencher.iter(|| black_box(&input.text).clone());
}

/// The cost of borrowing the stored text as a new `Text`, without copying its strings.
fn borrow(bencher: &mut Bencher, input: &StoredText) {
    bencher.iter(|| Text::from(black_box(&input.text)));
}

/// Clone the stored text into a paragraph and render it.
fn clone_render(bencher: &mut Bencher, input: &StoredText) {
    let mut buffer = Buffer::empty(AREA);
    bencher.iter(|| {
        Paragraph::new(input.text.clone())
            .wrap(WRAP)
            .render(AREA, &mut buffer);
    });
}

/// Borrow the stored text into a paragraph and render it.
fn borrow_render(bencher: &mut Bencher, input: &StoredText) {
    let mut buffer = Buffer::empty(AREA);
    bencher.iter(|| {
        Paragraph::new(&input.text)
            .wrap(WRAP)
            .render(AREA, &mut buffer);
    });
}

/// Clone the stored text into a paragraph and render it scrolled to the end, like a log viewer
/// following its output.
fn clone_render_end(bencher: &mut Bencher, input: &StoredText) {
    let mut buffer = Buffer::empty(AREA);
    bencher.iter(|| {
        Paragraph::new(input.text.clone())
            .wrap(WRAP)
            .scroll((input.scroll_end, 0))
            .render(AREA, &mut buffer);
    });
}

/// Borrow the stored text into a paragraph and render it scrolled to the end.
fn borrow_render_end(bencher: &mut Bencher, input: &StoredText) {
    let mut buffer = Buffer::empty(AREA);
    bencher.iter(|| {
        Paragraph::new(&input.text)
            .wrap(WRAP)
            .scroll((input.scroll_end, 0))
            .render(AREA, &mut buffer);
    });
}

/// Reference: clone and render only the lines that are visible when scrolled to the end.
fn slice_render(bencher: &mut Bencher, input: &StoredText) {
    let mut buffer = Buffer::empty(AREA);
    bencher.iter(|| {
        Paragraph::new(Text::from(input.text.lines[input.visible_start..].to_vec()))
            .wrap(WRAP)
            .scroll((input.visible_scroll, 0))
            .render(AREA, &mut buffer);
    });
}

/// Owned text as an application would store it, along with the scroll offsets that show its last
/// screen.
struct StoredText {
    text: Text<'static>,
    /// The scroll offset that shows the last screen of `text`.
    scroll_end: u16,
    /// The first line that can be visible when scrolled to the end. Every line takes up at least
    /// one row, so only the last `HEIGHT` lines can be visible.
    visible_start: usize,
    /// The scroll offset that shows the last screen of the lines from `visible_start`.
    visible_scroll: u16,
}

impl StoredText {
    fn new(line_count: usize, span_count: usize) -> Self {
        let text = owned_text(line_count, span_count);
        let rows_per_line = (span_count * SPAN_WIDTH).div_ceil(usize::from(WIDTH));
        let visible_start = line_count.saturating_sub(usize::from(HEIGHT));
        let input = Self {
            scroll_end: end_offset(line_count * rows_per_line),
            visible_start,
            visible_scroll: end_offset((line_count - visible_start) * rows_per_line),
            text,
        };
        assert_same_output(
            &Paragraph::new(input.text.clone())
                .wrap(WRAP)
                .scroll((input.scroll_end, 0)),
            &Paragraph::new(Text::from(input.text.lines[visible_start..].to_vec()))
                .wrap(WRAP)
                .scroll((input.visible_scroll, 0)),
            &format!("w{:05}-{:02}", line_count - 1, span_count - 1),
        );
        input
    }
}

/// The vertical scroll offset that shows the last screen of a text that wraps to `rows` rows.
fn end_offset(rows: usize) -> u16 {
    let rows = u16::try_from(rows).expect("wrapped text should fit in a u16 scroll offset");
    rows.saturating_sub(HEIGHT)
}

/// Check that both paragraphs render the same screen, ending with `last_word` on the bottom row.
fn assert_same_output(expected: &Paragraph, actual: &Paragraph, last_word: &str) {
    let mut expected_buffer = Buffer::empty(AREA);
    let mut actual_buffer = Buffer::empty(AREA);
    expected.render(AREA, &mut expected_buffer);
    actual.render(AREA, &mut actual_buffer);
    assert_eq!(expected_buffer, actual_buffer);

    let bottom_row: String = (0..WIDTH)
        .map(|x| expected_buffer[(x, HEIGHT - 1)].symbol())
        .collect();
    assert!(
        bottom_row.contains(last_word),
        "expected {last_word:?} on the bottom row, got {bottom_row:?}"
    );
}

/// Create an owned text where every span is a separately allocated string with its own style.
fn owned_text(line_count: usize, span_count: usize) -> Text<'static> {
    (0..line_count)
        .map(|line| {
            (0..span_count)
                .map(|span| {
                    let content = format!("w{line:05}-{span:02} ");
                    debug_assert_eq!(content.len(), SPAN_WIDTH);
                    Span::styled(content, span_style(line + span))
                })
                .collect::<Line>()
        })
        .collect()
}

const fn span_style(index: usize) -> Style {
    let style = Style::new().fg(Color::Indexed((index % 256) as u8));
    match index % 3 {
        0 => style.add_modifier(Modifier::BOLD),
        1 => style.bg(Color::Indexed((index * 7 % 256) as u8)),
        _ => style,
    }
}

criterion_group!(benches, stored_text);
