#![deny(deprecated)]

use ratatui_core::style::Color;
use ratatui_macros::span;

fn main() {
    let content = "hello";
    let _ = span!(Color::Blue; "hello");
    let _ = span!(Color::Blue; "hello {}", "world");
    let _ = span!(Color::Blue; content);
}
