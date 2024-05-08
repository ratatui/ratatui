/// A macro for creating a [`Line`] using vec! syntax.
///
/// `line!` is similar to the [`vec!`] macro, but it returns a [`Line`] instead of a `Vec`.
///
/// # Examples
///
/// * Create a [`Line`] containing a vector of [`Span`]s:
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::line;
///
/// let line = line!["hello", "world"];
/// let line = line!["hello".red(), "world".red().bold()];
/// ```
///
/// * Create a [`Line`] from a given [`Span`] repeated some amount of times:
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::line;
///
/// let line = line!["hello"; 2];
/// ```
///
/// * Use [`span!`] macro inside [`line!`] macro for formatting.
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::{line, span};
///
/// let line = line![span!("hello {}", "world"), span!(Modifier::BOLD; "goodbye {}", "world")];
/// ```
///
/// [`Line`]: crate::text::Line
/// [`Span`]: crate::text::Span
#[macro_export]
macro_rules! line {
    () => {
        ratatui::text::Line::default()
    };
    ($span:expr; $n:expr) => {
      ratatui::text::Line::from(vec![$span.into(); $n])
    };
    ($($span:expr),+ $(,)?) => {{
        ratatui::text::Line::from(vec![
        $(
            $span.into(),
        )+
        ])
    }};
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;

    #[test]
    fn line() {
        // literal
        let line = line!["hello", "world"];
        assert_eq!(line, Line::from(vec!["hello".into(), "world".into()]));

        // raw instead line
        let line = line![Span::raw("hello"), "world"];
        assert_eq!(line, Line::from(vec!["hello".into(), "world".into()]));

        // vec count syntax
        let line = line!["hello"; 2];
        assert_eq!(line, Line::from(vec!["hello".into(), "hello".into()]));

        // vec count syntax with span
        let line = line![crate::span!("hello"); 2];
        assert_eq!(line, Line::from(vec!["hello".into(), "hello".into()]));
    }
}
