#[path = "../title.rs"]
pub mod title;

use strum::{Display, EnumString};

pub use self::title::{Position, Title};
use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Styled},
    symbols::line,
    widgets::{Borders, Widget},
};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BorderType {
    #[default]
    Plain,
    Rounded,
    Double,
    Thick,
}

impl BorderType {
    pub const fn line_symbols(border_type: BorderType) -> line::Set {
        match border_type {
            BorderType::Plain => line::NORMAL,
            BorderType::Rounded => line::ROUNDED,
            BorderType::Double => line::DOUBLE,
            BorderType::Thick => line::THICK,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Padding {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Padding {
    pub const fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Padding {
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn zero() -> Self {
        Padding {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    pub const fn horizontal(value: u16) -> Self {
        Padding {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    pub const fn vertical(value: u16) -> Self {
        Padding {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    pub const fn uniform(value: u16) -> Self {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}

/// Base widget to be used with all upper level ones. It may be used to display a box border around
/// the widget and/or add a title.
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, BorderType, Borders};
/// # use ratatui::style::{Style, Color};
/// Block::default()
///     .title("Block")
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .border_type(BorderType::Rounded)
///     .style(Style::default().bg(Color::Black));
/// ```
///
/// You may also use multiple titles like in the following:
/// ```
/// # use ratatui::widgets::{Block, BorderType, Borders, block::title::{Position, Title}};
/// # use ratatui::style::{Style, Color};
/// Block::default()
///     .title("Title 1")
///     .title(Title::from("Title 2").position(Position::Bottom))
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .border_type(BorderType::Rounded)
///     .style(Style::default().bg(Color::Black));
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Block<'a> {
    /// List of titles
    titles: Vec<Title<'a>>,
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
    /// Type of the border. The default is plain lines but one can choose to have rounded or
    /// doubled lines instead.
    border_type: BorderType,

    /// Widget style
    style: Style,
    /// Block padding
    padding: Padding,
}

impl<'a> Block<'a> {
    pub const fn new() -> Self {
        Self {
            titles: Vec::new(),
            titles_style: Style::new(),
            titles_alignment: Alignment::Left,
            titles_position: Position::Top,
            borders: Borders::NONE,
            border_style: Style::new(),
            border_type: BorderType::Plain,
            style: Style::new(),
            padding: Padding::zero(),
        }
    }
    /// # Example
    /// ```
    /// # use ratatui::widgets::{Block, block::title::Title};
    /// # use ratatui::layout::Alignment;
    /// Block::default()
    ///    .title("Title") // By default in the top right corner
    ///    .title(Title::from("Left").alignment(Alignment::Left))
    ///    .title(
    ///        Title::from("Center")
    ///            .alignment(Alignment::Center),
    ///    );
    /// ```
    /// Adds a title to the block.
    ///
    /// The `title` function allows you to add a title to the block. You can call this function
    /// multiple times to add multiple titles.
    ///
    /// Each title will be rendered with a single space separating titles that are in the same
    /// position or alignment. When both centered and non-centered titles are rendered, the centered
    /// space is calculated based on the full width of the block, rather than the leftover width.
    ///
    /// You can provide various types as the title, including strings, string slices, borrowed
    /// strings (`Cow<str>`), spans, or vectors of spans (`Vec<Span>`).
    ///
    /// By default, the titles will avoid being rendered in the corners of the block but will align
    /// against the left or right edge of the block if there is no border on that edge.
    ///
    /// Note: If the block is too small and multiple titles overlap, the border might get cut off at
    /// a corner.
    pub fn title<T>(mut self, title: T) -> Block<'a>
    where
        T: Into<Title<'a>>,
    {
        self.titles.push(title.into());
        self
    }

    /// Applies the style to all titles. If a title already has a style, it will add on top of it.
    pub const fn title_style(mut self, style: Style) -> Block<'a> {
        self.titles_style = style;
        self
    }

    /// Aligns all elements that don't have an alignment
    /// # Example
    /// This example aligns all titles in the center except "right" title
    /// ```
    /// # use ratatui::widgets::{Block, block::title::Title};
    /// # use ratatui::layout::Alignment;
    /// Block::default()
    ///   // This title won't be aligned in the center
    ///   .title(Title::from("right").alignment(Alignment::Right))
    ///   .title("foo")
    ///   .title("bar")
    ///   .title_alignment(Alignment::Center);
    /// ```
    pub const fn title_alignment(mut self, alignment: Alignment) -> Block<'a> {
        self.titles_alignment = alignment;
        self
    }

    #[deprecated(since = "0.22.0", note = "You should use a `title_position` instead.")]
    /// This method just calls `title_position` with Position::Bottom
    pub fn title_on_bottom(self) -> Block<'a> {
        self.title_position(Position::Bottom)
    }

    /// Positions all titles that don't have a position
    /// # Example
    /// This example position all titles on the bottom except "top" title
    /// ```
    /// # use ratatui::widgets::{Block, BorderType, Borders, block::title::{Position, Title}};
    /// Block::default()
    ///   // This title won't be aligned in the center
    ///   .title(Title::from("top").position(Position::Top))
    ///   .title("foo")
    ///   .title("bar")
    ///   .title_position(Position::Bottom);
    /// ```
    pub const fn title_position(mut self, position: Position) -> Block<'a> {
        self.titles_position = position;
        self
    }

    pub const fn border_style(mut self, style: Style) -> Block<'a> {
        self.border_style = style;
        self
    }

    pub const fn style(mut self, style: Style) -> Block<'a> {
        self.style = style;
        self
    }

    pub const fn borders(mut self, flag: Borders) -> Block<'a> {
        self.borders = flag;
        self
    }

    pub const fn border_type(mut self, border_type: BorderType) -> Block<'a> {
        self.border_type = border_type;
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
    ///
    /// # Examples
    ///
    /// ```
    /// // Draw a block nested within another block
    /// use ratatui::{backend::TestBackend, buffer::Buffer, terminal::Terminal, widgets::{Block, Borders}};
    /// let backend = TestBackend::new(15, 5);
    /// let mut terminal = Terminal::new(backend).unwrap();
    /// let outer_block = Block::default()
    ///     .title("Outer Block")
    ///     .borders(Borders::ALL);
    /// let inner_block = Block::default()
    ///     .title("Inner Block")
    ///     .borders(Borders::ALL);
    /// terminal.draw(|f| {
    ///     let inner_area = outer_block.inner(f.size());
    ///     f.render_widget(outer_block, f.size());
    ///     f.render_widget(inner_block, inner_area);
    /// });
    /// let expected = Buffer::with_lines(vec![
    ///     "┌Outer Block──┐",
    ///     "│┌Inner Block┐│",
    ///     "││           ││",
    ///     "│└───────────┘│",
    ///     "└─────────────┘",
    /// ]);
    /// terminal.backend().assert_buffer(&expected);
    /// ```
    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = area;
        if self.borders.intersects(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.width = inner.width.saturating_sub(1);
        }
        if self.borders.intersects(Borders::TOP) || !self.titles.is_empty() {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.height = inner.height.saturating_sub(1);
        }
        if self.borders.intersects(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }
        if self.borders.intersects(Borders::BOTTOM) {
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

    pub const fn padding(mut self, padding: Padding) -> Block<'a> {
        self.padding = padding;
        self
    }

    fn render_borders(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let symbols = BorderType::line_symbols(self.border_type);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical)
                    .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal)
                    .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(self.border_style);
        }
    }

