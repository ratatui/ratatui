use anyhow::Result;
pub use app::*;
pub use colors::*;
pub use root::*;
pub use term::*;
pub use theme::*;

mod app;
mod big_text;
mod colors;
mod root;
mod tabs;
mod term;
mod theme;

fn main() -> Result<()> {
    App::run()
}
