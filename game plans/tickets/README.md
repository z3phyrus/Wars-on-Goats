# Game delivery roadmap

This folder captures the implementation plan for the current Wars on Goats game concept, with a project-creation phase first and a SpacetimeDB-first architecture throughout.

## Project creation phase

0. [00-project-bootstrap.md](00-project-bootstrap.md) — initialize the workspace, Rust toolchain, repo conventions, and required automation.
0a. [00a-spacetime-module-setup.md](00a-spacetime-module-setup.md) — install the SpacetimeDB toolchain, initialize the module, and define the first schema/reducer layout.
0b. [00b-local-dev-tooling-and-ci.md](00b-local-dev-tooling-and-ci.md) — set up local development commands, CI checks, and a repeatable runbook for the team.

## Game implementation phase

1. [01-foundation-server-shell.md](01-foundation-server-shell.md) — establish the Rust/Axum runtime, API surface, and SpacetimeDB integration boundary.
2. [02-game-state-player-lifecycle.md](02-game-state-player-lifecycle.md) — define the core player lifecycle and authoritative game-state model backed by SpacetimeDB tables and reducers.
3. [03-matchmaking-lobby-and-team-setup.md](03-matchmaking-lobby-and-team-setup.md) — support room creation, team assignment, class select, and match start flow using DB-backed state.
4. [04-map-world-and-visibility.md](04-map-world-and-visibility.md) — build large maps, side-on traversal, and field-of-view visibility rules that feed into synchronized table updates.
5. [05-combat-and-ability-framework.md](05-combat-and-ability-framework.md) — implement movement, health, damage, status effects, and shared ability logic through transactional reducer logic.
6. [06-class-and-talent-system.md](06-class-and-talent-system.md) — add mage, rogue, hunter, and warlock archetypes with two talent trees each, stored as build data and applied through reducers.
7. [07-capture-the-flag-mode.md](07-capture-the-flag-mode.md) — deliver the first competitive mini-game loop and scoring system using authoritative game-state updates.
8. [08-control-bases-mode.md](08-control-bases-mode.md) — add the second mini-game mode with bases, capture pressure, and score tracking on top of SpacetimeDB state.
9. [09-persistence-qa-and-launch-readiness.md](09-persistence-qa-and-launch-readiness.md) — finalize SpacetimeDB schema integration, monitoring, testing, and release checks.

## Recommended execution order

Start with project bootstrap and SpacetimeDB setup, then lock in the runtime and authoritative state model, then move through lobby, world simulation, abilities, and mini-game rules. Finish with QA, persistence validation, and release readiness. This keeps the project grounded in the actual backend architecture before gameplay systems get too large.
