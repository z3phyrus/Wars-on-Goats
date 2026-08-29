# Ticket 02: Game state and player lifecycle

## Goal
Build the shared in-memory model for players, teams, rooms, and match lifecycle so gameplay can be built on top of a consistent server state.

## Scope
- Define `Player` fields beyond the current minimal name/ID model.
- Add team membership, class selection, talent build, spawn state, health, and status effects.
- Define match/room state for a live game session.
- Make the shared state concurrency-safe for many simultaneous players.
- Add player registration, leave handling, and disconnect cleanup.

## Proposed data model
- `Player`
  - id
  - name
  - team_id
  - class
  - talent_tree
  - selected_talents
  - position
  - velocity
  - health
  - max_health
  - alive/dead state
  - statuses (slow, stun, poison, etc.)
- `GameRoom`
  - room_id
  - players
  - map_id
  - game_mode
  - match_state
  - timers
- `Team`
  - id
  - score
  - base_position
  - players

## Deliverables
- Player creation and registration flow.
- Team assignment and match join state.
- Clean state cleanup for disconnected players.
- A server-side object model that can be serialized for WebSocket updates.

## Acceptance criteria
- A player can join a room and be tracked in a shared, thread-safe structure.
- A player has enough metadata to participate in movement, combat, and abilities.
- The server can reliably remove players when they disconnect or leave a match.
- Match state can be expressed without hard-coding specific mini-game logic into the base player model.

## Notes
- Keep base player state generic and layer mini-game-specific flags on top.
- `Arc<Mutex<...>>` is useful for the current skeleton, but larger state may need more structured, domain-specific containers.
- This ticket is foundational for all gameplay tickets.

## Dependencies
- Ticket 01: foundation server shell
