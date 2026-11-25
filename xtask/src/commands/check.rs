use color_eyre::Result;
use color_eyre::eyre::bail;
use duct::cmd;

use crate::{CROSSTERM_COMMON_FEATURES, CROSSTERM_VERSION_FEATURES, ExpressionExt, Run, run_cargo};

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
                check_ratatui_crossterm_version(crossterm_feature)?;
            }

            run_cargo(vec![
                "hack",
                "--exclude",
                "ratatui-crossterm",
                #[cfg(windows)]
                "--exclude",
                #[cfg(windows)]
                "ratatui-termion",
                "check",
                "--all-targets",
                "--all-features",
            ])
        } else {
            run_cargo(vec!["check", "--all-targets"])
        }
    }
}

/// Check that `ratatui` built with only `crossterm_feature` depends on that crossterm version
/// alone, so an application using the same crossterm version gets matching types.
fn check_ratatui_crossterm_version(crossterm_feature: &str) -> Result<()> {
    let tree = cmd(
        "cargo",
        [
            "tree",
            "--package",
            "ratatui",
            "--no-default-features",
            "--features",
            crossterm_feature,
            "--edges",
            "normal",
            "--prefix",
            "none",
            "--format",
            "{p}",
        ],
    )
    .stdout_capture()
    .run_with_trace()?;
    let tree = String::from_utf8(tree.stdout)?;
    let expected = crossterm_feature.replace("crossterm_0_", "crossterm v0.");
    let crossterm_versions: Vec<&str> = tree
        .lines()
        .filter(|line| line.starts_with("crossterm v"))
        .collect();
    if crossterm_versions.is_empty()
        || crossterm_versions
            .iter()
            .any(|version| !version.starts_with(&expected))
    {
        bail!(
            "ratatui with only the `{crossterm_feature}` feature should depend on {expected} \
            only, but it depends on {crossterm_versions:?}"
        );
    }
    Ok(())
}
