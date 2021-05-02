# Contributing

## Building

[cargo-make]: https://github.com/sagiegurari/cargo-make "cargo-make"

`tui` is an ordinary Rust project where common tasks are managed with [cargo-make].
It wraps common `cargo` commands with sane defaults depending on your platform of choice.
Building the project should be as easy as running `cargo make build`.

## :hammer_and_wrench: Pull requests

All contributions are obviously welcome.
Please include as many details as possible in your PR description to help the reviewer (follow the provided template).
Make sure to highlight changes which may need additional attention or you are uncertain about.
Any idea with a large scale impact on the crate or its users should ideally be discussed in a "Feature Request" issue beforehand.

## Continuous Integration

We use Github Actions for the CI where we perform the following checks:
- The code should compile on `stable` and the Minimum Supported Rust Version (MSRV).
- The tests (docs, lib, tests and examples) should pass.
- The code should conform to the default format enforced by `rustfmt`.
- The code should not contain common style issues `clippy`.

You can also check most of those things yourself locally using `cargo make ci` which will offer you a shorter feedback loop.

## Tests

The test coverage of the crate is far from being ideal but we already have a fair amount of tests in place.
Beside the usual doc and unit tests, one of the most valuable test you can write for `tui` is a test again the `TestBackend`.
It allows you to assert the content of the output buffer that would have been flushed to the terminal after a given draw call.
See `widgets_block_renders` in [tests/widgets_block.rs](./tests/widget_block.rs) for an example.
