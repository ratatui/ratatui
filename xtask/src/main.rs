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
use itertools::{Itertools, Position};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
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
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Run CI checks (lint, build, test)
    CI,

    /// Build the project
    #[command(visible_alias = "b")]
    Build,

    #[command(visible_alias = "c")]
    Check(CheckCommand),

    /// Run cargo check with crossterm feature
    #[command(visible_alias = "cc")]
    CheckCrossterm,

    /// Run cargo check with termion feature
    #[command(visible_alias = "ct")]
    CheckTermion,

    /// Run cargo check with termwiz feature
    #[command(visible_alias = "cw")]
    CheckTermwiz,

    /// Check if README.md is up-to-date (using cargo-rdme)
    #[command(visible_alias = "cr", alias = "rdme")]
    Readme(ReadmeCommand),

    /// Generate code coverage report
    #[command(visible_alias = "cov")]
    Coverage,

    /// Generate code coverage for unit tests only
    #[command(visible_alias = "covu")]
    CoverageUnit,

    /// Lint formatting, typos, clippy, and docs
    #[command(visible_alias = "l")]
    Lint,

    /// Run clippy on the project
    #[command(visible_alias = "cl")]
    Clippy(ClippyCommand),

    /// Check documentation for errors and warnings
    #[command(name = "docs", visible_alias = "d")]
    Docs(DocsCommand),

    /// Check for formatting issues in the project
    #[command(visible_aliases = ["fmt", "f"])]
    Format(FormatCommand),

    /// Lint markdown files
    #[command(visible_alias = "md")]
    LintMarkdown,

    /// Check for typos in the project
    #[command(visible_alias = "ty")]
    Typos(TyposCommand),

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

    /// Run cargo hack to test each feature in isolation
    #[command(visible_alias = "h")]
    Hack,
}

/// Run cargo check
#[derive(Clone, Debug, clap::Args)]
struct CheckCommand {
    /// Check all features
    #[arg(long, visible_alias = "all")]
    all_features: bool,
}

/// Check documentation for errors and warnings
#[derive(Clone, Debug, clap::Args)]
struct DocsCommand {
    /// Open the documentation in the browser
    #[arg(long)]
    open: bool,
}

/// Check for formatting issues in the project
#[derive(Clone, Debug, clap::Args)]
struct FormatCommand {
    /// Check formatting issues
    #[arg(long)]
    check: bool,
}

/// Run clippy on the project
#[derive(Clone, Debug, clap::Args)]
struct ClippyCommand {
    /// Fix clippy warnings
    #[arg(long)]
    fix: bool,
}

/// Check if README.md is up-to-date (using cargo-rdme)
#[derive(Clone, Debug, clap::Args)]
struct ReadmeCommand {
    /// Check if README.md is up-to-date
    #[arg(long)]
    check: bool,
}

