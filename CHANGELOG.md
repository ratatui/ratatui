# Changelog

## To be released

## v0.9.0 - 2020-04-14

### Features

* Introduce stateful widgets, i.e widgets that can take advantage of keeping
some state around between two draw calls (#210 goes a bit more into the
details).
* Allow a `Table` row to be selected.
```rust
// State initialization
let mut state = TableState::default();

// In the terminal.draw closure
let header = ["Col1", "Col2", "Col"];
let rows = [
  Row::Data(["Row11", "Row12", "Row13"].into_iter())
];
let table = Table::new(header.into_iter(), rows.into_iter());
f.render_stateful_widget(table, area, &mut state);

// In response to some event:
state.select(Some(1));
```
* Add a way to choose the type of border used to draw a block. You can now
choose from plain, rounded, double and thick lines.
* Add a `graph_type` property on the `Dataset` of a `Chart` widget. By
default it will be `Scatter` where the points are drawn as is. An other
option is `Line` where a line will be draw between each consecutive points
of the dataset.
* Style methods are now const, allowing you to initialize const `Style`
objects.
* Improve control over whether the legend in the `Chart` widget is shown or
not. You can now set custom constraints using
`Chart::hidden_legend_constraints`.
* Add `Table::header_gap` to add some space between the header and the first
row.
* Remove `log` from the dependencies
* Add a way to use a restricted set of unicode symbols in several widgets to
improve portability in exchange of a degraded output. (see `BarChart::bar_set`,
`Sparkline::bar_set` and `Canvas::marker`). You can check how the
`--enhanced-graphics` flag is used in the demos.

### Breaking Changes

