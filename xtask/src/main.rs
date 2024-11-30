//! A simple task runner for the project.
//!
//! See <https://github.com/matklad/cargo-xtask> for details on the xtask pattern.
//!
//! Run `cargo xtask --help` for more information

use std::{fmt::Debug, io, process::Output, vec};

use cargo_metadata::{MetadataCommand, TargetKind};
use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::{eyre::Context, Result};
use duct::cmd;
use tracing::level_filters::LevelFilter;
use tracing_log::AsTrace;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level())
        .without_time()
        .init();

    match args.run() {
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

impl Args {
    fn run(self) -> Result<()> {
        self.command.run()
    }

    fn log_level(&self) -> LevelFilter {
        self.verbosity.log_level_filter().as_trace()
    }
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Run CI checks (lint, build, test)
    CI,

    /// Build the project
    #[command(visible_alias = "b")]
    Build,

    /// Run cargo check
    #[command(visible_alias = "c")]
    Check,

    /// Check if README.md is up-to-date
    #[command(visible_alias = "cr")]
    CheckReadme,

    /// Generate code coverage report
    #[command(visible_alias = "cov")]
    Coverage,

    /// Lint formatting, typos, clippy, and docs
    #[command(visible_alias = "l")]
    Lint,

    /// Run clippy on the project
    #[command(visible_alias = "cl")]
    LintClippy,

    /// Check documentation for errors and warnings
    #[command(visible_alias = "d")]
    LintDocs,

    /// Check for formatting issues in the project
    #[command(visible_alias = "lf")]
    LintFormatting,

    /// Lint markdown files
    #[command(visible_alias = "md")]
    LintMarkdown,

    /// Check for typos in the project
    #[command(visible_alias = "lt")]
    LintTypos,

    /// Fix clippy warnings in the project
    #[command(visible_alias = "fc")]
    FixClippy,

    /// Fix formatting issues in the project
    #[command(visible_alias = "fmt")]
    FixFormatting,

    /// Fix README.md (by running cargo-rdme)
    #[command(visible_alias = "fr")]
    FixReadme,

    /// Fix typos in the project
    #[command(visible_alias = "typos")]
    FixTypos,

    /// Run tests
    #[command(visible_alias = "t")]
    Test,

    /// Test backend
    #[command(visible_alias = "tb")]
    TestBackend { backend: Backend },

    /// Run doc tests
    #[command(visible_alias = "td")]
    TestDocs,

    /// Run lib tests
    #[command(visible_alias = "tl")]
    TestLibs,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Backend {
    Crossterm,
    Termion,
    Termwiz,
}

impl Command {
    fn run(self) -> Result<()> {
        match self {
            Command::CI => ci(),
            Command::Build => build(),
            Command::Check => check(),
            Command::CheckReadme => check_readme(),
            Command::Coverage => coverage(),
            Command::Lint => lint(),
            Command::LintClippy => lint_clippy(),
            Command::LintDocs => lint_docs(),
            Command::LintFormatting => lint_format(),
            Command::LintTypos => lint_typos(),
            Command::LintMarkdown => lint_markdown(),
            Command::FixClippy => fix_clippy(),
            Command::FixFormatting => fix_format(),
            Command::FixReadme => fix_readme(),
            Command::FixTypos => fix_typos(),
            Command::Test => test(),
            Command::TestBackend { backend } => test_backend(backend),
            Command::TestDocs => test_docs(),
            Command::TestLibs => test_libs(),
        }
    }
}

/// Run CI checks (lint, build, test)
fn ci() -> Result<()> {
    lint()?;
    build()?;
    test()?;
    Ok(())
}

/// Build the project
fn build() -> Result<()> {
    run_cargo(vec!["build", "--all-targets", "--all-features"])
}

/// Run cargo check
fn check() -> Result<()> {
    run_cargo(vec!["check", "--all-targets", "--all-features"])
}

/// Run cargo-rdme to check if README.md is up-to-date with the library documentation
fn check_readme() -> Result<()> {
    for package in get_workspace_packages(Some(TargetKind::Lib))? {
        run_cargo(vec!["rdme", "--workspace-project", &package, "--check"])?;
    }
    Ok(())
}

fn fix_readme() -> Result<()> {
    for package in get_workspace_packages(Some(TargetKind::Lib))? {
        run_cargo(vec!["rdme", "--workspace-project", &package])?;
    }
    Ok(())
}

/// Generate code coverage report
fn coverage() -> Result<()> {
    run_cargo(vec![
        "llvm-cov",
        "--lcov",
        "--output-path",
        "target/lcov.info",
        "--all-features",
    ])
}

/// Lint formatting, typos, clippy, and docs (and a soft fail on markdown)
fn lint() -> Result<()> {
    lint_clippy()?;
    lint_docs()?;
    lint_format()?;
    lint_typos()?;
    if let Err(err) = lint_markdown() {
        tracing::warn!("known issue: markdownlint is currently noisy and can be ignored: {err}");
    }
    Ok(())
}

/// Run clippy on the project
fn lint_clippy() -> Result<()> {
    run_cargo(vec![
        "clippy",
        "--all-targets",
        "--all-features",
        "--tests",
        "--benches",
        "--",
        "-D",
        "warnings",
    ])
}

/// Fix clippy warnings in the project
fn fix_clippy() -> Result<()> {
    run_cargo(vec![
        "clippy",
        "--all-targets",
        "--all-features",
        "--tests",
        "--benches",
        "--",
        "-D",
        "warnings",
        "--fix",
    ])
}

/// Check that docs build without errors using flags for docs.rs
fn lint_docs() -> Result<()> {
    for package in get_workspace_packages(Some(TargetKind::Lib))? {
        run_cargo_nightly(vec!["docs-rs", "--package", &package])?;
    }
    Ok(())
}

/// Return the available packages in the workspace
fn get_workspace_packages(target_filter: Option<TargetKind>) -> Result<Vec<String>> {
    let meta = MetadataCommand::new()
        .exec()
        .wrap_err("failed to get cargo metadata")?;
    let packages = meta
        .workspace_packages()
        .iter()
        .filter(|v| {
            if let Some(ref filter) = target_filter {
                v.targets.iter().any(|t| t.kind.contains(filter))
            } else {
                true
            }
        })
        .map(|v| v.name.clone())
        .collect();
    Ok(packages)
}

/// Lint formatting issues in the project
fn lint_format() -> Result<()> {
    run_cargo_nightly(vec!["fmt", "--all", "--check"])
}

/// Fix formatting issues in the project
fn fix_format() -> Result<()> {
    run_cargo_nightly(vec!["fmt", "--all"])
}

/// Lint markdown files using [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2)
fn lint_markdown() -> Result<()> {
    cmd!("markdownlint-cli2", "**/*.md", "!target").run_with_trace()?;
    Ok(())
}

/// Check for typos in the project using [typos-cli](https://github.com/crate-ci/typos/)
fn lint_typos() -> Result<()> {
    cmd!("typos").run_with_trace()?;
    Ok(())
}

/// Fix typos in the project
fn fix_typos() -> Result<()> {
    cmd!("typos", "-w").run_with_trace()?;
    Ok(())
}

/// Run tests for libs, backends, and docs
fn test() -> Result<()> {
    test_libs()?;
    test_backend(Backend::Crossterm)?;
    test_backend(Backend::Termion)?;
    test_backend(Backend::Termwiz)?;
    test_docs()?; // run last because it's slow
    Ok(())
}

/// Run tests for the specified backend
fn test_backend(backend: Backend) -> Result<()> {
    if cfg!(windows) && backend == Backend::Termion {
        tracing::error!("termion backend is not supported on Windows");
    }
    let backend = match backend {
        Backend::Crossterm => "crossterm",
        Backend::Termion => "termion",
        Backend::Termwiz => "termwiz",
    };
    run_cargo(vec![
        "test",
        "--all-targets",
        "--no-default-features",
        "--features",
        backend,
    ])
}

/// Run doc tests for the workspace's default packages
fn test_docs() -> Result<()> {
    run_cargo(vec!["test", "--doc", "--all-features"])
}

/// Run lib tests for the workspace's default packages
fn test_libs() -> Result<()> {
    run_cargo(vec!["test", "--all-targets", "--all-features"])
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