    /* Titles Rendering */
    fn get_title_y(&self, position: Position, area: Rect) -> u16 {
        match position {
            Position::Bottom => area.bottom() - 1,
            Position::Top => area.top(),
        }
    }

    fn title_filter(&self, title: &Title, alignment: Alignment, position: Position) -> bool {
        title.alignment.unwrap_or(self.titles_alignment) == alignment
            && title.position.unwrap_or(self.titles_position) == position
    }

    fn calculate_title_area_offsets(&self, area: Rect) -> (u16, u16, u16) {
        let left_border_dx = u16::from(self.borders.intersects(Borders::LEFT));
        let right_border_dx = u16::from(self.borders.intersects(Borders::RIGHT));

        let title_area_width = area
            .width
            .saturating_sub(left_border_dx)
            .saturating_sub(right_border_dx);

        (left_border_dx, right_border_dx, title_area_width)
    }

    fn render_left_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let (left_border_dx, _, title_area_width) = self.calculate_title_area_offsets(area);

        let mut current_offset = left_border_dx;
        self.titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Left, position))
            .for_each(|title| {
                let title_x = current_offset;
                current_offset += title.content.width() as u16 + 1;

                // Clone the title's content, applying block title style then the title style
                let mut content = title.content.clone();
                for span in content.spans.iter_mut() {
                    span.style = self.titles_style.patch(span.style);
                }

                buf.set_line(
                    title_x + area.left(),
                    self.get_title_y(position, area),
                    &content,
                    title_area_width,
                );
            });
    }

    fn render_center_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let (_, _, title_area_width) = self.calculate_title_area_offsets(area);

        let titles = self
            .titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Center, position));

        let titles_sum = titles
            .clone()
            .fold(-1, |acc, f| acc + f.content.width() as i16 + 1); // First element isn't spaced

        let mut current_offset = area.width.saturating_sub(titles_sum as u16) / 2;
        titles.for_each(|title| {
            let title_x = current_offset;
            current_offset += title.content.width() as u16 + 1;

            // Clone the title's content, applying block title style then the title style
            let mut content = title.content.clone();
            for span in content.spans.iter_mut() {
                span.style = self.titles_style.patch(span.style);
            }

            buf.set_line(
                title_x + area.left(),
                self.get_title_y(position, area),
                &content,
                title_area_width,
            );
        });
    }

    fn render_right_titles(&self, position: Position, area: Rect, buf: &mut Buffer) {
        let (_, right_border_dx, title_area_width) = self.calculate_title_area_offsets(area);

        let mut current_offset = right_border_dx;
        self.titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Right, position))
            .rev() // so that the titles appear in the order they have been set
            .for_each(|title| {
                current_offset += title.content.width() as u16 + 1;
                let title_x = current_offset - 1; // First element isn't spaced

                // Clone the title's content, applying block title style then the title style
                let mut content = title.content.clone();
                for span in content.spans.iter_mut() {
                    span.style = self.titles_style.patch(span.style);
                }

                buf.set_line(
                    area.width.saturating_sub(title_x) + area.left(),
                    self.get_title_y(position, area),
                    &content,
                    title_area_width,
                );
            });
    }

    fn render_title_position(&self, position: Position, area: Rect, buf: &mut Buffer) {
        // Note: the order in which these functions are called define the overlapping behavior
        self.render_right_titles(position, area, buf);
        self.render_center_titles(position, area, buf);
        self.render_left_titles(position, area, buf);
    }

    fn render_titles(&self, area: Rect, buf: &mut Buffer) {
        self.render_title_position(Position::Top, area, buf);
        self.render_title_position(Position::Bottom, area, buf);
    }
}