/// Check for typos in the project
#[derive(Clone, Debug, clap::Args)]
struct TyposCommand {
    /// Fix typos
    #[arg(long)]
    fix: bool,
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
            Command::Check(command) => command.run(),
            Command::CheckCrossterm => check_crossterm(),
            Command::CheckTermion => check_termion(),
            Command::CheckTermwiz => check_termwiz(),
            Command::Readme(command) => command.run(),
            Command::Coverage => coverage(),
            Command::CoverageUnit => coverage_unit(),
            Command::Lint => lint(),
            Command::Clippy(command) => command.run(),
            Command::Docs(command) => command.run(),
            Command::Format(command) => command.run(),
            Command::Typos(command) => command.run(),
            Command::LintMarkdown => lint_markdown(),
            Command::Test => test(),
            Command::TestBackend { backend } => test_backend(backend),
            Command::TestDocs => test_docs(),
            Command::TestLibs => test_libs(),
            Command::Hack => hack(),
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

impl CheckCommand {
    fn run(self) -> Result<()> {
        if self.all_features {
            run_cargo(vec!["check", "--all-targets", "--all-features"])
        } else {
            run_cargo(vec!["check", "--all-targets"])
        }
    }
}

/// Run cargo check with crossterm feature
fn check_crossterm() -> Result<()> {
    run_cargo(vec![
        "check",
        "--all-targets",
        "--all-features",
        "--no-default-features",
        "--features",
        "crossterm",
    ])
}

/// Run cargo check with termion feature
fn check_termion() -> Result<()> {
    run_cargo(vec![
        "check",
        "--all-targets",
        "--all-features",
        "--no-default-features",
        "--features",
        "termion",
    ])
}

/// Run cargo check with termwiz feature
fn check_termwiz() -> Result<()> {
    run_cargo(vec![
        "check",
        "--all-targets",
        "--all-features",
        "--no-default-features",
        "--features",
        "termwiz",
    ])
}

impl ReadmeCommand {
    fn run(self) -> Result<()> {
        let args = if self.check {
            vec!["rdme", "--check"]
        } else {
            vec!["rdme"]
        };
        for package in workspace_packages(TargetKind::Lib)? {
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

/// Generate code coverage for unit tests only
fn coverage_unit() -> Result<()> {
    run_cargo(vec![
        "llvm-cov",
        "--lcov",
        "--output-path",
        "target/lcov-unit.info",
        "--all-features",
        "--lib",
    ])
}

/// Lint formatting, typos, clippy, and docs (and a soft fail on markdown)
fn lint() -> Result<()> {
    ClippyCommand { fix: false }.run()?;
    DocsCommand { open: false }.run()?;
    FormatCommand { check: true }.run()?;
    TyposCommand { fix: false }.run()?;
    if let Err(err) = lint_markdown() {
        tracing::warn!("known issue: markdownlint is currently noisy and can be ignored: {err}");
    }
    Ok(())
}

impl ClippyCommand {
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

impl DocsCommand {
    fn run(self) -> Result<()> {
        let packages = workspace_packages(TargetKind::Lib)?;
        for (position, package) in packages.iter().with_position() {
            let mut args = vec!["docs-rs", "--package", &package];
            if self.open && matches!(position, Position::Last | Position::Only) {
                args.push("--open");
            }
            run_cargo_nightly(args)?;
        }
        Ok(())
    }
}

/// Return the available packages in the workspace
fn workspace_packages(kind: TargetKind) -> Result<Vec<String>> {
    let meta = MetadataCommand::new()
        .exec()
        .wrap_err("failed to get cargo metadata")?;
    let packages = meta
        .workspace_packages()
        .iter()
        .filter(|v| v.targets.iter().any(|t| t.kind.contains(&kind)))
        .map(|v| v.name.clone())
        .collect();
    Ok(packages)
}

impl FormatCommand {
    fn run(self) -> Result<()> {
        self.run_rustfmt()?;
        self.run_taplo()?;
        Ok(())
    }

    fn run_rustfmt(&self) -> Result<(), color_eyre::eyre::Error> {
        let mut args = vec!["fmt", "--all"];
        if self.check {
            args.push("--check");
        }
        run_cargo_nightly(args)?;
        Ok(())
    }

    fn run_taplo(&self) -> Result<(), color_eyre::eyre::Error> {
        let mut args = vec!["format", "--colors", "always"];
        if self.check {
            args.push("--check");
        }
        cmd("taplo", args).run_with_trace()?;
        Ok(())
    }
}

/// Lint markdown files using [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2)
fn lint_markdown() -> Result<()> {
    cmd!("markdownlint-cli2", "**/*.md", "!target").run_with_trace()?;
    Ok(())
}

impl TyposCommand {
    fn run(self) -> Result<()> {
        if self.fix {
            cmd!("typos", "--write-changes").run_with_trace()?;
        } else {
            cmd!("typos").run_with_trace()?;
        }
        Ok(())
    }
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
    run_cargo(vec!["test", "--lib", "--all-targets", "--all-features"])
}

/// Run cargo hack to test each feature in isolation
fn hack() -> Result<()> {
    run_cargo(vec![
        "hack",
        "test",
        "--lib",
        "--each-feature",
        "--workspace",
    ])
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
