use color_eyre::{config::HookBuilder, Result};

use crate::term;

pub fn init_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = term::restore();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = term::restore();
        panic(info);
    }));
    Ok(())
}
