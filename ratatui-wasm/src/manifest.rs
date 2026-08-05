//! Plugin manifest parsing for `ratatui.plugin.toml` files.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

/// A parsed plugin manifest.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginManifest {
    /// Metadata describing the plugin.
    pub plugin: PluginMeta,
    /// Capabilities requested by the plugin.
    #[serde(default)]
    pub capabilities: Capabilities,
}

/// Metadata section of a plugin manifest.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginMeta {
    /// Human-readable plugin name.
    pub name: String,
    /// Plugin version in semver form.
    pub version: String,
    /// Plugin author.
    pub author: Option<String>,
    /// Path to the `.wasm` component, relative to the manifest directory.
    pub entry: PathBuf,
}

/// Capability requirements declared by a plugin.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Capabilities {
    /// Capabilities the plugin cannot run without.
    #[serde(default)]
    pub required: Vec<String>,
    /// Capabilities the plugin can use if granted.
    #[serde(default)]
    pub optional: Vec<String>,
}

impl PluginManifest {
    /// Parse a manifest from a TOML string.
    pub fn from_str(content: &str) -> Result<Self> {
        toml::from_str(content).context("parsing plugin manifest")
    }

    /// Load and parse a manifest from the given path.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("reading {}", path.as_ref().display()))?;
        Self::from_str(&content)
    }

    /// Return the absolute path to the plugin entry wasm file.
    ///
    /// If `entry` is already absolute it is returned unchanged. Otherwise it is
    /// resolved relative to the manifest's directory.
    pub fn resolve_entry(&self, manifest_dir: impl AsRef<Path>) -> PathBuf {
        let entry = &self.plugin.entry;
        if entry.is_absolute() {
            entry.clone()
        } else {
            manifest_dir.as_ref().join(entry)
        }
    }

    /// Return all capabilities that should be granted to the plugin.
    ///
    /// Currently this returns only the required capabilities. Optional
    /// capabilities are granted through host configuration in the future.
    pub fn granted_capabilities(&self) -> Vec<String> {
        self.capabilities.required.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_manifest() {
        let content = r#"
[plugin]
name = "hello-widget"
version = "0.1.0"
author = "yunuservices"
entry = "target/wasm32-wasip2/release/hello_widget.wasm"

[capabilities]
required = ["stdio:stdout"]
optional = ["clock:read"]
"#;
        let manifest = PluginManifest::from_str(content).unwrap();
        assert_eq!(manifest.plugin.name, "hello-widget");
        assert_eq!(manifest.plugin.version, "0.1.0");
        assert_eq!(manifest.plugin.author.as_deref(), Some("yunuservices"));
        assert_eq!(
            manifest.plugin.entry,
            PathBuf::from("target/wasm32-wasip2/release/hello_widget.wasm")
        );
        assert_eq!(manifest.capabilities.required, vec!["stdio:stdout"]);
        assert_eq!(manifest.capabilities.optional, vec!["clock:read"]);
        assert_eq!(
            manifest.resolve_entry("/tmp"),
            PathBuf::from("/tmp/target/wasm32-wasip2/release/hello_widget.wasm")
        );
    }

    #[test]
    fn manifest_without_optional_caps() {
        let content = r#"
[plugin]
name = "minimal"
version = "1.0.0"
entry = "plugin.wasm"
"#;
        let manifest = PluginManifest::from_str(content).unwrap();
        assert!(manifest.capabilities.required.is_empty());
        assert!(manifest.capabilities.optional.is_empty());
        assert_eq!(manifest.granted_capabilities().len(), 0);
    }
}
