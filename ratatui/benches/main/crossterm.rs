use criterion::{criterion_group, BenchmarkId, Criterion};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::{backend::Backend, buffer::Cell};
use ratatui_crossterm::CrosstermBackend;
use std::io::stdout;

criterion_group!(benches, draw_random_buffer);

fn draw_random_buffer(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("crossterm");
    group.sample_size(10);
    let mut backend = CrosstermBackend::new(stdout());

    let size = backend.size().unwrap();
    let area = Rect::new(0, 0, size.width, size.height);
    let Rect { width, height, .. } = area;

    backend.hide_cursor().unwrap();
    for churn in [0.0, 0.1, 0.5, 1.0] {
        // draw a buffer where a certain percentage of cells have changed, using the diff
        let with_diff = format!("draw_random_buffer_{width}x{height}_with_diff");
        group.bench_function(BenchmarkId::new(with_diff, churn), |bencher| {
            let empty = Buffer::empty(area);
            let buffer = generate_random_buffer(area, churn);
            execute!(stdout(), EnterAlternateScreen).unwrap();
            bencher.iter(|| backend.draw(empty.diff(&buffer).into_iter()).unwrap());
            execute!(stdout(), LeaveAlternateScreen).unwrap();
        });

        // draw a buffer where every cell has changed
        let without_diff = format!("draw_random_buffer_{width}x{height}_without_diff");
        group.bench_function(BenchmarkId::new(without_diff, churn), |bencher| {
            let mut cell = Cell::new(".");
            cell.set_fg(Color::Red).set_bg(Color::White);
            let filled = Buffer::filled(area, cell);
            let buffer = generate_random_buffer(area, churn);
            let diff = filled.diff(&buffer);
            execute!(stdout(), EnterAlternateScreen).unwrap();
            bencher.iter(|| backend.draw(diff.clone().into_iter()).unwrap());
            execute!(stdout(), LeaveAlternateScreen).unwrap();
        });
    }
    group.finish();
    backend.show_cursor().unwrap();
}

fn generate_random_buffer(area: Rect, churn: f64) -> Buffer {
    let mut buffer = Buffer::empty(area);
    let mut rng = rand::thread_rng();
    for y in 0..area.height {
        if rng.gen::<f64>() > churn {
            continue;
        }
        for x in 0..area.width {
            let symbol = (rng.gen::<u8>() % 26 + b'A') as char;
            let fg = Color::Indexed(rng.gen_range(0..16));
            let bg = Color::Indexed(rng.gen_range(0..16));
            let style = Style::default().fg(fg).bg(bg);
            if let Some(cell) = buffer.cell_mut((x, y)) {
                cell.set_symbol(&symbol.to_string()).set_style(style);
            }
        }
    }
    buffer
}
