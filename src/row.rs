/// A macro for creating a [`Row`] using vec! syntax.
///
/// `row!` is similar to the [`vec!`] macro, but it returns a [`Row`] instead of a `Vec`.
///
/// # Examples
///
/// * Create a [`Row`] containing a vector of [`Cell`]s:
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::row;
///
/// let row = row!["hello", "world"];
/// let row = row!["hello".red(), "world".red().bold()];
/// ```
///
/// * Create a [`Row`] from a given [`Cell`] repeated some amount of times:
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::row;
///
/// let row = row!["hello"; 2];
/// ```
///
/// * Use [`text!`], [`line!`] or [`span!`] macro inside [`row!`] macro.
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::{row, line, text, span};
///
/// let row = row![
///     line!["hello", "world"], span!(Modifier::BOLD; "goodbye {}", "world"),
///     text!["hello", "world"],
/// ];
/// ```
///
/// [`Row`]: crate::widgets::Row
/// [`Cell`]: crate::widgets::Cell
#[macro_export]
macro_rules! row {
    () => {
        ratatui::widgets::Row::default()
    };
    ($cell:expr; $n:expr) => {
        ratatui::widgets::Row::new(vec![ratatui::widgets::Cell::from($cell); $n])
    };
    ($($cell:expr),+ $(,)?) => {{
        ratatui::widgets::Row::new(vec![
        $(
            ratatui::widgets::Cell::from($cell),
        )+
        ])
    }};
}

#[cfg(test)]
mod tests {

    use ratatui::{
        text::Text,
        widgets::{Cell, Row},
    };

    #[test]
    fn row() {
        // literal
        let row = row!["hello", "world"];
        assert_eq!(
            row,
            Row::new(vec![Cell::from("hello"), Cell::from("world")])
        );

        // explicit use of span and line
        let row = row![crate::line!("hello"), crate::span!["world"]];
        assert_eq!(
            row,
            Row::new(vec![Cell::from("hello"), Cell::from("world")])
        );

        // vec count syntax
        let row = row!["hello"; 2];
        assert_eq!(
            row,
            Row::new(vec![Cell::from("hello"), Cell::from("hello")])
        );

        use crate::text;
        let rows = [
            row!["Find File", text!["ctrl+f"].right_aligned()],
            row!["Open recent", text!["ctrl+r"].right_aligned()],
            row!["Open config", text!["ctrl+k"].right_aligned()],
        ];
        assert_eq!(
            rows,
            [
                Row::new([
                    Cell::from("Find File"),
                    Cell::from(Text::raw("ctrl+f").alignment(ratatui::layout::Alignment::Right)),
                ]),
                Row::new([
                    Cell::from("Open recent"),
                    Cell::from(Text::raw("ctrl+r").alignment(ratatui::layout::Alignment::Right)),
                ]),
                Row::new([
                    Cell::from("Open config"),
                    Cell::from(Text::raw("ctrl+k").alignment(ratatui::layout::Alignment::Right)),
                ]),
            ]
        );
    }
}
