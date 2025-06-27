//! A [`Canvas`] and a collection of [`Shape`]s.
//!
//! The [`Canvas`] is a blank space on which you can draw anything manually or use one of the
//! predefined [`Shape`]s.
//!
//! The available shapes are:
//!
//! - [`Circle`]: A basic circle
//! - [`Line`]: A line between two points
//! - [`Map`]: A world map
//! - [`Points`]: A scatter of points
//! - [`Rectangle`]: A basic rectangle
//!
//! You can also implement your own custom [`Shape`]s.
mod circle;
mod line;
mod map;
mod points;
mod rectangle;
mod world;

use std::{fmt::Debug, iter::zip, ops};

use itertools::Itertools;

pub use self::{
    circle::Circle,
    line::Line,
    map::{Map, MapResolution},
    points::Points,
    rectangle::Rectangle,
};
use crate::{prelude::*, symbols, text::Line as TextLine, widgets::Block};

/// Something that can be drawn on a [`Canvas`].
///
/// You may implement your own canvas custom widgets by implementing this trait.
pub trait Shape {
    /// Draws this [`Shape`] using the given [`Painter`].
    ///
    /// This is the only method required to implement a custom widget that can be drawn on a
    /// [`Canvas`].
    fn draw(&self, painter: &mut Painter);
}

/// Label to draw some text on the canvas
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Label<'a> {
    x: f64,
    y: f64,
    line: TextLine<'a>,
}

/// A single layer of the canvas.
///
/// This allows the canvas to be drawn in multiple layers. This is useful if you want to draw
/// multiple shapes on the canvas in specific order.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Layer {
    // A string of characters representing the grid. This will be wrapped to the width of the grid
    // when rendering
    string: String,
    // Colors for foreground and background of each cell
    colors: Vec<(Color, Color)>,
}

/// A grid of cells that can be painted on.
///
/// The grid represents a particular screen region measured in rows and columns. The underlying
/// resolution of the grid might exceed the number of rows and columns. For example, a grid of
/// Braille patterns will have a resolution of 2x4 dots per cell. This means that a grid of 10x10
/// cells will have a resolution of 20x40 dots.
trait Grid: Debug {
    /// Get the width of the grid in number of terminal columns
    fn width(&self) -> u16;
    /// Get the height of the grid in number of terminal rows
    fn height(&self) -> u16;
    /// Get the resolution of the grid in number of dots.
    ///
    /// This doesn't have to be the same as the number of rows and columns of the grid. For example,
    /// a grid of Braille patterns will have a resolution of 2x4 dots per cell. This means that a
    /// grid of 10x10 cells will have a resolution of 20x40 dots.
    fn resolution(&self) -> (f64, f64);
    /// Paint a point of the grid.
    ///
    /// The point is expressed in number of dots starting at the origin of the grid in the top left
    /// corner. Note that this is not the same as the `(x, y)` coordinates of the canvas.
    fn paint(&mut self, x: usize, y: usize, color: Color);
    /// Save the current state of the [`Grid`] as a layer to be rendered
    fn save(&self) -> Layer;
    /// Reset the grid to its initial state
    fn reset(&mut self);
}

/// The BrailleGrid is a grid made up of cells each containing a Braille pattern.
///
/// This makes it possible to draw shapes with a resolution of 2x4 dots per cell. This is useful
/// when you want to draw shapes with a high resolution. Font support for Braille patterns is
/// required to see the dots. If your terminal or font does not support this unicode block, you
/// will see unicode replacement characters (�) instead of braille dots.
///
/// This grid type only supports a single foreground color for each 2x4 dots cell. There is no way
/// to set the individual color of each dot in the braille pattern.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct BrailleGrid {
    /// Width of the grid in number of terminal columns
    width: u16,
    /// Height of the grid in number of terminal rows
    height: u16,
    /// Represents the unicode braille patterns. Will take a value between `0x2800` and `0x28FF`
    /// this is converted to an utf16 string when converting to a layer. See
    /// <https://en.wikipedia.org/wiki/Braille_Patterns> for more info.
    utf16_code_points: Vec<u16>,
    /// The color of each cell only supports foreground colors for now as there's no way to
    /// individually set the background color of each dot in the braille pattern.
    colors: Vec<Color>,
}

