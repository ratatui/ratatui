use std::hint::black_box;

use criterion::{Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Stylize;
use ratatui::text::InlineText;
use ratatui::widgets::Widget;

fn line_render(criterion: &mut Criterion) {
    for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
        let mut group = criterion.benchmark_group(format!("inline_text_render/{alignment}"));
        group.sample_size(1000);

        let inline = &InlineText::from(vec![
            "Hello, world!".red(),
            "The quick brown fox jumps over the lazy dog.".green(),
            "Pack my box with five dozen liquor jugs.".italic(),
            "Sphinx of black quartz, judge my vow.".blue(),
            "Lorem ipsum dolor sit amet...".bold(),
            "naïve café".into(),
            "東京 Rust 開発者会".into(),
            "🙂😇🤖".into(),
            "A\u{200B}".into(),
            "👩‍💻".into(),
        ])
        .space(1)
        .alignment(alignment);

        for width in [0, 7, 10, 42, 80, 120] {
            let area = Rect::new(0, 0, width, 10);
            group.bench_function(width.to_string(), |bencher| {
                let mut buffer = Buffer::empty(area);
                bencher.iter(|| black_box(inline).render(area, &mut buffer));
            });
        }
        group.finish();
    }
}

criterion_group!(benches, line_render);
