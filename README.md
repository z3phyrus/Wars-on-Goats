# Wars on Goats

A Rust backend prototype for a 2D multiplayer capture-the-flag game inspired by Warsong Gulch, designed around a SpacetimeDB-first architecture.

## Goals
- Host a shared server so players can connect from different computers and networks.
- Use Rust and Cargo for server-side implementation and domain logic.
- Use SpacetimeDB as the authoritative state layer for real-time gameplay and persistence.
- Keep HTTP/WebSocket surfaces thin while moving game logic into SpacetimeDB-backed reducers and tables.

## Architecture
- `src/main.rs`: server startup and runtime wiring.
- `src/app.rs`: HTTP/WebSocket API surface.
- `src/game.rs`: game domain logic and rules that map onto SpacetimeDB state.
- `src/db.rs`: SpacetimeDB client, schema boundary, and reducer access layer.
- `src/types.rs`: JSON request/response structures for the HTTP/WebSocket layer.
- `spacetime-module/`: first-pass SpacetimeDB module schema and reducer skeleton.
- `docs/`: contributor setup and development workflow notes.
- `.github/workflows/ci.yml`: build and validation pipeline for pull requests and mainline merges.

## SpacetimeDB design direction
The project is intentionally moving toward a model where SpacetimeDB owns as much of the game state as is feasible and sensible:

- player identity and profile data
- lobby and room membership
- team assignments
- class and talent selections
- match lifecycle and game phase transitions
- capture points, flags, and zone control state
- combat events, health, status effects, and cooldowns
- persistent match history and analytics

This does not mean every piece of logic must be forced into SpacetimeDB immediately, but it does mean that SpacetimeDB should be treated as the primary backend and real-time state system for the game.

## Local setup

### Prerequisites
- Install the Rust toolchain from rustup. This repo is pinned to Rust 1.93.0.
- On Windows, install the Microsoft C++ Build Tools or use the GNU target toolchain if you intentionally target that setup.
- Install the SpacetimeDB CLI when you are ready to run the module locally.

```bash
rustup toolchain install 1.93.0
rustup default 1.93.0
```

### Standard commands
```bash
cargo fmt
cargo check
cargo run
```

For a more verbose local run:
```bash
RUST_LOG=info cargo run
```

For the module and infrastructure scripts:
```bash
pwsh ./scripts/run-local.ps1
pwsh ./scripts/check.ps1
```

## Project conventions
- Keep route/transport code in `src/app.rs`.
- Keep gameplay rules in `src/game.rs` and keep them ready to map onto SpacetimeDB tables and reducers.
- Keep database integration and schema contracts isolated in `src/db.rs` and `spacetime-module/`.
- Keep game design and planning notes in `game plans/` and the docs directory.

## SpacetimeDB module skeleton
The repository now includes a first-pass module crate in `spacetime-module/` that defines the initial data model for players, rooms, and match state, along with reducer-style entry points for `join_room`, `start_match`, and `update_match_state`.

## Next steps
- Implement the `SpaceTimeClient` abstraction in `src/db.rs` as the real database boundary.
- Define the first SpacetimeDB tables and reducers for lobby, player, and match state.
- Add game rooms, teams, flag state, and player movement in `src/game.rs` using the SpacetimeDB model.
- Build a client that connects to `/ws` for live game updates.
