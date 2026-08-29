# Ticket 03: Matchmaking, lobby, and team setup

## Goal
Support the pre-game flow where players choose classes, assign teams, and start matches in a fair and stable way.

## Scope
- Create a room or queue system for one or more matches.
- Support 2-team match setup.
- Allow players to choose class and specialization before a match begins.
- Add ready-state checks, player limit enforcement, and match start countdown.
- Maintain team balance rules for fair matches.

## Proposed flow
1. Player connects to the server.
2. Player joins a lobby or queue.
3. Player selects a class and specialization.
4. Player is assigned to a team or waits for a room to fill.
5. Server verifies all required players are ready.
6. Match starts and the world state initializes.

## Deliverables
- Room creation and lookup.
- Team balancing or manual team assignment logic.
- Ready / not-ready state transitions.
- A match start event and initial spawn positions.

## Acceptance criteria
- A room can accept multiple players across two teams.
- Class selection is stored and validated before match start.
- The server rejects invalid or duplicate role choices.
- A countdown or immediate start trigger begins the match once the room is ready.

## Notes
- The current project has no UI layer yet, so pre-game selection can be modeled as server state and JSON messages.
- Keep the room lifecycle explicit: lobby -> ready -> active -> finished.

## Dependencies
- Ticket 02: game state and player lifecycle
- Ticket 01: foundation server shell
