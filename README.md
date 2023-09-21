<details>
<summary>Table of Contents</summary>

- [Ratatui](#ratatui)
  - [Installation](#installation)
  - [Introduction](#introduction)
  - [Other Documentation](#other-documentation)
  - [Quickstart](#quickstart)
  - [Status of this fork](#status-of-this-fork)
  - [Rust version requirements](#rust-version-requirements)
  - [Widgets](#widgets)
    - [Built in](#built-in)
    - [Third\-party libraries, bootstrapping templates and
      widgets](#third-party-libraries-bootstrapping-templates-and-widgets)
  - [Apps](#apps)
  - [Alternatives](#alternatives)
  - [Acknowledgments](#acknowledgments)
  - [License](#license)

</details>

<!-- cargo-rdme start -->

![Demo](https://raw.githubusercontent.com/ratatui-org/ratatui/aa09e59dc0058347f68d7c1e0c91f863c6f2b8c9/examples/demo2.gif)

<div align="center">

[![Crate Badge]](https://crates.io/crates/ratatui) [![License Badge]](./LICENSE) [![CI
Badge]](https://github.com/ratatui-org/ratatui/actions?query=workflow%3ACI+) [![Docs
Badge]](https://docs.rs/crate/ratatui/)<br>
[![Dependencies Badge]](https://deps.rs/repo/github/ratatui-org/ratatui) [![Codecov
Badge]](https://app.codecov.io/gh/ratatui-org/ratatui) [![Discord
Badge]](https://discord.gg/pMCEU9hNEj) [![Matrix
Badge]](https://matrix.to/#/#ratatui:matrix.org)<br>
[Documentation](https://docs.rs/ratatui) · [Ratatui Book](https://ratatui.rs) ·
[Examples](https://github.com/ratatui-org/ratatui/tree/main/examples) · [Report a
bug](https://github.com/ratatui-org/ratatui/issues/new?labels=bug&projects=&template=bug_report.md)
· [Request a
Feature](https://github.com/ratatui-org/ratatui/issues/new?labels=enhancement&projects=&template=feature_request.md)
· [Send a Pull Request](https://github.com/ratatui-org/ratatui/compare)

</div>

# Ratatui

[Ratatui] is a crate for cooking up terminal user interfaces in rust. It is a lightweight
library that provides a set of widgets and utilities to build complex rust TUIs. Ratatui was
forked from the [Tui-rs crate] in 2023 in order to continue its development.

## Installation

Add `ratatui` and `crossterm` as dependencies to your cargo.toml:

```shell
cargo add ratatui crossterm
```

Ratatui uses [Crossterm] by default as it works on most platforms. See the [Installation]
section of the [Ratatui Book] for more details on how to use other backends ([Termion] /
[Termwiz]).

## Introduction

Ratatui is based on the principle of immediate rendering with intermediate buffers. This means
that for each frame, your app must render all widgets that are supposed to be part of the UI.
This is in contrast to the retained mode style of rendering where widgets are updated and then
automatically redrawn on the next frame. See the [Rendering] section of the [Ratatui Book] for
more info.

## Other documentation

- [Ratatui Book] - explains the library's concepts and provides step-by-step tutorials
- [Examples] - a collection of examples that demonstrate how to use the library.
- [API Documentation] - the full API documentation for the library on docs.rs.
- [Changelog] - generated by [git-cliff] utilizing [Conventional Commits].
- [Contributing] - Please read this if you are interested in contributing to the project.

## Quickstart

The following example demonstrates the minimal amount of code necessary to setup a terminal and
render "Hello World!". The full code for this example which contains a little more detail is in
[hello_world.rs]. For more guidance on different ways to structure your application see the
[Application Patterns] and [Hello World tutorial] sections in the [Ratatui Book] and the various
[Examples]. There are also several starter templates available:

- [rust-tui-template]
- [ratatui-async-template] (book and template)
- [simple-tui-rs]

Every application built with `ratatui` needs to implement the following steps:

- Initialize the terminal
- A main loop to:
  - Handle input events
  - Draw the UI
- Restore the terminal state

The library contains a [`prelude`] module that re-exports the most commonly used traits and
types for convenience. Most examples in the documentation will use this instead of showing the
full path of each type.

### Initialize and restore the terminal

The [`Terminal`] type is the main entry point for any Ratatui application. It is a light
abstraction over a choice of [`Backend`] implementations that provides functionality to draw
each frame, clear the screen, hide the cursor, etc. It is parametrized over any type that
implements the [`Backend`] trait which has implementations for [Crossterm], [Termion] and
[Termwiz].

Most applications should enter the Alternate Screen when starting and leave it when exiting and
also enable raw mode to disable line buffering and enable reading key events. See the [`backend`
module] and the [Backends] section of the [Ratatui Book] for more info.

### Drawing the UI

The drawing logic is delegated to a closure that takes a [`Frame`] instance as argument. The
[`Frame`] provides the size of the area to draw to and allows the app to render any [`Widget`]
using the provided [`render_widget`] method. See the [Widgets] section of the [Ratatui Book] for
more info.

### Handling events

Ratatui does not include any input handling. Instead event handling can be implemented by
calling backend library methods directly. See the [Handling Events] section of the [Ratatui
Book] for more info. For example, if you are using [Crossterm], you can use the
[`crossterm::event`] module to handle events.

### Example

```rust
use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
       }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}
```

Running this example produces the following output:

![docsrs-hello](https://github.com/ratatui-org/ratatui/assets/381361/9afccfe3-5f33-42e9-9a55-2d143af3b128)

## Layout

The library comes with a basic yet useful layout management object called [`Layout`] which
allows you to split the available space into multiple areas and then render widgets in each
area. This lets you describe a responsive terminal UI by nesting layouts. See the [Layout]
section of the [Ratatui Book] for more info.

```rust
use ratatui::{prelude::*, widgets::*};

fn ui(frame: &mut Frame) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.size());
    frame.render_widget(Paragraph::new("Title Bar"), areas[0]);
    frame.render_widget(Paragraph::new("Status Bar"), areas[2]);

    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(areas[1]);
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Left"),
        areas[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Right"),
        areas[1],
    );
}
```

Running this example produces the following output:

![docsrs-layout](https://github.com/ratatui-org/ratatui/assets/381361/a18da2a3-1bf4-4939-a5e1-06f3e32bacd1)

## Text and styling

The [`Text`], [`Line`] and [`Span`] types are the building blocks of the library and are used in
many places. [`Text`] is a list of [`Line`]s and a [`Line`] is a list of [`Span`]s. A [`Span`]
is a string with a specific style.

The [`style` module] provides types that represent the various styling options. The most
important one is [`Style`] which represents the foreground and background colors and the text
attributes of a [`Span`]. The [`style` module] also provides a [`Stylize`] trait that allows
short-hand syntax to apply a style to widgets and text. See the [Styling Text] section of the
[Ratatui Book] for more info.

```rust
use ratatui::{prelude::*, widgets::*};

fn ui(frame: &mut Frame) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.size());

    let span1 = Span::raw("Hello ");
    let span2 = Span::styled(
        "World",
        Style::new()
            .fg(Color::Green)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let span3 = "!".red().on_light_yellow().italic();

    let line = Line::from(vec![span1, span2, span3]);
    let text: Text = Text::from(vec![line]);

    frame.render_widget(Paragraph::new(text), areas[0]);
    // or using the short-hand syntax and implicit conversions
    frame.render_widget(
        Paragraph::new("Hello World!".red().on_white().bold()),
        areas[1],
    );

    // to style the whole widget instead of just the text
    frame.render_widget(
        Paragraph::new("Hello World!").style(Style::new().red().on_white()),
        areas[2],
    );
    // or using the short-hand syntax
    frame.render_widget(Paragraph::new("Hello World!").blue().on_yellow(), areas[3]);
}
```

Running this example produces the following output:

![docsrs-styling](https://github.com/ratatui-org/ratatui/assets/381361/c16024f7-3d36-4f66-973c-5892b69bca7f)

[Ratatui Book]: https://ratatui.rs
[Installation]: https://ratatui.rs/installation.html
[Rendering]: https://ratatui.rs/concepts/rendering/index.html
[Application Patterns]: https://ratatui.rs/concepts/application_patterns/index.html
[Hello World tutorial]: https://ratatui.rs/tutorial/hello_world.html
[Backends]: https://ratatui.rs/concepts/backends/index.html
[Widgets]: https://ratatui.rs/how-to/widgets/index.html
[Handling Events]: https://ratatui.rs/concepts/event_handling.html
[Layout]: https://ratatui.rs/how-to/layout/index.html
[Styling Text]: https://ratatui.rs/how-to/render/style-text.html
[rust-tui-template]: https://github.com/ratatui-org/rust-tui-template
[ratatui-async-template]: https://ratatui-org.github.io/ratatui-async-template/
[simple-tui-rs]: https://github.com/pmsanford/simple-tui-rs
[Examples]: https://github.com/ratatui-org/ratatui/tree/main/examples
[git-cliff]: https://github.com/orhun/git-cliff
[Conventional Commits]: https://www.conventionalcommits.org
[API Documentation]: https://docs.rs/ratatui
[Changelog]: https://github.com/ratatui-org/ratatui/blob/main/CHANGELOG.md
[Contributing]: https:://github.com/ratatui-org/ratatui/blob/main/CONTRIBUTING.md
[`Frame`]: terminal::Frame
[`render_widget`]: terminal::Frame::render_widget
[`Widget`]: widgets::Widget
[`Layout`]: layout::Layout
[`Text`]: text::Text
[`Line`]: text::Line
[`Span`]: text::Span
[`Style`]: style::Style
[`style` module]: style
[`Stylize`]: style::Stylize
[`Backend`]: backend::Backend
[`backend` module]: backend
[`crossterm::event`]: https://docs.rs/crossterm/latest/crossterm/event/index.html
[Ratatui]: https://ratatui.rs
[Crossterm]: https://crates.io/crates/crossterm
[Termion]: https://crates.io/crates/termion
[Termwiz]: https://crates.io/crates/termwiz
[Tui-rs crate]: https://crates.io/crates/tui
[hello_world.rs]: https://github.com/ratatui-org/ratatui/blob/main/examples/hello_world.rs
[Crate Badge]: https://img.shields.io/crates/v/ratatui?logo=rust&style=flat-square
[CI Badge]:
    https://img.shields.io/github/actions/workflow/status/ratatui-org/ratatui/ci.yml?style=flat-square&logo=github
[Codecov Badge]:
    https://img.shields.io/codecov/c/github/ratatui-org/ratatui?logo=codecov&style=flat-square&token=BAQ8SOKEST
[Dependencies Badge]: https://deps.rs/repo/github/ratatui-org/ratatui/status.svg?style=flat-square
[Discord Badge]:
    https://img.shields.io/discord/1070692720437383208?label=discord&logo=discord&style=flat-square
[Docs Badge]: https://img.shields.io/docsrs/ratatui?logo=rust&style=flat-square
[License Badge]: https://img.shields.io/crates/l/ratatui?style=flat-square
[Matrix Badge]:
    https://img.shields.io/matrix/ratatui-general%3Amatrix.org?style=flat-square&logo=matrix&label=Matrix

<!-- cargo-rdme end -->

## Status of this fork

In response to the original maintainer [**Florian Dehau**](https://github.com/fdehau)'s issue
regarding the [future of `tui-rs`](https://github.com/fdehau/tui-rs/issues/654), several members of
the community forked the project and created this crate. We look forward to continuing the work
started by Florian 🚀

In order to organize ourselves, we currently use a [Discord server](https://discord.gg/pMCEU9hNEj),
feel free to join and come chat! There is also a [Matrix](https://matrix.org/) bridge available at
[#ratatui:matrix.org](https://matrix.to/#/#ratatui:matrix.org).

While we do utilize Discord for coordinating, it's not essential for contributing.
Our primary open-source workflow is centered around GitHub.
For significant discussions, we rely on GitHub — please open an issue, a discussion or a PR.

Please make sure you read the updated [contributing](./CONTRIBUTING.md) guidelines, especially if
you are interested in working on a PR or issue opened in the previous repository.

## Rust version requirements

Since version 0.23.0, The Minimum Supported Rust Version (MSRV) of `ratatui` is 1.67.0.

## Widgets

### Built in

The library comes with the following
[widgets](https://docs.rs/ratatui/latest/ratatui/widgets/index.html):

- [BarChart](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html)
- [Block](https://docs.rs/ratatui/latest/ratatui/widgets/block/struct.Block.html)
- [Calendar](https://docs.rs/ratatui/latest/ratatui/widgets/calendar/index.html)
- [Canvas](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/struct.Canvas.html) which allows
  rendering [points, lines, shapes and a world
  map](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html)
- [Chart](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html)
- [Clear](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html)
- [Gauge](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html)
- [List](https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html)
- [Paragraph](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html)
- [Scrollbar](https://docs.rs/ratatui/latest/ratatui/widgets/scrollbar/struct.Scrollbar.html)
- [Sparkline](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html)
- [Table](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html)
- [Tabs](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html)

Each widget has an associated example which can be found in the [examples](./examples/) folder. Run
each examples with cargo (e.g. to run the gauge example `cargo run --example gauge`), and quit by
pressing `q`.

You can also run all examples by running `cargo make run-examples` (requires `cargo-make` that can
be installed with `cargo install cargo-make`).

### Third-party libraries, bootstrapping templates and widgets

- [ansi-to-tui](https://github.com/uttarayan21/ansi-to-tui) — Convert ansi colored text to
  `ratatui::text::Text`
- [color-to-tui](https://github.com/uttarayan21/color-to-tui) — Parse hex colors to
  `ratatui::style::Color`
- [rust-tui-template](https://github.com/ratatui-org/rust-tui-template) — A template for
  bootstrapping a Rust TUI application with Tui-rs & crossterm
- [simple-tui-rs](https://github.com/pmsanford/simple-tui-rs) — A simple example tui-rs app
- [tui-builder](https://github.com/jkelleyrtp/tui-builder) — Batteries-included MVC framework for
  Tui-rs + Crossterm apps
- [tui-clap](https://github.com/kegesch/tui-clap-rs) — Use clap-rs together with Tui-rs
- [tui-log](https://github.com/kegesch/tui-log-rs) — Example of how to use logging with Tui-rs
- [tui-logger](https://github.com/gin66/tui-logger) — Logger and Widget for Tui-rs
- [tui-realm](https://github.com/veeso/tui-realm) — Tui-rs framework to build stateful applications
  with a React/Elm inspired approach
- [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) — Treeview component for
  Tui-realm
- [tui-rs-tree-widgets](https://github.com/EdJoPaTo/tui-rs-tree-widget): Widget for tree data
  structures.
- [tui-windows](https://github.com/markatk/tui-windows-rs) — Tui-rs abstraction to handle multiple
  windows and their rendering
- [tui-textarea](https://github.com/rhysd/tui-textarea): Simple yet powerful multi-line text editor
  widget supporting several key shortcuts, undo/redo, text search, etc.
- [tui-input](https://github.com/sayanarijit/tui-input): TUI input library supporting multiple
  backends and tui-rs.
- [tui-term](https://github.com/a-kenji/tui-term): A pseudoterminal widget library
  that enables the rendering of terminal applications as ratatui widgets.

## Apps

Check out the list of more than 50 [Apps using
`Ratatui`](https://github.com/ratatui-org/ratatui/wiki/Apps-using-Ratatui)!

## Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) for an alternative solution
to build text user interfaces in Rust.

## Acknowledgments

Special thanks to [**Pavel Fomchenkov**](https://github.com/nawok) for his work in designing **an
awesome logo** for the ratatui project and ratatui-org organization.

## License

[MIT](./LICENSE)
