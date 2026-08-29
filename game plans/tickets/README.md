# Game delivery roadmap

This folder captures the implementation plan for the current Wars on Goats game concept.

## Ticket index

1. [01-foundation-server-shell.md](01-foundation-server-shell.md) — establish the Rust/Axum runtime, server state, WebSocket layer, and API skeleton.
2. [02-game-state-player-lifecycle.md](02-game-state-player-lifecycle.md) — define the core game models and player lifecycle in memory.
3. [03-matchmaking-lobby-and-team-setup.md](03-matchmaking-lobby-and-team-setup.md) — support room creation, team assignment, class select, and match start flow.
4. [04-map-world-and-visibility.md](04-map-world-and-visibility.md) — build large maps, side-on traversal, and field-of-view visibility rules.
5. [05-combat-and-ability-framework.md](05-combat-and-ability-framework.md) — implement movement, health, damage, status effects, and shared ability logic.
6. [06-class-and-talent-system.md](06-class-and-talent-system.md) — add mage, rogue, hunter, and warlock archetypes with two talent trees each.
7. [07-capture-the-flag-mode.md](07-capture-the-flag-mode.md) — deliver the first competitive mini-game loop and scoring system.
8. [08-control-bases-mode.md](08-control-bases-mode.md) — add the second mini-game mode with bases, capture pressure, and score tracking.
9. [09-persistence-qa-and-launch-readiness.md](09-persistence-qa-and-launch-readiness.md) — wire persistence, monitoring, testing, and release checks.

## Recommended execution order

Start with the server shell and core game state, then move through lobby and world simulation, then abilities, then mini-game rules, then persistence and polish. This gives the backend a stable foundation before adding the more complex class and map systems.
