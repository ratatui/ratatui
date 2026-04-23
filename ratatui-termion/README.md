# Ratatui-termion

<!-- cargo-rdme start -->

This module provides the [`TermionBackend`] implementation for the [`Backend`] trait. It uses
the [Termion] crate to interact with the terminal.

Most application authors should start with the main [`ratatui`] crate and only depend on
`ratatui-termion` directly when they specifically want the Termion backend. This crate is the
backend layer, not the primary docs.rs entry point for building applications.

[`Backend`]: ratatui_core::backend::Backend
[Termion]: https://docs.rs/termion

## Crate Organization

`ratatui-termion` is part of the Ratatui workspace that was modularized in version 0.30.0.
This crate provides the [Termion] backend implementation for Ratatui.

**When to use `ratatui-termion`:**

- You want to depend on the Termion backend crate directly
- You prefer Termion's Unix-focused approach

**When to use the main [`ratatui`] crate:**

- Building applications
- You want backend selection to stay behind Ratatui's re-exports

For detailed information about the workspace organization, see [ARCHITECTURE.md].

[`ratatui`]: https://crates.io/crates/ratatui
[ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md

<!-- cargo-rdme end -->
