use std::hint::black_box;
use std::io;

use criterion::{BenchmarkId, Criterion};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::buffer::Cell;

criterion::criterion_group!(benches, draw_unicode_width_cases);

fn draw_unicode_width_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("crossterm/unicode_width");

    for (name, symbol, cell_width) in [
        ("ascii", "a", 1),
        ("latin_precomposed", "é", 1),
        ("latin_combining", "e\u{301}", 1),
        ("east_asian_ambiguous", "¡", 1),
        ("east_asian_wide", "界", 2),
        ("emoji", "😀", 2),
        ("indic", "हा", 2),
    ] {
        let cell = Cell::new(symbol);
        let updates = (0..120 / cell_width)
            .map(|index| ((index * cell_width) as u16, 0, &cell))
            .collect::<Vec<_>>();

        group.bench_with_input(BenchmarkId::from_parameter(name), &updates, |b, updates| {
            b.iter(|| {
                let mut backend = CrosstermBackend::new(io::sink());
                backend.draw(black_box(updates.iter().copied())).unwrap();
            });
        });
    }

    group.finish();
}