* `Widget::render` has been deleted. You should now use `Frame::render_widget`
to render a widget on the corresponding `Frame`. This makes the `Widget`
implementation totally decoupled from the `Frame`.
```rust
// Before
Block::default().render(&mut f, size);

// After
let block = Block::default();
f.render_widget(block, size);
```
* `Widget::draw` has been renamed to `Widget::render` and the signature has
been updated to reflect that widgets are consumable objects. Thus the method
takes `self` instead of `&mut self`.
```rust
// Before
impl Widget for MyWidget {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
  }
}

/// After
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
  }
}
```
* `Widget::background` has been replaced by `Buffer::set_background`
```rust
// Before
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
    self.background(area, buf, self.style.bg);
  }
}

// After
impl Widget for MyWidget {
  fn render(self, arera: Rect, buf: &mut Buffer) {
    buf.set_background(area, self.style.bg);
  }
}
```
* Update the `Shape` trait for objects that can be draw on a `Canvas` widgets.
Instead of returning an iterator over its points, a `Shape` is given a
`Painter` object that provides a `paint` as well as a `get_point` method. This
gives the `Shape` more information about the surface it will be drawn to. In
particular, this change allows the `Line` shape to use a more precise and
efficient drawing algorithm (Bresenham's line algorithm).
* `SelectableList` has been deleted. You can now take advantage of the
associated `ListState` of the `List` widget to select an item.
```rust
// Before
List::new(&["Item1", "Item2", "Item3"])
  .select(Some(1))
  .render(&mut f, area);

// After

// State initialization
let mut state = ListState::default();

// In the terminal.draw closure
let list = List::new(&["Item1", "Item2", "Item3"]);
f.render_stateful_widget(list, area, &mut state);

// In response to some events
state.select(Some(1));
```
* `widgets::Marker` has been moved to `symbols::Marker`

## v0.8.0 - 2019-12-15

### Breaking Changes

* Bump crossterm to 0.14.
* Add cross symbol to the symbols list.

### Bug Fixes

* Use the value of `title_style` to style the title of `Axis`.

## v0.7.0 - 2019-11-29

### Breaking Changes

* Use `Constraint` instead of integers to specify the widths of the `Table`
widget's columns. This will allow more responsive tables.

```rust
Table::new(header, row)
  .widths(&[15, 15, 10])
  .render(f, chunk);
```

becomes:

```rust
Table::new(header, row)
  .widths(&[
    Constraint::Length(15),
    Constraint::Length(15),
    Constraint::Length(10),
  ])
  .render(f, chunk);
```

* Bump crossterm to 0.13.
* Use Github Actions for CI (Travis and Azure Pipelines integrations have been deleted).

### Features

* Add support for horizontal and vertical margins in `Layout`.

## v0.6.2 - 2019-07-16

### Features

* `Text` implements PartialEq

### Bug Fixes

* Avoid overflow errors in canvas

## v0.6.1 - 2019-06-16

### Bug Fixes

* Avoid a division by zero when all values in a barchart are equal to 0.
* Fix the inverted cursor position in the curses backend.
* Ensure that the correct terminal size is returned when using the crossterm
backend.
* Avoid highlighting the separator after the selected item in the Tabs widget.

## v0.6.0 - 2019-05-18

### Breaking Changes

* Update crossterm backend

## v0.5.1 - 2019-04-14

### Bug Fixes

* Fix a panic in the Sparkline widget

## v0.5.0 - 2019-03-10

### Features

* Add a new curses backend (with Windows support thanks to `pancurses`).
* Add `Backend::get_cursor` and `Backend::set_cursor` methods to query and
set the position of the cursor.
* Add more constructors to the `Crossterm` backend.
* Add a demo for all backends using a shared UI and application state.
* Add `Ratio` as a new variant of layout `Constraint`. It can be used to define
exact ratios constraints.

### Breaking Changes

* Add support for multiple modifiers on the same `Style` by changing `Modifier`
from an enum to a bitflags struct.

So instead of writing:

```rust
let style = Style::default().modifier(Modifier::Italic);
```

one should use:

```rust
let style = Style::default().modifier(Modifier::ITALIC);
// or
let style = Style::default().modifier(Modifier::ITALIC | Modifier::BOLD);
```

### Bug Fixes

* Ensure correct behavoir of the alternate screens with the `Crossterm` backend.
* Fix out of bounds panic when two `Buffer` are merged.

## v0.4.0 - 2019-02-03

### Features

* Add a new canvas shape: `Rectangle`.
* Official support of `Crossterm` backend.
* Make it possible to choose the divider between `Tabs`.
* Add word wrapping on Paragraph.
* The gauge widget accepts a ratio (f64 between 0 and 1) in addition of a
percentage.

### Breaking Changes

* Upgrade to Rust 2018 edition.

### Bug Fixes

* Fix rendering of double-width characters.
* Fix race condition on the size of the terminal and expose a size that is
safe to use when drawing through `Frame::size`.
* Prevent unsigned int overflow on large screens.

## v0.3.0 - 2018-11-04

### Features

* Add experimental test backend

## v0.3.0-beta.3 - 2018-09-24

### Features

* `show_cursor` is called when `Terminal` is dropped if the cursor is hidden.

## v0.3.0-beta.2 - 2018-09-23

### Breaking Changes

* Remove custom `termion` backends. This is motivated by the fact that
`termion` structs are meant to be combined/wrapped to provide additional
functionalities to the terminal (e.g AlternateScreen, Mouse support, ...).
Thus providing exclusive types do not make a lot of sense and give a false
hint that additional features cannot be used together. The recommended
approach is now to create your own version of `stdout`:

```rust
let stdout = io::stdout().into_raw_mode()?;
let stdout = MouseTerminal::from(stdout);
let stdout = AlternateScreen::from(stdout);
```

and then to create the corresponding `termion` backend:

```rust
let backend = TermionBackend::new(stdout);
```

The resulting code is more verbose but it works with all combinations of
additional `termion` features.

## v0.3.0-beta.1 - 2018-09-08

### Breaking Changes

* Replace `Item` by a generic and flexible `Text` that can be used in both
`Paragraph` and `List` widgets.
* Remove unecessary borrows on `Style`.

## v0.3.0-beta.0 - 2018-09-04

### Features

* Add a basic `Crossterm` backend

### Breaking Changes

* Remove `Group` and introduce `Layout` in its place
  - `Terminal` is no longer required to compute a layout
  - `Size` has been renamed `Constraint`
* Widgets are rendered on a `Frame` instead of a `Terminal` in order to
avoid mixing `draw` and `render` calls
* `draw` on `Terminal` expects a closure where the UI is built by rendering
widgets on the given `Frame`
* Update `Widget` trait
  - `draw` takes area by value
  - `render` takes a `Frame` instead of a `Terminal`
* All widgets use the consumable builder pattern
* `SelectableList` can have no selected item and the highlight symbol is hidden
in this case
* Remove markup langage inside `Paragraph`. `Paragraph` now expects an iterator
of `Text` items

## v0.2.3 - 2018-06-09

### Features

* Add `start_corner` option for `List`
* Add more text aligment options for `Paragraph`

## v0.2.2 - 2018-05-06

### Features

* `Terminal` implements `Debug`

### Breaking Changes

* Use `FnOnce` instead of `FnMut` in Group::render

## v0.2.1 - 2018-04-01

### Features

* Add `AlternateScreenBackend` in `termion` backend
* Add `TermionBackend::with_stdout` in order to let an user of the library
provides its own termion struct
* Add tests and documentation for `Buffer::pos_of`
* Remove leading whitespaces when wrapping text

### Bug Fixes

* Fix `debug_assert` in `Buffer::pos_of`
* Pass the style of `SelectableList` to the underlying `List`
* Fix missing character when wrapping text
* Fix panic when specifying layout constraints

## v0.2.0 - 2017-12-26

### Features

* Add `MouseBackend` in `termion` backend to handle scroll and mouse events
* Add generic `Item` for items in a `List`
* Drop `log4rs` as a dev-dependencies in favor of `stderrlog`

### Breaking Changes

* Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
* Generic parameters for `List` to allow passing iterators as items
* Generic parameters for `Table` to allow using iterators as rows and header
* Generic parameters for `Tabs`
* Rename `border` bitflags to `Borders`