impl BrailleGrid {
    /// Create a new BrailleGrid with the given width and height measured in terminal columns and
    /// rows respectively.
    fn new(width: u16, height: u16) -> BrailleGrid {
        let length = usize::from(width * height);
        BrailleGrid {
            width,
            height,
            utf16_code_points: vec![symbols::braille::BLANK; length],
            colors: vec![Color::Reset; length],
        }
    }
}

impl Grid for BrailleGrid {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn resolution(&self) -> (f64, f64) {
        (f64::from(self.width) * 2.0, f64::from(self.height) * 4.0)
    }

    fn save(&self) -> Layer {
        let string = String::from_utf16(&self.utf16_code_points).unwrap();
        // the background color is always reset for braille patterns
        let colors = self.colors.iter().map(|c| (*c, Color::Reset)).collect();
        Layer { string, colors }
    }

    fn reset(&mut self) {
        self.utf16_code_points.fill(symbols::braille::BLANK);
        self.colors.fill(Color::Reset);
    }

    fn paint(&mut self, x: usize, y: usize, color: Color) {
        let index = y / 4 * self.width as usize + x / 2;
        // using get_mut here because we are indexing the vector with usize values
        // and we want to make sure we don't panic if the index is out of bounds
        if let Some(c) = self.utf16_code_points.get_mut(index) {
            *c |= symbols::braille::DOTS[y % 4][x % 2];
        }
        if let Some(c) = self.colors.get_mut(index) {
            *c = color;
        }
    }
}

/// The CharGrid is a grid made up of cells each containing a single character.
///
/// This makes it possible to draw shapes with a resolution of 1x1 dots per cell. This is useful
/// when you want to draw shapes with a low resolution.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct CharGrid {
    /// Width of the grid in number of terminal columns
    width: u16,
    /// Height of the grid in number of terminal rows
    height: u16,
    /// Represents a single character for each cell
    cells: Vec<char>,
    /// The color of each cell
    colors: Vec<Color>,
    /// The character to use for every cell - e.g. a block, dot, etc.
    cell_char: char,
}

impl CharGrid {
    /// Create a new CharGrid with the given width and height measured in terminal columns and
    /// rows respectively.
    fn new(width: u16, height: u16, cell_char: char) -> CharGrid {
        let length = usize::from(width * height);
        CharGrid {
            width,
            height,
            cells: vec![' '; length],
            colors: vec![Color::Reset; length],
            cell_char,
        }
    }
}

impl Grid for CharGrid {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn resolution(&self) -> (f64, f64) {
        (f64::from(self.width), f64::from(self.height))
    }

    fn save(&self) -> Layer {
        Layer {
            string: self.cells.iter().collect(),
            colors: self.colors.iter().map(|c| (*c, Color::Reset)).collect(),
        }
    }

    fn reset(&mut self) {
        self.cells.fill(' ');
        self.colors.fill(Color::Reset);
    }

    fn paint(&mut self, x: usize, y: usize, color: Color) {
        let index = y * self.width as usize + x;
        // using get_mut here because we are indexing the vector with usize values
        // and we want to make sure we don't panic if the index is out of bounds
        if let Some(c) = self.cells.get_mut(index) {
            *c = self.cell_char;
        }
        if let Some(c) = self.colors.get_mut(index) {
            *c = color;
        }
    }
}

/// The HalfBlockGrid is a grid made up of cells each containing a half block character.
///
/// In terminals, each character is usually twice as tall as it is wide. Unicode has a couple of
/// vertical half block characters, the upper half block '▀' and lower half block '▄' which take up
/// half the height of a normal character but the full width. Together with an empty space ' ' and a
/// full block '█', we can effectively double the resolution of a single cell. In addition, because
/// each character can have a foreground and background color, we can control the color of the upper
/// and lower half of each cell. This allows us to draw shapes with a resolution of 1x2 "pixels" per
/// cell.
///
/// This allows for more flexibility than the BrailleGrid which only supports a single
/// foreground color for each 2x4 dots cell, and the CharGrid which only supports a single
/// character for each cell.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct HalfBlockGrid {
    /// Width of the grid in number of terminal columns
    width: u16,
    /// Height of the grid in number of terminal rows
    height: u16,
    /// Represents a single color for each "pixel" arranged in column, row order
    pixels: Vec<Vec<Color>>,
}

