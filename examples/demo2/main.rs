//! # [Ratatui] Demo2 example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

mod app;
mod big_text;
mod colors;
mod destroy;
mod tabs;
mod theme;

use app::App;
use color_eyre::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    TerminalOptions, Viewport,
};

pub use self::{
    colors::{color_from_oklab, RgbSwatch},
    theme::THEME,
};

fn main() -> Result<()> {
    // this size is to match the size of the terminal when running the demo
    // using vhs in a 1280x640 sized window (github social preview size)
    let options = TerminalOptions {
        viewport: Viewport::Fixed(Rect::new(0, 0, 81, 18)),
    };
    let terminal = CrosstermBackend::stdout_with_defaults()?.to_terminal_with_options(options)?;
    App::default().run(terminal)
}
