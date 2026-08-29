# Ticket 04: Map, world, and visibility model

## Goal
Create the large 2D side-on gameplay world and define how visibility, terrain, and player state are simulated and transmitted.

## Scope
- Define a map format for large, multi-room, or multi-zone battlegrounds.
- Add terrain features like platforms, walls, climb surfaces, and base zones.
- Create a world update loop for moving entities.
- Implement field-of-view rules so players are only visible when within range.
- Support map-wide awareness without exposing the whole map to everyone at once.

## Proposed systems
- `WorldMap`
  - size, spawn points, bases, obstacles, platforms, zones
- `EntityPosition`
  - x, y, z or a simplified 2D coordinate system
  - facing direction
  - moving / jumping / climbing state
- `VisibilityState`
  - what each player can currently see
  - hidden enemies behind obstacles or outside FOV

## Deliverables
- Basic map representation and collision support.
- Position updates over the WebSocket layer.
- FOV logic and partial visibility rules.
- Spawn points and base boundaries.

## Acceptance criteria
- Players can move across a large map without seeing the whole battlefield.
- Terrain and obstacles affect movement and line-of-sight decisions.
- The server can send only the relevant subset of world state to each player.
- Base and spawn zones are represented as gameplay objects rather than just coordinates.

## Notes
- This is the key structural ticket for the platformer-like combat feel.
- The design mentions both large maps and visibility constraints, so map synchronization will be a major network and gameplay concern.

## Dependencies
- Ticket 02: game state and player lifecycle
- Ticket 03: matchmaking and team setup
