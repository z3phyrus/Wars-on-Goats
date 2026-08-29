# Ticket 09: Persistence, QA, and launch readiness

## Goal
Put the game in a stable state for real-world use by wiring in persistence, testing, observability, and release checks.

## Scope
- Complete the SpaceTime DB client integration path in `src/db.rs`.
- Add validation and error handling for all major API and gameplay events.
- Add automated tests for game-room lifecycle, class selection, and combat rules.
- Add telemetry, logging, and metrics for server health and match flow.
- Verify the game can run reliably and be debugged locally.

## Deliverables
- Persistence interface for player metadata and match history.
- Error handling for invalid room joins, invalid commands, and disconnected clients.
- Automated test suite for the gameplay backend.
- Monitoring hooks and operational documentation.

## Acceptance criteria
- Persistent records can be written and read through the DB abstraction.
- Core gameplay flows are covered by automated tests.
- The server logs are useful for debugging runtime problems without ad hoc instrumentation.
- The project has a clear runbook for local development and initial deployment.

## Notes
- Persistence is described as a future requirement, so this ticket should not block earlier game-mechanics tickets.
- The main goal here is operational readiness, especially once the game is playable enough to test with real players.

## Dependencies
- Ticket 07: Capture the Flag mode
- Ticket 08: Control Bases mode
- Ticket 06: class and talent system
