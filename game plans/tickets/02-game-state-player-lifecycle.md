# Ticket 02: Game state and player lifecycle

## Goal
Build the authoritative model for players, teams, rooms, and match lifecycle so gameplay can be built on top of a consistent server state, while keeping queue balancing fair without class/talent-based placement.

## Scope
- Define `Player` fields beyond the current minimal name/ID model.
- Add team membership, selected class, talent build, spawn state, health, and status effects.
- Define match/room state for a live game session.
- Add queue metadata and optional rank fields for stretch-goal matchmaking.
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
  - wins
  - losses
  - rank (stretch goal)
- `GameRoom`
  - room_id
  - players
  - map_id
  - game_mode
  - match_state
  - timers
  - queue_metadata
- `Team`
  - id
  - score
  - base_position
  - players
  - team_size_target
- `MatchmakingEntry`
  - player_id
  - selected_class
  - selected_talents
  - rank (stretch goal)
  - queue_join_time

## Deliverables
- Player creation and registration flow.
- Team assignment and match join state.
- Queue metadata and team-size balancing logic.
- Clean state cleanup for disconnected players.
- A server-side object model that can be serialized for WebSocket updates.

## Acceptance criteria
- A player can join a room and be tracked in a shared, thread-safe structure.
- A player has enough metadata to participate in movement, combat, and abilities.
- The server can reliably remove players when they disconnect or leave a match.
- Match state can be expressed without hard-coding specific mini-game logic into the base player model.
- Team balancing prioritizes team size fairness without using class/talent as a placement heuristic.
- Rank and win/loss metadata can be added as optional stretch-goal fields without breaking the base model.

## Notes
- Keep base player state generic and layer mini-game-specific flags on top.
- `Arc<Mutex<...>>` is useful for the current skeleton, but larger state may need more structured, domain-specific containers.
- This ticket is foundational for all gameplay tickets.
- Ranking fields are intentionally optional and should not block early queue matching.

## Dependencies
- Ticket 01: foundation server shell