impl HalfBlockGrid {
    /// Create a new `HalfBlockGrid` with the given width and height measured in terminal columns
    /// and rows respectively.
    fn new(width: u16, height: u16) -> HalfBlockGrid {
        HalfBlockGrid {
            width,
            height,
            pixels: vec![vec![Color::Reset; width as usize]; height as usize * 2],
        }
    }
}

impl Grid for HalfBlockGrid {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn resolution(&self) -> (f64, f64) {
        (f64::from(self.width), f64::from(self.height) * 2.0)
    }

    fn save(&self) -> Layer {
        // Given that we store the pixels in a grid, and that we want to use 2 pixels arranged
        // vertically to form a single terminal cell, which can be either empty, upper half block,
        // lower half block or full block, we need examine the pixels in vertical pairs to decide
        // what character to print in each cell. So these are the 4 states we use to represent each
        // cell:
        //
        // 1. upper: reset, lower: reset => ' ' fg: reset / bg: reset
        // 2. upper: reset, lower: color => '▄' fg: lower color / bg: reset
        // 3. upper: color, lower: reset => '▀' fg: upper color / bg: reset
        // 4. upper: color, lower: color => '▀' fg: upper color / bg: lower color
        //
        // Note that because the foreground reset color (i.e. default foreground color) is usually
        // not the same as the background reset color (i.e. default background color), we need to
        // swap around the colors for that state (2 reset/color).
        //
        // When the upper and lower colors are the same, we could continue to use an upper half
        // block, but we choose to use a full block instead. This allows us to write unit tests that
        // treat the cell as a single character instead of two half block characters.

        // first we join each adjacent row together to get an iterator that contains vertical pairs
        // of pixels, with the lower row being the first element in the pair
        let vertical_color_pairs = self
            .pixels
            .iter()
            .tuples()
            .flat_map(|(upper_row, lower_row)| zip(upper_row, lower_row));

        // then we work out what character to print for each pair of pixels
        let string = vertical_color_pairs
            .clone()
            .map(|(upper, lower)| match (upper, lower) {
                (Color::Reset, Color::Reset) => ' ',
                (Color::Reset, _) => symbols::half_block::LOWER,
                (_, Color::Reset) => symbols::half_block::UPPER,
                (&lower, &upper) => {
                    if lower == upper {
                        symbols::half_block::FULL
                    } else {
                        symbols::half_block::UPPER
                    }
                }
            })
            .collect();

        // then we convert these each vertical pair of pixels into a foreground and background color
        let colors = vertical_color_pairs
            .map(|(upper, lower)| {
                let (fg, bg) = match (upper, lower) {
                    (Color::Reset, Color::Reset) => (Color::Reset, Color::Reset),
                    (Color::Reset, &lower) => (lower, Color::Reset),
                    (&upper, Color::Reset) => (upper, Color::Reset),
                    (&upper, &lower) => (upper, lower),
                };
                (fg, bg)
            })
            .collect();

        Layer { string, colors }
    }

    fn reset(&mut self) {
        self.pixels.fill(vec![Color::Reset; self.width as usize]);
    }

    fn paint(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }
}

/// Painter is an abstraction over the [`Context`] that allows to draw shapes on the grid.
///
/// It is used by the [`Shape`] trait to draw shapes on the grid. It can be useful to think of this
/// as similar to the [`Buffer`] struct that is used to draw widgets on the terminal.
#[derive(Debug)]
pub struct Painter<'a, 'b> {
    context: &'a mut Context<'b>,
    resolution: (f64, f64),
}

