# Ticket 00b: Local dev tooling and CI

## Goal
Create the local developer workflow and the automated checks that make the project reproducible and safe to extend.

## Scope
- Define local commands for running the server and module.
- Add health checks and build validation scripts.
- Integrate CI for compile/test checks.
- Document the expected development flow for contributors and future AI agents.

## Deliverables
- Runbook for local setup and module startup.
- A minimum CI pipeline covering build and tests.
- Standardized commands for `check`, `build`, and local server execution.
- Clear repo conventions for working in the game backend and the SpacetimeDB module.

## Acceptance criteria
- A new contributor can set up the project without unclear or hidden steps.
- CI catches compile or test regressions before merge.
- Local developer commands are documented and consistent.
- Development flow is compatible with the SpacetimeDB-first design.

## Notes
- This ticket should be treated as project infrastructure rather than gameplay work.
- The goal is reliable repeatability so the game can grow without ad hoc tooling.

## Dependencies
- Ticket 00: Project bootstrap
- Ticket 00a: SpacetimeDB module setup
