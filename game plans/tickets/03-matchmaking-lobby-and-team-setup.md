# Ticket 03: Matchmaking, lobby, and team setup

## Goal
Support the workflow where players choose their class and talent build at the main menu, then queue for a match with that identity already established while keeping team sizes fair without biasing placement by class or talent.

## Scope
- Create a room or queue system for one or more matches.
- Support 2-team match setup.
- Allow players to choose class, specialization, and talent build while on the main menu before joining a queue.
- Add ready-state checks, player limit enforcement, and match start countdown.
- Maintain team balance rules that focus on team size and queue fairness, not class/talent-based team placement.
- Add an optional ranking system as a stretch goal for better queue matching.

## Proposed flow
1. Player opens the game and chooses a profile / account state.
2. Player selects a class, specialization, and talent build at the main menu.
3. Player joins a queue or selects a match type.
4. Matchmaker groups players into two teams of roughly equal size, without assigning players to a side based on class, specialization, or talent build.
5. Server verifies all required players are ready.
6. Match starts and the world state initializes with the selected loadout already applied.

## Deliverables
- Room creation and lookup.
- Fair team balancing logic based primarily on team size and queue fill conditions.
- Ready / not-ready state transitions.
- A match start event and initial spawn positions.
- Optional ranking metadata model for stretch-goal matchmaking.

## Acceptance criteria
- A player can define their class and talent build at the menu before queueing.
- A room can accept multiple players across two teams.
- Team assignment prioritizes fairness of team size and queue fill over character class or talent distribution.
- The selected class and talent build is stored and validated before match start.
- The server rejects invalid or duplicate role choices.
- A countdown or immediate start trigger begins the match once the room is ready.
- A ranking field may be recorded per player for future matchmaking refinements, but it does not block basic queueing.

## Notes
- Team balance should be fair in size, but not artificially curated by class talent composition.
- This keeps the game more varied and avoids players being strategically split by archetype.
- A stretch-goal ranking system may track wins/losses and skill tier per player or team, and can be used to reduce skill mismatch when enough people are queued.
- If there are not enough players to fill both teams fairly, the game may temporarily accept a less-than-ideal match to avoid long waits.
- Keep the room lifecycle explicit: queued -> ready -> active -> finished.

## Dependencies
- Ticket 02: game state and player lifecycle
- Ticket 01: foundation server shell
