# tui-rs

[![Build Status](https://github.com/fdehau/tui-rs/workflows/CI/badge.svg)](https://github.com/fdehau/tui-rs/actions?query=workflow%3ACI+)
[![Crate Status](https://img.shields.io/crates/v/tui.svg)](https://crates.io/crates/tui)
[![Docs Status](https://docs.rs/tui/badge.svg)](https://docs.rs/crate/tui/)

<img src="./assets/demo.gif" alt="Demo cast under Linux Termite with Inconsolata font 12pt">

# What is this fork?

This fork was created to continue maintenance on the original TUI project. The original maintainer had created an [issue](https://github.com/fdehau/tui-rs/issues/654) explaining how he couldn't find time to continue development, which led to us creating this fork. From here, we hope to continue developing the TUI crate.

In order to organize ourselves, we have created a **temporary** [discord server](https://discord.gg/pMCEU9hNEj). We have not yet determined with the community what will be our definitive communication medium.

Please make sure you read the updated contributing guidelines, especially if you are interested in working on a PR or issue opened in the previous repository.

# Introduction

`tui-rs` is a [Rust](https://www.rust-lang.org) library to build rich terminal
user interfaces and dashboards. It is heavily inspired by the `Javascript`
library [blessed-contrib](https://github.com/yaronn/blessed-contrib) and the
`Go` library [termui](https://github.com/gizak/termui).

The library supports multiple backends:

- [crossterm](https://github.com/crossterm-rs/crossterm) [default]
- [termion](https://github.com/ticki/termion)

The library is based on the principle of immediate rendering with intermediate
buffers. This means that at each new frame you should build all widgets that are
supposed to be part of the UI. While providing a great flexibility for rich and
interactive UI, this may introduce overhead for highly dynamic content. So, the
implementation try to minimize the number of ansi escapes sequences generated to
draw the updated UI. In practice, given the speed of `Rust` the overhead rather
comes from the terminal emulator than the library itself.

Moreover, the library does not provide any input handling nor any event system and
you may rely on the previously cited libraries to achieve such features.

## Rust version requirements

Since version 0.17.0, `tui` requires **rustc version 1.59.0 or greater**.

# Documentation

The documentation can be found on [docs.rs.](https://docs.rs/tui)

# Demo

The demo shown in the gif can be run with all available backends.

```
# crossterm
cargo run --example demo --release -- --tick-rate 200
# termion
cargo run --example demo --no-default-features --features=termion --release -- --tick-rate 200
```

where `tick-rate` is the UI refresh rate in ms.

The UI code is in [examples/demo/ui.rs](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/demo/ui.rs) while the
application state is in [examples/demo/app.rs](https://github.com/fdehau/tui-rs/blob/v0.19.0/examples/demo/app.rs).

If the user interface contains glyphs that are not displayed correctly by your terminal, you may want to run
the demo without those symbols:

```
cargo run --example demo --release -- --tick-rate 200 --enhanced-graphics false
```

# Widgets

## Built in

The library comes with the following list of widgets:

- [Block](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/block.rs)
- [Gauge](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/gauge.rs)
- [Sparkline](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/sparkline.rs)
- [Chart](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/chart.rs)
- [BarChart](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/barchart.rs)
- [List](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/list.rs)
- [Table](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/table.rs)
- [Paragraph](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/paragraph.rs)
- [Canvas (with line, point cloud, map)](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/canvas.rs)
- [Tabs](https://github.com/fdehau/tui-rs/blob/v0.16.0/examples/tabs.rs)

Click on each item to see the source of the example. Run the examples with with
cargo (e.g. to run the gauge example `cargo run --example gauge`), and quit by pressing `q`.

You can run all examples by running `cargo make run-examples` (require
`cargo-make` that can be installed with `cargo install cargo-make`).

### Third-party libraries, bootstrapping templates and widgets

- [ansi-to-tui](https://github.com/uttarayan21/ansi-to-tui) — Convert ansi colored text to `tui::text::Text`
- [color-to-tui](https://github.com/uttarayan21/color-to-tui) — Parse hex colors to `tui::style::Color`
- [rust-tui-template](https://github.com/orhun/rust-tui-template) — A template for bootstrapping a Rust TUI application with Tui-rs & crossterm
- [simple-tui-rs](https://github.com/pmsanford/simple-tui-rs) — A simple example tui-rs app
- [tui-builder](https://github.com/jkelleyrtp/tui-builder) — Batteries-included MVC framework for Tui-rs + Crossterm apps
- [tui-clap](https://github.com/kegesch/tui-clap-rs) — Use clap-rs together with Tui-rs
- [tui-log](https://github.com/kegesch/tui-log-rs) — Example of how to use logging with Tui-rs
- [tui-logger](https://github.com/gin66/tui-logger) — Logger and Widget for Tui-rs
- [tui-realm](https://github.com/veeso/tui-realm) — Tui-rs framework to build stateful applications with a React/Elm inspired approach
- [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) — Treeview component for Tui-realm
- [tui tree widget](https://github.com/EdJoPaTo/tui-rs-tree-widget) — Tree Widget for Tui-rs
- [tui-windows](https://github.com/markatk/tui-windows-rs) — Tui-rs abstraction to handle multiple windows and their rendering
- [tui-textarea](https://github.com/rhysd/tui-textarea): Simple yet powerful multi-line text editor widget supporting several key shortcuts, undo/redo, text search, etc.
- [tui-rs-tree-widgets](https://github.com/EdJoPaTo/tui-rs-tree-widget): Widget for tree data structures.
- [tui-input](https://github.com/sayanarijit/tui-input): TUI input library supporting multiple backends and tui-rs.

# Apps using tui

- [adsb_deku/radar](https://github.com/wcampbell0x2a/adsb_deku#radar-tui) — Rust ADS-B decoder + TUI radar application
- [bandwhich](https://github.com/imsnif/bandwhich) — Terminal utility for displaying current network utilization by process, connection and remote IP/hostname
- [battleship.rs](https://github.com/deepu105/battleship-rs) — A terminal battleship game in Rust
- [bottom](https://github.com/ClementTsang/bottom) — Yet another cross-platform graphical process/system monitor
- [conclusive](https://github.com/mrusme/conclusive) — Command line client for Plausible Analytics
- [cotp](https://github.com/replydev/cotp) — Trustworthy, encrypted, command-line TOTP/HOTP authenticator app with import functionality
- [cube timer](https://github.com/paarthmadan/cube) — A tui-based Rubik's cube timer written in Rust
- [desed](https://github.com/SoptikHa2/desed) — Debugger for Sed: demystify and debug your sed scripts, from comfort of your terminal
- [diskonaut](https://github.com/imsnif/diskonaut) — Terminal disk space navigator
- [exhaust](https://github.com/heyrict/exhaust) — Exhaust all your possibilities.. for the next coming exam
- [game-of-life-rs](https://github.com/kachark/game-of-life-rs) — Conway's Game of Life implemented in Rust and visualized with Tui-rs
- [gitui](https://github.com/extrawurst/gitui) — Blazing fast terminal-ui for Git written in Rust
- [gpg-tui](https://github.com/orhun/gpg-tui) — Manage your GnuPG keys with ease!
- [gping](https://github.com/orf/gping) — Ping, but with a graph
- [joshuto](https://github.com/kamiyaa/joshuto) — Ranger-like terminal file manager written in Rust
- [kDash](https://github.com/kdash-rs/kdash) — A simple and fast dashboard for Kubernetes
- [kmon](https://github.com/orhun/kmon) — Linux Kernel Manager and Activity Monitor
- [kubectl-watch](https://github.com/imuxin/kubectl-watch) — A kubectl plugin to provide a pretty delta change view of being watched kubernetes resources
- [minesweep](https://github.com/cpcloud/minesweep-rs) — Sweep some mines for fun, and probably not for profit
- [oha](https://github.com/hatoo/oha) — HTTP load generator, inspired by rakyll/hey with tui animation
- [oxker](https://github.com/mrjackwills/oxker) — a simple tui to view & control docker containers
- [poketex](https://github.com/ckaznable/poketex) — A simple pokedex based on TUI
- [rrtop](https://github.com/wojciech-zurek/rrtop) — Redis monitoring (top like) app
- [rust-sadari-cli](https://github.com/24seconds/rust-sadari-cli) — Sadari game based on terminal
- [rusty-krab-manager](https://github.com/aryakaul/rusty-krab-manager) — Time-management TUI in Rust
- [spotify-tui](https://github.com/Rigellute/spotify-tui) — Spotify for the terminal written in Rust
- [systeroid](https://github.com/orhun/systeroid) - A more powerful alternative to sysctl(8) with a terminal user interface
- [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui) — A terminal user interface for Taskwarrior
- [termchat](https://github.com/lemunozm/termchat) — Terminal chat through the LAN with video streaming and file transfer
- [termscp](https://github.com/veeso/termscp) — A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3
- [tick-rs](https://github.com/tarkah/tickrs) — Realtime ticker data in your terminal
- [tsuchita](https://github.com/kamiyaa/tsuchita) — Client-server notification center for dbus desktop notifications
- [tuinance](https://github.com/landchad/tuinance) — Display financial data on the terminal
- [vector](https://vector.dev) — A lightweight, ultra-fast tool for building observability pipelines
- [xplr](https://github.com/sayanarijit/xplr) — A hackable, minimal, fast TUI file explorer
- [ytop](https://github.com/cjbassi/ytop) — A TUI system monitor written in Rust (no longer maintained)
- [zenith](https://github.com/bvaisvil/zenith) — Sort of like top or htop but with zoom-able charts, CPU, GPU, network, and disk usage

### Alternatives

You might want to checkout [Cursive](https://github.com/gyscos/Cursive) for an
alternative solution to build text user interfaces in Rust.

# License

[MIT](LICENSE)
