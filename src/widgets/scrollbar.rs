#![warn(missing_docs)]
use std::iter;

use itertools::Itertools;
use strum::{Display, EnumString};
use unicode_width::UnicodeWidthStr;

use super::StatefulWidget;
use crate::{
    prelude::*,
    symbols::scrollbar::{Set, DOUBLE_HORIZONTAL, DOUBLE_VERTICAL},
};

/// An enum representing a scrolling direction.
///
/// This is used with [`ScrollbarState::scroll`].
///
/// It is useful for example when you want to store in which direction to scroll.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ScrollDirection {
    /// Forward scroll direction, usually corresponds to scrolling downwards or rightwards.
    #[default]
    Forward,
    /// Backward scroll direction, usually corresponds to scrolling upwards or leftwards.
    Backward,
}

/// A struct representing the state of a Scrollbar widget.
///
/// # Important
///
/// It's essential to set the `content_length` field when using this struct. This field
/// represents the total length of the scrollable content. The default value is zero
/// which will result in the Scrollbar not rendering.
///
/// For example, in the following list, assume there are 4 bullet points:
///
/// - the `content_length` is 4
/// - the `position` is 0
/// - the `viewport_content_length` is 2
///
/// ```text
/// ┌───────────────┐
/// │1. this is a   █
/// │   single item █
/// │2. this is a   ║
/// │   second item ║
/// └───────────────┘
/// ```
///
/// If you don't have multi-line content, you can leave the `viewport_content_length` set to the
/// default of 0 and it'll use the track size as a `viewport_content_length`.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollbarState {
    /// The total length of the scrollable content.
    content_length: usize,
    /// The current position within the scrollable content.
    position: usize,
    /// The length of content in current viewport.
    viewport_content_length: usize,
}

impl ScrollbarState {
    /// Constructs a new ScrollbarState with the specified content length.
    ///
    /// `content_length` is the total number of element, that can be scrolled. See
    /// [`ScrollbarState`] for more details.
    pub fn new(content_length: usize) -> Self {
        Self {
            content_length,
            ..Default::default()
        }
    }

    /// Sets the scroll position of the scrollbar.
    ///
    /// This represents the number of scrolled items.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Sets the length of the scrollable content.
    ///
    /// This is the number of scrollable items. If items have a length of one, then this is the
    /// same as the number of scrollable cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.content_length = content_length;
        self
    }

    /// Sets the items' size.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn viewport_content_length(mut self, viewport_content_length: usize) -> Self {
        self.viewport_content_length = viewport_content_length;
        self
    }

    /// Decrements the scroll position by one, ensuring it doesn't go below zero.
    pub fn prev(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Increments the scroll position by one, ensuring it doesn't exceed the length of the content.
    pub fn next(&mut self) {
        self.position = self
            .position
            .saturating_add(1)
            .min(self.content_length.saturating_sub(1))
    }

    /// Sets the scroll position to the start of the scrollable content.
    pub fn first(&mut self) {
        self.position = 0;
    }

    /// Sets the scroll position to the end of the scrollable content.
    pub fn last(&mut self) {
        self.position = self.content_length.saturating_sub(1)
    }

    /// Changes the scroll position based on the provided [`ScrollDirection`].
    pub fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Forward => {
                self.next();
            }
            ScrollDirection::Backward => {
                self.prev();
            }
        }
    }
}

/// This is the position of the scrollbar around a given area.
///
/// ```plain
///           HorizontalTop
///             ┌───────┐
/// VerticalLeft│       │VerticalRight
///             └───────┘
///          HorizontalBottom
/// ```
#[derive(Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum ScrollbarOrientation {
    /// Positions the scrollbar on the right, scrolling vertically
    #[default]
    VerticalRight,
    /// Positions the scrollbar on the left, scrolling vertically
    VerticalLeft,
    /// Positions the scrollbar on the bottom, scrolling horizontally
    HorizontalBottom,
    /// Positions the scrollbar on the top, scrolling horizontally
    HorizontalTop,
}

