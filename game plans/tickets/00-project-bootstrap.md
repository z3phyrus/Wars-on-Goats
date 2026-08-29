# Ticket 00: Project bootstrap

## Goal
Initialize the project structure, toolchain, and repo conventions required to build a SpacetimeDB-first game backend cleanly and repeatedly.

## Scope
- Confirm the Rust toolchain and Cargo workspace layout.
- Define repo conventions for source layout, docs, and planning artifacts.
- Ensure the project can build locally and in CI.
- Prepare the environment for a future SpacetimeDB module and game client integration.

## Deliverables
- A clean repo root with explicit project structure.
- A documented local setup flow for Rust and any required tooling.
- A standard build/test/check workflow for the repo.
- A clear separation between runtime code, game logic, and DB/schema code.

## Acceptance criteria
- A developer can install dependencies and run the project from a clean machine.
- `cargo check` and the local run command work reliably.
- The repo has a consistent convention for docs, planning notes, and source files.
- The structure is ready for SpacetimeDB module integration without major refactoring.

## Notes
- This is the project-creation ticket that surfaces the tooling and conventions before gameplay implementation begins.
- Keep the repo architecture explicit: app layer, domain/game layer, and SpacetimeDB boundary.

## Dependencies
- None
