use criterion::{criterion_group, criterion_main, BatchSize, Bencher, BenchmarkId, Criterion};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Alignment,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Widget,
    },
};

/// Benchmark for rendering a block.
fn block(c: &mut Criterion) {
    let mut group = c.benchmark_group("block");

    for buffer_size in [
        Rect::new(0, 0, 100, 50),  // vertically split screen
        Rect::new(0, 0, 200, 50),  // 1080p fullscreen with medium font
        Rect::new(0, 0, 256, 256), // Max sized area
    ] {
        let buffer_area = buffer_size.area();

        // Render an empty block
        group.bench_with_input(
            BenchmarkId::new("render_empty", buffer_area),
            &Block::new(),
            |b, block| render(b, block, buffer_size),
        );

        // Render with all features
        group.bench_with_input(
            BenchmarkId::new("render_all_feature", buffer_area),
            &Block::new()
                .borders(Borders::ALL)
                .title("test title")
                .title(
                    Title::from("bottom left title")
                        .alignment(Alignment::Right)
                        .position(Position::Bottom),
                )
                .padding(Padding::new(5, 5, 2, 2)),
            |b, block| render(b, block, buffer_size),
        );
    }

    group.finish();
}

/// render the block into a buffer of the given `size`
fn render(bencher: &mut Bencher, block: &Block, size: Rect) {
    let mut buffer = Buffer::empty(size);
    // We use `iter_batched` to clone the value in the setup function.
    // See https://github.com/ratatui-org/ratatui/pull/377.
    bencher.iter_batched(
        || block.to_owned(),
        |bench_block| {
            bench_block.render(buffer.area, &mut buffer);
        },
        BatchSize::SmallInput,
    );
}

criterion_group!(benches, block);
criterion_main!(benches);
