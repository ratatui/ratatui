/// A macro for creating a [`Span`] using formatting syntax.
///
/// `span!` is similar to the [`format!`] macro, but it returns a [`Span`] instead of a `String`.
///
/// If semicolon follows the first argument, then the first argument is a [`Style`] and a styled [`Span`] will be created.
/// Otherwise, the [`Span`] will be created as a raw span (i.e. with style set to `Style::default()`).
///
/// # Examples
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::span;
///
/// let content = "content";
///
/// let span = span!("test content");
/// let span = span!("test {}", "content");
/// let span = span!("{} {}", "test", "content");
/// let span = span!("test {content}");
/// let span = span!("test {content}", content = "content");
///
/// // with format specifiers
/// let span = span!("test {:4}", 123);
/// let span = span!("test {:04}", 123);
///
/// let style = Style::new().green();
/// let span = span!(style; "test content");
/// let span = span!(style; "test {}", "content");
/// let span = span!(style; "{} {}", "test", "content");
/// let span = span!(style; "test {content}");
/// let span = span!(style; "test {content}", content = "content");
///
/// // accepts any type that is convertible to Style
/// let span = span!(Style::new().green(); "test {content}");
/// let span = span!(Color::Green; "test {content}");
/// let span = span!(Modifier::BOLD; "test {content}");
///
/// // with format specifiers
/// let span = span!(style; "test {:4}", 123);
/// let span = span!(style; "test {:04}", 123);
/// ```
///
/// [`Color`]: crate::style::Color
/// [`Style`]: crate::style::Style
/// [`Span`]: crate::text::Span
/// [`Style`]: crate::style::Style
#[macro_export]
macro_rules! span {
    ($style:expr; $($arg:tt)*) => {
        ratatui::text::Span::styled(format!($($arg)*), $style)
    };
    ($($arg:tt)*) => {
        ratatui::text::Span::raw(format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;

    #[test]
    fn raw() {
        let test = "test";
        let content = "content";
        let number = 123;

        // literal
        let span = span!("test content");
        assert_eq!(span, Span::raw("test content"));

        // string
        let span = span!("test {}", "content");
        assert_eq!(span, Span::raw("test content"));

        // string variable
        let span = span!("test {}", content);
        assert_eq!(span, Span::raw("test content"));

        // string variable in the format string
        let span = span!("test {content}");
        assert_eq!(span, Span::raw("test content"));

        // named variable
        let span = span!("test {content}", content = "content");
        assert_eq!(span, Span::raw("test content"));

        // named variable pointing at a local variable
        let span = span!("test {content}", content = content);
        assert_eq!(span, Span::raw("test content"));

        // two strings
        let span = span!("{} {}", "test", "content");
        assert_eq!(span, Span::raw("test content"));

        // two string variables
        let span = span!("{test} {content}");
        assert_eq!(span, Span::raw("test content"));

        // a number
        let span = span!("test {number}");
        assert_eq!(span, Span::raw("test 123"));

        // a number with a format specifier
        let span = span!("test {number:04}");
        assert_eq!(span, Span::raw("test 0123"));
    }

    #[test]
    fn styled() {
        const STYLE: Style = Style::new().fg(Color::Green);

        let test = "test";
        let content = "content";
        let number = 123;

        // literal
        let span = span!(STYLE; "test content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // string
        let span = span!(STYLE; "test {}", "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // string variable
        let span = span!(STYLE; "test {}", content);
        assert_eq!(span, Span::styled("test content", STYLE));

        // string variable in the format string
        let span = span!(STYLE; "test {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        // named variable
        let span = span!(STYLE; "test {content}", content = "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // named variable pointing at a local variable
        let span = span!(STYLE; "test {content}", content = content);
        assert_eq!(span, Span::styled("test content", STYLE));

        // two strings
        let span = span!(STYLE; "{} {}", "test", "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // two string variables
        let span = span!(STYLE; "{test} {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        // a number
        let span = span!(STYLE; "test {number}");
        assert_eq!(span, Span::styled("test 123", STYLE));

        // a number with a format specifier
        let span = span!(STYLE; "test {number:04}");
        assert_eq!(span, Span::styled("test 0123", STYLE));

        // accepts any type that is convertible to Style
        let span = span!(Color::Green; "test {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        let span = span!(Modifier::BOLD; "test {content}");
        assert_eq!(span, Span::styled("test content", Style::new().bold()));
    }
}
