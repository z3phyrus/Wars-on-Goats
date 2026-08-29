# Ticket 08: Control Bases mode

## Goal
Implement the second mini-game mode where teams fight for control of neutral bases to accumulate points.

## Scope
- Add several bases spread across the map.
- Define capture progress and control state for each base.
- Support contested territory and team hold timers.
- Track score accumulation and victory conditions.
- Support downtime, reset, and rematch flow.

## Proposed rules
- Multiple neutral bases exist across a map.
- Bases can be captured by teams standing in or contesting them.
- A team gains points while it controls more bases.
- The first team to reach 1000 points wins.
- Bases may flip ownership if a rival team holds them longer or contests capture pressure.

## Deliverables
- Base entity model and capture state.
- Score tracking and round end detection.
- Team control and zone occupancy logic.
- Event updates for base state changes and team score changes.

## Acceptance criteria
- The server can determine which team controls a base at any given moment.
- Score grows continuously as a team maintains control.
- Capture pressure and contesting are represented clearly in the game state.
- The match ends once one team reaches the target score.

## Notes
- This mode depends on the same underlying world and combat logic but uses different win-state and scoring rules.
- If implemented well, the base-capture system should be reusable across multiple future modes.

## Dependencies
- Ticket 04: map, world, and visibility model
- Ticket 05: combat and ability framework
- Ticket 06: class and talent system
