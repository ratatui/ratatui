use color_eyre::Result;

use crate::{run_cargo, Run};

/// Run cargo check
#[derive(Clone, Debug, clap::Args)]
pub struct Check {
    /// Check all features
    #[arg(long, visible_alias = "all")]
    all_features: bool,
}

impl Run for Check {
    fn run(self) -> Result<()> {
        if self.all_features {
            run_cargo(vec!["check", "--all-targets", "--all-features"])
        } else {
            run_cargo(vec!["check", "--all-targets"])
        }
    }
}
