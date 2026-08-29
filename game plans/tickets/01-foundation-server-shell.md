# Ticket 01: Foundation server shell

## Goal
Establish the Rust backend runtime, HTTP/WebSocket service layer, and the SpaceTimeDB integration boundary so the rest of the game can be built on stable infrastructure.

## Scope
- Confirm the Axum + Tokio server setup in the current project.
- Keep the existing route structure: `/`, `/join`, and `/ws`.
- Define request/response payloads in the shared types layer.
- Keep the game-state and DB client in their own modules instead of coupling everything together.
- Define the `SpaceTimeClient` abstraction as the persistence boundary for player data, match state, and future backend records.
- Add structured logging and startup configuration.

## Deliverables
- Working local server startup via `cargo run`.
- Health endpoint returning a valid JSON response.
- Join flow that creates a player record and returns a player ID.
- WebSocket endpoint that accepts a connection and handles a first-message handshake.
- SpaceTimeDB client placeholder that is clearly isolated in `src/db.rs` and ready for real persistence calls.
- Clean separation between `main.rs`, `app.rs`, `game.rs`, and `db.rs`.

## Acceptance criteria
- The app starts on a local port without panics.
- `/` responds with a success payload.
- `/join` accepts a JSON payload and returns a generated player ID.
- `/ws` connects successfully from a client and opens a live socket.
- The configuration is easy to change for local dev, staging, and production.

## Notes
- This is the technical baseline for everything else.
- Keep the API contract stable while adding richer gameplay data later.
- SpaceTimeDB is still compatible with this architecture, but it should be treated as a persistence layer rather than the real-time game simulation authority.
- Avoid adding game logic into route handlers; route handlers should delegate to shared state objects.
- For a multiplayer game, the live battle state should usually remain in server memory for speed, while SpaceTimeDB can store persistent records such as player identities, profiles, match summaries, and analytics.

## Dependencies
- None; this is the first milestone.
