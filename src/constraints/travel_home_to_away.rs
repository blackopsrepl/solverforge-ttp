use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// SOFT (TR): a team that hosts in round r and is away in round r + 1
/// travels from its home city to the opponent's venue.
///
/// Directed cross-join keyed (home_idx, round + 1) on the hosting game and
/// (away_idx, round) on the road game: the pair aligns exactly when the same
/// team hosts at round r and then plays away at round r + 1. Each leg falls
/// out of exactly one probe direction, so each leg is counted once.
///
/// `Game.distances` carries the distance row of the game's home team's city —
/// the fact data this constraint reads, populated by the loader.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join((
            Schedule::games(),
            joiner::equal_bi(
                |left: &Game| (left.home_idx, left.round.map(|r| r + 1)),
                |right: &Game| (right.away_idx, right.round),
            ),
        ))
        .penalize(travel_weight)
        .named("travel_home_to_away")
}

fn travel_weight(left: &Game, right: &Game) -> HardSoftScore {
    // left  : same team hosts at round r      (venue: its home city)
    // right : that team is away at round r + 1 (venue: right.home_idx's city)
    // distance home -> venue, indexed off the right game's venue row
    // (symmetric matrix).
    <HardSoftScore as Score>::from_level_numbers(&[0, right.distances[left.home_idx]])
}
