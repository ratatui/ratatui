//! Elements related to the `Block` base widget.
//!
//! This holds everything needed to display and configure a [`Block`].
//!
//! In its simplest form, a `Block` is a [border](Borders) around another widget. It can have a
//! [title](Block::title) and [padding](Block::padding).

use itertools::Itertools;
use strum::{Display, EnumString};

use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Styled},
    symbols::border,
    text::Line,
    widgets::{Borders, Widget, WidgetRef},
};

mod padding;
pub mod title;

pub use padding::Padding;
pub use title::{Position, Title};

/// Base widget to be used to display a box border around all [upper level ones](crate::widgets).
///
/// The borders can be configured with [`Block::borders`] and others. A block can have multiple
/// [`Title`] using [`Block::title`]. It can also be [styled](Block::style) and
/// [padded](Block::padding).
///
/// You can call the title methods multiple times to add multiple titles. Each title will be
/// rendered with a single space separating titles that are in the same position or alignment. When
/// both centered and non-centered titles are rendered, the centered space is calculated based on
/// the full width of the block, rather than the leftover width.
///
/// Titles are not rendered in the corners of the block unless there is no border on that edge. If
/// the block is too small and multiple titles overlap, the border may get cut off at a corner.
///
/// ```plain
/// ┌With at least a left border───
///
/// Without left border───
/// ```
/// # Constructor methods
///
/// - [`Block::new`] creates a new [`Block`] with no border or paddings.
/// - [`Block::bordered`] Create a new block with all borders shown.
///
/// # Setter methods
///
/// These methods are fluent setters. They return a new [`Block`] with the specified property set.
///
/// - [`Block::borders`] Defines which borders to display.
/// - [`Block::border_style`] Defines the style of the borders.
/// - [`Block::border_type`] Sets the symbols used to display the border (e.g. single line, double
///   line, thick or rounded borders).
/// - [`Block::padding`] Defines the padding inside a [`Block`].
/// - [`Block::style`] Sets the base style of the widget.
/// - [`Block::title`] Adds a title to the block.
/// - [`Block::title_alignment`] Sets the default [`Alignment`] for all block titles.
/// - [`Block::title_style`] Applies the style to all titles.
/// - [`Block::title_top`] Adds a title to the top of the block.
/// - [`Block::title_bottom`] Adds a title to the bottom of the block.
/// - [`Block::title_position`] Adds a title to the block.
///
/// # Other Methods
/// - [`Block::inner`] Compute the inner area of a block based on its border visibility rules.
///
/// [`Style`]s are applied first to the entire block, then to the borders, and finally to the
/// titles. If the block is used as a container for another widget, the inner widget can also be
/// styled. See [`Style`] for more information on how merging styles works.
///
/// # Examples
///
/// ```
/// use ratatui::{
///     style::{Color, Style},
///     widgets::{Block, BorderType, Borders},
/// };
///
/// Block::new()
///     .border_type(BorderType::Rounded)
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .style(Style::default().bg(Color::Black))
///     .title("Block");
/// ```
///
/// You may also use multiple titles like in the following:
/// ```
/// use ratatui::widgets::{
///     block::{Position, Title},
///     Block,
/// };
///
/// Block::new()
///     .title("Title 1")
///     .title(Title::from("Title 2").position(Position::Bottom));
/// ```
///
/// You can also pass it as parameters of another widget so that the block surrounds them:
/// ```
/// use ratatui::widgets::{Block, Borders, List};
///
/// let surrounding_block = Block::default()
///     .borders(Borders::ALL)
///     .title("Here is a list of items");
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items).block(surrounding_block);
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Block<'a> {
    /// List of titles
    titles: Vec<(Option<Position>, Line<'a>)>,
    /// The style to be patched to all titles of the block
    titles_style: Style,
    /// The default alignment of the titles that don't have one
    titles_alignment: Alignment,
    /// The default position of the titles that don't have one
    titles_position: Position,
    /// Visible borders
    borders: Borders,
    /// Border style
    border_style: Style,
    /// The symbols used to render the border. The default is plain lines but one can choose to
    /// have rounded or doubled lines instead or a custom set of symbols
    border_set: border::Set,
    /// Widget style
    style: Style,
    /// Block padding
    padding: Padding,
}

/// The type of border of a [`Block`].
///
/// See the [`borders`](Block::borders) method of `Block` to configure its borders.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BorderType {
    /// A plain, simple border.
    ///
    /// This is the default
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌───────┐
    /// │       │
    /// └───────┘
    /// ```
    #[default]
    Plain,
    /// A plain border with rounded corners.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╭───────╮
    /// │       │
    /// ╰───────╯
    /// ```
    Rounded,
    /// A doubled border.
    ///
    /// Note this uses one character that draws two lines.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╔═══════╗
    /// ║       ║
    /// ╚═══════╝
    /// ```
    Double,
    /// A thick border.
    ///
    /// # Example
    ///
    /// ```plain
    /// ┏━━━━━━━┓
    /// ┃       ┃
    /// ┗━━━━━━━┛
    /// ```
    Thick,
    /// A border with a single line on the inside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▗▄▄▄▄▄▄▄▖
    /// ▐       ▌
    /// ▐       ▌
    /// ▝▀▀▀▀▀▀▀▘
    QuadrantInside,

    /// A border with a single line on the outside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▛▀▀▀▀▀▀▀▜
    /// ▌       ▐
    /// ▌       ▐
    /// ▙▄▄▄▄▄▄▄▟
    QuadrantOutside,
}

impl<'a> Block<'a> {
    /// Creates a new block with no [`Borders`] or [`Padding`].
    pub const fn new() -> Self {
        Self {
            titles: Vec::new(),
            titles_style: Style::new(),
            titles_alignment: Alignment::Left,
            titles_position: Position::Top,
            borders: Borders::NONE,
            border_style: Style::new(),
            border_set: BorderType::Plain.to_border_set(),
            style: Style::new(),
            padding: Padding::ZERO,
        }
    }

    /// Create a new block with [all borders](Borders::ALL) shown
    ///
    /// ```
    /// use ratatui::widgets::{Block, Borders};
    ///
    /// assert_eq!(Block::bordered(), Block::new().borders(Borders::ALL));
    /// ```
    pub const fn bordered() -> Self {
        let mut block = Self::new();
        block.borders = Borders::ALL;
        block
    }