impl<'a, 'b> Painter<'a, 'b> {
    /// Convert the `(x, y)` coordinates to location of a point on the grid
    ///
    /// `(x, y)` coordinates are expressed in the coordinate system of the canvas. The origin is in
    /// the lower left corner of the canvas (unlike most other coordinates in `Ratatui` where the
    /// origin is the upper left corner). The `x` and `y` bounds of the canvas define the specific
    /// area of some coordinate system that will be drawn on the canvas. The resolution of the grid
    /// is used to convert the `(x, y)` coordinates to the location of a point on the grid.
    ///
    /// The grid coordinates are expressed in the coordinate system of the grid. The origin is in
    /// the top left corner of the grid. The x and y bounds of the grid are always `[0, width - 1]`
    /// and `[0, height - 1]` respectively. The resolution of the grid is used to convert the
    /// `(x, y)` coordinates to the location of a point on the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::{prelude::*, widgets::canvas::*};
    ///
    /// let mut ctx = Context::new(2, 2, [1.0, 2.0], [0.0, 2.0], symbols::Marker::Braille);
    /// let mut painter = Painter::from(&mut ctx);
    ///
    /// let point = painter.get_point(1.0, 0.0);
    /// assert_eq!(point, Some((0, 7)));
    ///
    /// let point = painter.get_point(1.5, 1.0);
    /// assert_eq!(point, Some((1, 3)));
    ///
    /// let point = painter.get_point(0.0, 0.0);
    /// assert_eq!(point, None);
    ///
    /// let point = painter.get_point(2.0, 2.0);
    /// assert_eq!(point, Some((3, 0)));
    ///
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
        if width == 0.0 || height == 0.0 {
            return None;
        }
        let x = ((x - left) * (self.resolution.0 - 1.0) / width) as usize;
        let y = ((top - y) * (self.resolution.1 - 1.0) / height) as usize;
        Some((x, y))
    }

    /// Paint a point of the grid
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::{prelude::*, widgets::canvas::*};
    ///
    /// let mut ctx = Context::new(1, 1, [0.0, 2.0], [0.0, 2.0], symbols::Marker::Braille);
    /// let mut painter = Painter::from(&mut ctx);
    /// painter.paint(1, 3, Color::Red);
    /// ```
    pub fn paint(&mut self, x: usize, y: usize, color: Color) {
        self.context.grid.paint(x, y, color);
    }

    /// Get the bounds of the canvas coordinate system.
    pub const fn bounds(&self) -> (&[f64; 2], &[f64; 2]) {
        (&self.context.x_bounds, &self.context.y_bounds)
    }

    /// Get the resolution of the grid in number of dots.
    pub const fn resolution(&self) -> (f64, f64) {
        self.resolution
    }
}

impl<'a, 'b> From<&'a mut Context<'b>> for Painter<'a, 'b> {
    fn from(context: &'a mut Context<'b>) -> Painter<'a, 'b> {
        let resolution = context.grid.resolution();
        Painter {
            context,
            resolution,
        }
    }
}

/// Holds the state of the [`Canvas`] when painting to it.
///
/// This is used by the [`Canvas`] widget to draw shapes on the grid. It can be useful to think of
/// this as similar to the [`Frame`] struct that is used to draw widgets on the terminal.
///
/// [`Frame`]: crate::prelude::Frame
#[derive(Debug)]
pub struct Context<'a> {
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    grid: Box<dyn Grid>,
    dirty: bool,
    layers: Vec<Layer>,
    labels: Vec<Label<'a>>,
}

impl<'a> Context<'a> {
    /// Create a new Context with the given width and height measured in terminal columns and rows
    /// respectively. The `x` and `y` bounds define the specific area of some coordinate system that
    /// will be drawn on the canvas. The marker defines the type of points used to draw the shapes.
    ///
    /// Applications should not use this directly but rather use the [`Canvas`] widget. This will be
    /// created by the [`Canvas::paint`] method and passed to the closure that is used to draw on
    /// the canvas.
    ///
    /// The `x` and `y` bounds should be specified as left/right and bottom/top respectively. For
    /// example, if you want to draw a map of the world, you might want to use the following bounds:
    ///
    /// ```
    /// use ratatui::{prelude::*, widgets::canvas::*};
    ///
    /// let ctx = Context::new(
    ///     100,
    ///     100,
    ///     [-180.0, 180.0],
    ///     [-90.0, 90.0],
    ///     symbols::Marker::Braille,
    /// );
    /// ```
    pub fn new(
        width: u16,
        height: u16,
        x_bounds: [f64; 2],
        y_bounds: [f64; 2],
        marker: symbols::Marker,
    ) -> Context<'a> {
        let dot = symbols::DOT.chars().next().unwrap();
        let block = symbols::block::FULL.chars().next().unwrap();
        let bar = symbols::bar::HALF.chars().next().unwrap();
        let grid: Box<dyn Grid> = match marker {
            symbols::Marker::Dot => Box::new(CharGrid::new(width, height, dot)),
            symbols::Marker::Block => Box::new(CharGrid::new(width, height, block)),
            symbols::Marker::Bar => Box::new(CharGrid::new(width, height, bar)),
            symbols::Marker::Braille => Box::new(BrailleGrid::new(width, height)),
            symbols::Marker::HalfBlock => Box::new(HalfBlockGrid::new(width, height)),
        };
        Context {
            x_bounds,
            y_bounds,
            grid,
            dirty: false,
            layers: Vec::new(),
            labels: Vec::new(),
        }
    }

    /// Draw the given [`Shape`] in this context
    pub fn draw<S>(&mut self, shape: &S)
    where
        S: Shape,
    {
        self.dirty = true;
        let mut painter = Painter::from(self);
        shape.draw(&mut painter);
    }

    /// Save the existing state of the grid as a layer.
    ///
    /// Save the existing state as a layer to be rendered and reset the grid to its initial
    /// state for the next layer.
    ///
    /// This allows the canvas to be drawn in multiple layers. This is useful if you want to
    /// draw multiple shapes on the [`Canvas`] in specific order.
    pub fn layer(&mut self) {
        self.layers.push(self.grid.save());
        self.grid.reset();
        self.dirty = false;
    }

    /// Print a [`Text`] on the [`Canvas`] at the given position.
    ///
    /// Note that the text is always printed on top of the canvas and is **not** affected by the
    /// layers.
    pub fn print<T>(&mut self, x: f64, y: f64, line: T)
    where
        T: Into<TextLine<'a>>,
    {
        self.labels.push(Label {
            x,
            y,
            line: line.into(),
        });
    }

    /// Save the last layer if necessary
    fn finish(&mut self) {
        if self.dirty {
            self.layer();
        }
    }
}

