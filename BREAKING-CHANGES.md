# Breaking Changes

This document contains a list of breaking changes in each version and some notes to help migrate
between versions. It is compile manually from the commit history and changelog. We also tag PRs on
github with a [breaking change] label.

[breaking change]: (https://github.com/ratatui-org/ratatui/issues?q=label%3A%22breaking+change%22)

## Summary

This is a quick summary of the sections below:

- Unreleased (0.24.1)
  - `List::start_corner` is renamed to `List::direction`
  - `List::new()` now accepts `IntoIterator<Item = Into<ListItem<'a>>>`
  - `Table::new()` now requires specifying the widths
  - `Table::widths()` now accepts `IntoIterator<Item = AsRef<Constraint>>`
  - Layout::new() now accepts direction and constraint parameters
  - The default `Tabs::highlight_style` is now `Style::new().reversed()`

- [v0.24.0](#v0240)
  - MSRV is now 1.70.0
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

## Unreleased (v0.24.1)

### `List::start_corner` is renamed to `List::direction` ([#673])

[#673]: https://github.com/ratatui-org/ratatui/pull/673

Previously `List::start_corner` didn't communicate the intent of the method. It also used an
inadequate `Corner` enum. The method is now renamed `direction` and a new enum
`ListDirection` has been added.

```diff
- List::new(/* items */).start_corner(Corner::TopLeft);
- List::new(/* items */).start_corner(Corner::TopRight);
// This is not an error, BottomRight rendered top to bottom previously
- List::new(/* items */).start_corner(Corner::BottomRight);
// all becomes
+ List::new(/* items */).direction(ListDirection::TopToBottom);
```

```diff
- List::new(/* items */).start_corner(Corner::BottomLeft);
// becomes
+ List::new(/* items */).direction(ListDirection::BottomToTop);
```

### `List::new()` now accepts `IntoIterator<Item = Into<ListItem<'a>>>` ([#672])

[#672]: https://github.com/ratatui-org/ratatui/pull/672

Previously `List::new()` took `Into<Vec<ListItem<'a>>>`. This change will throw a compilation 
error for `IntoIterator`s with an indeterminate item (e.g. empty vecs).

E.g.

```diff
- let list = List::new(vec![]);
// becomes
+ let list = List::default();
```

### The default `Tabs::highlight_style` is now `Style::new().reversed()` ([#635])

Previously the default highlight style for tabs was `Style::default()`, which meant that a `Tabs`
widget in the default configuration would not show any indication of the selected tab.

[#635]: https://github.com/ratatui-org/ratatui/pull/635

### The default `Tabs::highlight_style` is now `Style::new().reversed()` ([#635])

Previously the default highlight style for tabs was `Style::default()`, which meant that a `Tabs`
widget in the default configuration would not show any indication of the selected tab.


### `Table::new()` now requires specifying the widths of the columns (#664)

[#664]: https://github.com/ratatui-org/ratatui/pull/664

Previously `Table`s could be constructed without widths. In almost all cases this is an error.
A new widths parameter is now mandatory on `Table::new()`. Existing code of the form:

```diff
- Table::new(rows).widths(widths)
```

Should be updated to:

```diff
+ Table::new(rows, widths)
```

For ease of automated replacement in cases where the amount of code broken by this change is large
or complex, it may be convenient to replace `Table::new` with `Table::default().rows`.

```diff
- Table::new(rows).block(block).widths(widths);
// becomes
+ Table::default().rows(rows).widths(widths)
```

### `Table::widths()` now accepts `IntoIterator<Item = AsRef<Constraint>>` ([#663])

[#663]: https://github.com/ratatui-org/ratatui/pull/663

Previously `Table::widths()` took a slice (`&'a [Constraint]`). This change will introduce clippy
`needless_borrow` warnings for places where slices are passed to this method. To fix these, remove
the `&`.

E.g.

```diff
- let table = Table::new(rows).widths(&[Constraint::Length(1)]);
// becomes
+ let table = Table::new(rows).widths([Constraint::Length(1)]);
```

### Layout::new() now accepts direction and constraint parameters ([#557])

[#557]: https://github.com/ratatui-org/ratatui/pull/557

Previously layout new took no parameters. Existing code should either use `Layout::default()` or
the new constructor.

```rust
let layout = layout::new()
  .direction(Direction::Vertical)
  .constraints([Constraint::Min(1), Constraint::Max(2)]);
// becomes either
let layout = layout::default()
  .direction(Direction::Vertical)
  .constraints([Constraint::Min(1), Constraint::Max(2)]);
// or
let layout = layout::new(Direction::Vertical, [Constraint::Min(1), Constraint::Max(2)]);
```

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

```diff
- let line_set: symbols::line::Set = BorderType::line_symbols(BorderType::Plain);
// becomes
+ let border_set: symbols::border::Set = BorderType::border_symbols(BorderType::Plain);
```

### Generic `Backend` parameter removed from `Frame` ([#530])

[#530]: https://github.com/ratatui-org/ratatui/issues/530

`Frame` is no longer generic over Backend. Code that accepted `Frame<Backend>` will now need to
accept `Frame`. To migrate existing code, remove any generic parameters from code that uses an
instance of a Frame. E.g.:

```diff
- fn ui<B: Backend>(frame: &mut Frame<B>) { ... }
// becomes
+ fn ui(frame: Frame) { ... }
```

### `Stylize` shorthands now consume rather than borrow `String` ([#466])

[#466]: https://github.com/ratatui-org/ratatui/issues/466

In order to support using `Stylize` shorthands (e.g. `"foo".red()`) on temporary `String` values, a
new implementation of `Stylize` was added that returns a `Span<'static>`. This causes the value to
be consumed rather than borrowed. Existing code that expects to use the string after a call will no
longer compile. E.g.

```diff
- let s = String::new("foo");
- let span1 = s.red();
- let span2 = s.blue(); // will no longer compile as s is consumed by the previous line
// becomes
+ let span1 = s.clone().red();
+ let span2 = s.blue();
```

### Deprecated `Spans` type removed (replaced with `Line`) ([#426])

[#426]: https://github.com/ratatui-org/ratatui/issues/426

`Spans` was replaced with `Line` in 0.21.0. `Buffer::set_spans` was replaced with
`Buffer::set_line`.

```diff
- let spans = Spans::from(some_string_str_span_or_vec_span);
- buffer.set_spans(0, 0, spans, 10);
// becomes
+ let line - Line::from(some_string_str_span_or_vec_span);
+ buffer.set_line(0, 0, line, 10);
```

## [v0.23.0](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0)

### `Scrollbar::track_symbol()` now takes an `Option<&str>` instead of `&str` ([#360])

[#360]: https://github.com/ratatui-org/ratatui/issues/360

The track symbol of `Scrollbar` is now optional, this method now takes an optional value.

```diff
- let scrollbar = Scrollbar::default().track_symbol("|");
// becomes
+ let scrollbar = Scrollbar::default().track_symbol(Some("|"));
```

### `Scrollbar` symbols moved to `symbols::scrollbar` and `widgets::scrollbar` module is private ([#330])

[#330]: https://github.com/ratatui-org/ratatui/issues/330

The symbols for defining scrollbars have been moved to the `symbols` module from the
`widgets::scrollbar` module which is no longer public. To update your code update any imports to the
new module locations. E.g.:

```diff
- use ratatui::{widgets::scrollbar::{Scrollbar, Set}};
// becomes
+ use ratatui::{widgets::Scrollbar, symbols::scrollbar::Set} 
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

```diff
let terminal = Terminal::with_options(backend, TerminalOptions {
-    viewport: Viewport::fixed(area),
});
// becomes
let terminal = Terminal::with_options(backend, TerminalOptions {
+    viewport: Viewport::Fixed(area),
});
```

### Code that binds `Into<Text<'a>>` now requires type annotations ([#168])

[#168]: https://github.com/ratatui-org/ratatui/issues/168

A new type `Masked` was introduced that implements `From<Text<'a>>`. This causes any code that did
previously did not need to use type annotations to fail to compile.  To fix this, annotate or call
to_string() / to_owned() / as_str() on the value. E.g.:

```diff
- let paragraph = Paragraph::new("".as_ref());
// becomes
+ let paragraph = Paragraph::new("".as_str());
```

### `Marker::Block` now renders as a block rather than a bar character ([#133])

[#133]: https://github.com/ratatui-org/ratatui/issues/133

Code using the `Block` marker that previously rendered using a half block character (`'▀'``) now
renders using the full block character (`'█'`). A new marker variant`Bar` is introduced to replace
the existing code.

```diff
- let canvas = Canvas::default().marker(Marker::Block);
// becomes
+ let canvas = Canvas::default().marker(Marker::Bar);
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
