//! # [Ratatui] Minimal example
//!
//! This is a bare minimum example. There are many approaches to running an application loop, so
//! this is not meant to be prescriptive. See the [examples] folder for more complete examples.
//! In particular, the [hello-world] example is a good starting point.
//!
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [hello-world]: https://github.com/ratatui-org/ratatui/blob/main/examples/hello_world.rs
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
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use ratatui::{
    crossterm::event::{self, Event},
    text::Text,
    Frame,
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| frame.render_widget(Text::raw("Hello World!"), frame.area()))
            .expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}
