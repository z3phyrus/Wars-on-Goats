# Ticket 00a: SpacetimeDB module setup

## Goal
Set up the SpacetimeDB toolchain and initialize the project’s first module, tables, and reducer skeleton in the way the game will actually use it.

## Scope
- Install and validate the SpacetimeDB CLI/tooling.
- Create or connect the project to a first SpacetimeDB module.
- Define the initial module layout: tables, reducers, and subscription patterns.
- Prototype how player, room, and match data will live in the DB.

## Deliverables
- Working SpacetimeDB module initialization for the project.
- Initial schema proposal for core game tables.
- Reducer scaffolding for essential actions like join room, start match, and update state.
- A documented local development flow for the module runtime.

## Acceptance criteria
- The SpacetimeDB module can be started locally.
- The project has a first-pass schema for players, rooms, and match lifecycle.
- Reducers are clearly separated from transport-layer handlers.
- The module structure supports the eventual game-state and combat logic without heavy rewrites.

## Notes
- This ticket is foundational for the project’s architecture and should happen before heavy gameplay implementation.
- The goal is not full game logic yet; it is to establish authoritative state tables and reducer conventions.

## Dependencies
- Ticket 00: Project bootstrap
