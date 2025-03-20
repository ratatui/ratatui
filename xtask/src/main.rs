//! A simple task runner for the project.
//!
//! See <https://github.com/matklad/cargo-xtask> for details on the xtask pattern.
//!
//! Run `cargo xtask --help` for more information

use std::io;
use std::process::Output;

use cargo_metadata::{MetadataCommand, TargetKind};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::eyre::Context;
use color_eyre::Result;
use commands::Command;
use duct::cmd;

mod commands;

pub trait Run {
    fn run(self) -> Result<()>;
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .without_time()
        .init();

    match args.command.run() {
        Ok(_) => (),
        Err(err) => {
            tracing::error!("{err}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(bin_name = "cargo xtask", styles = clap_cargo::style::CLAP_STYLING)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,
}

/// Return the available libs in the workspace
fn workspace_libs() -> Result<Vec<String>> {
    let meta = MetadataCommand::new()
        .exec()
        .wrap_err("failed to get cargo metadata")?;
    let packages = meta
        .workspace_packages()
        .iter()
        .filter(|v| v.targets.iter().any(|t| t.kind.contains(&TargetKind::Lib)))
        .map(|v| v.name.clone())
        .collect();
    Ok(packages)
}

/// Run a cargo subcommand with the default toolchain
fn run_cargo(args: Vec<&str>) -> Result<()> {
    cmd("cargo", args).run_with_trace()?;
    Ok(())
}

/// Run a cargo subcommand with the nightly toolchain
fn run_cargo_nightly(args: Vec<&str>) -> Result<()> {
    cmd("cargo", args)
        // CARGO env var is set because we're running in a cargo subcommand
        .env_remove("CARGO")
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .run_with_trace()?;
    Ok(())
}

/// An extension trait for `duct::Expression` that logs the command being run
/// before running it.
trait ExpressionExt {
    /// Run the command and log the command being run
    fn run_with_trace(&self) -> io::Result<Output>;
}

impl ExpressionExt for duct::Expression {
    fn run_with_trace(&self) -> io::Result<Output> {
        tracing::info!("running command: {:?}", self);
        self.run().inspect_err(|_| {
            // The command that was run may have scrolled off the screen, so repeat it here
            tracing::error!("failed to run command: {:?}", self);
        })
    }
}
