//! WebAssembly host for loading and rendering `ratatui:widget/widget` components.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::generated::WasmWidget as WasmWidgetBinding;
use crate::{blit_cells, rect_to_wit};

/// A loaded WASM widget with its granted capabilities.
pub struct PluginWidget {
    store: Store<WasiState>,
    binding: Box<WasmWidgetBinding>,
    capabilities: Vec<String>,
}

impl PluginWidget {
    /// Load a `.wasm` component from the given path.
    pub fn from_file(path: impl AsRef<Path>, capabilities: &[String]) -> Result<Self> {
        let engine = Engine::default();
        let component = Component::from_file(&engine, path.as_ref())
            .with_context(|| format!("loading wasm component from {}", path.as_ref().display()))?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let wasi = WasiCtxBuilder::new().inherit_stdout().build();
        let mut store = Store::new(&engine, WasiState::new(wasi));

        let binding = Box::new(
            WasmWidgetBinding::instantiate(&mut store, &component, &linker)
                .context("instantiating wasm widget component")?,
        );

        let caps = binding
            .ratatui_widget_widget()
            .call_capabilities(&mut store)
            .context("calling widget capabilities")?;

        let granted: Vec<String> = capabilities.iter().cloned().collect();
        for required in &caps {
            if !granted.contains(required) {
                anyhow::bail!("widget requires capability `{required}` which was not granted");
            }
        }

        Ok(Self {
            store,
            binding,
            capabilities: granted,
        })
    }

    /// Render the widget into the given buffer area.
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) -> Result<()> {
        let wit_area = rect_to_wit(area);
        let cells = self
            .binding
            .ratatui_widget_widget()
            .call_render(&mut self.store, wit_area, None)
            .context("calling widget render")?
            .map_err(|e| anyhow::anyhow!("widget render failed: {e:?}"))?;
        blit_cells(area, &cells, buf);
        Ok(())
    }

    /// Capabilities granted to this widget instance.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }
}

/// A host capable of loading multiple WASM widgets.
#[derive(Debug, Default)]
pub struct WasmWidgetHost;

impl WasmWidgetHost {
    /// Create a new empty host.
    pub fn new() -> Self {
        Self
    }
}

/// A native Ratatui [`ratatui_core::widgets::Widget`] wrapper around a WASM plugin.
///
/// `WasmWidget` can be passed directly to `Frame::render_widget`. The guest WASM
/// module is instantiated on every render, which keeps the wrapper cheaply
/// cloneable and trivially hot-reloadable.
#[derive(Clone, Debug)]
pub struct WasmWidget {
    path: PathBuf,
    capabilities: Vec<String>,
}

impl WasmWidget {
    /// Create a WASM-backed widget from the given component path and granted
    /// capabilities.
    pub fn from_file(path: impl AsRef<Path>, capabilities: &[String]) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            capabilities: capabilities.to_vec(),
        }
    }
}

impl ratatui_core::widgets::Widget for WasmWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match PluginWidget::from_file(&self.path, &self.capabilities) {
            Ok(mut plugin) => {
                if let Err(err) = plugin.render(area, buf) {
                    tracing::debug!("failed to render WASM widget: {err:#}");
                }
            }
            Err(err) => {
                tracing::debug!("failed to load WASM widget: {err:#}");
            }
        }
    }
}

struct WasiState {
    ctx: WasiCtx,
    table: wasmtime::component::ResourceTable,
}

impl WasiState {
    fn new(ctx: WasiCtx) -> Self {
        Self {
            ctx,
            table: wasmtime::component::ResourceTable::new(),
        }
    }
}

impl WasiView for WasiState {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
