use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: a team can play at most one away game per round.
///
/// Keys on (away_idx, round): any joined pair of games IS a violation, so the
/// weight is a constant one hard per joined pair.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join(joiner::equal(double_booking_key))
        .penalize(hard_weight(|_: &Game, _: &Game| {
            <HardSoftScore as Score>::one_hard()
        }))
        .named("away_double_booking")
}

fn double_booking_key(game: &Game) -> (usize, Option<usize>) {
    (game.away_idx, game.round)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::test_util::plan_from_triples;
    use solverforge::ConstraintSet;

    #[test]
    fn penalizes_two_away_games_same_round() {
        let p = plan_from_triples(4, 6, &[(1, 0, 2), (2, 0, 2)]);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), -1);
    }

    #[test]
    fn allows_different_rounds() {
        let p = plan_from_triples(4, 6, &[(1, 0, 2), (2, 0, 3)]);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }
}
