# Copilot instructions for Wars on Goats

This repository is a Rust backend skeleton for a 2D Warsong Gulch style capture-the-flag game.

## Architecture
- `src/main.rs`: bootstraps the runtime, initializes shared app state, and starts the local service layer.
- `src/app.rs`: defines HTTP routes, WebSocket upgrade handling, and the API boundary that clients hit.
- `src/game.rs`: contains the domain logic and server-side game rules that eventually map onto SpacetimeDB state and reducers.
- `src/db.rs`: SpacetimeDB client and schema boundary for live state, persistence, and reducer calls.
- `src/types.rs`: shared JSON response wrapper and payload DTOs for the HTTP/WebSocket surface.

## Current behavior
- `/` returns a simple server health response.
- `/join` accepts `JoinRequest { name }` and returns `JoinResponse` with a generated UUID player ID.
- `/ws` upgrades to WebSocket and currently echoes the first received text message.
- The current server code still uses a minimal in-memory model as a local scaffold while the project moves toward a SpacetimeDB-first architecture.

## Key patterns
- Treat SpacetimeDB as the authoritative backend and real-time state layer for gameplay, persistence, and synchronization.
- Keep transport and API logic in `src/app.rs`; keep domain rules and gameplay logic in `src/game.rs`.
- Use `serde` for JSON payloads where the server still exposes HTTP/WebSocket DTOs.
- Keep SpaceTimeDB logic isolated in `src/db.rs`; do not embed reducer or database concerns directly inside route handlers.
- Prefer SpacetimeDB tables and reducers for lobby, match, player, team, flag, and combat state wherever feasible.

## Development workflows
- Run the server locally with `cargo run`.
- Check compilation with `cargo check`.
- Use `RUST_LOG=info cargo run` for runtime logs.
- Treat SpaceTimeDB as the production-grade place for persistent state and synchronized gameplay state, even while the local Rust server scaffolding is still being developed.

## Agent guidance
- Build the gameplay domain in `src/game.rs` in a way that can be mapped onto SpacetimeDB tables and reducers.
- Add new payload shapes in `src/types.rs` and keep API contracts stable.
- Wire SpacetimeDB calls through `SpaceTimeClient` in `src/db.rs` rather than embedding them in handlers.
- Preserve the existing route names and JSON shapes unless explicitly changing the API.
- Use SpacetimeDB as the authoritative backend for lobby, player, match, team, and gameplay state wherever possible.
- don't change the branch I've checked out unless I explicitly ask you to.
- don't commit any changes to the repository unless I explicitly ask you to.
- ask questions in-line, rather than making me type another command.
- end all work with a short summary of what you did, and any next steps I should take.
