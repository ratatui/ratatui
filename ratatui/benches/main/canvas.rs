use criterion::{BatchSize, Bencher, BenchmarkId, Criterion, criterion_group};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::Widget;
use ratatui::widgets::canvas::{Canvas, Circle, Context};

/// Canvas bounds used by most benchmarks. On an 80 x 24 braille canvas this is 160 x 96 dots, so
/// circles are ellipses in grid coordinates, as they usually are in practice.
const BOUNDS: [f64; 2] = [-10.0, 10.0];

/// Benchmarks for drawing circles on a canvas.
fn circle(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas/circle");

    // Radius as a fraction of the canvas half-height: a few dots up to touching the edges.
    for marker in [
        Marker::Dot,
        Marker::HalfBlock,
        Marker::Quadrant,
        Marker::Braille,
    ] {
        for radius in [0.1, 1.0, 5.0, 10.0] {
            let circles = [Circle::new(0.0, 0.0, radius, Color::Reset)];
            group.bench_with_input(
                BenchmarkId::new(format!("marker/{marker:?}"), radius),
                &(Size::STANDARD, marker, &circles[..]),
                draw,
            );
        }
    }

    // The same circle on canvases with more dots, where sampling a fixed number of points
    // leaves gaps.
    for (name, size) in [
        ("20x10", Size::new(20, 10)),
        ("80x24", Size::STANDARD),
        ("200x60", Size::new(200, 60)),
        ("400x120", Size::new(400, 120)),
    ] {
        let circles = [Circle::new(0.0, 0.0, 8.0, Color::Reset)];
        group.bench_with_input(
            BenchmarkId::new("canvas_size", name),
            &(size, Marker::Braille, &circles[..]),
            draw,
        );
    }

    // Circles that are partly or entirely outside the canvas. Cost should follow what is
    // visible, not the full circumference.
    for (name, circle) in [
        ("half_visible", Circle::new(10.0, 0.0, 5.0, Color::Reset)),
        ("corner_arc", Circle::new(-10.0, -10.0, 15.0, Color::Reset)),
        ("mostly_off_side", Circle::new(17.0, 0.0, 9.0, Color::Reset)),
        ("huge_arc", Circle::new(0.0, -1000.0, 1005.0, Color::Reset)),
        ("enclosing", Circle::new(0.0, 0.0, 100.0, Color::Reset)),
        ("outside", Circle::new(50.0, 50.0, 5.0, Color::Reset)),
    ] {
        let circles = [circle];
        group.bench_with_input(
            BenchmarkId::new("clipped", name),
            &(Size::STANDARD, Marker::Braille, &circles[..]),
            draw,
        );
    }

    // Many circles in one frame, like markers on a plot.
    let scattered: Vec<Circle> = (0..100)
        .map(|i| {
            let x = f64::from(i % 10).mul_add(2.0, -9.0);
            let y = f64::from(i / 10).mul_add(2.0, -9.0);
            let radius = 0.05 + f64::from(i % 7) * 0.15;
            Circle::new(x, y, radius, Color::Reset)
        })
        .collect();
    group.bench_with_input(
        BenchmarkId::new("many", 100),
        &(Size::STANDARD, Marker::Braille, &*scattered),
        draw,
    );

    // Three small circles of different sizes, like a gauge or indicator page on a small display.
    let indicators = [
        Circle::new(0.0, 0.0, 1.0, Color::Reset),
        Circle::new(0.5, 0.2, 0.18, Color::Reset),
        Circle::new(-0.4, -0.3, 0.08, Color::Reset),
    ];
    group.bench_with_input(
        BenchmarkId::new("indicators", 3),
        &(Size::new(40, 12), Marker::Braille, &indicators[..]),
        draw_with_bounds([-1.2, 1.2]),
    );

    group.finish();

    // The full widget render, including the grid and buffer work around the shapes.
    let mut group = c.benchmark_group("canvas/circle/render");
    for radius in [1.0, 8.0] {
        let circle = Circle::new(0.0, 0.0, radius, Color::Reset);
        group.bench_with_input(BenchmarkId::new("braille", radius), &circle, render);
    }
    group.finish();
}

#[derive(Clone, Copy)]
struct Size {
    width: u16,
    height: u16,
}

impl Size {
    const STANDARD: Self = Self::new(80, 24);

    const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

type Input<'a> = (Size, Marker, &'a [Circle]);

/// Draw the circles into a fresh context using the default bounds.
fn draw(bencher: &mut Bencher, input: &Input) {
    draw_with_bounds(BOUNDS)(bencher, input);
}

/// Draw the circles into a fresh context with the given bounds on both axes.
fn draw_with_bounds(bounds: [f64; 2]) -> impl Fn(&mut Bencher, &Input) {
    move |bencher, &(size, marker, circles)| {
        bencher.iter_batched(
            || Context::new(size.width, size.height, bounds, bounds, marker),
            |mut ctx| {
                for circle in circles {
                    ctx.draw(circle);
                }
                ctx
            },
            BatchSize::SmallInput,
        );
    }
}

/// Render a canvas containing the circle into an 80 x 24 buffer.
fn render(bencher: &mut Bencher, circle: &Circle) {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 24));
    bencher.iter(|| {
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds(BOUNDS)
            .y_bounds(BOUNDS)
            .paint(|ctx| ctx.draw(circle))
            .render(buffer.area, &mut buffer);
    });
}

criterion_group!(benches, circle);