    /// Adds a title to the block.
    ///
    /// The `title` function allows you to add a title to the block. You can call this function
    /// multiple times to add multiple titles.
    ///
    /// Each title will be rendered with a single space separating titles that are in the same
    /// position or alignment. When both centered and non-centered titles are rendered, the centered
    /// space is calculated based on the full width of the block, rather than the leftover width.
    ///
    /// You can provide any type that can be converted into [`Title`] including: strings, string
    /// slices (`&str`), borrowed strings (`Cow<str>`), [spans](crate::text::Span), or vectors of
    /// [spans](crate::text::Span) (`Vec<Span>`).
    ///
    /// By default, the titles will avoid being rendered in the corners of the block but will align
    /// against the left or right edge of the block if there is no border on that edge. The
    /// following demonstrates this behavior, notice the second title is one character off to the
    /// left.
    ///
    /// ```plain
    /// ┌With at least a left border───
    ///
    /// Without left border───
    /// ```
    ///
    /// Note: If the block is too small and multiple titles overlap, the border might get cut off at
    /// a corner.
    ///
    /// # Examples
    ///
    /// See the [Block example] for a visual representation of how the various borders and styles
    /// look when rendered.
    ///
    /// The following example demonstrates:
    /// - Default title alignment
    /// - Multiple titles (notice "Center" is centered according to the full with of the block, not
    ///   the leftover space)
    /// - Two titles with the same alignment (notice the left titles are separated)
    /// ```
    /// use ratatui::{
    ///     text::Line,
    ///     widgets::{Block, Borders},
    /// };
    ///
    /// Block::new()
    ///     .title("Title") // By default in the top left corner
    ///     .title(Line::from("Left").left_aligned()) // also on the left
    ///     .title(Line::from("Right").right_aligned())
    ///     .title(Line::from("Center").centered());
    /// // Renders
    /// // ┌Title─Left────Center─────────Right┐
    /// ```
    ///
    /// # See also
    ///
    /// Titles attached to a block can have default behaviors. See
    /// - [`Block::title_style`]
    /// - [`Block::title_alignment`]
    /// - [`Block::title_position`]
    ///
    /// # Future improvements
    ///
    /// In a future release of Ratatui this method will be changed to accept `Into<Line>` instead of
    /// `Into<Title>`. This allows us to remove the unnecessary `Title` struct and store the
    /// position in the block itself. For more information see
    /// <https://github.com/ratatui/ratatui/issues/738>.
    ///
    /// [Block example]: https://github.com/ratatui/ratatui/blob/main/examples/README.md#block
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title<'a>>,
    {
        let title = title.into();
        let position = title.position;
        let mut content = title.content;
        if let Some(alignment) = title.alignment {
            content = content.alignment(alignment);
        }
        self.titles.push((position, content));
        self
    }

