//! Small CLI for managing Ratatui WASM plugins.
//!
//! Enabled with the `cli` feature. Build with:
//!
//! ```sh
//! cargo build -p ratatui-wasm --features cli
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ratatui_wasm::PluginWidget;
use ratatui_wasm::manifest::PluginManifest;

#[derive(Parser)]
#[command(name = "ratatui-wasm", about = "Manage Ratatui WASM plugins")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Discover plugin manifests in a directory tree.
    List {
        /// Directory to search.
        dir: PathBuf,
    },
    /// Validate a plugin manifest and try to load the wasm component.
    Check {
        /// Path to a `ratatui.plugin.toml` file.
        manifest: PathBuf,
    },
    /// Build a plugin guest crate for `wasm32-wasip2`.
    Build {
        /// Directory containing the guest `Cargo.toml`.
        guest_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::List { dir } => list_plugins(&dir),
        Command::Check { manifest } => check_manifest(&manifest),
        Command::Build { guest_dir } => build_guest(&guest_dir),
    }
}

fn list_plugins(dir: &Path) -> Result<()> {
    let found = discover_manifests(dir)?;
    if found.is_empty() {
        println!("no ratatui.plugin.toml files found in {}", dir.display());
        return Ok(());
    }
    for path in found {
        let manifest = PluginManifest::from_file(&path)?;
        println!(
            "{name}@{version} ({author}) => {entry}",
            name = manifest.plugin.name,
            version = manifest.plugin.version,
            author = manifest.plugin.author.as_deref().unwrap_or("unknown"),
            entry = manifest
                .resolve_entry(path.parent().unwrap_or(dir))
                .display(),
        );
    }
    Ok(())
}

fn check_manifest(manifest: &Path) -> Result<()> {
    let widget = PluginWidget::from_manifest(manifest)?;
    let capabilities = widget
        .capabilities()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "{}: OK (capabilities: [{}])",
        manifest.display(),
        capabilities
    );
    Ok(())
}

fn build_guest(guest_dir: &Path) -> Result<()> {
    let status = std::process::Command::new("cargo")
        .args(["build", "--target", "wasm32-wasip2", "--release"])
        .current_dir(guest_dir)
        .status()
        .with_context(|| format!("spawning cargo in {}", guest_dir.display()))?;
    anyhow::ensure!(
        status.success(),
        "cargo build failed for {}",
        guest_dir.display()
    );
    Ok(())
}

fn discover_manifests(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut manifests = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let entries = fs::read_dir(&current)
            .with_context(|| format!("reading directory {}", current.display()))?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            match path.file_name().and_then(|n| n.to_str()) {
                Some("ratatui.plugin.toml") => manifests.push(path),
                _ if path.is_dir() => stack.push(path),
                _ => {}
            }
        }
    }
    manifests.sort();
    Ok(manifests)
}