/// A widget to display a scrollbar
///
/// The following components of the scrollbar are customizable in symbol and style. Note the
/// scrollbar is represented horizontally but it can also be set vertically (which is actually the
/// default).
///
/// ```text
/// <--▮------->
/// ^  ^   ^   ^
/// │  │   │   └ end
/// │  │   └──── track
/// │  └──────── thumb
/// └─────────── begin
/// ```
///
/// # Examples
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// # fn render_paragraph_with_scrollbar(frame: &mut Frame, area: Rect) {
/// let vertical_scroll = 0; // from app state
///
/// let items = vec![
///     Line::from("Item 1"),
///     Line::from("Item 2"),
///     Line::from("Item 3"),
/// ];
/// let paragraph = Paragraph::new(items.clone())
///     .scroll((vertical_scroll as u16, 0))
///     .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar
///
/// let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
///     .begin_symbol(Some("↑"))
///     .end_symbol(Some("↓"));
///
/// let mut scrollbar_state = ScrollbarState::new(items.len()).position(vertical_scroll);
///
/// let area = frame.size();
/// // Note we render the paragraph
/// frame.render_widget(paragraph, area);
/// // and the scrollbar, those are separate widgets
/// frame.render_stateful_widget(
///     scrollbar,
///     area.inner(&Margin {
///         // using an inner vertical margin of 1 unit makes the scrollbar inside the block
///         vertical: 1,
///         horizontal: 0,
///     }),
///     &mut scrollbar_state,
/// );
/// # }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Scrollbar<'a> {
    orientation: ScrollbarOrientation,
    thumb_style: Style,
    thumb_symbol: &'a str,
    track_style: Style,
    track_symbol: Option<&'a str>,
    begin_symbol: Option<&'a str>,
    begin_style: Style,
    end_symbol: Option<&'a str>,
    end_style: Style,
}

impl<'a> Default for Scrollbar<'a> {
    fn default() -> Self {
        Self {
            orientation: ScrollbarOrientation::default(),
            thumb_symbol: DOUBLE_VERTICAL.thumb,
            thumb_style: Style::default(),
            track_symbol: Some(DOUBLE_VERTICAL.track),
            track_style: Style::default(),
            begin_symbol: Some(DOUBLE_VERTICAL.begin),
            begin_style: Style::default(),
            end_symbol: Some(DOUBLE_VERTICAL.end),
            end_style: Style::default(),
        }
    }
}

