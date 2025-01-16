use color_eyre::Result;
use itertools::{Itertools, Position};

use crate::{run_cargo_nightly, workspace_libs, Run};

/// Check documentation for errors and warnings
#[derive(Clone, Debug, clap::Args)]
pub struct Docs {
    /// Open the documentation in the browser
    #[arg(long)]
    pub open: bool,
}

impl Run for Docs {
    fn run(self) -> Result<()> {
        let packages = workspace_libs()?;
        for (position, package) in packages.iter().with_position() {
            let mut args = vec!["docs-rs", "--package", &package];
            if self.open && matches!(position, Position::Last | Position::Only) {
                args.push("--open");
            }
            run_cargo_nightly(args)?;
        }
        Ok(())
    }
}
