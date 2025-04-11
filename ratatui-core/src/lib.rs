#![no_std]
// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
//! **ratatui-core** is the core library of the [ratatui] project,
//! providing the essential building blocks for creating rich terminal user interfaces in Rust.
//!
//! [ratatui]: https://github.com/ratatui/ratatui
//!
//! ## Why `ratatui-core`?
//!
//! The `ratatui-core` crate is split from the main [`ratatui`](https://crates.io/crates/ratatui) crate
//! to offer better stability for widget library authors. Widget libraries should generally depend
//! on `ratatui-core`, benefiting from a stable API and reducing the need for frequent updates.
//!
//! Applications, on the other hand, should depend on the main `ratatui` crate, which includes
//! built-in widgets and additional features.
//!
//! # Installation
//!
//! Add `ratatui-core` to your `Cargo.toml`:
//!
//! ```shell
//! cargo add ratatui-core
//! ```
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! # Contributing
//!
//! We welcome contributions from the community! Please see our [CONTRIBUTING](../CONTRIBUTING.md)
//! guide for more details on how to get involved.
//!
//! ## License
//!
//! This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]

extern crate std;

extern crate alloc;

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod widgets;
