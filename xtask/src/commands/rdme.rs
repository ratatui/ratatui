use color_eyre::Result;

use crate::{run_cargo, workspace_libs, Run};

/// Check if README.md is up-to-date (using cargo-rdme)
#[derive(Clone, Debug, clap::Args)]
pub struct Readme {
    /// Check if README.md is up-to-date
    #[arg(long)]
    check: bool,
}

impl Run for Readme {
    fn run(self) -> Result<()> {
        let args = if self.check {
            vec!["rdme", "--check"]
        } else {
            vec!["rdme"]
        };
        for package in workspace_libs()? {
            if package == "ratatui" {
                // Skip the main crate as we removed rdme
                continue;
            }
            let mut package_args = args.clone();
            package_args.push("--workspace-project");
            package_args.push(&package);
            run_cargo(package_args)?;
        }
        Ok(())
    }
}
