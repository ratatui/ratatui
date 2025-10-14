use criterion::{BatchSize, Bencher, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;

/// Benchmark for rendering a text.
fn text(c: &mut Criterion) {
    let mut group = c.benchmark_group("text");
    for (width, height) in [
        (1, 256),   // Heavily vertically skewed
        (256, 1),   // Heavily horizontally skewed
        (50, 50),   // Typical rendering area
        (100, 50),  // Vertically split screen
        (200, 50),  // 1080p fullscreen with medium font
        (256, 256), // Max sized area
    ] {
        let buffer_size = Rect::new(0, 0, width, height);
        group.bench_with_input(
            format!("render/{width}x{height}"),
            &Text::from(vec![
                Line::from("The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs."),
                Line::from("🦀 Rustaceans unite! 東京・İstanbul・Sydney・San Francisco・Warsaw 🌏 RustConf連携中！").bold(),
                Line::from("naïve cafés ☕ serve résumé-ready developers 👩‍💻🧑🏾‍💻 testing text rendering engines.").green(),
                Line::from("ゼロ幅スペース\u{200B}、結合絵文字👨‍👩‍👧‍👦、全角文字ＡＢＣ、半角abcが混在。").blue(),
                Line::from("Emoji test: 🙂😇🤖👩🏻‍🎨🧑‍🚀 — wrapped in a 50x50 buffer for layout & clipping check.").italic(),
            ]),
            |b, text| render(b, text, buffer_size),
        );
    }
    group.finish();
}

/// Renders the text into a buffer of the given `size`
fn render(bencher: &mut Bencher, text: &Text, size: Rect) {
    let mut buffer = Buffer::empty(size);
    // We use `iter_batched` to clone the value in the setup function.
    // See https://github.com/ratatui/ratatui/pull/377.
    bencher.iter_batched(
        || text.to_owned(),
        |bench_text| {
            bench_text.render(buffer.area, &mut buffer);
        },
        BatchSize::SmallInput,
    );
}

criterion_group!(benches, text);
