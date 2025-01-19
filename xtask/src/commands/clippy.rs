use color_eyre::Result;

use crate::{run_cargo, Run};

/// Run clippy on the project
#[derive(Clone, Debug, clap::Args)]
pub struct Clippy {
    /// Fix clippy warnings
    #[arg(long)]
    pub fix: bool,
}

impl Run for Clippy {
    fn run(self) -> Result<()> {
        let mut args = vec![
            "clippy",
            "--all-targets",
            "--all-features",
            "--tests",
            "--benches",
            "--",
            "-D",
            "warnings",
        ];
        if self.fix {
            args.push("--fix");
        }
        run_cargo(args)
    }
}