impl<'a> Scrollbar<'a> {
    /// Creates a new scrollbar with the given position.
    ///
    /// Most of the time you'll want [`ScrollbarOrientation::VerticalLeft`] or
    /// [`ScrollbarOrientation::HorizontalBottom`]. See [`ScrollbarOrientation`] for more options.
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self::default().orientation(orientation)
    }

    /// Sets the position of the scrollbar.
    ///
    /// The orientation of the scrollbar is the position it will take around a [`Rect`]. See
    /// [`ScrollbarOrientation`] for more details.
    ///
    /// Resets the symbols to [`DOUBLE_VERTICAL`] or [`DOUBLE_HORIZONTAL`] based on orientation.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation;
        let set = if self.is_vertical() {
            DOUBLE_VERTICAL
        } else {
            DOUBLE_HORIZONTAL
        };
        self.symbols(set)
    }

    /// Sets the orientation and symbols for the scrollbar from a [`Set`].
    ///
    /// This has the same effect as calling [`Scrollbar::orientation`] and then
    /// [`Scrollbar::symbols`]. See those for more details.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn orientation_and_symbol(mut self, orientation: ScrollbarOrientation, set: Set) -> Self {
        self.orientation = orientation;
        self.symbols(set)
    }

    /// Sets the symbol that represents the thumb of the scrollbar.
    ///
    /// The thumb is the handle representing the progression on the scrollbar. See [`Scrollbar`]
    /// for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn thumb_symbol(mut self, thumb_symbol: &'a str) -> Self {
        self.thumb_symbol = thumb_symbol;
        self
    }

    /// Sets the style on the scrollbar thumb.
    ///
    /// The thumb is the handle representing the progression on the scrollbar. See [`Scrollbar`]
    /// for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn thumb_style<S: Into<Style>>(mut self, thumb_style: S) -> Self {
        self.thumb_style = thumb_style.into();
        self
    }

    /// Sets the symbol that represents the track of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn track_symbol(mut self, track_symbol: Option<&'a str>) -> Self {
        self.track_symbol = track_symbol;
        self
    }

    /// Sets the style that is used for the track of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn track_style<S: Into<Style>>(mut self, track_style: S) -> Self {
        self.track_style = track_style.into();
        self
    }

    /// Sets the symbol that represents the beginning of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn begin_symbol(mut self, begin_symbol: Option<&'a str>) -> Self {
        self.begin_symbol = begin_symbol;
        self
    }

    /// Sets the style that is used for the beginning of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn begin_style<S: Into<Style>>(mut self, begin_style: S) -> Self {
        self.begin_style = begin_style.into();
        self
    }

    /// Sets the symbol that represents the end of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn end_symbol(mut self, end_symbol: Option<&'a str>) -> Self {
        self.end_symbol = end_symbol;
        self
    }

    /// Sets the style that is used for the end of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn end_style<S: Into<Style>>(mut self, end_style: S) -> Self {
        self.end_style = end_style.into();
        self
    }

    /// Sets the symbols used for the various parts of the scrollbar from a [`Set`].
    ///
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    ///
    /// Only sets begin_symbol, end_symbol and track_symbol if they already contain a value.
    /// If they were set to `None` explicitly, this function will respect that choice. Use their
    /// respective setters to change their value.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn symbols(mut self, symbol: Set) -> Self {
        self.thumb_symbol = symbol.thumb;
        if self.track_symbol.is_some() {
            self.track_symbol = Some(symbol.track);
        }
        if self.begin_symbol.is_some() {
            self.begin_symbol = Some(symbol.begin);
        }
        if self.end_symbol.is_some() {
            self.end_symbol = Some(symbol.end);
        }
        self
    }

    /// Sets the style used for the various parts of the scrollbar from a [`Style`].
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        let style = style.into();
        self.track_style = style;
        self.thumb_style = style;
        self.begin_style = style;
        self.end_style = style;
        self
    }

    fn is_vertical(&self) -> bool {
        match self.orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => true,
            ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => false,
        }
    }

    fn get_viewport_len(&self, area: Rect) -> u16 {
        match self.orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => area.height,
            ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => {
                area.width
            }
        }
    }

    /// Returns information about scrollbar track
    ///
    /// For ScrollbarOrientation::VerticalRight
    ///
    /// ```plain
    ///                   ┌───────── track_axis
    ///                   v
    ///   ┌───────────────┐
    ///   │               ║
    ///   │               █
    ///   │               █
    ///   │               ║
    ///   └───────────────┘
    /// ```
    ///
    /// For ScrollbarOrientation::HorizontalBottom
    ///
    /// ```plain
    ///   ┌───────────────┐
    ///   │               │
    ///   │               │
    ///   │               │
    ///   └═══███═════════┘<──────── track_axis
    /// ```
    fn get_track_axis(&self, area: Rect) -> u16 {
        match self.orientation {
            ScrollbarOrientation::VerticalRight => area.x,
            ScrollbarOrientation::VerticalLeft => area.x + area.width.saturating_sub(1),
            ScrollbarOrientation::HorizontalBottom => area.y + area.height.saturating_sub(1),
            ScrollbarOrientation::HorizontalTop => area.y,
        }
    }

    /// Returns length segment information about scrollbar track
    ///
    /// For ScrollbarOrientation::VerticalRight
    ///
    /// ```plain
    ///   ┌───────────────┐
    ///   │               ║<──────── track_start
    ///   │               █<──────── thumb_end
    ///   │               █
    ///   │               █<──────── thumb_start
    ///   │               ║<──────── track_end
    ///   └───────────────┘
    /// ```
    ///
    /// For ScrollbarOrientation::HorizontalBottom
    ///
    /// ```plain
    ///   ┌───────────────┐
    ///   │               │
    ///   │               │
    ///   │               │
    ///   └═══█████═══════┘
    ///    ^  ^   ^      ^
    ///    │  │   │      └────────── track_end
    ///    │  │   │
    ///    │  │   └───────────────── thumb_end
    ///    │  │
    ///    │  └───────────────────── thumb_start
    ///    │
    ///    └──────────────────────── track_start
    /// ```
    ///
    /// Specifically this function returns the lengths of the different segments:
    ///
    /// ```plain
    ///         ┌──────────── thumb_len
    ///       vvvvv
    ///    ═══█████═══════
    ///    ^^^     ^^^^^^^
    ///     │         └────── track_end_len
    ///     │
    ///     └──────────────── track_start_len
    /// ```
    fn get_track_lens(&self, area: Rect, state: &mut ScrollbarState) -> (usize, usize, usize) {
        let (mut track_start, mut track_end) = match self.orientation {
            ScrollbarOrientation::VerticalRight => (area.y, (area.y + area.height)),
            ScrollbarOrientation::VerticalLeft => (area.y, (area.y + area.height)),
            ScrollbarOrientation::HorizontalBottom => (area.x, (area.x + area.width)),
            ScrollbarOrientation::HorizontalTop => (area.x, (area.x + area.width)),
        };
        // if scrollbar has begin and end symbols:
        //
        // <═══█████═══════>
        //
        // then increment and decrement track_start and track_end respectively
        if let Some(s) = self.begin_symbol {
            track_start = track_start.saturating_add(s.width() as u16);
        };
        if let Some(s) = self.end_symbol {
            track_end = track_end.saturating_sub(s.width() as u16);
        };
        let track_len = track_end.saturating_sub(track_start) as f64;

        let viewport_len = self.get_viewport_len(area) as f64;

        let content_length = state.content_length as f64;
        // if user passes in position > content_length, we shouldn't panic
        // this will prevent rendering outside of available area
        let position = state.position.min(state.content_length - 1) as f64;

        // vscode style scrolling behavior
        let scrollable_content_len = content_length + viewport_len - 1.0;
        let thumb_start = position * track_len / scrollable_content_len;
        let thumb_end = (position + viewport_len) * track_len / scrollable_content_len;

        // round() as usize gives closest int, as opposed to `floor` or `ceil`
        let track_start_len = thumb_start.round() as usize;
        let thumb_end = thumb_end.round() as usize;
        let thumb_len = thumb_end.saturating_sub(track_start_len);
        let track_end_len = track_len as usize - track_start_len - thumb_len;

        (track_start_len, thumb_len, track_end_len)
    }

    fn bar_builder(&self, area: Rect, state: &mut ScrollbarState) -> Vec<(u16, u16, &str, Style)> {
        // ```plain
        // ________________ <── track_axis
        // ^^^^^^^^^^^^^^^^
        //       └───────────── track_len
        // ```
        let track_axis = self.get_track_axis(area);

        let (track_start_len, thumb_len, track_end_len) = self.get_track_lens(area, state);

        let track = self.track_symbol.map(|s| (s, self.track_style));
        let thumb = Some((self.thumb_symbol, self.thumb_style));

        let begin = self.begin_symbol.map(|s| (s, self.begin_style));
        let end = self.end_symbol.map(|s| (s, self.end_style));

        iter::once(begin)
            // Current state of the iterator
            //
            // ```plain
            // ┌─────────────────── begin
            // v
            // <________________
            // ```
            .chain(iter::repeat(track).take(track_start_len))
            // Current state of the iterator
            //
            // ```plain
            // <═══_____________
            //  ^^^
            //   └──────────────── track_start_len
            // ```
            .chain(iter::repeat(thumb).take(thumb_len))
            // Current state of the iterator
            //
            // ```plain
            // <═══█████═══════_
            //     ^^^^^
            //       └──────────── thumb_len
            // ```
            .chain(iter::repeat(track).take(track_end_len))
            // Current state of the iterator
            //
            // ```plain
            // <═══█████═══════_
            //          ^^^^^^^
            //             └────── track_end_len
            // ```
            .chain(iter::once(end))
            // Current state of the iterator
            //
            // ```plain
            // <═══█████═══════>
            //                 ^
            //                 └── end
            // ```
            .flatten() // We want to skip any values that are `None`
            .enumerate() // gives each element an index that maps to buf location on track
            // TODO: is there a way to check that iterator len matches the `area.len` here?
            .map(|(i, (symbol, style))| {
                // convert index to coordinate system
                if self.is_vertical() {
                    let y = i as u16;
                    let x = track_axis;
                    (x, y, symbol, style)
                } else {
                    let x = i as u16;
                    let y = track_axis;
                    (x, y, symbol, style)
                }
            })
            .collect_vec()
    }
}

