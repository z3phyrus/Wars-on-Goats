# Local development workflow

## Prerequisites
- Install Rust 1.93.0 or newer via `rustup`.
- Install the SpacetimeDB CLI and the standalone server if you want to run the module runtime locally.
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

The CLI installed with Cargo is not enough for `start`: the local server is a separate `spacetimedb-standalone` executable. On Windows, install the official SpacetimeDB bootstrapper once:
```powershell
$installer = "$env:TEMP\spacetime-install.exe"
Invoke-WebRequest https://github.com/clockworklabs/SpacetimeDB/releases/latest/download/spacetimedb-update-x86_64-pc-windows-msvc.exe -OutFile $installer
& $installer --yes
Remove-Item $installer
```

After the standalone runtime is installed, `start` starts the database server itself and does not accept `--project-path`. Use the project workflow instead:
```bash
spacetime login
spacetime dev --project-path ./spacetime-module --server-only
```

If you installed the Cargo CLI directly, the equivalent command is:
```bash
spacetimedb-cli login
spacetimedb-cli dev --project-path ./spacetime-module --server-only
```

If `spacetime start` or `spacetimedb-cli start` still reports that `spacetimedb-standalone.exe` cannot be found, the CLI and the official runtime are in different installation directories. Use the official launcher installed by the bootstrapper, or reinstall the CLI through the official installer so both binaries share the same version directory.

## CI expectations
The GitHub Actions workflow runs formatting, linting, Cargo check, and tests for every PR and mainline push.
