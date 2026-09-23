# Ticket 04a: Local playable gameplay slice

## Goal
Create the first playable browser client so gameplay ideas can be visualised and iterated locally before multiplayer and full match rules are complete.

## Scope
- Add a browser client that connects to the Rust server over WebSocket.
- Render a side-on 2D map with the local player, platforms, obstacles, targets, and basic world state.
- Support keyboard movement and jump input.
- Support mouse world-space aiming and ability activation.
- Add a solo local mode with training targets or simple stationary AI.
- Display health, cooldowns, active status effects, and basic combat feedback.
- Keep ability definitions and visual effect parameters easy to change while iterating.

## Proposed systems
- `PlayableClient`
  - browser entry point and render/update loop
  - keyboard and mouse input handling
  - WebSocket connection and reconnect handling
- `LocalSoloSession`
  - local player spawn
  - training targets or simple AI opponents
  - reset/restart flow
- `ClientWorldRenderer`
  - map, platforms, obstacles, players, target indicators, and effects
- `GameplayInput`
  - movement direction
  - jump
  - ability activation
  - world-space mouse aim position

## Deliverables
- A browser page that opens directly into a playable solo session.
- Keyboard movement and jumping visible in the world.
- Mouse-directed area abilities visible and testable against targets.
- Server-authoritative player state and combat results shown in the client.
- A simple reset control so ability and movement iterations can be tested repeatedly.
- Local development instructions for starting the server and client together.

## Acceptance criteria
- A developer can start the local server and open the client without matchmaking or a second player.
- The player can move, jump, aim with the mouse, and activate abilities.
- Abilities can visibly miss, hit one target, or hit multiple targets according to their area geometry.
- Health, death, cooldown, and status-effect changes are visible.
- The session can be reset without restarting the server.
- The client remains useful while class, talent, art, and final ability designs are still changing.

## Non-goals
- Production-quality art or final UI.
- Matchmaking, persistence, ranked play, or a complete competitive ruleset.
- Final balance for classes and abilities.

## Dependencies
- Ticket 01: foundation server shell
- Ticket 02: game state and player lifecycle
- Ticket 04: map, world, and visibility model
- Ticket 05: combat and ability framework

## Notes
- This ticket is intentionally a vertical slice rather than a replacement for the backend gameplay tickets.
- The first implementation may use simple geometric rendering and training targets so gameplay can be evaluated before art production begins.
