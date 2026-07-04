use std::env;

use color_eyre::Result;
use duct::cmd;

use crate::{ExpressionExt, Run, run_cargo_nightly};

/// Check documentation for errors and warnings
#[derive(Clone, Debug, clap::Args)]
pub struct Docs {
    /// Open the documentation in the browser
    #[arg(long)]
    pub open: bool,
}

impl Run for Docs {
    fn run(self) -> Result<()> {
        lint_workspace_docs()?;

        // cargo +nightly hack --all --ignore-private docs-rs
        let mut args = vec!["hack", "--all", "--ignore-private", "docs-rs"];
        if self.open {
            args.push("--open");
        }
        run_cargo_nightly(args)
    }
}

fn lint_workspace_docs() -> Result<()> {
    let rustdocflags = match env::var("RUSTDOCFLAGS") {
        Ok(flags) if !flags.is_empty() => format!("{flags} -Dwarnings"),
        _ => "-Dwarnings".to_string(),
    };

    cmd!("cargo", "doc", "--all-features", "--no-deps")
        .env("RUSTDOCFLAGS", rustdocflags)
        .run_with_trace()?;
    Ok(())
}
