use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// SOFT (TR): consecutive away games form one road trip — the team travels
/// directly between the two venues, without going home in between.
///
/// Directed cross-join keyed (away_idx, round + 1) on the earlier away game
/// and (away_idx, round) on the next away game.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join((
            Schedule::games(),
            joiner::equal_bi(
                |left: &Game| (left.away_idx, left.round.map(|r| r + 1)),
                |right: &Game| (right.away_idx, right.round),
            ),
        ))
        .penalize(travel_weight)
        .named("travel_away_to_away")
}

fn travel_weight(left: &Game, right: &Game) -> HardSoftScore {
    // both games are the same team's road games; venues are the two hosts'
    // cities (left's venue row indexed by the next venue).
    <HardSoftScore as Score>::from_level_numbers(&[0, left.distances[right.home_idx]])
}