impl<'a> StatefulWidget for Scrollbar<'a> {
    type State = ScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.content_length == 0 {
            return;
        }

        let bar = self.bar_builder(area, state);

        let len = if self.is_vertical() {
            area.height
        } else {
            area.width
        };

        if bar.len() as u16 != len {
            // something went wrong here with the construction of bar
            return;
        };

        for (x, y, symbol, style) in bar {
            buf.set_string(x, y, symbol, style);
        }
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;
    use strum::ParseError;
    use unicode_width::UnicodeWidthStr;

    use super::*;

    #[test]
    fn scroll_direction_to_string() {
        assert_eq!(ScrollDirection::Forward.to_string(), "Forward");
        assert_eq!(ScrollDirection::Backward.to_string(), "Backward");
    }

    #[test]
    fn scroll_direction_from_str() {
        assert_eq!(
            "Forward".parse::<ScrollDirection>(),
            Ok(ScrollDirection::Forward)
        );
        assert_eq!(
            "Backward".parse::<ScrollDirection>(),
            Ok(ScrollDirection::Backward)
        );
        assert_eq!(
            "".parse::<ScrollDirection>(),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn scrollbar_orientation_to_string() {
        assert_eq!(
            ScrollbarOrientation::VerticalRight.to_string(),
            "VerticalRight"
        );
        assert_eq!(
            ScrollbarOrientation::VerticalLeft.to_string(),
            "VerticalLeft"
        );
        assert_eq!(
            ScrollbarOrientation::HorizontalBottom.to_string(),
            "HorizontalBottom"
        );
        assert_eq!(
            ScrollbarOrientation::HorizontalTop.to_string(),
            "HorizontalTop"
        );
    }

    #[test]
    fn scrollbar_orientation_from_str() {
        assert_eq!(
            "VerticalRight".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::VerticalRight)
        );
        assert_eq!(
            "VerticalLeft".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::VerticalLeft)
        );
        assert_eq!(
            "HorizontalBottom".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::HorizontalBottom)
        );
        assert_eq!(
            "HorizontalTop".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::HorizontalTop)
        );
        assert_eq!(
            "".parse::<ScrollbarOrientation>(),
            Err(ParseError::VariantNotFound)
        );
    }

    #[rstest]
    #[case("█═", 0, 2, "position_0")]
    #[case("═█", 1, 2, "position_1")]
    fn render_scrollbar_simplest(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width() as u16;
        let mut buffer = Buffer::empty(Rect::new(0, 0, size, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message
        );
    }

    #[rstest]
    #[case("#####-----", 0, 10, "position_0")]
    #[case("-#####----", 1, 10, "position_1")]
    #[case("-#####----", 2, 10, "position_2")]
    #[case("--#####---", 3, 10, "position_3")]
    #[case("--#####---", 4, 10, "position_4")]
    #[case("---#####--", 5, 10, "position_5")]
    #[case("---#####--", 6, 10, "position_6")]
    #[case("----#####-", 7, 10, "position_7")]
    #[case("----#####-", 8, 10, "position_8")]
    #[case("-----#####", 9, 10, "position_9")]
    fn render_scrollbar_simple(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width();
        let mut buffer = Buffer::empty(Rect::new(0, 0, size as u16, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalTop)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("-"))
            .thumb_symbol("#")
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message,
        );
    }

    #[rstest]
    #[case("          ", 0, 0, "position_0")]
    fn render_scrollbar_nobar(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width();
        let mut buffer = Buffer::empty(Rect::new(0, 0, size as u16, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalTop)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("-"))
            .thumb_symbol("#")
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message,
        );
    }

    #[rstest]
    #[case("##########", 0, 1, "position_0")]
    fn render_scrollbar_fullbar(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width();
        let mut buffer = Buffer::empty(Rect::new(0, 0, size as u16, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalTop)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("-"))
            .thumb_symbol("#")
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message,
        );
    }

    #[rstest]
    #[case("#########-", 0, 2, "position_0")]
    #[case("-#########", 1, 2, "position_1")]
    fn render_scrollbar_almost_fullbar(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width();
        let mut buffer = Buffer::empty(Rect::new(0, 0, size as u16, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalTop)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("-"))
            .thumb_symbol("#")
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message,
        );
    }

    #[rstest]
    #[case("█████═════", 0, 10, "position_0")]
    #[case("═█████════", 1, 10, "position_1")]
    #[case("═█████════", 2, 10, "position_2")]
    #[case("══█████═══", 3, 10, "position_3")]
    #[case("══█████═══", 4, 10, "position_4")]
    #[case("═══█████══", 5, 10, "position_5")]
    #[case("═══█████══", 6, 10, "position_6")]
    #[case("════█████═", 7, 10, "position_7")]
    #[case("════█████═", 8, 10, "position_8")]
    #[case("═════█████", 9, 10, "position_9")]
    #[case("═════█████", 100, 10, "position_out_of_bounds")]
    fn render_scrollbar_without_symbols(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width() as u16;
        let mut buffer = Buffer::empty(Rect::new(0, 0, size, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message
        );
    }

    #[rstest]
    #[case("<####---->", 0, 10, "position_0")]
    #[case("<#####--->", 1, 10, "position_1")]
    #[case("<-####--->", 2, 10, "position_2")]
    #[case("<-####--->", 3, 10, "position_3")]
    #[case("<--####-->", 4, 10, "position_4")]
    #[case("<--####-->", 5, 10, "position_5")]
    #[case("<---####->", 6, 10, "position_6")]
    #[case("<---####->", 7, 10, "position_7")]
    #[case("<---#####>", 8, 10, "position_8")]
    #[case("<----####>", 9, 10, "position_9")]
    #[case("<----####>", 10, 10, "position_one_out_of_bounds")]
    #[case("<----####>", 15, 10, "position_few_out_of_bounds")]
    #[case("<----####>", 500, 10, "position_very_many_out_of_bounds")]
    fn render_scrollbar_with_symbols(
        #[case] expected: &str,
        #[case] position: usize,
        #[case] content_length: usize,
        #[case] assertion_message: &str,
    ) {
        let size = expected.width() as u16;
        let mut buffer = Buffer::empty(Rect::new(0, 0, size, 1));
        let mut state = ScrollbarState::default()
            .position(position)
            .content_length(content_length);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalTop)
            .begin_symbol(Some("<"))
            .end_symbol(Some(">"))
            .track_symbol(Some("-"))
            .thumb_symbol("#")
            .render(buffer.area, &mut buffer, &mut state);
        assert_eq!(
            buffer,
            Buffer::with_lines(vec![expected]),
            "{}",
            assertion_message,
        );
    }
}
