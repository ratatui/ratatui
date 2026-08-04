//! Component cache keyed by file modification time.
//!
//! This module provides hot-reload semantics: a `.wasm` file is reloaded only
//! when its modification time changes. Otherwise the already parsed
//! [`Component`] is reused.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::time::SystemTime;

use anyhow::{Context, Result};
use wasmtime::component::Component;
use wasmtime::Engine;

struct CachedComponent {
    engine: Engine,
    component: Component,
    mtime: SystemTime,
}

static COMPONENT_CACHE: LazyLock<Mutex<HashMap<PathBuf, CachedComponent>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Load a [`Component`] from `path`, reusing a cached parse when the file has
/// not changed on disk.
pub fn get_component(path: &Path) -> Result<(Engine, Component)> {
    let mut cache = COMPONENT_CACHE.lock().expect("component cache poisoned");
    let mtime = fs::metadata(path)
        .and_then(|m| m.modified())
        .with_context(|| format!("reading metadata for {}", path.display()))?;

    if let Some(cached) = cache.get(path) {
        if cached.mtime == mtime {
            return Ok((cached.engine.clone(), cached.component.clone()));
        }
    }

    let engine = Engine::default();
    let component = Component::from_file(&engine, path)
        .with_context(|| format!("loading wasm component from {}", path.display()))?;

    cache.insert(
        path.to_path_buf(),
        CachedComponent {
            engine: engine.clone(),
            component: component.clone(),
            mtime,
        },
    );

    Ok((engine, component))
}
