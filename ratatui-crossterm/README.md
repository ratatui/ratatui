# Ratatui-crossterm

<!-- cargo-rdme start -->

This module provides the [`CrosstermBackend`] implementation for the [`Backend`] trait. It uses
the [Crossterm] crate to interact with the terminal.

[Crossterm]: https://crates.io/crates/crossterm

## Crate Organization

`ratatui-crossterm` is part of the Ratatui workspace that was modularized in version 0.30.0.
This crate provides the [Crossterm] backend implementation for Ratatui.

**When to use `ratatui-crossterm`:**

- You need fine-grained control over dependencies
- Building a widget library that needs backend functionality
- You want to use only the Crossterm backend without other backends

**When to use the main [`ratatui`] crate:**

- Building applications (recommended - includes crossterm backend by default)
- You want the convenience of having everything available

For detailed information about the workspace organization, see [ARCHITECTURE.md].

[`ratatui`]: https://crates.io/crates/ratatui
[ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md

<!-- cargo-rdme end -->