/// The Canvas widget provides a means to draw shapes (Lines, Rectangles, Circles, etc.) on a grid.
///
/// By default the grid is made of Braille patterns but you may change the marker to use a different
/// set of symbols. If your terminal or font does not support this unicode block, you will see
/// unicode replacement characters (�) instead of braille dots. The Braille patterns provide a more
/// fine grained result (2x4 dots) but you might want to use a simple dot, block, or bar instead by
/// calling the [`marker`] method if your target environment does not support those symbols,
///
/// See [Unicode Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns) for more info.
///
/// The HalfBlock marker is useful when you want to draw shapes with a higher resolution than a
/// CharGrid but lower than a BrailleGrid. This grid type supports a foreground and background color
/// for each terminal cell. This allows for more flexibility than the BrailleGrid which only
/// supports a single foreground color for each 2x4 dots cell.
///
/// The Canvas widget is used by calling the [`Canvas::paint`] method and passing a closure that
/// will be used to draw on the canvas. The closure will be passed a [`Context`] object that can be
/// used to draw shapes on the canvas.
///
/// The [`Context`] object provides a [`Context::draw`] method that can be used to draw shapes on
/// the canvas. The [`Context::layer`] method can be used to save the current state of the canvas
/// and start a new layer. This is useful if you want to draw multiple shapes on the canvas in
/// specific order. The [`Context`] object also provides a [`Context::print`] method that can be
/// used to print text on the canvas. Note that the text is always printed on top of the canvas and
/// is not affected by the layers.
///
/// # Examples
///
/// ```
/// use ratatui::{
///     style::Color,
///     widgets::{canvas::*, *},
/// };
///
/// Canvas::default()
///     .block(Block::default().title("Canvas").borders(Borders::ALL))
///     .x_bounds([-180.0, 180.0])
///     .y_bounds([-90.0, 90.0])
///     .paint(|ctx| {
///         ctx.draw(&Map {
///             resolution: MapResolution::High,
///             color: Color::White,
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
///             color: Color::Red,
///         });
///     });
/// ```
///
/// [`marker`]: #method.marker
#[derive(Debug, Clone, PartialEq)]
pub struct Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    block: Option<Block<'a>>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    paint_func: Option<F>,
    background_color: Color,
    marker: symbols::Marker,
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
            paint_func: None,
            background_color: Color::Reset,
            marker: symbols::Marker::Braille,
        }
    }
}

