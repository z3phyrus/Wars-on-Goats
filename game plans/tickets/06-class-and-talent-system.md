# Ticket 06: Class and talent system

## Goal
Add the four classes and their talent trees so players can choose an archetype and build a unique loadout before each match.

## Scope
- Define the class roster: Mage, Rogue, Hunter, Warlock.
- Add the two specialization trees for each class.
- Define base abilities shared across each class.
- Define specialization-specific abilities and talent choices.
- Model talent costs, branching choices, and build validation.

## Class plan
- Mage
  - Frost Mage
  - Electric Mage
  - Shared base abilities: Fireball, Teleport
- Rogue
  - Stealth Rogue
  - Warrior Rogue
  - Shared base abilities: Large Strike, Small Strike
- Hunter
  - Beast Hunter
  - Survival Hunter
  - Shared base abilities: Throw Mud, Long Shot
- Warlock
  - Demon Warlock
  - Acid Warlock
  - Shared base abilities: Parasite, Corrupted Beam

## Talent tree rules
- Each class has two specializations.
- Each specialization has a branching talent tree with multiple unlockable nodes.
- Players receive a limited number of talent points.
- They can choose a build that fits their playstyle while cannot fully unlock everything.
- Some talents modify existing abilities, while others add new ones or alter behavior.

## Deliverables
- Class metadata and ability mapping.
- Specialization tree data model and validation logic.
- Talent point budgeting and selection flow.
- Support for future client UI to visualize chosen talents.

## Acceptance criteria
- Each class has its own identity and combat role.
- Players can select a specialization and receive a valid subset of abilities.
- Talent choices alter the behavior of the class in a way that is understandable and testable.
- The build system stays within a bounded set of available points and unlock rules.

## Notes
- This ticket intentionally builds directly on the ability framework and on the class concept documents in the planning folder.
- The linear/branching talent-tree structure should be represented as data, not hard-coded for a single UI.

## Dependencies
- Ticket 05: combat and ability framework
- Ticket 03: matchmaking and team setup
