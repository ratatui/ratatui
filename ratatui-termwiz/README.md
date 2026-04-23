# Ratatui-termwiz

<!-- cargo-rdme start -->

This module provides the [`TermwizBackend`] implementation for the [`Backend`] trait. It uses
the [Termwiz] crate to interact with the terminal.

Most application authors should start with the main [`ratatui`] crate and only depend on
`ratatui-termwiz` directly when they specifically want the Termwiz backend or its advanced
terminal capabilities. This crate is the backend layer, not the primary docs.rs entry point for
building applications.

[`Backend`]: trait.Backend.html
[Termwiz]: https://crates.io/crates/termwiz

## Crate Organization

`ratatui-termwiz` is part of the Ratatui workspace that was modularized in version 0.30.0.
This crate provides the [Termwiz] backend implementation for Ratatui.

**When to use `ratatui-termwiz`:**

- You want to depend on the Termwiz backend crate directly
- You need Termwiz's advanced terminal capabilities

**When to use the main [`ratatui`] crate:**

- Building applications
- You want backend selection to stay behind Ratatui's re-exports

For detailed information about the workspace organization, see [ARCHITECTURE.md].

[`ratatui`]: https://crates.io/crates/ratatui
[ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md

<!-- cargo-rdme end -->