impl<'a, F> Canvas<'a, F>
where
    F: Fn(&mut Context),
{
    /// Wraps the canvas with a custom [`Block`] widget.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Canvas<'a, F> {
        self.block = Some(block);
        self
    }

    /// Define the viewport of the canvas.
    ///
    /// If you were to "zoom" to a certain part of the world you may want to choose different
    /// bounds.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn x_bounds(mut self, bounds: [f64; 2]) -> Canvas<'a, F> {
        self.x_bounds = bounds;
        self
    }

    /// Define the viewport of the canvas.
    ///
    /// If you were to "zoom" to a certain part of the world you may want to choose different
    /// bounds.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn y_bounds(mut self, bounds: [f64; 2]) -> Canvas<'a, F> {
        self.y_bounds = bounds;
        self
    }

    /// Store the closure that will be used to draw to the [`Canvas`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn paint(mut self, f: F) -> Canvas<'a, F> {
        self.paint_func = Some(f);
        self
    }

    /// Change the background [`Color`] of the entire canvas
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn background_color(mut self, color: Color) -> Canvas<'a, F> {
        self.background_color = color;
        self
    }

    /// Change the type of points used to draw the shapes.
    ///
    /// By default the [`Braille`] patterns are used as they provide a more fine grained result,
    /// but you might want to use the simple [`Dot`] or [`Block`] instead if the targeted terminal
    /// does not support those symbols.
    ///
    /// The [`HalfBlock`] marker is useful when you want to draw shapes with a higher resolution
    /// than with a grid of characters (e.g. with [`Block`] or [`Dot`]) but lower than with
    /// [`Braille`]. This grid type supports a foreground and background color for each terminal
    /// cell. This allows for more flexibility than the BrailleGrid which only supports a single
    /// foreground color for each 2x4 dots cell.
    ///
    /// [`Braille`]: crate::symbols::Marker::Braille
    /// [`HalfBlock`]: crate::symbols::Marker::HalfBlock
    /// [`Dot`]: crate::symbols::Marker::Dot
    /// [`Block`]: crate::symbols::Marker::Block
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::{prelude::*, widgets::canvas::*};
    ///
    /// Canvas::default()
    ///     .marker(symbols::Marker::Braille)
    ///     .paint(|ctx| {});
    ///
    /// Canvas::default()
    ///     .marker(symbols::Marker::HalfBlock)
    ///     .paint(|ctx| {});
    ///
    /// Canvas::default()
    ///     .marker(symbols::Marker::Dot)
    ///     .paint(|ctx| {});
    ///
    /// Canvas::default()
    ///     .marker(symbols::Marker::Block)
    ///     .paint(|ctx| {});
    /// ```
    pub fn marker(mut self, marker: symbols::Marker) -> Canvas<'a, F> {
        self.marker = marker;
        self
    }
}

