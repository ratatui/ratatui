mod app;
mod big_text;
mod colors;
mod destroy;
mod errors;
mod tabs;
mod term;
mod theme;

pub use app::*;
use color_eyre::Result;
pub use colors::*;
pub use term::*;
pub use theme::*;

fn main() -> Result<()> {
    errors::init_hooks()?;
    let terminal = &mut term::init()?;
    App::new().run(terminal)?;
    term::restore()?;
    Ok(())
}
