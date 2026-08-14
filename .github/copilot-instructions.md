# Copilot instructions for Wars on Goats

This repository is a Rust backend skeleton for a 2D Warsong Gulch style capture-the-flag game.

## Architecture
- `src/main.rs`: bootstraps a Tokio runtime, builds shared state, and starts the Axum server.
- `src/app.rs`: defines HTTP routes, WebSocket upgrade handling, and shared state wiring.
- `src/game.rs`: holds in-memory game state and player registration logic.
- `src/db.rs`: stubbed `SpaceTimeClient` for future SpaceTime DB persistence integration.
- `src/types.rs`: shared JSON response wrapper and payload DTOs.

## Current behavior
- `/` returns a simple server health response.
- `/join` accepts `JoinRequest { name }` and returns `JoinResponse` with a generated UUID player ID.
- `/ws` upgrades to WebSocket and currently echoes the first received text message.
- `GameState` uses `Arc<Mutex<HashMap<String, Player>>>` and `uuid` for player IDs.

## Key patterns
- Shared application state is passed with `Arc<AppState>` and `Extension`.
- Keep transport and API logic in `src/app.rs` and game mechanics/state in `src/game.rs`.
- Use `serde` derive for all JSON payloads.
- Keep database persistence isolated in `src/db.rs`; do not embed SpaceTime logic in route handlers.

## Development workflows
- Run the server locally with `cargo run`.
- Check compilation with `cargo check`.
- Use `RUST_LOG=info cargo run` for runtime logs.

## Agent guidance
- Extend the game state in `src/game.rs` before changing route handlers.
- Add new payload shapes in `src/types.rs` and keep API contracts stable.
- Wire SpaceTime DB calls through `SpaceTimeClient` in `src/db.rs`.
- Preserve the existing route names and JSON shapes unless explicitly changing the API.
