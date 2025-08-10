use std::hint::black_box;
use std::rc::Rc;

use criterion::{Criterion, criterion_group};
use ratatui::layout::Constraint::{Fill, Length, Max, Min, Percentage, Ratio};
use ratatui::layout::{Direction, Layout, Rect};

const SPLIT_BY: u16 = 10;

fn generate_layout(constraint: &str, area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints({
            let mut cs = Vec::new();
            for _ in 0..SPLIT_BY {
                match constraint {
                    "Fill" => {
                        cs.push(Fill(1));
                    }
                    "Length" => {
                        cs.push(Length(area.width / SPLIT_BY));
                    }
                    "Max" => {
                        cs.push(Max(area.width / SPLIT_BY));
                    }
                    "Min" => {
                        cs.push(Min(area.width / SPLIT_BY));
                    }
                    "Percentage" => {
                        cs.push(Percentage(100 / SPLIT_BY));
                    }
                    "Ratio" => {
                        cs.push(Ratio(1, SPLIT_BY.into()));
                    }
                    _ => todo!(),
                }
            }
            cs
        })
        .split(area)
}

fn constraints_render(criterion: &mut Criterion) {
    for size in [16, 64, 255] {
        let mut group = criterion.benchmark_group(format!("constraints {size}x{size}"));
        group.sample_size(1000);
        let area = Rect::new(0, 0, size, size);
        for constraint in ["Fill", "Length", "Max", "Min", "Percentage", "Ratio"] {
            group.bench_function(constraint.to_string(), |bencher| {
                bencher.iter(|| {
                    _ = black_box(generate_layout(constraint, area));
                });
            });
        }
        group.finish();
    }
}

criterion_group!(benches, constraints_render);
