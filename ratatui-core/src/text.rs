//! Primitives for styled text.
//!
//! A terminal UI is at its root a lot of strings. In order to make it accessible and stylish, those
//! strings may be associated to a set of styles. `ratatui` has three ways to represent them:
//! - A single line string where all graphemes have the same style is represented by a [`Span`].
//! - A single line string where each grapheme may have its own style is represented by [`Line`].
//! - A multiple line string where each grapheme may have its own style is represented by a
//!   [`Text`].
//!
//! These types form a hierarchy: [`Line`] is a collection of [`Span`] and each line of [`Text`] is
//! a [`Line`].
//!
//! Keep it mind that a lot of widgets will use those types to advertise what kind of string is
//! supported for their properties. Moreover, `ratatui` provides convenient `From` implementations
//! so that you can start by using simple `String` or `&str` and then promote them to the previous
//! primitives when you need additional styling capabilities.
//!
//! For example, for the `Block` widget, all the following calls are valid to set its `title`
//! property (which is a [`Line`] under the hood):
//!
//! ```rust,ignore
//! use ratatui_core::{
//!     style::{Color, Style},
//!     text::{Line, Span},
//!     widgets::Block,
//! };
//!
//! // A simple string with no styling.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { .. } }
//! // ])
//! let block = Block::new().title("My title");
//!
//! // A simple string with a unique style.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { fg: Some(Color::Yellow), .. }
//! // ])
//! let block = Block::new().title(Span::styled("My title", Style::default().fg(Color::Yellow)));
//!
//! // A string with multiple styles.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My"), style: Style { fg: Some(Color::Yellow), .. } },
//! //   Span { content: Cow::Borrowed(" title"), .. }
//! // ])
//! let block = Block::new().title(vec![
//!     Span::styled("My", Style::default().fg(Color::Yellow)),
//!     Span::raw(" title"),
//! ]);
//! ```
//!
//! # Storing styled text
//!
//! The lifetime on [`Span`], [`Line`], and [`Text`] is the lifetime of borrowed string content.
//! Each span stores its content as a [`Cow<'a, str>`](alloc::borrow::Cow), so it either borrows a
//! `&'a str` or owns a `String`. Text that owns all of its strings has the type `Text<'static>`
//! (and likewise `Line<'static>` and `Span<'static>`). It can be stored in your application state
//! without adding a lifetime parameter to your own types.
//!
//! Building text from a `String`, for example the result of [`format!`](alloc::format!), produces
//! owned content, so the result is already `'static`. Text that borrows from data you don't keep,
//! for example a line built from a `&str` field of a request, can be converted with
//! [`Span::into_owned`], [`Line::into_owned`], or [`Text::into_owned`]. These copy only the content
//! that is borrowed and keep all styles. [`Clone`] does not do this: a cloned `Text<'a>` still
//! borrows and keeps the `'a` lifetime.
//!
//! ```rust
//! use ratatui_core::style::Stylize;
//! use ratatui_core::text::{Line, Text};
//!
//! struct App {
//!     status: Line<'static>,
//!     log: Text<'static>,
//! }
//!
//! let count = 3;
//! let mut app = App {
//!     status: Line::from(format!("{count} items")),
//!     log: Text::default(),
//! };
//!
//! let name = String::from("inbox");
//! let borrowed = Line::from(vec!["loaded ".into(), name.as_str().bold()]);
//! app.log.push_line(borrowed.into_owned());
//! drop(name);
//!
//! assert_eq!(app.log.to_string(), "loaded inbox");
//! ```
//!
//! # Rendering stored text
//!
//! [`Text`], [`Line`], and [`Span`] implement [`Widget`](crate::widgets::Widget) for references,
//! so stored text can be rendered without cloning it:
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::text::Text;
//! use ratatui_core::widgets::Widget;
//!
//! struct App {
//!     log: Text<'static>,
//! }
//!
//! impl Widget for &App {
//!     fn render(self, area: Rect, buf: &mut Buffer) {
//!         (&self.log).render(area, buf);
//!     }
//! }
//!
//! let app = App {
//!     log: Text::from(String::from("hello")),
//! };
//! let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
//! app.render(buf.area, &mut buf);
//! ```
//!
//! Widgets that take text, such as `Paragraph`, accept anything that converts into a [`Text`].
//! A `&Text`, `&Line`, or `&Span` converts into a new value that borrows its content from the
//! original (see the `From<&Text>` impl on [`Text`]). This allocates new `Vec`s for the lines and
//! spans but copies no string data, so it is cheaper than cloning stored text that owns its
//! strings. The usual pattern is to keep the styled text in your state and build the widget from a
//! reference to it in each frame, for example `Paragraph::new(&app.log)`. A widget you keep around
//! can also be rendered by reference, because `Paragraph` implements `Widget` for `&Paragraph`.

mod grapheme;
pub use grapheme::StyledGrapheme;

mod line;
pub use line::{Line, ToLine};

mod masked;
pub use masked::Masked;

mod span;
pub use span::{Span, ToSpan};

mod text;
pub use text::{Text, ToText};
