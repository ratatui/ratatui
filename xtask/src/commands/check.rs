use color_eyre::Result;

use crate::{CROSSTERM_COMMON_FEATURES, CROSSTERM_VERSION_FEATURES, Run, run_cargo};

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
            let common_features = CROSSTERM_COMMON_FEATURES.join(",");

            // Run `cargo check` on `ratatui-crossterm` with specific crossterm versions
            for crossterm_feature in CROSSTERM_VERSION_FEATURES {
                let features = format!("{common_features},{crossterm_feature}");
                let command = vec![
                    "check",
                    "--all-targets",
                    "--package",
                    "ratatui-crossterm",
                    "--no-default-features",
                    "--features",
                    features.as_str(),
                ];
                run_cargo(command)?;
            }

            // Run `cargo check` on all other workspace packages with --all-features
            run_cargo(vec![
                "hack",
                "--exclude",
                "ratatui-crossterm",
                "check",
                "--all-targets",
                "--all-features",
            ])?;
        } else {
            run_cargo(vec!["check", "--all-targets"])?;
        }
        Ok(())
    }
}
