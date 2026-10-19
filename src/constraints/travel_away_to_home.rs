use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// SOFT (TR): a team that is away in round r and hosts in round r + 1
/// returns from the opponent's venue to its home city.
///
/// Directed cross-join keyed (away_idx, round + 1) on the road game and
/// (home_idx, round) on the hosting game; each leg falls out of exactly one
/// probe direction, so each leg is counted once.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join((
            Schedule::games(),
            joiner::equal_bi(
                |left: &Game| (left.away_idx, left.round.map(|r| r + 1)),
                |right: &Game| (right.home_idx, right.round),
            ),
        ))
        .penalize(travel_weight)
        .named("travel_away_to_home")
}

fn travel_weight(left: &Game, right: &Game) -> HardSoftScore {
    // left  : same team is away at round r      (venue: left.home_idx's city)
    // right : that team hosts at round r + 1    (venue: its home city)
    <HardSoftScore as Score>::from_level_numbers(&[0, left.distances[right.home_idx]])
}
