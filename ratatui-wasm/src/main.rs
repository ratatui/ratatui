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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[test]
    fn clap_command_parses_list() {
        let cli = Cli::parse_from(["ratatui-wasm", "list", "/tmp"]);
        match cli.command {
            Command::List { dir } => assert_eq!(dir, PathBuf::from("/tmp")),
            _ => panic!("expected List command"),
        }
    }

    #[test]
    fn clap_command_parses_check() {
        let cli = Cli::parse_from(["ratatui-wasm", "check", "/tmp/test.plugin.toml"]);
        match cli.command {
            Command::Check { manifest } => {
                assert_eq!(manifest, PathBuf::from("/tmp/test.plugin.toml"));
            }
            _ => panic!("expected Check command"),
        }
    }

    #[test]
    fn clap_command_parses_build() {
        let cli = Cli::parse_from(["ratatui-wasm", "build", "/tmp/guest"]);
        match cli.command {
            Command::Build { guest_dir } => {
                assert_eq!(guest_dir, PathBuf::from("/tmp/guest"));
            }
            _ => panic!("expected Build command"),
        }
    }

    #[test]
    fn clap_command_debug_assert() {
        <Cli as clap::CommandFactory>::command().debug_assert();
    }

    #[test]
    fn discover_manifests_finds_nested_manifests() {
        let temp_dir =
            std::env::temp_dir().join(format!("ratatui-wasm-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        fs::create_dir_all(temp_dir.join("nested")).unwrap();
        fs::write(
            temp_dir.join("ratatui.plugin.toml"),
            "[plugin]\nname = \"root\"\nversion = \"1.0\"\nentry = \"root.wasm\"\n",
        )
        .unwrap();
        fs::write(
            temp_dir.join("nested").join("ratatui.plugin.toml"),
            "[plugin]\nname = \"nested\"\nversion = \"1.0\"\nentry = \"nested.wasm\"\n",
        )
        .unwrap();
        fs::write(temp_dir.join("other.txt"), "ignored").unwrap();

        let found = discover_manifests(&temp_dir).unwrap();
        assert_eq!(found.len(), 2);
        assert!(
            found
                .iter()
                .any(|p| p.ends_with("nested/ratatui.plugin.toml"))
        );
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn discover_manifests_returns_empty_when_none() {
        let temp_dir =
            std::env::temp_dir().join(format!("ratatui-wasm-empty-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(temp_dir.join("other.txt"), "ignored").unwrap();

        let found = discover_manifests(&temp_dir).unwrap();
        assert!(found.is_empty());
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn list_plugins_with_valid_manifest() {
        let manifest = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../examples/wasm-widgets/hello-rust/ratatui.plugin.toml"
        );
        let dir = Path::new(manifest).parent().unwrap();
        list_plugins(dir).expect("list_plugins should succeed");
    }

    #[test]
    fn check_manifest_with_valid_widget() {
        let manifest = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../examples/wasm-widgets/hello-rust/ratatui.plugin.toml"
        );
        check_manifest(Path::new(manifest)).expect("check_manifest should succeed");
    }

    #[test]
    fn list_plugins_empty_directory() {
        let temp_dir =
            std::env::temp_dir().join(format!("ratatui-wasm-empty-list-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        list_plugins(&temp_dir).expect("list_plugins should succeed on empty dir");
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn discover_manifests_fails_when_dir_missing() {
        let result = discover_manifests(Path::new("/nonexistent-ratatui-wasm-dir"));
        assert!(result.is_err());
    }

    #[test]
    fn check_manifest_fails_when_widget_missing() {
        let temp_dir =
            std::env::temp_dir().join(format!("ratatui-wasm-check-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        let manifest = temp_dir.join("ratatui.plugin.toml");
        fs::write(
            &manifest,
            "[plugin]\nname = \"missing\"\nversion = \"1.0\"\nentry = \"missing.wasm\"\n",
        )
        .unwrap();
        let result = check_manifest(&manifest);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
