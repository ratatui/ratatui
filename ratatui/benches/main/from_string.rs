use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use ratatui::text::{Line, Text};

/// Benchmark for building a [`Line`] or [`Text`] from an owned [`String`], such as the result of
/// `format!` in a render loop.
fn from_string(c: &mut Criterion) {
    let long = "The quick brown fox jumps over the lazy dog. ".repeat(20);
    let inputs = [
        ("short", String::from("Frame 42: 16.7ms")),
        ("short_trailing_newline", String::from("Frame 42: 16.7ms\n")),
        ("long", long.clone()),
        ("multi_line", format!("{long}\n{long}\n{long}")),
    ];

    let mut group = c.benchmark_group("from_string");
    for (name, input) in &inputs {
        group.bench_with_input(BenchmarkId::new("line", name), input, |b, input| {
            b.iter_batched(|| input.to_owned(), Line::from, BatchSize::SmallInput);
        });
        group.bench_with_input(BenchmarkId::new("text", name), input, |b, input| {
            b.iter_batched(|| input.to_owned(), Text::from, BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(benches, from_string);
