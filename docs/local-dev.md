# Local development workflow

## Prerequisites
- Install Rust 1.93.0 or newer via `rustup`.
- Install the SpacetimeDB CLI if you want to run the module runtime locally.
- Ensure `cargo`, `rustfmt`, and `clippy` are available in your PATH.

```bash
rustup toolchain install 1.93.0
rustup default 1.93.0
```

## Common commands
```bash
cargo fmt
cargo check
cargo test
cargo run
RUST_LOG=info cargo run
```

## SpacetimeDB module commands
```bash
cargo run -p spacetime_module
```

This package is intentionally defined as a binary crate so the documented command works directly from the module scaffold.

If you are using the SpacetimeDB CLI directly, start the local module from the module directory with the project-specific command from your local installation, for example:
```bash
spacetime login
spacetime start --project-path ./spacetime-module
```

## CI expectations
The GitHub Actions workflow runs formatting, linting, Cargo check, and tests for every PR and mainline push.
