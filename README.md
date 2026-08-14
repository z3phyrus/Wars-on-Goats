# Wars on Goats

A Rust backend prototype for a 2D multiplayer capture-the-flag game inspired by Warsong Gulch.

## Goals
- Host a shared server so players can connect from different computers and networks.
- Use Rust and Cargo for server-side implementation.
- Add SpaceTime DB integration for persistence and backend state storage.

## Initial project structure
- `src/main.rs`: server startup and configuration.
- `src/app.rs`: HTTP / WebSocket routing.
- `src/game.rs`: shared game state and player registration.
- `src/db.rs`: placeholder SpaceTime DB client.
- `src/types.rs`: JSON request/response structures.

## Run locally
```bash
cargo run
```

Use:
```bash
RUST_LOG=info cargo run
```

## Next steps
- Implement `SpaceTimeClient` in `src/db.rs` for real database persistence.
- Add game rooms, teams, flag state, and player movement in `src/game.rs`.
- Build a client that connects to `/ws` for live game updates.
