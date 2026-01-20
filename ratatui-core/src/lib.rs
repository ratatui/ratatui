#![no_std]
// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
//! Foundational types and traits for building terminal user interfaces with Ratatui.
//!
//! Ratatui is a modular workspace. The `ratatui` facade crate re-exports everything for application
//! developers, while `ratatui-core` holds the primitives that make widgets, layouts, and rendering
//! possible. Keeping these APIs in a dedicated crate gives widget authors and minimalist
//! applications a stable, dependency-light surface area to build on without depending on every
//! backend or built-in widget.
//!
//! `ratatui-core` is `#![no_std]` by default with an `alloc` dependency, making it viable on
//! embedded and wasmtime targets. Opt into features when you need integrations that require the
//! Rust standard library or third-party crates.
//!
//! # Overview
//!
//! `ratatui-core` is the contract between widget authors, backend maintainers, and the higher level
//! `ratatui` facade. Reach for this crate when you are crafting reusable widgets that implement
//! [`Widget`] or [`StatefulWidget`], experimenting with new layout primitives, or targeting
//! environments where minimizing dependencies and build times matters. The API surface here changes
//! infrequently, which keeps third-party widget libraries stable across Ratatui releases.
//!
//! Applications that want batteries included—built-in widgets, backend wiring, terminal management,
//! and documentation examples—should depend on [`ratatui`], which re-exports the workspace while
//! remaining powered by these same core abstractions. The facade crate trades a slightly larger
//! dependency tree for convenience, so most end-user binaries stick with it.
//!
//! In practice, widget libraries declare `ratatui-core` as their dependency and opt into features
//! like `std`, `serde`, or `layout-cache` as needed, while application crates depend on [`ratatui`]
//! and pull in widgets, backends, and core types in one go. This split keeps the foundational APIs
//! stable and lightweight for library authors without slowing down application teams that prefer a
//! turnkey stack.
//!
//! Use `ratatui-core` when:
//! - You maintain widgets or supporting libraries that implement [`Widget`] or [`StatefulWidget`].
//! - You target constrained environments and want minimal, no-std-friendly dependencies.
//! - You need the most stable API surface across Ratatui releases.
//!
//! Use [`ratatui`] when:
//! - You are building applications that rely on built-in widgets and backend integrations.
//! - You prefer a single dependency that re-exports the entire Ratatui ecosystem.
//! - You value convenience tooling and examples over the smallest possible dependency set.
//!
//! Other workspace crates such as [`ratatui-widgets`] and [`ratatui-crossterm`] build on
//! `ratatui-core` to offer ready-made widgets and backend integrations. They can evolve at their
//! own pace because the shared types and traits live here. For a deeper dive into how these pieces
//! fit together or how to migrate between crates, see [`ARCHITECTURE.md`].
//!
//! # Crate organization
//!
//! The complete workspace layout, including macros and backend crates, is mapped out in the main
//! [`ratatui` crate documentation](https://docs.rs/ratatui/latest/ratatui/#crate-organization) and
//! in [`ARCHITECTURE.md`]. Consult those resources when you need an inventory of sibling crates.
//!
//! # Usage
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::text::Text;
//! use ratatui_core::widgets::Widget;
//!
//! struct Greeting<'a> {
//!     message: Text<'a>,
//! }
//!
//! impl<'a> Widget for Greeting<'a> {
//!     fn render(self, area: Rect, buf: &mut Buffer) {
//!         self.message.render(area, buf);
//!     }
//! }
//!
//! # fn main() {
//! let mut buf = Buffer::empty(Rect::new(0, 0, 13, 1));
//! let message = Text::from("Hello, world!");
//! let greeting = Greeting { message };
//! greeting.render(buf.area, &mut buf);
//! assert_eq!(buf, Buffer::with_lines(["Hello, world!"]));
//! # }
//! ```
//!
//! # Modules
//!
//! - [`backend`]: traits that describe drawing surfaces, terminal size queries, and event hooks.
//! - [`buffer`]: the [`Buffer`] grid structure and helpers for writing symbols and styled text.
//! - [`layout`]: constraints, rectangles, and layout helpers for splitting terminal space.
//! - [`style`]: colors, modifiers, and builder-like utilities for styling content.
//! - [`symbols`]: curated symbol sets for charts, borders, and other glyph-heavy widgets.
//! - [`terminal`]: shared terminal management primitives used by backend implementations.
//! - [`text`]: rich text containers such as [`Span`], [`Line`], and [`Text`].
//! - [`widgets`]: the [`Widget`] and [`StatefulWidget`] traits implemented by every Ratatui widget.
#![cfg_attr(feature = "document-features", doc = "\n# Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! # Contributing
//!
//! We welcome contributions from the community. Please review [`CONTRIBUTING`][CONTRIBUTING] for
//! guidance on participating.
//!
//! # License
//!
//! This project is licensed under the MIT License. See [`LICENSE`][LICENSE] for details.
//!
//! [`Buffer`]: crate::buffer::Buffer
//! [`Span`]: crate::text::Span
//! [`Line`]: crate::text::Line
//! [`Text`]: crate::text::Text
//! [`Widget`]: crate::widgets::Widget
//! [`StatefulWidget`]: crate::widgets::StatefulWidget
//! [`ratatui`]: https://crates.io/crates/ratatui
//! [`ratatui-widgets`]: https://crates.io/crates/ratatui-widgets
//! [`ratatui-crossterm`]: https://crates.io/crates/ratatui-crossterm
//! [CONTRIBUTING]: ../CONTRIBUTING.md
//! [LICENSE]: ../LICENSE
//! [`ARCHITECTURE.md`]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md
//! [`Ratatui Website`]: https://ratatui.rs

#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod widgets;
