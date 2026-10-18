use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput};

/// Benchmarks drawing a full 200x50 frame with the crossterm backend into a `Vec<u8>`.
///
/// The throughput is the number of bytes written per frame, so the report shows how much output
/// each case produces as well as how long it takes:
///
/// - `plain`: no hyperlinks.
/// - `cell_extensions` (with the `cell-extensions` feature): every 10th row is a hyperlink set with
///   `Buffer::set_hyperlink`, written as one OSC 8 run per row.
/// - `osc8_per_cell`: the same links written by the old workaround, which stored a complete OSC 8
///   sequence in every linked cell's symbol.
#[cfg(feature = "crossterm")]
fn draw(c: &mut Criterion) {
    use std::num::NonZeroU16;

    use ratatui::backend::{Backend, CrosstermBackend};
    use ratatui::buffer::{Buffer, CellDiffOption};
    use ratatui::layout::Rect;
    use ratatui::style::Style;

    const URI: &str = "https://ratatui.rs";
    let area = Rect::new(0, 0, 200, 50);
    let linked_rows = || (area.top()..area.bottom()).step_by(10);
    let mut plain = Buffer::empty(area);
    for y in area.top()..area.bottom() {
        plain.set_string(0, y, "lorem ipsum ".repeat(17), Style::new());
    }

    let mut osc8_per_cell = plain.clone();
    for y in linked_rows() {
        for x in area.left()..area.right() {
            let cell = &mut osc8_per_cell[(x, y)];
            let symbol = format!("\x1b]8;;{URI}\x1b\\{}\x1b]8;;\x1b\\", cell.symbol());
            cell.set_symbol(&symbol)
                .set_diff_option(CellDiffOption::ForcedWidth(NonZeroU16::MIN));
        }
    }

    #[cfg_attr(not(feature = "cell-extensions"), allow(unused_mut))]
    let mut cases = vec![("plain", plain.clone()), ("osc8_per_cell", osc8_per_cell)];
    #[cfg(feature = "cell-extensions")]
    {
        let mut linked = plain;
        let link = ratatui::buffer::Hyperlink::new(URI);
        for y in linked_rows() {
            linked.set_hyperlink(Rect::new(area.x, y, area.width, 1), Some(&link));
        }
        cases.push(("cell_extensions", linked));
    }

    let empty = Buffer::empty(area);
    let draw = |buffer: &Buffer, out: &mut Vec<u8>| {
        out.clear();
        CrosstermBackend::new(out)
            .draw(empty.diff_iter(buffer))
            .unwrap();
    };
    let mut group = c.benchmark_group("backend/crossterm_draw");
    for (name, buffer) in &cases {
        let mut out = Vec::new();
        draw(buffer, &mut out);
        group.throughput(Throughput::Bytes(out.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), buffer, |b, buffer| {
            b.iter(|| draw(black_box(buffer), &mut out));
        });
    }
    group.finish();
}

#[cfg(not(feature = "crossterm"))]
const fn draw(_: &mut Criterion) {}

criterion::criterion_group!(benches, draw);
