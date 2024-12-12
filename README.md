<details>
<summary>Table of Contents</summary>

- [Quickstart](#quickstart)
- [Documentation](#documentation)
- [Built with Ratatui](#built-with-ratatui)
- [Templates](#templates)
- [Alternatives](#alternatives)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)
- [License](#license)

</details>

![Demo](https://github.com/ratatui/ratatui/blob/87ae72dbc756067c97f6400d3e2a58eeb383776e/examples/demo2-destroy.gif?raw=true)

<div align="center">

[![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![CI Badge]][CI Workflow] [![Deps.rs
Badge]][Deps.rs]<br> [![Codecov Badge]][Codecov] [![License Badge]](./LICENSE) [![Sponsors
Badge]][GitHub Sponsors]<br>
[Website][Ratatui Website] · [API Docs] · [Widget Examples] · [App Examples] · [Changelog] · [Breaking Changes]<br>
[Contributing] · [Report a bug] · [Request a Feature]

</div>

[Ratatui][Ratatui Website] is a crate for cooking up terminal user interfaces in Rust. It is a
lightweight library that provides a set of widgets and utilities to build complex TUIs.

> Ratatui was forked from the [tui-rs] crate in 2023 in order to continue its development.

## Quickstart

Add Ratatui to your `Cargo.toml`:

```sh
cargo add ratatui
```

Then you can create a simple "Hello World" application:

```rust
use ratatui::crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
```

See the [documentation](#documentation) for more examples and tutorials.

## Documentation

- [Ratatui Website] - explains the library's concepts and provides step-by-step tutorials.
- [Ratatui Forum][Forum] - a place to ask questions and discuss the library.
- [API Docs] - the full API documentation for the library on docs.rs.
- [Widget Examples] - a collection of examples that demonstrate how to use the library.
- [App Examples] - a collection of more complex examples that demonstrate how to build apps.
- [Changelog] - generated by [git-cliff] utilizing [Conventional Commits].
- [Breaking Changes] - a list of breaking changes in the library.

You can also watch the [EuroRust 2024 talk] to learn about common concepts in Ratatui and what's possible to build with it.

## Built with Ratatui

[![Awesome](https://awesome.re/badge-flat2.svg)][awesome-ratatui]

Check out the [showcase] section of the website, or the [awesome-ratatui] repository
for a curated list of awesome apps/libraries built with Ratatui!

## Templates

If you're looking to get started quickly, you can use one of the available templates from the [templates] repository.

## Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) or
[iocraft](https://github.com/ccbrown/iocraft/) for an alternative solutions
to build text user interfaces in Rust.

## Contributing

[![Discord Badge]][Discord Server] [![Matrix Badge]][Matrix]
[![Forum Badge]][Forum]<br>

Feel free to join our [Discord server](https://discord.gg/pMCEU9hNEj) for discussions and questions! There is also a [Matrix](https://matrix.org/) bridge available at
[#ratatui:matrix.org](https://matrix.to/#/#ratatui:matrix.org). We have also recently launched the [Ratatui Forum][Forum].

We rely on GitHub for [bugs][Report a bug] and [feature requests][Request a Feature].

Please make sure you read the [contributing](./CONTRIBUTING.md) guidelines before [creating a pull request][Create a Pull Request].

## Acknowledgements

- None of this could be possible without [Florian Dehau] who originally created [tui-rs] which inspired many Rust TUIs.

- Special thanks to [Pavel Fomchenkov] for his work in designing an awesome logo for the Ratatui project and organization.

- A now unknown person in our community for coming up with the name "Ratatui".

## License

[MIT](./LICENSE)

[Ratatui Website]: https://ratatui.rs/
[Forum]: https://forum.ratatui.rs
[API Docs]: https://docs.rs/ratatui
[Widget Examples]: https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples
[App Examples]: https://github.com/ratatui/ratatui/tree/main/examples
[Changelog]: https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md
[git-cliff]: https://git-cliff.org
[Conventional Commits]: https://www.conventionalcommits.org
[Breaking Changes]: https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md
[EuroRust 2024 talk]: https://www.youtube.com/watch?v=hWG51Mc1DlM
[Report a bug]: https://github.com/ratatui/ratatui/issues/new?labels=bug&projects=&template=bug_report.md
[Request a Feature]: https://github.com/ratatui/ratatui/issues/new?labels=enhancement&projects=&template=feature_request.md
[Create a Pull Request]: https://github.com/ratatui/ratatui/compare
[Contributing]: https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md
[Crate]: https://crates.io/crates/ratatui
[tui-rs]: https://crates.io/crates/tui
[GitHub Sponsors]: https://github.com/sponsors/ratatui
[Crate Badge]: https://img.shields.io/crates/v/ratatui?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[License Badge]: https://img.shields.io/crates/l/ratatui?style=flat-square&color=1370D3
[CI Badge]: https://img.shields.io/github/actions/workflow/status/ratatui/ratatui/ci.yml?style=flat-square&logo=github
[CI Workflow]: https://github.com/ratatui/ratatui/actions/workflows/ci.yml
[Codecov Badge]: https://img.shields.io/codecov/c/github/ratatui/ratatui?logo=codecov&style=flat-square&token=BAQ8SOKEST&color=C43AC3&logoColor=C43AC3
[Codecov]: https://app.codecov.io/gh/ratatui/ratatui
[Deps.rs Badge]: https://deps.rs/repo/github/ratatui/ratatui/status.svg?style=flat-square
[Deps.rs]: https://deps.rs/repo/github/ratatui/ratatui
[Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=discord&logo=discord&style=flat-square&color=1370D3&logoColor=1370D3
[Discord Server]: https://discord.gg/pMCEU9hNEj
[Docs Badge]: https://img.shields.io/docsrs/ratatui?logo=rust&style=flat-square&logoColor=E05D44
[Matrix Badge]: https://img.shields.io/matrix/ratatui-general%3Amatrix.org?style=flat-square&logo=matrix&label=Matrix&color=C43AC3
[Matrix]: https://matrix.to/#/#ratatui:matrix.org
[Forum Badge]: https://img.shields.io/discourse/likes?server=https%3A%2F%2Fforum.ratatui.rs&style=flat-square&logo=discourse&label=forum&color=C43AC3
[Sponsors Badge]: https://img.shields.io/github/sponsors/ratatui?logo=github&style=flat-square&color=1370D3
[templates]: https://github.com/ratatui/templates/
[showcase]: https://ratatui.rs/showcase/
[awesome-ratatui]: https://github.com/ratatui/awesome-ratatui
[Pavel Fomchenkov]: https://github.com/nawok
[Florian Dehau]: https://github.com/fdehau
