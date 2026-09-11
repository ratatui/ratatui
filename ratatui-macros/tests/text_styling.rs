#![deny(deprecated)]

use ratatui_core::style::{Color, Modifier, Style, Stylize};
use ratatui_core::text::{Line, Span, Text};
use ratatui_macros::{line, span, text};

#[test]
fn span_formats_styled_content() {
    let name = "world";
    let number = 42;
    assert_eq!(
        span!(Color::Blue => "hello"),
        Span::styled("hello", Color::Blue)
    );
    assert_eq!(span!(Color::Blue => name), Span::styled(name, Color::Blue));
    assert_eq!(
        span!(Color::Blue => number),
        Span::styled("42", Color::Blue)
    );
    assert_eq!(
        span!(Style::new().blue().bold() => "hello {name}: {number:04}"),
        Span::styled("hello world: 0042", Style::new().blue().bold())
    );
    assert_eq!(
        span!(Modifier::BOLD => "{} {value:04}", name, value = number,),
        Span::styled("world 0042", Modifier::BOLD)
    );
}

#[test]
#[allow(deprecated)]
fn span_preserves_semicolon_syntax() {
    let name = "world";
    assert_eq!(span!(Color::Blue; "hello"), span!(Color::Blue => "hello"));
    assert_eq!(span!(Color::Blue; name), span!(Color::Blue => name));
    assert_eq!(
        span!(Modifier::BOLD; "{name}: {value:04}", value = 42),
        span!(Modifier::BOLD => "{name}: {value:04}", value = 42)
    );
}

#[test]
fn line_styles_empty_single_and_multiple_spans() {
    assert_eq!(line![Color::Blue =>], Line::default().blue());
    assert_eq!(line![Modifier::BOLD => "hello"], Line::from("hello").bold());
    let line = line![Color::Blue => "hello ", "world".red(),];
    assert_eq!(line.style, Style::new().blue());
    assert_eq!(
        line.spans,
        vec![Span::raw("hello "), Span::raw("world").red()]
    );
    let graphemes: Vec<_> = line.styled_graphemes(Style::default()).collect();
    assert_eq!(graphemes[0].style, Style::new().blue());
    assert_eq!(graphemes[6].style, Style::new().red());
}

#[test]
fn text_styles_empty_single_and_multiple_lines() {
    assert_eq!(text![Color::Blue =>], Text::default().blue());
    assert_eq!(text![Modifier::BOLD => "hello"], Text::from("hello").bold());
    let text = text![Color::Blue => "hello", line![Color::Red => "world"],];
    assert_eq!(text.style, Style::new().blue());
    assert_eq!(
        text.lines,
        vec![Line::from("hello"), Line::from("world").red()]
    );
}

#[test]
fn styled_repetition_preserves_child_styles() {
    let count = 3;
    let line = line![Modifier::BOLD => span!(Color::Red => "hello"); count];
    assert_eq!(line.style, Style::new().bold());
    assert_eq!(line.spans, vec![Span::raw("hello").red(); count]);
    let text = text![Modifier::BOLD => line![Color::Red => "hello"]; count];
    assert_eq!(text.style, Style::new().bold());
    assert_eq!(text.lines, vec![Line::from("hello").red(); count]);
    assert_eq!(line![Color::Blue => "hello"; 0], Line::default().blue());
    assert_eq!(text![Color::Blue => "hello"; 0], Text::default().blue());
}

#[test]
fn styles_are_evaluated_once_before_content() {
    let mut events = Vec::new();
    let _ = span!({ events.push("style"); Color::Blue } => {
        events.push("content"); "hello"
    });
    assert_eq!(events, ["style", "content"]);

    events.clear();
    let _ = line!({ events.push("style"); Color::Blue } => {
        events.push("content"); "hello"
    }; { events.push("count"); 3 });
    assert_eq!(events, ["style", "content", "count"]);

    events.clear();
    let _ = text!({ events.push("style"); Color::Blue } => {
        events.push("content"); "hello"
    }; { events.push("count"); 0 });
    assert_eq!(events, ["style", "content", "count"]);
}
