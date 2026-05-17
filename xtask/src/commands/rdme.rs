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
        // `ratatui` is excluded: it has a hand-written README. Other library crates use cargo-rdme.
        // cargo-rdme 1.5.0+ supports `--manifest-path`, which cargo-hack sets per `-p` package.
        let mut args = vec![
            "hack",
            "-p",
            "ratatui-core",
            "-p",
            "ratatui-crossterm",
            "-p",
            "ratatui-macros",
            "-p",
            "ratatui-termion",
            "-p",
            "ratatui-termwiz",
            "-p",
            "ratatui-widgets",
            "rdme",
        ];
        if self.check {
            args.push("--check");
        }
        run_cargo(args)?;
        Ok(())
    }
}