    /// Adds a title to the top of the block.
    ///
    /// You can provide any type that can be converted into [`Line`] including: strings, string
    /// slices (`&str`), borrowed strings (`Cow<str>`), [spans](crate::text::Span), or vectors of
    /// [spans](crate::text::Span) (`Vec<Span>`).
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::{ widgets::Block, text::Line };
    ///
    /// Block::bordered()
    ///     .title_top("Left1") // By default in the top left corner
    ///     .title_top(Line::from("Left2").left_aligned())
    ///     .title_top(Line::from("Right").right_aligned())
    ///     .title_top(Line::from("Center").centered());
    ///
    /// // Renders
    /// // ┌Left1─Left2───Center─────────Right┐
    /// // │                                  │
    /// // └──────────────────────────────────┘
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn title_top<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        let line = title.into();
        self.titles.push((Some(Position::Top), line));
        self
    }

    /// Adds a title to the bottom of the block.
    ///
    /// You can provide any type that can be converted into [`Line`] including: strings, string
    /// slices (`&str`), borrowed strings (`Cow<str>`), [spans](crate::text::Span), or vectors of
    /// [spans](crate::text::Span) (`Vec<Span>`).
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::{ widgets::Block, text::Line };
    ///
    /// Block::bordered()
    ///     .title_bottom("Left1") // By default in the top left corner
    ///     .title_bottom(Line::from("Left2").left_aligned())
    ///     .title_bottom(Line::from("Right").right_aligned())
    ///     .title_bottom(Line::from("Center").centered());
    ///
    /// // Renders
    /// // ┌──────────────────────────────────┐
    /// // │                                  │
    /// // └Left1─Left2───Center─────────Right┘
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn title_bottom<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        let line = title.into();
        self.titles.push((Some(Position::Bottom), line));
        self
    }

    /// Applies the style to all titles.
    ///
    /// This style will be applied to all titles of the block. If a title has a style set, it will
    /// be applied after this style. This style will be applied after any [`Block::style`] or
    /// [`Block::border_style`] is applied.
    ///
    /// See [`Style`] for more information on how merging styles works.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn title_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.titles_style = style.into();
        self
    }

    /// Sets the default [`Alignment`] for all block titles.
    ///
    /// Titles that explicitly set an [`Alignment`] will ignore this.
    ///
    /// # Example
    ///
    /// This example aligns all titles in the center except the "right" title which explicitly sets
    /// [`Alignment::Right`].
    /// ```
    /// use ratatui::{layout::Alignment, text::Line, widgets::Block};
    ///
    /// Block::new()
    ///     .title_alignment(Alignment::Center)
    ///     // This title won't be aligned in the center
    ///     .title(Line::from("right").right_aligned())
    ///     .title("foo")
    ///     .title("bar");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn title_alignment(mut self, alignment: Alignment) -> Self {
        self.titles_alignment = alignment;
        self
    }

    /// Sets the default [`Position`] for all block [titles](Title).
    ///
    /// Titles that explicitly set a [`Position`] will ignore this.
    ///
    /// # Example
    ///
    /// This example positions all titles on the bottom except the "top" title which explicitly sets
    /// [`Position::Top`].
    /// ```
    /// use ratatui::widgets::{block::Position, Block};
    ///
    /// Block::new()
    ///     .title_position(Position::Bottom)
    ///     // This title won't be aligned in the center
    ///     .title_top("top")
    ///     .title("foo")
    ///     .title("bar");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn title_position(mut self, position: Position) -> Self {
        self.titles_position = position;
        self
    }

    /// Defines the style of the borders.
    ///
    /// This style is applied only to the areas covered by borders, and is applied to the block
    /// after any [`Block::style`] is applied.
    ///
    /// See [`Style`] for more information on how merging styles works.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Example
    ///
    /// This example shows a `Block` with blue borders.
    /// ```
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     widgets::Block,
    /// };
    /// Block::bordered().border_style(Style::new().blue());
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.border_style = style.into();
        self
    }

    /// Defines the style of the entire block.
    ///
    /// This is the most generic [`Style`] a block can receive, it will be merged with any other
    /// more specific styles. Elements can be styled further with [`Block::title_style`] and
    /// [`Block::border_style`], which will be applied on top of this style. If the block is used as
    /// a container for another widget (e.g. a [`Paragraph`]), then the style of the widget is
    /// generally applied before this style.
    ///
    /// See [`Style`] for more information on how merging styles works.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::{
    ///     style::{Color, Style, Stylize},
    ///     widgets::{Block, Paragraph},
    /// };
    ///
    /// let block = Block::new().style(Style::new().red().on_black());
    ///
    /// // For border and title you can additionally apply styles on top of the block level style.
    /// let block = Block::new()
    ///     .style(Style::new().red().bold().italic())
    ///     .border_style(Style::new().not_italic()) // will be red and bold
    ///     .title_style(Style::new().not_bold()) // will be red and italic
    ///     .title("Title");
    ///
    /// // To style the inner widget, you can style the widget itself.
    /// let paragraph = Paragraph::new("Content")
    ///     .block(block)
    ///     .style(Style::new().white().not_bold()); // will be white, and italic
    /// ```
    ///
    /// [`Paragraph`]: crate::widgets::Paragraph
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Defines which borders to display.
    ///
    /// [`Borders`] can also be styled with [`Block::border_style`] and [`Block::border_type`].
    ///
    /// # Examples
    ///
    /// Display left and right borders.
    /// ```
    /// use ratatui::widgets::{Block, Borders};
    /// Block::new().borders(Borders::LEFT | Borders::RIGHT);
    /// ```
    ///
    /// To show all borders you can abbreviate this with [`Block::bordered`]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn borders(mut self, flag: Borders) -> Self {
        self.borders = flag;
        self
    }

    /// Sets the symbols used to display the border (e.g. single line, double line, thick or
    /// rounded borders).
    ///
    /// Setting this overwrites any custom [`border_set`](Block::border_set) that was set.
    ///
    /// See [`BorderType`] for the full list of available symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{Block, BorderType};
    /// Block::bordered()
    ///     .border_type(BorderType::Rounded)
    ///     .title("Block");
    /// // Renders
    /// // ╭Block╮
    /// // │     │
    /// // ╰─────╯
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_set = border_type.to_border_set();
        self
    }

    /// Sets the symbols used to display the border as a [`crate::symbols::border::Set`].
    ///
    /// Setting this overwrites any [`border_type`](Block::border_type) that was set.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::{widgets::Block, symbols};
    ///
    /// Block::bordered().border_set(symbols::border::DOUBLE).title("Block");
    /// // Renders
    /// // ╔Block╗
    /// // ║     ║
    /// // ╚═════╝
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn border_set(mut self, border_set: border::Set) -> Self {
        self.border_set = border_set;
        self
    }

    /// Defines the padding inside a `Block`.
    ///
    /// See [`Padding`] for more information.
    ///
    /// # Examples
    ///
    /// This renders a `Block` with no padding (the default).
    /// ```
    /// use ratatui::widgets::{Block, Padding};
    ///
    /// Block::bordered().padding(Padding::ZERO);
    /// // Renders
    /// // ┌───────┐
    /// // │content│
    /// // └───────┘
    /// ```
    ///
    /// This example shows a `Block` with padding left and right ([`Padding::horizontal`]).
    /// Notice the two spaces before and after the content.
    /// ```
    /// use ratatui::widgets::{Block, Padding};
    ///
    /// Block::bordered().padding(Padding::horizontal(2));
    /// // Renders
    /// // ┌───────────┐
    /// // │  content  │
    /// // └───────────┘
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
    ///
    /// # Examples
    ///
    /// Draw a block nested within another block
    /// ```
    /// use ratatui::{widgets::Block, Frame};
    ///
    /// # fn render_nested_block(frame: &mut Frame) {
    /// let outer_block = Block::bordered().title("Outer");
    /// let inner_block = Block::bordered().title("Inner");
    ///
    /// let outer_area = frame.area();
    /// let inner_area = outer_block.inner(outer_area);
    ///
    /// frame.render_widget(outer_block, outer_area);
    /// frame.render_widget(inner_block, inner_area);
    /// # }
    /// // Renders
    /// // ┌Outer────────┐
    /// // │┌Inner──────┐│
    /// // ││           ││
    /// // │└───────────┘│
    /// // └─────────────┘
    /// ```
    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = area;
        if self.borders.intersects(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.width = inner.width.saturating_sub(1);
        }
        if self.borders.intersects(Borders::TOP) || self.has_title_at_position(Position::Top) {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.height = inner.height.saturating_sub(1);
        }
        if self.borders.intersects(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }
        if self.borders.intersects(Borders::BOTTOM) || self.has_title_at_position(Position::Bottom)
        {
            inner.height = inner.height.saturating_sub(1);
        }

        inner.x = inner.x.saturating_add(self.padding.left);
        inner.y = inner.y.saturating_add(self.padding.top);

        inner.width = inner
            .width
            .saturating_sub(self.padding.left + self.padding.right);
        inner.height = inner
            .height
            .saturating_sub(self.padding.top + self.padding.bottom);

        inner
    }

    fn has_title_at_position(&self, position: Position) -> bool {
        self.titles
            .iter()
            .any(|(pos, _)| pos.unwrap_or(self.titles_position) == position)
    }
}