impl<'a> Widget for Block<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }
        self.render_borders(area, buf);
        self.render_titles(area, buf);
    }
}

impl<'a> Styled for Block<'a> {
    type Item = Block<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;
    use crate::{
        assert_buffer_eq,
        layout::Rect,
        style::{Color, Modifier, Stylize},
    };

    #[test]
    fn inner_takes_into_account_the_borders() {
        // No borders
        assert_eq!(
            Block::default().inner(Rect::default()),
            Rect::new(0, 0, 0, 0),
            "no borders, width=0, height=0"
        );
        assert_eq!(
            Block::default().inner(Rect::new(0, 0, 1, 1)),
            Rect::new(0, 0, 1, 1),
            "no borders, width=1, height=1"
        );

        // Left border
        assert_eq!(
            Block::default()
                .borders(Borders::LEFT)
                .inner(Rect::new(0, 0, 0, 1)),
            Rect::new(0, 0, 0, 1),
            "left, width=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::LEFT)
                .inner(Rect::new(0, 0, 1, 1)),
            Rect::new(1, 0, 0, 1),
            "left, width=1"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::LEFT)
                .inner(Rect::new(0, 0, 2, 1)),
            Rect::new(1, 0, 1, 1),
            "left, width=2"
        );

        // Top border
        assert_eq!(
            Block::default()
                .borders(Borders::TOP)
                .inner(Rect::new(0, 0, 1, 0)),
            Rect::new(0, 0, 1, 0),
            "top, height=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::TOP)
                .inner(Rect::new(0, 0, 1, 1)),
            Rect::new(0, 1, 1, 0),
            "top, height=1"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::TOP)
                .inner(Rect::new(0, 0, 1, 2)),
            Rect::new(0, 1, 1, 1),
            "top, height=2"
        );

        // Right border
        assert_eq!(
            Block::default()
                .borders(Borders::RIGHT)
                .inner(Rect::new(0, 0, 0, 1)),
            Rect::new(0, 0, 0, 1),
            "right, width=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::RIGHT)
                .inner(Rect::new(0, 0, 1, 1)),
            Rect::new(0, 0, 0, 1),
            "right, width=1"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::RIGHT)
                .inner(Rect::new(0, 0, 2, 1)),
            Rect::new(0, 0, 1, 1),
            "right, width=2"
        );

        // Bottom border
        assert_eq!(
            Block::default()
                .borders(Borders::BOTTOM)
                .inner(Rect::new(0, 0, 1, 0)),
            Rect::new(0, 0, 1, 0),
            "bottom, height=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::BOTTOM)
                .inner(Rect::new(0, 0, 1, 1)),
            Rect::new(0, 0, 1, 0),
            "bottom, height=1"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::BOTTOM)
                .inner(Rect::new(0, 0, 1, 2)),
            Rect::new(0, 0, 1, 1),
            "bottom, height=2"
        );

        // All borders
        assert_eq!(
            Block::default()
                .borders(Borders::ALL)
                .inner(Rect::default()),
            Rect::new(0, 0, 0, 0),
            "all borders, width=0, height=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::ALL)
                .inner(Rect::new(0, 0, 1, 1)),
            Rect::new(1, 1, 0, 0),
            "all borders, width=1, height=1"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::ALL)
                .inner(Rect::new(0, 0, 2, 2)),
            Rect::new(1, 1, 0, 0),
            "all borders, width=2, height=2"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::ALL)
                .inner(Rect::new(0, 0, 3, 3)),
            Rect::new(1, 1, 1, 1),
            "all borders, width=3, height=3"
        );
    }

    #[test]
    fn inner_takes_into_account_the_title() {
        assert_eq!(
            Block::default().title("Test").inner(Rect::new(0, 0, 0, 1)),
            Rect::new(0, 1, 0, 0),
        );
        assert_eq!(
            Block::default()
                .title(Title::from("Test").alignment(Alignment::Center))
                .inner(Rect::new(0, 0, 0, 1)),
            Rect::new(0, 1, 0, 0),
        );
        assert_eq!(
            Block::default()
                .title(Title::from("Test").alignment(Alignment::Right))
                .inner(Rect::new(0, 0, 0, 1)),
            Rect::new(0, 1, 0, 0),
        );
    }

    #[test]
    fn border_type_can_be_const() {
        const _PLAIN: line::Set = BorderType::line_symbols(BorderType::Plain);
    }

    #[test]
    fn padding_new() {
        assert_eq!(
            Padding::new(1, 2, 3, 4),
            Padding {
                left: 1,
                right: 2,
                top: 3,
                bottom: 4
            }
        )
    }

    #[test]
    fn padding_constructors() {
        assert_eq!(Padding::zero(), Padding::new(0, 0, 0, 0));
        assert_eq!(Padding::horizontal(1), Padding::new(1, 1, 0, 0));
        assert_eq!(Padding::vertical(1), Padding::new(0, 0, 1, 1));
        assert_eq!(Padding::uniform(1), Padding::new(1, 1, 1, 1));
    }

    #[test]
    fn padding_can_be_const() {
        const _PADDING: Padding = Padding::new(1, 1, 1, 1);
        const _UNI_PADDING: Padding = Padding::uniform(1);
        const _NO_PADDING: Padding = Padding::zero();
        const _HORIZONTAL: Padding = Padding::horizontal(1);
        const _VERTICAL: Padding = Padding::vertical(1);
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
                border_type: BorderType::Plain,
                style: Style::new(),
                padding: Padding::zero(),
            }
        )
    }

    #[test]
    fn block_can_be_const() {
        const _DEFAULT_STYLE: Style = Style::new();
        const _DEFAULT_PADDING: Padding = Padding::uniform(1);
        const _DEFAULT_BLOCK: Block = Block::new()
            .title_style(_DEFAULT_STYLE)
            .title_alignment(Alignment::Left)
            .title_position(Position::Top)
            .borders(Borders::ALL)
            .border_style(_DEFAULT_STYLE)
            .style(_DEFAULT_STYLE)
            .padding(_DEFAULT_PADDING);
    }

    #[test]
    fn can_be_stylized() {
        let block = Block::default().black().on_white().bold().not_dim();
        assert_eq!(
            block.style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
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
            Block::default()
                .title("test")
                .title_alignment(alignment)
                .render(buffer.area, &mut buffer);
            assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
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
            Block::default()
                .title(Title::from("test").alignment(alignment))
                .title_alignment(block_title_alignment)
                .render(buffer.area, &mut buffer);
            assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        }
    }

    #[test]
    fn title_on_bottom() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        #[allow(deprecated)]
        Block::default()
            .title("test")
            .title_on_bottom()
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["    ", "test"]));
    }

    #[test]
    fn title_position() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        Block::default()
            .title("test")
            .title_position(Position::Bottom)
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["    ", "test"]));
    }

    #[test]
    fn title_content_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::default()
                .title("test".yellow())
                .title_alignment(alignment)
                .render(buffer.area, &mut buffer);

            let mut expected_buffer = Buffer::with_lines(vec!["test"]);
            expected_buffer.set_style(Rect::new(0, 0, 4, 1), Style::new().yellow());

            assert_buffer_eq!(buffer, expected_buffer);
        }
    }

    #[test]
    fn block_title_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::default()
                .title("test")
                .title_style(Style::new().yellow())
                .title_alignment(alignment)
                .render(buffer.area, &mut buffer);

            let mut expected_buffer = Buffer::with_lines(vec!["test"]);
            expected_buffer.set_style(Rect::new(0, 0, 4, 1), Style::new().yellow());

            assert_buffer_eq!(buffer, expected_buffer);
        }
    }

    #[test]
    fn title_style_overrides_block_title_style() {
        for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
            Block::default()
                .title("test".yellow())
                .title_style(Style::new().green().on_red())
                .title_alignment(alignment)
                .render(buffer.area, &mut buffer);

            let mut expected_buffer = Buffer::with_lines(vec!["test"]);
            expected_buffer.set_style(Rect::new(0, 0, 4, 1), Style::new().yellow().on_red());

            assert_buffer_eq!(buffer, expected_buffer);
        }
    }

    #[test]
    fn title_border_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::default()
            .title("test")
            .borders(Borders::ALL)
            .border_style(Style::new().yellow())
            .render(buffer.area, &mut buffer);

        let mut expected_buffer = Buffer::with_lines(vec![
            "┌test─────────┐",
            "│             │",
            "└─────────────┘",
        ]);
        expected_buffer.set_style(Rect::new(0, 0, 15, 3), Style::new().yellow());
        expected_buffer.set_style(Rect::new(1, 1, 13, 1), Style::reset());

        assert_buffer_eq!(buffer, expected_buffer);
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
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "┌─────────────┐",
                "│             │",
                "└─────────────┘"
            ])
        );
    }
    #[test]
    fn render_rounded_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "╭─────────────╮",
                "│             │",
                "╰─────────────╯"
            ])
        );
    }
    #[test]
    fn render_double_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "╔═════════════╗",
                "║             ║",
                "╚═════════════╝"
            ])
        );
    }

    #[test]
    fn render_solid_border() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "┏━━━━━━━━━━━━━┓",
                "┃             ┃",
                "┗━━━━━━━━━━━━━┛"
            ])
        );
    }
}
