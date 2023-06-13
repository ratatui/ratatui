#![allow(unused_variables)]

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::{Line, StyledGrapheme};
use crate::{layout::Alignment, style::Style};

// Interface for all text types that may be wrapped before rendering.
pub trait Wrappable<'a> {
    // TODO: This actually does not need to return multiple lines because it truncates, maybe change return type?
    fn wrap_truncate(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        horizontal_scroll: usize,
    ) -> Vec<Line>;
    fn wrap_char_boundary(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        trim: bool,
    ) -> Vec<Line>;
    fn wrap_word_boundary(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        trim: bool,
    ) -> Vec<Line>;
}

impl<'a> Wrappable<'a> for Line<'_> {
    fn wrap_truncate(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        horizontal_scroll: usize,
    ) -> Vec<Line> {
        // If the area to draw the text is 0 wide, return an empty vector.
        if max_width == 0 {
            return vec![];
        }

        // Create an iterator over the styled graphemes in the line
        let styled_graphemes = self.styled_graphemes(base_style);

        // If the line has an alignment, use it. Otherwise, use the base alignment.
        let alignment = self.alignment.unwrap_or(base_alignment);

        let mut working_line = Vec::new();
        let mut working_line_width = 0;
        let mut remaining_scroll = horizontal_scroll;

        for StyledGrapheme { symbol, style } in styled_graphemes {
            // Ignore characters that are wider than the maximum width.
            if symbol.width() > max_width {
                continue;
            }

            // Truncate the line once the maximum width is reached.
            if working_line_width + symbol.width() > max_width {
                break;
            }

            // TODO: It seems that horizontal scroll is only supported on left-aligned lines.
            // Before adding the symbol to the line, adjust it for horizontal scroll.
            // This means that the symbol may or may not be truncated based on its position in the
            // line and whether it would be rendered outside (to the left) of the widget border.
            let scrolled_symbol = adjust_symbol_for_horizontal_scroll(
                symbol,
                style,
                &mut remaining_scroll,
                alignment,
            );

            working_line_width += scrolled_symbol.width();
            working_line.push(StyledGrapheme {
                symbol: scrolled_symbol,
                style,
            });
        }

        // Convert the `Vec<StyledGrapheme>` into a `Line` so that it can be returned.
        vec![working_line.into_iter().collect::<Line>()]
    }

    fn wrap_char_boundary(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        trim: bool,
    ) -> Vec<Line> {
        todo!()
    }

    fn wrap_word_boundary(
        &self,
        max_width: usize,
        base_style: Style,
        base_alignment: Alignment,
        trim: bool,
    ) -> Vec<Line> {
        todo!()
    }
}

// This function trims a symbol by a given offset, allowing for only some parts of
// the symbol to be rendered based on horizontal scroll.
fn trim_symbol(symbol: &str, mut offset: usize) -> &str {
    let mut start = 0;
    for grapheme in UnicodeSegmentation::graphemes(symbol, true) {
        let grapheme_width = grapheme.width();
        if grapheme_width <= offset {
            offset -= grapheme_width;
            start += grapheme.len();
        } else {
            break;
        }
    }
    &symbol[start..]
}

// This function adjusts a symbol for horizontal scroll, based on whether it would be
// rendered outside of the left border of the widget.
fn adjust_symbol_for_horizontal_scroll<'a>(
    symbol: &'a str,
    style: Style,
    remaining_scroll: &mut usize,
    alignment: Alignment,
) -> &'a str {
    let mut scrolled_symbol = "";

    // If the line is left-aligned and horizontal scroll is to be applied, skip symbols
    // until the horizontal scroll is reached.
    // Otherwise, just use the symbol without any modification.
    if alignment == Alignment::Left && *remaining_scroll > 0 {
        let symbol_width = symbol.width();

        // If the symbol is wider than the remaining horizontal scroll, this means that
        // it must be rendered, at least partially (the symbol is trimmed to fit).
        // If the symbol is not wider than the remaining horizontal scroll, this means that
        // it does not need to be rendered, so it is kept empty and the remaining scroll
        // is adjusted accordingly.
        if symbol_width > *remaining_scroll {
            scrolled_symbol = trim_symbol(symbol, *remaining_scroll);
            // If the symbol is being rendered, this means that scrolling has finished.
            *remaining_scroll = 0;
        } else {
            *remaining_scroll -= symbol_width;
        }
    } else {
        scrolled_symbol = symbol;
    }

    scrolled_symbol
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn truncate_without_horizontal_scroll() {
        let line = Line::from("Hello, world!");
        let truncated = line.wrap_truncate(5, Style::default(), Alignment::Left, 0);
        assert_eq!(truncated.len(), 1);
        assert_eq!(truncated[0], Line::from("Hello"));
    }

    #[test]
    fn truncate_with_horizontal_scroll() {
        let line = Line::from("Hello, world!");
        let truncated = line.wrap_truncate(5, Style::default(), Alignment::Left, 7);
        assert_eq!(truncated.len(), 1);
        assert_eq!(truncated[0], Line::from("world"));
    }
}
