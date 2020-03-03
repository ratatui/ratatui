mod line;
mod map;
mod points;
mod rectangle;
mod world;

pub use self::line::Line;
pub use self::map::{Map, MapResolution};
pub use self::points::Points;
pub use self::rectangle::Rectangle;

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use std::fmt::Debug;

pub const DOTS: [[u16; 2]; 4] = [
    [0x0001, 0x0008],
    [0x0002, 0x0010],
    [0x0004, 0x0020],
    [0x0040, 0x0080],
];
pub const BRAILLE_OFFSET: u16 = 0x2800;
pub const BRAILLE_BLANK: char = '⠀';

/// Interface for all shapes that may be drawn on a Canvas widget.
pub trait Shape {
    fn draw(&self, painter: &mut Painter);
}

/// Label to draw some text on the canvas
#[derive(Debug, Clone)]
pub struct Label<'a> {
    pub x: f64,
    pub y: f64,
    pub text: &'a str,
    pub color: Color,
}

#[derive(Debug, Clone)]
struct Layer {
    string: String,
    colors: Vec<Color>,
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<u16>,
    colors: Vec<Color>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![BRAILLE_OFFSET; width * height],
            colors: vec![Color::Reset; width * height],
        }
    }

    fn save(&self) -> Layer {
        Layer {
            string: String::from_utf16(&self.cells).unwrap(),
            colors: self.colors.clone(),
        }
    }

    fn reset(&mut self) {
        for c in &mut self.cells {
            *c = BRAILLE_OFFSET;
        }
        for c in &mut self.colors {
            *c = Color::Reset;
        }
    }
}

#[derive(Debug)]
pub struct Painter<'a, 'b> {
    context: &'a mut Context<'b>,
    resolution: [f64; 2],
}

impl<'a, 'b> Painter<'a, 'b> {
    /// Convert the (x, y) coordinates to location of a braille dot on the grid
    ///
    /// # Examples:
    /// ```
    /// use tui::widgets::canvas::{Painter, Context};
    ///
    /// let mut ctx = Context::new(2, 2, [1.0, 2.0], [0.0, 2.0]);
    /// let mut painter = Painter::from(&mut ctx);
    /// let point = painter.get_point(1.0, 0.0);
    /// assert_eq!(point, Some((0, 7)));
    /// let point = painter.get_point(1.5, 1.0);
    /// assert_eq!(point, Some((1, 3)));
    /// let point = painter.get_point(0.0, 0.0);
    /// assert_eq!(point, None);
    /// let point = painter.get_point(2.0, 2.0);
    /// assert_eq!(point, Some((3, 0)));
    /// let point = painter.get_point(1.0, 2.0);
    /// assert_eq!(point, Some((0, 0)));
    /// ```
    pub fn get_point(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let left = self.context.x_bounds[0];
        let right = self.context.x_bounds[1];
        let top = self.context.y_bounds[1];
        let bottom = self.context.y_bounds[0];
        if x < left || x > right || y < bottom || y > top {
            return None;
        }
        let width = (self.context.x_bounds[1] - self.context.x_bounds[0]).abs();
        let height = (self.context.y_bounds[1] - self.context.y_bounds[0]).abs();
        let x = ((x - left) * self.resolution[0] / width) as usize;
        let y = ((top - y) * self.resolution[1] / height) as usize;
        Some((x, y))
    }

    /// Paint a braille dot
    ///
    /// # Examples:
    /// ```
    /// use tui::{style::Color, widgets::canvas::{Painter, Context}};
    ///
    /// let mut ctx = Context::new(1, 1, [0.0, 2.0], [0.0, 2.0]);
    /// let mut painter = Painter::from(&mut ctx);
    /// let cell = painter.paint(1, 3, Color::Red);
    /// ```
    pub fn paint(&mut self, x: usize, y: usize, color: Color) {
        let index = y / 4 * self.context.width as usize + x / 2;
        if let Some(c) = self.context.grid.cells.get_mut(index) {
            *c |= DOTS[y % 4][x % 2];
        }
        if let Some(c) = self.context.grid.colors.get_mut(index) {
            *c = color;
        }
    }
}

impl<'a, 'b> From<&'a mut Context<'b>> for Painter<'a, 'b> {
    fn from(context: &'a mut Context<'b>) -> Painter<'a, 'b> {
        Painter {
            resolution: [
                f64::from(context.width) * 2.0 - 1.0,
                f64::from(context.height) * 4.0 - 1.0,
            ],
            context,
        }
    }
}

