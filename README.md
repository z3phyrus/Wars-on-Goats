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

## Run locally
```bash
cargo run
```

Use:
```bash
RUST_LOG=info cargo run
```

## Next steps
- Implement the `SpaceTimeClient` abstraction in `src/db.rs` as the real database boundary.
- Define the first SpacetimeDB tables and reducers for lobby, player, and match state.
- Add game rooms, teams, flag state, and player movement in `src/game.rs` using the SpacetimeDB model.
- Build a client that connects to `/ws` for live game updates.
