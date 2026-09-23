# Ticket 05: Combat and ability framework

## Goal
Implement the shared combat loop: movement, health regeneration, damage, status effects, ability casting, and targeting rules.

## Scope
- Add movement and jump inputs to the simulated world.
- Establish health, regen, max health, knockback, and death states.
- Create a generic ability system with cooldowns, activation windows, effects, and area-based target validation.
- Add status effects such as stun, slow, DOT, burning, poison, and silence.
- Keep the system flexible enough to support all four classes and their trees.

## Proposed systems
- `AbilityDefinition`
  - id
  - name
  - type
  - target type
  - range
  - cooldown
  - cast time
  - effects
- `AbilityArea`
  - circle, beam, or cone geometry
  - caster- or aim-point anchoring
  - duration and whether the area follows the caster
- `AbilityCastRequest`
  - world-space mouse aim position
- `CombatEvent`
  - source player
  - zero or more affected players
  - damage or healing values
  - status effect IDs
- `StatusEffect`
  - type
  - duration
  - magnitude
  - stackability

## Deliverables
- Shared combat pipeline from input to event resolution.
- Damage and healing logic with proper validation.
- Cooldowns and cast-time management.
- Basic ability definitions for the shared class skills, with room for specialization-specific skills later.

## Acceptance criteria
- Players can take damage, heal, and die according to the rules.
- Ability actions are validated against range, cooldown, state restrictions, and area geometry.
- A valid cast can affect zero targets; the cast and its cooldown still resolve.
- Status effects apply and expire correctly.
- Movement and combat phases do not break the server’s concurrency model.

## Notes
- This ticket is the largest gameplay engine layer and will likely be extended by the class and talent tickets.
- Keep events explicit; they are easier to debug than hidden side effects.

## Dependencies
- Ticket 02: game state and player lifecycle
- Ticket 04: map, world, and visibility model
