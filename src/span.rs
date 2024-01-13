/// A macro for creating a raw (unstyled) [`Span`] using formatting syntax.
///
/// `raw!` is similar to the [`format!`] macro, but it returns a [`Span`] instead of a `String`.
///
/// # Examples
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::raw;
///
/// let content = "content";
///
/// let span = raw!("test content");
/// let span = raw!("test {}", "content");
/// let span = raw!("{} {}", "test", "content");
/// let span = raw!("test {content}");
/// let span = raw!("test {content}", content = "content");
///
/// // with format specifiers
/// let span = raw!("test {:4}", 123);
/// let span = raw!("test {:04}", 123);
/// ```
///
/// [`Span`]: crate::text::Span
#[macro_export]
macro_rules! raw {
    ($($arg:tt)*) => {
        ratatui::text::Span::raw(format!($($arg)*))
    };
}

/// A macro for creating a styled [`Span`] using formatting syntax.
///
/// `styled!` is similar to the [`format!`] macro, but it returns a [`Span`] instead of a `String`.
///
/// The first argument is a [`Style`] or any type that is convertible to [`Style`] (e.g. [`Color`]).
///
/// # Examples
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::styled;
///
/// let content = "content";
///
/// // styled
/// let style = Style::new().green();
/// let span = styled!(style, "test content");
/// let span = styled!(style, "test {}", "content");
/// let span = styled!(style, "{} {}", "test", "content");
/// let span = styled!(style, "test {content}");
/// let span = styled!(style, "test {content}", content = "content");
///
/// // accepts any type that is convertible to Style
/// let span = styled!(Style::new().green(), "test {content}");
/// let span = styled!(Color::Green, "test {content}");
/// let span = styled!(Modifier::BOLD, "test {content}");
///
/// // with format specifiers
/// let span = styled!(style, "test {:4}", 123);
/// let span = styled!(style, "test {:04}", 123);
/// ```
///
/// [`Color`]: crate::style::Color
/// [`Style`]: crate::style::Style
/// [`Span`]: crate::text::Span
#[macro_export]
macro_rules! styled {
    ($style:expr, $($arg:tt)*) => {
        ratatui::text::Span::styled(format!($($arg)*), $style)
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
        let span = raw!("test content");
        assert_eq!(span, Span::raw("test content"));

        // string
        let span = raw!("test {}", "content");
        assert_eq!(span, Span::raw("test content"));

        // string variable
        let span = raw!("test {}", content);
        assert_eq!(span, Span::raw("test content"));

        // string variable in the format string
        let span = raw!("test {content}");
        assert_eq!(span, Span::raw("test content"));

        // named variable
        let span = raw!("test {content}", content = "content");
        assert_eq!(span, Span::raw("test content"));

        // named variable pointing at a local variable
        let span = raw!("test {content}", content = content);
        assert_eq!(span, Span::raw("test content"));

        // two strings
        let span = raw!("{} {}", "test", "content");
        assert_eq!(span, Span::raw("test content"));

        // two string variables
        let span = raw!("{test} {content}");
        assert_eq!(span, Span::raw("test content"));

        // a number
        let span = raw!("test {number}");
        assert_eq!(span, Span::raw("test 123"));

        // a number with a format specifier
        let span = raw!("test {number:04}");
        assert_eq!(span, Span::raw("test 0123"));
    }

    #[test]
    fn styled() {
        const STYLE: Style = Style::new().fg(Color::Green);

        let test = "test";
        let content = "content";
        let number = 123;

        // literal
        let span = styled!(STYLE, "test content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // string
        let span = styled!(STYLE, "test {}", "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // string variable
        let span = styled!(STYLE, "test {}", content);
        assert_eq!(span, Span::styled("test content", STYLE));

        // string variable in the format string
        let span = styled!(STYLE, "test {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        // named variable
        let span = styled!(STYLE, "test {content}", content = "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // named variable pointing at a local variable
        let span = styled!(STYLE, "test {content}", content = content);
        assert_eq!(span, Span::styled("test content", STYLE));

        // two strings
        let span = styled!(STYLE, "{} {}", "test", "content");
        assert_eq!(span, Span::styled("test content", STYLE));

        // two string variables
        let span = styled!(STYLE, "{test} {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        // a number
        let span = styled!(STYLE, "test {number}");
        assert_eq!(span, Span::styled("test 123", STYLE));

        // a number with a format specifier
        let span = styled!(STYLE, "test {number:04}");
        assert_eq!(span, Span::styled("test 0123", STYLE));

        // accepts any type that is convertible to Style
        let span = styled!(Color::Green, "test {content}");
        assert_eq!(span, Span::styled("test content", STYLE));

        let span = styled!(Modifier::BOLD, "test {content}");
        assert_eq!(span, Span::styled("test content", Style::new().bold()));
    }
}
