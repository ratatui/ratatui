#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod layout;
mod line;
mod row;
mod span;
mod text;

// Re-export the core crate to use the types in macros
pub use ratatui_core;