/// Holds the state of the Canvas when painting to it.
#[derive(Debug, Clone)]
pub struct Context<'a> {
    width: u16,
    height: u16,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    grid: Grid,
    dirty: bool,
    layers: Vec<Layer>,
    labels: Vec<Label<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(width: u16, height: u16, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> Context<'a> {
        Context {
            width,
            height,
            x_bounds,
            y_bounds,
            grid: Grid::new(width as usize, height as usize),
            dirty: false,
            layers: Vec::new(),
            labels: Vec::new(),
        }
    }

    /// Draw any object that may implement the Shape trait
    pub fn draw<S>(&mut self, shape: &S)
    where
        S: Shape,
    {
        self.dirty = true;
        let mut painter = Painter::from(self);
        shape.draw(&mut painter);
    }

    /// Go one layer above in the canvas.
    pub fn layer(&mut self) {
        self.layers.push(self.grid.save());
        self.grid.reset();
        self.dirty = false;
    }

    /// Print a string on the canvas at the given position
    pub fn print(&mut self, x: f64, y: f64, text: &'a str, color: Color) {
        self.labels.push(Label { x, y, text, color });
    }

    /// Push the last layer if necessary
    fn finish(&mut self) {
        if self.dirty {
            self.layer()
        }
    }
}

/// The Canvas widget may be used to draw more detailed figures using braille patterns (each
/// cell can have a braille character in 8 different positions).
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders};
/// # use tui::layout::Rect;
/// # use tui::widgets::canvas::{Canvas, Shape, Line, Rectangle, Map, MapResolution};
/// # use tui::style::Color;
/// Canvas::default()
///     .block(Block::default().title("Canvas").borders(Borders::ALL))
///     .x_bounds([-180.0, 180.0])
///     .y_bounds([-90.0, 90.0])
///     .paint(|ctx| {
///         ctx.draw(&Map {
///             resolution: MapResolution::High,
///             color: Color::White
///         });
///         ctx.layer();
///         ctx.draw(&Line {
///             x1: 0.0,
///             y1: 10.0,
///             x2: 10.0,
///             y2: 10.0,
///             color: Color::White,
///         });
///         ctx.draw(&Rectangle {
///             x: 10.0,
///             y: 20.0,
///             width: 10.0,
///             height: 10.0,
///             color: Color::Red
///         });
///     });
/// ```
pub struct Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    block: Option<Block<'a>>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    painter: Option<F>,
    background_color: Color,
}

impl<'a, F> Default for Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    fn default() -> Canvas<'a, F> {
        Canvas {
            block: None,
            x_bounds: [0.0, 0.0],
            y_bounds: [0.0, 0.0],
            painter: None,
            background_color: Color::Reset,
        }
    }
}

impl<'a, F> Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    pub fn block(mut self, block: Block<'a>) -> Canvas<'a, F> {
        self.block = Some(block);
        self
    }
    pub fn x_bounds(mut self, bounds: [f64; 2]) -> Canvas<'a, F> {
        self.x_bounds = bounds;
        self
    }
    pub fn y_bounds(mut self, bounds: [f64; 2]) -> Canvas<'a, F> {
        self.y_bounds = bounds;
        self
    }

    /// Store the closure that will be used to draw to the Canvas
    pub fn paint(mut self, f: F) -> Canvas<'a, F> {
        self.painter = Some(f);
        self
    }

    pub fn background_color(mut self, color: Color) -> Canvas<'a, F> {
        self.background_color = color;
        self
    }
}

impl<'a, F> Widget for Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let canvas_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        let width = canvas_area.width as usize;

        let painter = match self.painter {
            Some(ref p) => p,
            None => return,
        };

        // Create a blank context that match the size of the canvas
        let mut ctx = Context::new(
            canvas_area.width,
            canvas_area.height,
            self.x_bounds,
            self.y_bounds,
        );
        // Paint to this context
        painter(&mut ctx);
        ctx.finish();

        // Retreive painted points for each layer
        for layer in ctx.layers {
            for (i, (ch, color)) in layer
                .string
                .chars()
                .zip(layer.colors.into_iter())
                .enumerate()
            {
                if ch != BRAILLE_BLANK {
                    let (x, y) = (i % width, i / width);
                    buf.get_mut(x as u16 + canvas_area.left(), y as u16 + canvas_area.top())
                        .set_char(ch)
                        .set_fg(color)
                        .set_bg(self.background_color);
                }
            }
        }

        // Finally draw the labels
        let style = Style::default().bg(self.background_color);
        let left = self.x_bounds[0];
        let right = self.x_bounds[1];
        let top = self.y_bounds[1];
        let bottom = self.y_bounds[0];
        let width = (self.x_bounds[1] - self.x_bounds[0]).abs();
        let height = (self.y_bounds[1] - self.y_bounds[0]).abs();
        let resolution = {
            let width = f64::from(canvas_area.width - 1);
            let height = f64::from(canvas_area.height - 1);
            (width, height)
        };
        for label in ctx
            .labels
            .iter()
            .filter(|l| l.x >= left && l.x <= right && l.y <= top && l.y >= bottom)
        {
            let x = ((label.x - left) * resolution.0 / width) as u16 + canvas_area.left();
            let y = ((top - label.y) * resolution.1 / height) as u16 + canvas_area.top();
            buf.set_stringn(
                x,
                y,
                label.text,
                (canvas_area.right() - x) as usize,
                style.fg(label.color),
            );
        }
    }
}
