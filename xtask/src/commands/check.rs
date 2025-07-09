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
            let base_command_parts = vec!["check", "--all-targets"];

            let common_features = CROSSTERM_COMMON_FEATURES.join(",");

            // Run `cargo check` on `ratatui-crossterm` with specific crossterm versions
            for crossterm_feature in CROSSTERM_VERSION_FEATURES {
                let mut command_args = base_command_parts.clone();
                let features = format!("{common_features},{crossterm_feature}");
                command_args.extend(vec![
                    "--package",
                    "ratatui-crossterm",
                    "--no-default-features",
                    "--features",
                    features.as_str(),
                ]);
                run_cargo(command_args)?;
            }

            // Run `cargo check` on all other workspace packages with --all-features
            let mut command_args_workspace = base_command_parts.clone();
            command_args_workspace.extend(vec![
                "--all-features",
                "--workspace",
                "--exclude",
                "ratatui-crossterm",
            ]);
            run_cargo(command_args_workspace)?;
        } else {
            run_cargo(vec!["check", "--all-targets"])?;
        }
        Ok(())
    }
}