impl BorderType {
    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn border_symbols(border_type: Self) -> border::Set {
        match border_type {
            Self::Plain => border::PLAIN,
            Self::Rounded => border::ROUNDED,
            Self::Double => border::DOUBLE,
            Self::Thick => border::THICK,
            Self::QuadrantInside => border::QUADRANT_INSIDE,
            Self::QuadrantOutside => border::QUADRANT_OUTSIDE,
        }
    }

    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn to_border_set(self) -> border::Set {
        Self::border_symbols(self)
    }
}

impl Widget for Block<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Block<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        buf.set_style(area, self.style);
        self.render_borders(area, buf);
        self.render_titles(area, buf);
    }
}

impl Block<'_> {
    fn render_borders(&self, area: Rect, buf: &mut Buffer) {
        self.render_left_side(area, buf);
        self.render_top_side(area, buf);
        self.render_right_side(area, buf);
        self.render_bottom_side(area, buf);

        self.render_bottom_right_corner(buf, area);
        self.render_top_right_corner(buf, area);
        self.render_bottom_left_corner(buf, area);
        self.render_top_left_corner(buf, area);
    }

    fn render_titles(&self, area: Rect, buf: &mut Buffer) {
        self.render_title_position(Position::Top, area, buf);
        self.render_title_position(Position::Bottom, area, buf);
    }

    fn render_title_position(&self, position: Position, area: Rect, buf: &mut Buffer) {
        // NOTE: the order in which these functions are called defines the overlapping behavior
        self.render_right_titles(position, area, buf);
        self.render_center_titles(position, area, buf);
        self.render_left_titles(position, area, buf);
    }

    fn render_left_side(&self, area: Rect, buf: &mut Buffer) {
        if self.borders.contains(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf[(area.left(), y)]
                    .set_symbol(self.border_set.vertical_left)
                    .set_style(self.border_style);
            }
        }
    }

    fn render_top_side(&self, area: Rect, buf: &mut Buffer) {
        if self.borders.contains(Borders::TOP) {
            for x in area.left()..area.right() {
                buf[(x, area.top())]
                    .set_symbol(self.border_set.horizontal_top)
                    .set_style(self.border_style);
            }
        }
    }

    fn render_right_side(&self, area: Rect, buf: &mut Buffer) {
        if self.borders.contains(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf[(x, y)]
                    .set_symbol(self.border_set.vertical_right)
                    .set_style(self.border_style);
            }
        }
    }

    fn render_bottom_side(&self, area: Rect, buf: &mut Buffer) {
        if self.borders.contains(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf[(x, y)]
                    .set_symbol(self.border_set.horizontal_bottom)
                    .set_style(self.border_style);
            }
        }
    }

    fn render_bottom_right_corner(&self, buf: &mut Buffer, area: Rect) {
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf[(area.right() - 1, area.bottom() - 1)]
                .set_symbol(self.border_set.bottom_right)
                .set_style(self.border_style);
        }
    }

    fn render_top_right_corner(&self, buf: &mut Buffer, area: Rect) {
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf[(area.right() - 1, area.top())]
                .set_symbol(self.border_set.top_right)
                .set_style(self.border_style);
        }
    }

    fn render_bottom_left_corner(&self, buf: &mut Buffer, area: Rect) {
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf[(area.left(), area.bottom() - 1)]
                .set_symbol(self.border_set.bottom_left)
                .set_style(self.border_style);
        }
    }

    fn render_top_left_corner(&self, buf: &mut Buffer, area: Rect) {
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf[(area.left(), area.top())]
                .set_symbol(self.border_set.top_left)
                .set_style(self.border_style);
        }
    }

    /// Render titles aligned to the right of the block
    ///
    /// Currently (due to the way lines are truncated), the right side of the leftmost title will
    /// be cut off if the block is too small to fit all titles. This is not ideal and should be
    /// the left side of that leftmost that is cut off. This is due to the line being truncated
    /// incorrectly. See <https://github.com/ratatui/ratatui/issues/932>
    #[allow(clippy::similar_names)]
    fn render_right_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let titles = self.filtered_titles(position, Alignment::Right);
        let mut titles_area = self.titles_area(area, position);

        // render titles in reverse order to align them to the right
        for title in titles.rev() {
            if titles_area.is_empty() {
                break;
            }
            let title_width = title.width() as u16;
            let title_area = Rect {
                x: titles_area
                    .right()
                    .saturating_sub(title_width)
                    .max(titles_area.left()),
                width: title_width.min(titles_area.width),
                ..titles_area
            };
            buf.set_style(title_area, self.titles_style);
            title.render_ref(title_area, buf);

            // bump the width of the titles area to the left
            titles_area.width = titles_area
                .width
                .saturating_sub(title_width)
                .saturating_sub(1); // space between titles
        }
    }

    /// Render titles in the center of the block
    ///
    /// Currently this method aligns the titles to the left inside a centered area. This is not
    /// ideal and should be fixed in the future to align the titles to the center of the block and
    /// truncate both sides of the titles if the block is too small to fit all titles.
    #[allow(clippy::similar_names)]
    fn render_center_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let titles = self
            .filtered_titles(position, Alignment::Center)
            .collect_vec();
        let total_width = titles
            .iter()
            .map(|title| title.width() as u16 + 1) // space between titles
            .sum::<u16>()
            .saturating_sub(1); // no space for the last title

        let titles_area = self.titles_area(area, position);
        let mut titles_area = Rect {
            x: titles_area.left() + (titles_area.width.saturating_sub(total_width) / 2),
            ..titles_area
        };
        for title in titles {
            if titles_area.is_empty() {
                break;
            }
            let title_width = title.width() as u16;
            let title_area = Rect {
                width: title_width.min(titles_area.width),
                ..titles_area
            };
            buf.set_style(title_area, self.titles_style);
            title.render_ref(title_area, buf);

            // bump the titles area to the right and reduce its width
            titles_area.x = titles_area.x.saturating_add(title_width + 1);
            titles_area.width = titles_area.width.saturating_sub(title_width + 1);
        }
    }

    /// Render titles aligned to the left of the block
    #[allow(clippy::similar_names)]
    fn render_left_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let titles = self.filtered_titles(position, Alignment::Left);
        let mut titles_area = self.titles_area(area, position);
        for title in titles {
            if titles_area.is_empty() {
                break;
            }
            let title_width = title.width() as u16;
            let title_area = Rect {
                width: title_width.min(titles_area.width),
                ..titles_area
            };
            buf.set_style(title_area, self.titles_style);
            title.render_ref(title_area, buf);

            // bump the titles area to the right and reduce its width
            titles_area.x = titles_area.x.saturating_add(title_width + 1);
            titles_area.width = titles_area.width.saturating_sub(title_width + 1);
        }
    }

    /// An iterator over the titles that match the position and alignment
    fn filtered_titles(
        &self,
        position: Position,
        alignment: Alignment,
    ) -> impl DoubleEndedIterator<Item = &Line> {
        self.titles
            .iter()
            .filter(move |(pos, _)| pos.unwrap_or(self.titles_position) == position)
            .filter(move |(_, line)| line.alignment.unwrap_or(self.titles_alignment) == alignment)
            .map(|(_, line)| line)
    }

    /// An area that is one line tall and spans the width of the block excluding the borders and
    /// is positioned at the top or bottom of the block.
    fn titles_area(&self, area: Rect, position: Position) -> Rect {
        let left_border = u16::from(self.borders.contains(Borders::LEFT));
        let right_border = u16::from(self.borders.contains(Borders::RIGHT));
        Rect {
            x: area.left() + left_border,
            y: match position {
                Position::Top => area.top(),
                Position::Bottom => area.bottom() - 1,
            },
            width: area
                .width
                .saturating_sub(left_border)
                .saturating_sub(right_border),
            height: 1,
        }
    }

    /// Calculate the left, and right space the [`Block`] will take up.
    ///
    /// The result takes the [`Block`]'s, [`Borders`], and [`Padding`] into account.
    pub(crate) fn horizontal_space(&self) -> (u16, u16) {
        let left = self
            .padding
            .left
            .saturating_add(u16::from(self.borders.contains(Borders::LEFT)));
        let right = self
            .padding
            .right
            .saturating_add(u16::from(self.borders.contains(Borders::RIGHT)));
        (left, right)
    }

    /// Calculate the top, and bottom space that the [`Block`] will take up.
    ///
    /// Takes the [`Padding`], [`Title`]'s position, and the [`Borders`] that are selected into
    /// account when calculating the result.
    pub(crate) fn vertical_space(&self) -> (u16, u16) {
        let has_top =
            self.borders.contains(Borders::TOP) || self.has_title_at_position(Position::Top);
        let top = self.padding.top + u16::from(has_top);
        let has_bottom =
            self.borders.contains(Borders::BOTTOM) || self.has_title_at_position(Position::Bottom);
        let bottom = self.padding.bottom + u16::from(has_bottom);
        (top, bottom)
    }
}

