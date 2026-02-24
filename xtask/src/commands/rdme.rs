use color_eyre::Result;

use crate::{Run, run_cargo};

/// Check if README.md is up-to-date (using cargo-rdme)
#[derive(Clone, Debug, clap::Args)]
pub struct Readme {
    /// Check if README.md is up-to-date
    #[arg(long)]
    check: bool,
}

impl Run for Readme {
    fn run(self) -> Result<()> {
        // Delegate workspace traversal to cargo-hack so we don't have to call cargo-rdme for each
        // package manually. We still exclude the `ratatui` crate because its README is
        // hand-crafted rather than generated from doc comments.
        let mut args = vec![
            "hack",
            "--workspace",
            "--ignore-private",
            "--exclude",
            "ratatui",
            "rdme",
        ];

        if self.check {
            args.push("--check");
        }

        run_cargo(args)?;
        Ok(())
    }
}
