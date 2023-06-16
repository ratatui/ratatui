use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, Bencher, BenchmarkGroup,
    BenchmarkId, Criterion,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Wrap},
};

/// because the scroll offset is a u16, the maximum number of lines that can be scrolled is 65535.
/// This is a limitation of the current implementation and may be fixed by changing the type of the
/// scroll offset to a u32.
const MAX_SCROLL_OFFSET: u16 = u16::MAX;

/// Benchmark for rendering a paragraph with a given number of lines. The design of this benchmark
/// allows comparison of the performance of rendering a paragraph with different numbers of lines.
/// as well as comparing with the various settings on the scroll and wrap features.
pub fn paragraph(c: &mut Criterion) {
    let mut group = c.benchmark_group("paragraph");
    for &line_count in [10, 100, 1000, 10000, MAX_SCROLL_OFFSET].iter() {
        let lines = random_lines(line_count);
        let lines = lines.as_str();

        bench_paragraph_new(&mut group, line_count, lines);
        bench_paragraph_render(&mut group, line_count, lines);
        bench_paragraph_render_scroll(&mut group, line_count, lines);
        bench_paragraph_render_wrap(&mut group, line_count, lines);
    }
    group.finish();
}

/// baseline benchmark that helps measure the overhead of creating a paragraph
fn bench_paragraph_new(group: &mut BenchmarkGroup<'_, WallTime>, line_count: u16, lines: &str) {
    group.bench_with_input(BenchmarkId::new("new", line_count), lines, |b, lines| {
        b.iter(|| Paragraph::new(black_box(lines)))
    });
}

/// benchmark for rendering a paragraph without any scrolling or wrapping
fn bench_paragraph_render(group: &mut BenchmarkGroup<'_, WallTime>, line_count: u16, lines: &str) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));

    group.bench_with_input(BenchmarkId::new("render", line_count), lines, |b, lines| {
        b.iter(|| {
            let paragraph = Paragraph::new(lines);
            paragraph.render(buffer.area, &mut buffer);
        })
    });
}

fn bench_paragraph_render_scroll(
    group: &mut BenchmarkGroup<'_, WallTime>,
    line_count: u16,
    lines: &str,
) {
    group.bench_with_input(
        BenchmarkId::new("scroll_0", line_count),
        lines,
        bench_paragraph_render_scroll_offset(0),
    );
    group.bench_with_input(
        BenchmarkId::new("scroll_1", line_count),
        lines,
        bench_paragraph_render_scroll_offset(0),
    );
    group.bench_with_input(
        BenchmarkId::new("scroll_half", line_count),
        lines,
        bench_paragraph_render_scroll_offset(line_count / 2),
    );
    group.bench_with_input(
        BenchmarkId::new("scroll_full", line_count),
        lines,
        bench_paragraph_render_scroll_offset(line_count),
    );
}

fn bench_paragraph_render_scroll_offset(offset: u16) -> impl Fn(&mut Bencher<'_>, &str) {
    move |b, lines| {
        let offset = (0u16, offset);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 200, 50));
        b.iter(|| {
            let paragraph = Paragraph::new(lines).scroll(offset);
            paragraph.render(buffer.area, &mut buffer);
        })
    }
}

fn bench_paragraph_render_wrap(
    group: &mut BenchmarkGroup<'_, WallTime>,
    line_count: u16,
    lines: &str,
) {
    group.bench_with_input(
        BenchmarkId::new("wrap_no_trim_100", line_count),
        lines,
        bench_paragraph_render_wrap_no_trim(100),
    );
    group.bench_with_input(
        BenchmarkId::new("wrap_no_trim_200", line_count),
        lines,
        bench_paragraph_render_wrap_no_trim(200),
    );
    group.bench_with_input(
        BenchmarkId::new("wrap_no_trim_300", line_count),
        lines,
        bench_paragraph_render_wrap_no_trim(300),
    );
    group.bench_with_input(
        BenchmarkId::new("wrap_trim_100", line_count),
        lines,
        bench_paragraph_render_wrap_trim(100),
    );
    group.bench_with_input(
        BenchmarkId::new("wrap_trim_200", line_count),
        lines,
        bench_paragraph_render_wrap_trim(200),
    );
    group.bench_with_input(
        BenchmarkId::new("wrap_trim_300", line_count),
        lines,
        bench_paragraph_render_wrap_trim(300),
    );
}

/// benchmark for rendering a paragraph with a given wrap width
fn bench_paragraph_render_wrap_no_trim(width: u16) -> impl Fn(&mut Bencher<'_>, &str) {
    move |b, lines| {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, 50));
        let wrap = Wrap { trim: false };
        b.iter(|| {
            Paragraph::new(black_box(lines))
                .wrap(black_box(wrap))
                .render(buffer.area, &mut buffer)
        })
    }
}

/// benchmark for rendering a paragraph with a given wrap width
fn bench_paragraph_render_wrap_trim(width: u16) -> impl Fn(&mut Bencher<'_>, &str) {
    move |b, lines| {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, 50));
        let wrap = Wrap { trim: true };
        b.iter(|| {
            Paragraph::new(black_box(lines))
                .wrap(black_box(wrap))
                .render(buffer.area, &mut buffer)
        })
    }
}

/// Create a string with the given number of lines filled with nonsense words
///
/// English language has about 5.1 average characters per word so including the space between words
/// this should emit around 200 characters per paragraph on average.
fn random_lines(count: u16) -> String {
    let count = count as i64;
    let sentence_count = 3;
    let word_count = 11;
    fakeit::words::paragraph(count, sentence_count, word_count, "\n".into())
}

criterion_group!(benches, paragraph);
criterion_main!(benches);
