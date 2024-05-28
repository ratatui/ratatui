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

![Demo](https://github.com/ratatui-org/ratatui/blob/1d39444e3dea6f309cf9035be2417ac711c1abc9/examples/demo2-destroy.gif?raw=true)

<div align="center">

[![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![CI Badge]][CI Workflow] [![License
Badge]](./LICENSE) [![Sponsors Badge]][GitHub Sponsors]<br>
[![Codecov Badge]][Codecov] [![Deps.rs Badge]][Deps.rs] [![Discord Badge]][Discord Server]
[![Matrix Badge]][Matrix]<br>

[Ratatui Website] · [API Docs] · [Examples] · [Changelog] · [Breaking Changes]<br>
[Contributing] · [Report a bug] · [Request a Feature] · [Create a Pull Request]

</div>

# Ratatui

[Ratatui][Ratatui Website] is a crate for cooking up terminal user interfaces in Rust. It is a
lightweight library that provides a set of widgets and utilities to build complex Rust TUIs.
Ratatui was forked from the [tui-rs] crate in 2023 in order to continue its development.

## Installation

Add `ratatui` as a dependency to your cargo.toml:

```shell
cargo add ratatui
```

Ratatui uses [Crossterm] by default as it works on most platforms. See the [Installation]
section of the [Ratatui Website] for more details on how to use other backends ([Termion] /
[Termwiz]).

## Introduction

Ratatui is based on the principle of immediate rendering with intermediate buffers. This means
that for each frame, your app must render all widgets that are supposed to be part of the UI.
This is in contrast to the retained mode style of rendering where widgets are updated and then
automatically redrawn on the next frame. See the [Rendering] section of the [Ratatui Website]
for more info.

You can also watch the [FOSDEM 2024 talk] about Ratatui which gives a brief introduction to
terminal user interfaces and showcases the features of Ratatui, along with a hello world demo.

## Other documentation

- [Ratatui Website] - explains the library's concepts and provides step-by-step tutorials
- [API Docs] - the full API documentation for the library on docs.rs.
- [Examples] - a collection of examples that demonstrate how to use the library.
- [Contributing] - Please read this if you are interested in contributing to the project.
- [Changelog] - generated by [git-cliff] utilizing [Conventional Commits].
- [Breaking Changes] - a list of breaking changes in the library.

## Quickstart

The following example demonstrates the minimal amount of code necessary to setup a terminal and
render "Hello World!". The full code for this example which contains a little more detail is in
the [Examples] directory. For more guidance on different ways to structure your application see
the [Application Patterns] and [Hello World tutorial] sections in the [Ratatui Website] and the
various [Examples]. There are also several starter templates available in the [templates]
repository.

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
module] and the [Backends] section of the [Ratatui Website] for more info.

### Drawing the UI

The drawing logic is delegated to a closure that takes a [`Frame`] instance as argument. The
[`Frame`] provides the size of the area to draw to and allows the app to render any [`Widget`]
using the provided [`render_widget`] method. After this closure returns, a diff is performed and
only the changes are drawn to the terminal. See the [Widgets] section of the [Ratatui Website]
for more info.

### Handling events

Ratatui does not include any input handling. Instead event handling can be implemented by
calling backend library methods directly. See the [Handling Events] section of the [Ratatui
Website] for more info. For example, if you are using [Crossterm], you can use the
[`crossterm::event`] module to handle events.

### Example

```rust
use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

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
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.size(),
    );
}
```

Running this example produces the following output:

![docsrs-hello]

## Layout

The library comes with a basic yet useful layout management object called [`Layout`] which
allows you to split the available space into multiple areas and then render widgets in each
area. This lets you describe a responsive terminal UI by nesting layouts. See the [Layout]
section of the [Ratatui Website] for more info.

```rust
use ratatui::{prelude::*, widgets::*};

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);
    frame.render_widget(Block::bordered().title("Left"), inner_layout[0]);
    frame.render_widget(Block::bordered().title("Right"), inner_layout[1]);
}
```

Running this example produces the following output:

![docsrs-layout]

## Text and styling

The [`Text`], [`Line`] and [`Span`] types are the building blocks of the library and are used in
many places. [`Text`] is a list of [`Line`]s and a [`Line`] is a list of [`Span`]s. A [`Span`]
is a string with a specific style.

The [`style` module] provides types that represent the various styling options. The most
important one is [`Style`] which represents the foreground and background colors and the text
attributes of a [`Span`]. The [`style` module] also provides a [`Stylize`] trait that allows
short-hand syntax to apply a style to widgets and text. See the [Styling Text] section of the
[Ratatui Website] for more info.

```rust
use ratatui::{prelude::*, widgets::*};

fn ui(frame: &mut Frame) {
    let areas = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ],
    )
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

![docsrs-styling]

[Ratatui Website]: https://ratatui.rs/
[Installation]: https://ratatui.rs/installation/
[Rendering]: https://ratatui.rs/concepts/rendering/
[Application Patterns]: https://ratatui.rs/concepts/application-patterns/
[Hello World tutorial]: https://ratatui.rs/tutorials/hello-world/
[Backends]: https://ratatui.rs/concepts/backends/
[Widgets]: https://ratatui.rs/how-to/widgets/
[Handling Events]: https://ratatui.rs/concepts/event-handling/
[Layout]: https://ratatui.rs/how-to/layout/
[Styling Text]: https://ratatui.rs/how-to/render/style-text/
[templates]: https://github.com/ratatui-org/templates/
[Examples]: https://github.com/ratatui-org/ratatui/tree/main/examples/README.md
[Report a bug]: https://github.com/ratatui-org/ratatui/issues/new?labels=bug&projects=&template=bug_report.md
[Request a Feature]: https://github.com/ratatui-org/ratatui/issues/new?labels=enhancement&projects=&template=feature_request.md
[Create a Pull Request]: https://github.com/ratatui-org/ratatui/compare
[git-cliff]: https://git-cliff.org
[Conventional Commits]: https://www.conventionalcommits.org
[API Docs]: https://docs.rs/ratatui
[Changelog]: https://github.com/ratatui-org/ratatui/blob/main/CHANGELOG.md
[Contributing]: https://github.com/ratatui-org/ratatui/blob/main/CONTRIBUTING.md
[Breaking Changes]: https://github.com/ratatui-org/ratatui/blob/main/BREAKING-CHANGES.md
[FOSDEM 2024 talk]: https://www.youtube.com/watch?v=NU0q6NOLJ20
[docsrs-hello]: https://github.com/ratatui-org/ratatui/blob/c3c3c289b1eb8d562afb1931adb4dc719cd48490/examples/docsrs-hello.png?raw=true
[docsrs-layout]: https://github.com/ratatui-org/ratatui/blob/c3c3c289b1eb8d562afb1931adb4dc719cd48490/examples/docsrs-layout.png?raw=true
[docsrs-styling]: https://github.com/ratatui-org/ratatui/blob/c3c3c289b1eb8d562afb1931adb4dc719cd48490/examples/docsrs-styling.png?raw=true
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
[Crate]: https://crates.io/crates/ratatui
[Crossterm]: https://crates.io/crates/crossterm
[Termion]: https://crates.io/crates/termion
[Termwiz]: https://crates.io/crates/termwiz
[tui-rs]: https://crates.io/crates/tui
[GitHub Sponsors]: https://github.com/sponsors/ratatui-org
[Crate Badge]: https://img.shields.io/crates/v/ratatui?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[License Badge]: https://img.shields.io/crates/l/ratatui?style=flat-square&color=1370D3
[CI Badge]: https://img.shields.io/github/actions/workflow/status/ratatui-org/ratatui/ci.yml?style=flat-square&logo=github
[CI Workflow]: https://github.com/ratatui-org/ratatui/actions/workflows/ci.yml
[Codecov Badge]:https://img.shields.io/codecov/c/github/ratatui-org/ratatui?logo=codecov&style=flat-square&token=BAQ8SOKEST&color=C43AC3&logoColor=C43AC3
[Codecov]: https://app.codecov.io/gh/ratatui-org/ratatui
[Deps.rs Badge]: https://deps.rs/repo/github/ratatui-org/ratatui/status.svg?style=flat-square
[Deps.rs]: https://deps.rs/repo/github/ratatui-org/ratatui
[Discord Badge]:https://img.shields.io/discord/1070692720437383208?label=discord&logo=discord&style=flat-square&color=1370D3&logoColor=1370D3
[Discord Server]: https://discord.gg/pMCEU9hNEj
[Docs Badge]: https://img.shields.io/docsrs/ratatui?logo=rust&style=flat-square&logoColor=E05D44
[Matrix Badge]: https://img.shields.io/matrix/ratatui-general%3Amatrix.org?style=flat-square&logo=matrix&label=Matrix&color=C43AC3
[Matrix]: https://matrix.to/#/#ratatui:matrix.org
[Sponsors Badge]: https://img.shields.io/github/sponsors/ratatui-org?logo=github&style=flat-square&color=1370D3

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

Each widget has an associated example which can be found in the [Examples] folder. Run each example
with cargo (e.g. to run the gauge example `cargo run --example gauge`), and quit by pressing `q`.

You can also run all examples by running `cargo make run-examples` (requires `cargo-make` that can
be installed with `cargo install cargo-make`).

### Third-party libraries, bootstrapping templates and widgets

- [ansi-to-tui](https://github.com/uttarayan21/ansi-to-tui) — Convert ansi colored text to
  `ratatui::text::Text`
- [color-to-tui](https://github.com/uttarayan21/color-to-tui) — Parse hex colors to
  `ratatui::style::Color`
- [templates](https://github.com/ratatui-org/templates) — Starter templates for
  bootstrapping a Rust TUI application with Ratatui & crossterm
- [tui-builder](https://github.com/jkelleyrtp/tui-builder) — Batteries-included MVC framework for
  Tui-rs + Crossterm apps
- [tui-clap](https://github.com/kegesch/tui-clap-rs) — Use clap-rs together with Tui-rs
- [tui-log](https://github.com/kegesch/tui-log-rs) — Example of how to use logging with Tui-rs
- [tui-logger](https://github.com/gin66/tui-logger) — Logger and Widget for Tui-rs
- [tui-realm](https://github.com/veeso/tui-realm) — Tui-rs framework to build stateful applications
  with a React/Elm inspired approach
- [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) — Treeview component for
  Tui-realm
- [tui-rs-tree-widgets](https://github.com/EdJoPaTo/tui-rs-tree-widget) — Widget for tree data
  structures.
- [tui-windows](https://github.com/markatk/tui-windows-rs) — Tui-rs abstraction to handle multiple
  windows and their rendering
- [tui-textarea](https://github.com/rhysd/tui-textarea) — Simple yet powerful multi-line text editor
  widget supporting several key shortcuts, undo/redo, text search, etc.
- [tui-input](https://github.com/sayanarijit/tui-input) — TUI input library supporting multiple
  backends and tui-rs.
- [tui-term](https://github.com/a-kenji/tui-term) — A pseudoterminal widget library
  that enables the rendering of terminal applications as ratatui widgets.

## Apps

Check out [awesome-ratatui](https://github.com/ratatui-org/awesome-ratatui) for a curated list of
awesome apps/libraries built with `ratatui`!

## Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) for an alternative solution
to build text user interfaces in Rust.

## Acknowledgments

Special thanks to [**Pavel Fomchenkov**](https://github.com/nawok) for his work in designing **an
awesome logo** for the ratatui project and ratatui-org organization.

## License

[MIT](./LICENSE)
