# Ratatui Termina Backend

<!-- cargo-rdme start -->

Render Ratatui frames through a [`termina::Terminal`](https://docs.rs/termina/latest/termina/terminal/trait.Terminal.html).

[`TerminaBackend`](https://docs.rs/ratatui-termina/latest/ratatui_termina/struct.TerminaBackend.html) writes Termina CSI/SGR escape sequences for Ratatui's [`Backend`] contract:
drawing cells, moving and querying the cursor, clearing regions, flushing output, and reading
terminal size. It wraps a caller-provided [`termina::Terminal`](https://docs.rs/termina/latest/termina/terminal/trait.Terminal.html), so applications can keep using
Termina's event reader and typed terminal protocol surface alongside Ratatui rendering.
The Termina crate is re-exported as `ratatui_termina::termina`, so callers can use the same
Termina types as the backend.

The backend does not enter raw mode, switch to the alternate screen, enable bracketed paste, or
install cleanup by itself. Configure those terminal modes with Termina before creating the
backend and restore them when the session ends. The `termina` example in this crate shows the
direct setup path with a small key-event loop.

[`Backend`]: https://docs.rs/ratatui_core/latest/ratatui_core/backend/trait.Backend.html

<!-- cargo-rdme end -->