impl<F> Widget for Canvas<'_, F>
where
    F: Fn(&mut Context),
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl<F> WidgetRef for Canvas<'_, F>
where
    F: Fn(&mut Context),
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.block.render_ref(area, buf);
        let canvas_area = self.block.inner_if_some(area);
        if canvas_area.is_empty() {
            return;
        }

        buf.set_style(canvas_area, Style::default().bg(self.background_color));

        let width = canvas_area.width as usize;

        let Some(ref painter) = self.paint_func else {
            return;
        };

        // Create a blank context that match the size of the canvas
        let mut ctx = Context::new(
            canvas_area.width,
            canvas_area.height,
            self.x_bounds,
            self.y_bounds,
            self.marker,
        );
        // Paint to this context
        painter(&mut ctx);
        ctx.finish();

        // Retrieve painted points for each layer
        for layer in ctx.layers {
            for (index, (ch, colors)) in layer.string.chars().zip(layer.colors).enumerate() {
                if ch != ' ' && ch != '\u{2800}' {
                    let (x, y) = (
                        (index % width) as u16 + canvas_area.left(),
                        (index / width) as u16 + canvas_area.top(),
                    );
                    let cell = buf.get_mut(x, y).set_char(ch);
                    if colors.0 != Color::Reset {
                        cell.set_fg(colors.0);
                    }
                    if colors.1 != Color::Reset {
                        cell.set_bg(colors.1);
                    }
                }
            }
        }

        // Finally draw the labels
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
            buf.set_line(x, y, &label.line, canvas_area.right() - x);
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::{buffer::Cell, symbols::Marker};

    // helper to test the canvas checks that drawing a vertical and horizontal line
    // results in the expected output
    fn test_marker(marker: Marker, expected: &str) {
        let area = Rect::new(0, 0, 5, 5);
        let mut cell = Cell::default();
        cell.set_char('x');
        let mut buf = Buffer::filled(area, &cell);
        let horizontal_line = Line {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 0.0,
            color: Color::Reset,
        };
        let vertical_line = Line {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 10.0,
            color: Color::Reset,
        };
        Canvas::default()
            .marker(marker)
            .paint(|ctx| {
                ctx.draw(&vertical_line);
                ctx.draw(&horizontal_line);
            })
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .render(area, &mut buf);
        assert_eq!(buf, Buffer::with_lines(expected.lines()));
    }

    #[test]
    fn test_bar_marker() {
        test_marker(
            Marker::Bar,
            indoc!(
                "
                ▄xxxx
                ▄xxxx
                ▄xxxx
                ▄xxxx
                ▄▄▄▄▄"
            ),
        );
    }

    #[test]
    fn test_block_marker() {
        test_marker(
            Marker::Block,
            indoc!(
                "
                █xxxx
                █xxxx
                █xxxx
                █xxxx
                █████"
            ),
        );
    }

    #[test]
    fn test_braille_marker() {
        test_marker(
            Marker::Braille,
            indoc!(
                "
                ⡇xxxx
                ⡇xxxx
                ⡇xxxx
                ⡇xxxx
                ⣇⣀⣀⣀⣀"
            ),
        );
    }

    #[test]
    fn test_dot_marker() {
        test_marker(
            Marker::Dot,
            indoc!(
                "
                •xxxx
                •xxxx
                •xxxx
                •xxxx
                •••••"
            ),
        );
    }

    mod painter {
        use rstest::rstest;

        use crate::symbols::Marker;

        use super::*;

        #[rstest]
        #[case::braille(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::Braille, (20.0, 80.0))]
        #[case::block(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::Block, (10.0, 20.0))]
        #[case::dot(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::Dot, (10.0, 20.0))]
        #[case::bar(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::Bar, (10.0, 20.0))]
        #[case::halfblock(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::HalfBlock, (10.0, 40.0))]
        fn from(
            #[case] width: u16,
            #[case] height: u16,
            #[case] x_bounds: [f64; 2],
            #[case] y_bounds: [f64; 2],
            #[case] marker: Marker,
            #[case] resolution: (f64, f64),
        ) {
            let mut ctx = Context::new(width, height, x_bounds, y_bounds, marker);
            let painter = Painter::from(&mut ctx);
            assert_eq!(painter.resolution, resolution);
        }

        #[rstest]
        #[case(0.0, 0.0, None)]
        #[case(0.0, 1000.0, None)]
        #[case(0.0, 1500.0, None)]
        #[case(0.0, 2000.0, None)]
        #[case[0.0, 3000.0, None]]
        #[case(100.0, 0.0, None)]
        #[case(100.0, 1000.0, Some((0, 79)))]
        #[case(100.0, 1500.0, Some((0, 39)))]
        #[case(100.0, 2000.0, Some((0, 0)))]
        #[case(100.0, 3000.0, None)]
        #[case(150.0, 0.0, None)]
        #[case(150.0, 1000.0, Some((9, 79)))]
        #[case(150.0, 1500.0, Some((9, 39)))]
        #[case(150.0, 2000.0, Some((9, 0)))]
        #[case(150.0, 3000.0, None)]
        #[case(200.0, 0.0, None)]
        #[case(200.0, 1000.0, Some((19, 79)))]
        #[case(200.0, 1500.0, Some((19, 39)))]
        #[case(200.0, 2000.0, Some((19, 0)))]
        #[case(200.0, 3000.0, None)]
        #[case(300.0, 0.0, None)]
        #[case(300.0, 1000.0, None)]
        #[case(300.0, 1500.0, None)]
        #[case(300.0, 2000.0, None)]
        #[case(300.0, 3000.0, None)]
        fn get_point(#[case] x: f64, #[case] y: f64, #[case] expected: Option<(usize, usize)>) {
            let mut ctx = Context::new(10, 20, [100.0, 200.0], [1000.0, 2000.0], Marker::Braille);
            let painter = Painter::from(&mut ctx);
            let point = painter.get_point(x, y);
            assert_eq!(point, expected);
        }

        #[rstest]
        #[case(0.0, 0.0)]
        #[case(10.0, 0.0)]
        #[case(0.0, 20.0)]
        fn get_point_zero_width_or_height(#[case] x_max: f64, #[case] y_max: f64) {
            let mut ctx = Context::new(10, 20, [0.0, x_max], [0.0, y_max], Marker::Braille);
            let painter = Painter::from(&mut ctx);
            let point = painter.get_point(0.0, 0.0);
            assert_eq!(point, None);
        }
    }
}