/// An extension trait for [`Block`] that provides some convenience methods.
///
/// This is implemented for [`Option<Block>`](Option) to simplify the common case of having a
/// widget with an optional block.
pub trait BlockExt {
    /// Return the inner area of the block if it is `Some`. Otherwise, returns `area`.
    ///
    /// This is a useful convenience method for widgets that have an `Option<Block>` field
    fn inner_if_some(&self, area: Rect) -> Rect;
}

impl BlockExt for Option<Block<'_>> {
    fn inner_if_some(&self, area: Rect) -> Rect {
        self.as_ref().map_or(area, |block| block.inner(area))
    }
}

impl<'a> Styled for Block<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use strum::ParseError;

    use super::*;
    use crate::style::{Color, Modifier, Stylize};

    #[test]
    fn create_with_all_borders() {
        let block = Block::bordered();
        assert_eq!(block.borders, Borders::all());
    }

    #[rstest]
    #[case::none_0(Borders::NONE, Rect::ZERO, Rect::ZERO)]
    #[case::none_1(Borders::NONE, Rect::new(0, 0, 1, 1), Rect::new(0, 0, 1, 1))]
    #[case::left_0(Borders::LEFT, Rect::ZERO, Rect::ZERO)]
    #[case::left_w1(Borders::LEFT, Rect::new(0, 0, 0, 1), Rect::new(0, 0, 0, 1))]
    #[case::left_w2(Borders::LEFT, Rect::new(0, 0, 1, 1), Rect::new(1, 0, 0, 1))]
    #[case::left_w3(Borders::LEFT, Rect::new(0, 0, 2, 1), Rect::new(1, 0, 1, 1))]
    #[case::top_0(Borders::TOP, Rect::ZERO, Rect::ZERO)]
    #[case::top_h1(Borders::TOP, Rect::new(0, 0, 1, 0), Rect::new(0, 0, 1, 0))]
    #[case::top_h2(Borders::TOP, Rect::new(0, 0, 1, 1), Rect::new(0, 1, 1, 0))]
    #[case::top_h3(Borders::TOP, Rect::new(0, 0, 1, 2), Rect::new(0, 1, 1, 1))]
    #[case::right_0(Borders::RIGHT, Rect::ZERO, Rect::ZERO)]
    #[case::right_w1(Borders::RIGHT, Rect::new(0, 0, 0, 1), Rect::new(0, 0, 0, 1))]
    #[case::right_w2(Borders::RIGHT, Rect::new(0, 0, 1, 1), Rect::new(0, 0, 0, 1))]
    #[case::right_w3(Borders::RIGHT, Rect::new(0, 0, 2, 1), Rect::new(0, 0, 1, 1))]
    #[case::bottom_0(Borders::BOTTOM, Rect::ZERO, Rect::ZERO)]
    #[case::bottom_h1(Borders::BOTTOM, Rect::new(0, 0, 1, 0), Rect::new(0, 0, 1, 0))]
    #[case::bottom_h2(Borders::BOTTOM, Rect::new(0, 0, 1, 1), Rect::new(0, 0, 1, 0))]
    #[case::bottom_h3(Borders::BOTTOM, Rect::new(0, 0, 1, 2), Rect::new(0, 0, 1, 1))]
    #[case::all_0(Borders::ALL, Rect::ZERO, Rect::ZERO)]
    #[case::all_1(Borders::ALL, Rect::new(0, 0, 1, 1), Rect::new(1, 1, 0, 0))]
    #[case::all_2(Borders::ALL, Rect::new(0, 0, 2, 2), Rect::new(1, 1, 0, 0))]
    #[case::all_3(Borders::ALL, Rect::new(0, 0, 3, 3), Rect::new(1, 1, 1, 1))]
    fn inner_takes_into_account_the_borders(
        #[case] borders: Borders,
        #[case] area: Rect,
        #[case] expected: Rect,
    ) {
        let block = Block::new().borders(borders);
        assert_eq!(block.inner(area), expected);
    }

    #[rstest]
    #[case::left(Alignment::Left)]
    #[case::center(Alignment::Center)]
    #[case::right(Alignment::Right)]
    fn inner_takes_into_account_the_title(#[case] alignment: Alignment) {
        let area = Rect::new(0, 0, 0, 1);
        let expected = Rect::new(0, 1, 0, 0);

        let block = Block::new().title(Line::from("Test").alignment(alignment));
        assert_eq!(block.inner(area), expected);
    }

    #[rstest]
    #[case::top_top(Block::new().title_top("Test").borders(Borders::TOP), Rect::new(0, 1, 0, 1))]
    #[case::top_bot(Block::new().title_top("Test").borders(Borders::BOTTOM), Rect::new(0, 1, 0, 0))]
    #[case::bot_top(Block::new().title_bottom("Test").borders(Borders::TOP), Rect::new(0, 1, 0, 0))]
    #[case::bot_bot(Block::new().title_bottom("Test").borders(Borders::BOTTOM), Rect::new(0, 0, 0, 1))]
    fn inner_takes_into_account_border_and_title(#[case] block: Block, #[case] expected: Rect) {
        let area = Rect::new(0, 0, 0, 2);
        assert_eq!(block.inner(area), expected);
    }

    #[test]
    fn has_title_at_position_takes_into_account_all_positioning_declarations() {
        let block = Block::new();
        assert!(!block.has_title_at_position(Position::Top));
        assert!(!block.has_title_at_position(Position::Bottom));

        let block = Block::new().title_top("test");
        assert!(block.has_title_at_position(Position::Top));
        assert!(!block.has_title_at_position(Position::Bottom));

        let block = Block::new().title_bottom("test");
        assert!(!block.has_title_at_position(Position::Top));
        assert!(block.has_title_at_position(Position::Bottom));

        #[allow(deprecated)] // until Title is removed
        let block = Block::new()
            .title(Title::from("Test").position(Position::Top))
            .title_position(Position::Bottom);
        assert!(block.has_title_at_position(Position::Top));
        assert!(!block.has_title_at_position(Position::Bottom));

        #[allow(deprecated)] // until Title is removed
        let block = Block::new()
            .title(Title::from("Test").position(Position::Bottom))
            .title_position(Position::Top);
        assert!(!block.has_title_at_position(Position::Top));
        assert!(block.has_title_at_position(Position::Bottom));

        let block = Block::new().title_top("test").title_bottom("test");
        assert!(block.has_title_at_position(Position::Top));
        assert!(block.has_title_at_position(Position::Bottom));

        #[allow(deprecated)] // until Title is removed
        let block = Block::new()
            .title(Title::from("Test").position(Position::Top))
            .title(Title::from("Test"))
            .title_position(Position::Bottom);
        assert!(block.has_title_at_position(Position::Top));
        assert!(block.has_title_at_position(Position::Bottom));

        #[allow(deprecated)] // until Title is removed
        let block = Block::new()
            .title(Title::from("Test"))
            .title(Title::from("Test").position(Position::Bottom))
            .title_position(Position::Top);
        assert!(block.has_title_at_position(Position::Top));
        assert!(block.has_title_at_position(Position::Bottom));
    }

    #[rstest]
    #[case::none(Borders::NONE, (0, 0))]
    #[case::top(Borders::TOP, (1, 0))]
    #[case::right(Borders::RIGHT, (0, 0))]
    #[case::bottom(Borders::BOTTOM, (0, 1))]
    #[case::left(Borders::LEFT, (0, 0))]
    #[case::top_right(Borders::TOP | Borders::RIGHT, (1, 0))]
    #[case::top_bottom(Borders::TOP | Borders::BOTTOM, (1, 1))]
    #[case::top_left(Borders::TOP | Borders::LEFT, (1, 0))]
    #[case::bottom_right(Borders::BOTTOM | Borders::RIGHT, (0, 1))]
    #[case::bottom_left(Borders::BOTTOM | Borders::LEFT, (0, 1))]
    #[case::left_right(Borders::LEFT | Borders::RIGHT, (0, 0))]
    fn vertical_space_takes_into_account_borders(
        #[case] borders: Borders,
        #[case] vertical_space: (u16, u16),
    ) {
        let block = Block::new().borders(borders);
        assert_eq!(block.vertical_space(), vertical_space);
    }

    #[rstest]
    #[case::top_border_top_p1(Borders::TOP, Padding::new(0, 0, 1, 0), (2, 0))]
    #[case::right_border_top_p1(Borders::RIGHT, Padding::new(0, 0, 1, 0), (1, 0))]
    #[case::bottom_border_top_p1(Borders::BOTTOM, Padding::new(0, 0, 1, 0), (1, 1))]
    #[case::left_border_top_p1(Borders::LEFT, Padding::new(0, 0, 1, 0), (1, 0))]
    #[case::top_bottom_border_all_p3(Borders::TOP | Borders::BOTTOM, Padding::new(100, 100, 4, 5), (5, 6))]
    #[case::no_border(Borders::NONE, Padding::new(100, 100, 10, 13), (10, 13))]
    #[case::all(Borders::ALL, Padding::new(100, 100, 1, 3), (2, 4))]
    fn vertical_space_takes_into_account_padding(
        #[case] borders: Borders,
        #[case] padding: Padding,
        #[case] vertical_space: (u16, u16),
    ) {
        let block = Block::new().borders(borders).padding(padding);
        assert_eq!(block.vertical_space(), vertical_space);
    }

    #[test]
    fn vertical_space_takes_into_account_titles() {
        let block = Block::new().title_top("Test");
        assert_eq!(block.vertical_space(), (1, 0));

        let block = Block::new().title_bottom("Test");
        assert_eq!(block.vertical_space(), (0, 1));
    }

    #[rstest]
    #[case::top_border_top_title(Block::new(), Borders::TOP, Position::Top, (1, 0))]
    #[case::right_border_top_title(Block::new(), Borders::RIGHT, Position::Top, (1, 0))]
    #[case::bottom_border_top_title(Block::new(), Borders::BOTTOM, Position::Top, (1, 1))]
    #[case::left_border_top_title(Block::new(), Borders::LEFT, Position::Top, (1, 0))]
    #[case::top_border_top_title(Block::new(), Borders::TOP, Position::Bottom, (1, 1))]
    #[case::right_border_top_title(Block::new(), Borders::RIGHT, Position::Bottom, (0, 1))]
    #[case::bottom_border_top_title(Block::new(), Borders::BOTTOM, Position::Bottom, (0, 1))]
    #[case::left_border_top_title(Block::new(), Borders::LEFT, Position::Bottom, (0, 1))]
    fn vertical_space_takes_into_account_borders_and_title(
        #[case] block: Block,
        #[case] borders: Borders,
        #[case] pos: Position,
        #[case] vertical_space: (u16, u16),
    ) {
        let block = block.borders(borders).title_position(pos).title("Test");
        assert_eq!(block.vertical_space(), vertical_space);
    }

    #[test]
    fn horizontal_space_takes_into_account_borders() {
        let block = Block::bordered();
        assert_eq!(block.horizontal_space(), (1, 1));

        let block = Block::new().borders(Borders::LEFT);
        assert_eq!(block.horizontal_space(), (1, 0));

        let block = Block::new().borders(Borders::RIGHT);
        assert_eq!(block.horizontal_space(), (0, 1));
    }

    #[test]
    fn horizontal_space_takes_into_account_padding() {
        let block = Block::new().padding(Padding::new(1, 1, 100, 100));
        assert_eq!(block.horizontal_space(), (1, 1));

        let block = Block::new().padding(Padding::new(3, 5, 0, 0));
        assert_eq!(block.horizontal_space(), (3, 5));

        let block = Block::new().padding(Padding::new(0, 1, 100, 100));
        assert_eq!(block.horizontal_space(), (0, 1));

        let block = Block::new().padding(Padding::new(1, 0, 100, 100));
        assert_eq!(block.horizontal_space(), (1, 0));
    }

    #[rstest]
    #[case::all_bordered_all_padded(Block::bordered(), Padding::new(1, 1, 1, 1), (2, 2))]
    #[case::all_bordered_left_padded(Block::bordered(), Padding::new(1, 0, 0, 0), (2, 1))]
    #[case::all_bordered_right_padded(Block::bordered(), Padding::new(0, 1, 0, 0), (1, 2))]
    #[case::all_bordered_top_padded(Block::bordered(), Padding::new(0, 0, 1, 0), (1, 1))]
    #[case::all_bordered_bottom_padded(Block::bordered(), Padding::new(0, 0, 0, 1), (1, 1))]
    #[case::left_bordered_left_padded(Block::new().borders(Borders::LEFT), Padding::new(1, 0, 0, 0), (2, 0))]
    #[case::left_bordered_right_padded(Block::new().borders(Borders::LEFT), Padding::new(0, 1, 0, 0), (1, 1))]
    #[case::right_bordered_right_padded(Block::new().borders(Borders::RIGHT), Padding::new(0, 1, 0, 0), (0, 2))]
    #[case::right_bordered_left_padded(Block::new().borders(Borders::RIGHT), Padding::new(1, 0, 0, 0), (1, 1))]
    fn horizontal_space_takes_into_account_borders_and_padding(
        #[case] block: Block,
        #[case] padding: Padding,
        #[case] horizontal_space: (u16, u16),
    ) {
        let block = block.padding(padding);
        assert_eq!(block.horizontal_space(), horizontal_space);
    }

    #[test]
    const fn border_type_can_be_const() {
        const _PLAIN: border::Set = BorderType::border_symbols(BorderType::Plain);
    }

    #[test]
    fn block_new() {
        assert_eq!(
            Block::new(),
            Block {
                titles: Vec::new(),
                titles_style: Style::new(),
                titles_alignment: Alignment::Left,
                titles_position: Position::Top,
                borders: Borders::NONE,
                border_style: Style::new(),
                border_set: BorderType::Plain.to_border_set(),
                style: Style::new(),
                padding: Padding::ZERO,
            }
        );
    }

    #[test]
    const fn block_can_be_const() {
        const _DEFAULT_STYLE: Style = Style::new();
        const _DEFAULT_PADDING: Padding = Padding::uniform(1);
        const _DEFAULT_BLOCK: Block = Block::bordered()
            // the following methods are no longer const because they use Into<Style>
            // .style(_DEFAULT_STYLE)           // no longer const
            // .border_style(_DEFAULT_STYLE)    // no longer const
            // .title_style(_DEFAULT_STYLE)     // no longer const
            .title_alignment(Alignment::Left)
            .title_position(Position::Top)
            .padding(_DEFAULT_PADDING);
    }

    /// Ensure Style from/into works the way a user would use it.
    #[test]
    fn style_into_works_from_user_view() {
        // nominal style
        let block = Block::new().style(Style::new().red());
        assert_eq!(block.style, Style::new().red());

        // auto-convert from Color
        let block = Block::new().style(Color::Red);
        assert_eq!(block.style, Style::new().red());

        // auto-convert from (Color, Color)
        let block = Block::new().style((Color::Red, Color::Blue));
        assert_eq!(block.style, Style::new().red().on_blue());

        // auto-convert from Modifier
        let block = Block::new().style(Modifier::BOLD | Modifier::ITALIC);
        assert_eq!(block.style, Style::new().bold().italic());

        // auto-convert from (Modifier, Modifier)
        let block = Block::new().style((Modifier::BOLD | Modifier::ITALIC, Modifier::DIM));
        assert_eq!(block.style, Style::new().bold().italic().not_dim());

        // auto-convert from (Color, Modifier)
        let block = Block::new().style((Color::Red, Modifier::BOLD));
        assert_eq!(block.style, Style::new().red().bold());

        // auto-convert from (Color, Color, Modifier)
        let block = Block::new().style((Color::Red, Color::Blue, Modifier::BOLD));
        assert_eq!(block.style, Style::new().red().on_blue().bold());

        // auto-convert from (Color, Color, Modifier, Modifier)
        let block = Block::new().style((
            Color::Red,
            Color::Blue,
            Modifier::BOLD | Modifier::ITALIC,
            Modifier::DIM,
        ));
        assert_eq!(
            block.style,
            Style::new().red().on_blue().bold().italic().not_dim()
        );
    }

    #[test]
    fn can_be_stylized() {
        let block = Block::new().black().on_white().bold().not_dim();
        assert_eq!(
            block.style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn title() {
        use Alignment::*;
        use Position::*;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 11, 3));
        #[allow(deprecated)] // until Title is removed
        Block::bordered()
            .title(Title::from("A").position(Top).alignment(Left))
            .title(Title::from("B").position(Top).alignment(Center))
            .title(Title::from("C").position(Top).alignment(Right))
            .title(Title::from("D").position(Bottom).alignment(Left))
            .title(Title::from("E").position(Bottom).alignment(Center))
            .title(Title::from("F").position(Bottom).alignment(Right))
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌A───B───C┐",
            "│         │",
            "└D───E───F┘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn title_top_bottom() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 11, 3));
        Block::bordered()
            .title_top(Line::raw("A").left_aligned())
            .title_top(Line::raw("B").centered())
            .title_top(Line::raw("C").right_aligned())
            .title_bottom(Line::raw("D").left_aligned())
            .title_bottom(Line::raw("E").centered())
            .title_bottom(Line::raw("F").right_aligned())
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌A───B───C┐",
            "│         │",
            "└D───E───F┘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn title_alignment() {
        let tests = vec![
            (Alignment::Left, "test    "),
            (Alignment::Center, "  test  "),
            (Alignment::Right, "    test"),
        ];
        for (alignment, expected) in tests {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 1));
            Block::new()
                .title_alignment(alignment)
                .title("test")
                .render(buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines([expected]));
        }
    }

    #[test]
    fn title_alignment_overrides_block_title_alignment() {
        let tests = vec![
            (Alignment::Right, Alignment::Left, "test    "),
            (Alignment::Left, Alignment::Center, "  test  "),
            (Alignment::Center, Alignment::Right, "    test"),
        ];
        for (block_title_alignment, alignment, expected) in tests {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 1));
            Block::new()
                .title_alignment(block_title_alignment)
                .title(Line::from("test").alignment(alignment))
                .render(buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines([expected]));
        }
    }

    /// This is a regression test for bug <https://github.com/ratatui/ratatui/issues/929>
    #[test]
    fn render_right_aligned_empty_title() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::new()
            .title_alignment(Alignment::Right)
            .title("")
            .render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["               "; 3]));
    }

    #[test]
    fn title_position() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        Block::new()
            .title_position(Position::Bottom)
            .title("test")
            .render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["    ", "test"]));
    }

    #[test]
    fn title_content_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::new()
                .title_alignment(alignment)
                .title("test".yellow())
                .render(buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines(["test".yellow()]));
        }
    }

    #[test]
    fn block_title_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::new()
                .title_alignment(alignment)
                .title_style(Style::new().yellow())
                .title("test")
                .render(buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines(["test".yellow()]));
        }
    }

    #[test]
    fn title_style_overrides_block_title_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::new()
                .title_alignment(alignment)
                .title_style(Style::new().green().on_red())
                .title("test".yellow())
                .render(buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines(["test".yellow().on_red()]));
        }
    }

    #[test]
    fn title_border_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .title("test")
            .border_style(Style::new().yellow())
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let mut expected = Buffer::with_lines([
            "┌test────┐",
            "│        │",
            "└────────┘",
        ]);
        expected.set_style(Rect::new(0, 0, 10, 3), Style::new().yellow());
        expected.set_style(Rect::new(1, 1, 8, 1), Style::reset());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn border_type_to_string() {
        assert_eq!(format!("{}", BorderType::Plain), "Plain");
        assert_eq!(format!("{}", BorderType::Rounded), "Rounded");
        assert_eq!(format!("{}", BorderType::Double), "Double");
        assert_eq!(format!("{}", BorderType::Thick), "Thick");
    }

    #[test]
    fn border_type_from_str() {
        assert_eq!("Plain".parse(), Ok(BorderType::Plain));
        assert_eq!("Rounded".parse(), Ok(BorderType::Rounded));
        assert_eq!("Double".parse(), Ok(BorderType::Double));
        assert_eq!("Thick".parse(), Ok(BorderType::Thick));
        assert_eq!("".parse::<BorderType>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn render_plain_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::Plain)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌────────┐",
            "│        │",
            "└────────┘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_rounded_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "╭────────╮",
            "│        │",
            "╰────────╯",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_double_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::Double)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "╔════════╗",
            "║        ║",
            "╚════════╝",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_quadrant_inside() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::QuadrantInside)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "▗▄▄▄▄▄▄▄▄▖",
            "▐        ▌",
            "▝▀▀▀▀▀▀▀▀▘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_border_quadrant_outside() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::QuadrantOutside)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "▛▀▀▀▀▀▀▀▀▜",
            "▌        ▐",
            "▙▄▄▄▄▄▄▄▄▟",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_solid_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_type(BorderType::Thick)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┏━━━━━━━━┓",
            "┃        ┃",
            "┗━━━━━━━━┛",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_custom_border_set() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        Block::bordered()
            .border_set(border::Set {
                top_left: "1",
                top_right: "2",
                bottom_left: "3",
                bottom_right: "4",
                vertical_left: "L",
                vertical_right: "R",
                horizontal_top: "T",
                horizontal_bottom: "B",
            })
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "1TTTTTTTT2",
            "L        R",
            "3BBBBBBBB4",
        ]);
        assert_eq!(buffer, expected);
    }
}
