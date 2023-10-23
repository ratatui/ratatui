# Breaking Changes

This document contains a list of breaking changes in each version and some notes to help migrate
between versions. It is compile manually from the commit history and changelog. We also tag PRs on
github with a [breaking change] label.

[breaking change]: (https://github.com/ratatui-org/ratatui/issues?q=label%3A%22breaking+change%22)

## Summary

This is a quick summary of the sections below:

- [v0.24.0](#v0240)
  - `ScrollbarState`: `position`, `content_length`, and `viewport_content_length` are now `usize`
  - `BorderType`: `line_symbols` is now `border_symbols` and returns `symbols::border::set`
  - `Frame<'a, B: Backend>` is now `Frame<'a>`
  - `Stylize` shorthands for `String` now consume the value and return `Span<'static>`
  - `Spans` is removed
- [v0.23.0](#v0230)
  - `Scrollbar`: `track_symbol` now takes `Option<&str>`
  - `Scrollbar`: symbols moved to `symbols` module
  - MSRV is now 1.67.0
- [v0.22.0](#v0220)
  - serde representation of `Borders` and `Modifiers` has changed
- [v0.21.0](#v0210)
  - MSRV is now 1.65.0
  - `terminal::ViewPort` is now an enum
  - `"".as_ref()` must be annotated to implement `Into<Text<'a>>`
  - `Marker::Block` renders as a block char instead of a bar char
- [v0.20.0](#v0200)
  - MSRV is now 1.63.0
  - `List` no longer ignores empty strings

## [v0.24.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.24.0)

### ScrollbarState field type changed from `u16` to `usize` ([#456])

[#456]: https://github.com/ratatui-org/ratatui/pull/456

In order to support larger content lengths, the `position`, `content_length` and
`viewport_content_length` methods on `ScrollbarState` now take `usize` instead of `u16`

### `BorderType::line_symbols` renamed to `border_symbols` ([#529])

[#529]: https://github.com/ratatui-org/ratatui/issues/529

Applications can now set custom borders on a `Block` by calling `border_set()`. The
`BorderType::line_symbols()` is renamed to `border_symbols()` and now returns a new struct
`symbols::border::Set`. E.g.:

```rust
let line_set: symbols::line::Set = BorderType::line_symbols(BorderType::Plain);
// becomes
let border_set: symbols::border::Set = BorderType::border_symbols(BorderType::Plain);
```

### Generic `Backend` parameter removed from `Frame` ([#530])

[#530]: https://github.com/ratatui-org/ratatui/issues/530

`Frame` is no longer generic over Backend. Code that accepted `Frame<Backend>` will now need to
accept `Frame`. To migrate existing code, remove any generic parameters from code that uses an
instance of a Frame. E.g.:

```rust
fn ui<B: Backend>(frame: &mut Frame<B>) { ... }
// becomes
fn ui(frame: Frame) { ... }
```

### `Stylize` shorthands now consume rather than borrow `String` ([#466])

[#466]: https://github.com/ratatui-org/ratatui/issues/466

In order to support using `Stylize` shorthands (e.g. `"foo".red()`) on temporary `String` values, a
new implementation of `Stylize` was added that returns a `Span<'static>`. This causes the value to
be consumed rather than borrowed. Existing code that expects to use the string after a call will no
longer compile. E.g.

```rust
let s = String::new("foo");
let span1 = s.red();
let span2 = s.blue(); // will no longer compile as s is consumed by the previous line
// becomes
let span1 = s.clone().red();
let span2 = s.blue();
```

### Deprecated `Spans` type removed (replaced with `Line`) ([#426])

[#426]: https://github.com/ratatui-org/ratatui/issues/426

`Spans` was replaced with `Line` in 0.21.0. `Buffer::set_spans` was replaced with
`Buffer::set_line`.

```rust
let spans = Spans::from(some_string_str_span_or_vec_span);
buffer.set_spans(0, 0, spans, 10);
// becomes
let line - Line::from(some_string_str_span_or_vec_span);
buffer.set_line(0, 0, line, 10);
```

## [v0.23.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0)

### `Scrollbar::track_symbol()` now takes an `Option<&str>` instead of `&str` ([#360])

[#360]: https://github.com/ratatui-org/ratatui/issues/360

The track symbol of `Scrollbar` is now optional, this method now takes an optional value.

```rust
let scrollbar = Scrollbar::default().track_symbol("|");
// becomes
let scrollbar = Scrollbar::default().track_symbol(Some("|"));
```

### `Scrollbar` symbols moved to `symbols::scrollbar` and `widgets::scrollbar` module is private ([#330])

[#330]: https://github.com/ratatui-org/ratatui/issues/330

The symbols for defining scrollbars have been moved to the `symbols` module from the
`widgets::scrollbar` module which is no longer public. To update your code update any imports to the
new module locations. E.g.:

```rust
use ratatui::{widgets::scrollbar::{Scrollbar, Set}};
// becomes
use ratatui::{widgets::Scrollbar, symbols::scrollbar::Set} 
```

### MSRV updated to 1.67 ([#361])

[#361]: https://github.com/ratatui-org/ratatui/issues/361

The MSRV of ratatui is now 1.67 due to an MSRV update in a dependency (`time`).

## [v0.22.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.22.0)

### bitflags updated to 2.3 ([#205])

[#205]: https://github.com/ratatui-org/ratatui/issues/205

The serde representation of bitflags has changed. Any existing serialized types that have Borders or
Modifiers will need to be re-serialized. This is documented in the [bitflags
changelog](https://github.com/bitflags/bitflags/blob/main/CHANGELOG.md#200-rc2)..

## [v0.21.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.21.0)

### MSRV is 1.65.0  ([#171])

[#171]: https://github.com/ratatui-org/ratatui/issues/171

The minimum supported rust version is now 1.65.0.

### `Terminal::with_options()` stabilized to allow configuring the viewport ([#114])

[#114]: https://github.com/ratatui-org/ratatui/issues/114

In order to support inline viewports, the unstable method `Terminal::with_options()` was stabilized
and  `ViewPort` was changed from a struct to an enum.

```rust
let terminal = Terminal::with_options(backend, TerminalOptions {
    viewport: Viewport::fixed(area),
});
// becomes
let terminal = Terminal::with_options(backend, TerminalOptions {
    viewport: Viewport::Fixed(area),
});
```

### Code that binds `Into<Text<'a>>` now requires type annotations ([#168])

[#168]: https://github.com/ratatui-org/ratatui/issues/168

A new type `Masked` was introduced that implements `From<Text<'a>>`. This causes any code that did
previously did not need to use type annotations to fail to compile.  To fix this, annotate or call
to_string() / to_owned() / as_str() on the value. E.g.:

```rust
let paragraph = Paragraph::new("".as_ref());
// becomes
let paragraph = Paragraph::new("".as_str());
```

### `Marker::Block` now renders as a block rather than a bar character ([#133])

[#133]: https://github.com/ratatui-org/ratatui/issues/133

Code using the `Block` marker that previously rendered using a half block character (`'▀'``) now
renders using the full block character (`'█'`). A new marker variant`Bar` is introduced to replace
the existing code.

```rust
let canvas = Canvas::default().marker(Marker::Block);
// becomes
let canvas = Canvas::default().marker(Marker::Bar);
```

## [v0.20.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.20.0)

v0.20.0 was the first release of Ratatui - versions prior to this were release as tui-rs. See the
[Changelog](./CHANGELOG.md) for more details.

### MSRV is update to 1.63.0 ([#80])

[#80]: https://github.com/ratatui-org/ratatui/issues/80

The minimum supported rust version is 1.63.0

### List no longer ignores empty string in items ([#42])

[#42]: https://github.com/ratatui-org/ratatui/issues/42

The following code now renders 3 items instead of 2. Code which relies on the previous behavior will
need to manually filter empty items prior to display.

```rust
let items = vec![
    ListItem::new("line one"),
    ListItem::new(""),
    ListItem::new("line four"),
];
```
