use color_eyre::Result;

use crate::{run_cargo, Run};

/// Generate code coverage report
#[derive(Clone, Debug, clap::Args)]
pub struct Coverage {
    /// Only generate coverage for unit tests
    #[arg(long)]
    pub lib: bool,
}

impl Run for Coverage {
    fn run(self) -> Result<()> {
        let mut args = vec![
            "llvm-cov",
            "--lcov",
            "--output-path",
            "target/lcov.info",
            "--all-features",
        ];
        if self.lib {
            args.push("--lib");
        }
        run_cargo(args)
    }
}
