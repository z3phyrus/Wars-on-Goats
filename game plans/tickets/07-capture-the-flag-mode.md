# Ticket 07: Capture the Flag mode

## Goal
Implement the first full mini-game loop: capture the opposing team’s flag and return it to your own base while defending your flag.

## Scope
- Add a flag object and base zones for each team.
- Define scoring and win conditions.
- Track flag carrier state and return logic.
- Create match flow states for gameplay start, flag capture, and round win.
- Support a best-of-three or points-based win condition.

## Proposed rules
- Each team has a base and a flag.
- A flag can be picked up by an enemy player who is alive and in range.
- A carried flag must be returned to the opposing base to score.
- While a flag is stolen, defenders protect their own flag and attempt to intercept the enemy carrier.
- Match ends after three captures or when a fixed round limit is reached.

## Deliverables
- Flag entity model and pickup/drop logic.
- Base ownership and scoring rules.
- Team victory flow and match reset flow.
- WebSocket events for flag state and score updates.

## Acceptance criteria
- A team can score by returning the enemy flag to base.
- A flag returns to base when the carrier dies or leaves the map zone.
- Server events accurately reflect flag status and score changes.
- A match can be reset cleanly for another round or rematch.

## Notes
- This is the first mode that fully combines the class system, map model, and combat logic.
- The design says the team wins after three captures, but the final balance may need to be tuned after playtesting.

## Dependencies
- Ticket 05: combat and ability framework
- Ticket 06: class and talent system
- Ticket 04: map, world, and visibility model
