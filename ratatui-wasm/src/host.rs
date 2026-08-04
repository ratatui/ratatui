//! WebAssembly host for loading and rendering `ratatui:widget/widget` components.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::exports::ratatui::widget::widget::{Event, RenderResult};
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

        let mut builder = WasiCtxBuilder::new();
        apply_capabilities(&mut builder, capabilities);
        let wasi = builder.build();
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

    /// Render the widget into the given buffer area without state persistence.
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) -> Result<()> {
        let mut state = Vec::new();
        self.render_stateful(area, buf, &mut state)
    }

    /// Render the widget while persisting opaque state across frames.
    pub fn render_stateful(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut Vec<u8>,
    ) -> Result<()> {
        let wit_area = rect_to_wit(area);
        let input_state = if state.is_empty() { None } else { Some(state.clone()) };
        let RenderResult { cells, state: new_state } = self
            .binding
            .ratatui_widget_widget()
            .call_render(&mut self.store, wit_area, input_state.as_deref())
            .context("calling widget render")?
            .map_err(|e| anyhow::anyhow!("widget render failed: {e:?}"))?;
        if let Some(new_state) = new_state {
            *state = new_state;
        }
        blit_cells(area, &cells, buf);
        Ok(())
    }

    /// Deliver an input event to the widget.
    ///
    /// Returns `true` if the widget consumed the event.
    pub fn handle_event(&mut self, event: &Event) -> Result<bool> {
        self.binding
            .ratatui_widget_widget()
            .call_handle_event(&mut self.store, event)
            .context("calling widget handle_event")?
            .map_err(|e| anyhow::anyhow!("widget handle_event failed: {e:?}"))
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

/// A stateful Ratatui widget backed by a WASM plugin.
///
/// Use this with `Frame::render_stateful_widget` to let the guest persist opaque
/// state across frames.
#[derive(Clone, Debug)]
pub struct StatefulWasmWidget {
    path: PathBuf,
    capabilities: Vec<String>,
}

impl StatefulWasmWidget {
    /// Create a stateful WASM-backed widget from the given component path and
    /// granted capabilities.
    pub fn from_file(path: impl AsRef<Path>, capabilities: &[String]) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            capabilities: capabilities.to_vec(),
        }
    }
}

impl StatefulWidget for StatefulWasmWidget {
    type State = Vec<u8>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Vec<u8>) {
        match PluginWidget::from_file(&self.path, &self.capabilities) {
            Ok(mut plugin) => {
                if let Err(err) = plugin.render_stateful(area, buf, state) {
                    tracing::debug!("failed to render stateful WASM widget: {err:#}");
                }
            }
            Err(err) => {
                tracing::debug!("failed to load stateful WASM widget: {err:#}");
            }
        }
    }
}

/// Configure a WASI context builder according to the granted capabilities.
fn apply_capabilities(builder: &mut WasiCtxBuilder, capabilities: &[String]) {
    if capabilities.iter().any(|c| c == "stdio:stdout") {
        builder.inherit_stdout();
    }
    if capabilities.iter().any(|c| c == "stdio:stderr") {
        builder.inherit_stderr();
    }
    if capabilities.iter().any(|c| c == "stdio:stdin") {
        builder.inherit_stdin();
    }
    if capabilities.iter().any(|c| c == "env:read") {
        builder.inherit_env();
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
