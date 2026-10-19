use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: a team can play at most one game per round — covers the pairs the
/// home-side and away-side self-joins cannot see, where one of the team's
/// games in a round is a home game and the other is an away game.
///
/// Directed cross-join keyed (home_idx, round) on the hosting side and
/// (away_idx, round) on the visiting side; the violating (team, round) pair
/// falls out of exactly one probe direction, so the weight is a constant one
/// hard per joined pair.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join((
            Schedule::games(),
            joiner::equal_bi(
                |left: &Game| (left.home_idx, left.round),
                |right: &Game| (right.away_idx, right.round),
            ),
        ))
        .penalize(hard_weight(|_: &Game, _: &Game| {
            <HardSoftScore as Score>::one_hard()
        }))
        .named("cross_side_double_booking")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::test_util::plan_from_triples;
    use solverforge::ConstraintSet;

    #[test]
    fn penalizes_home_and_away_same_round() {
        let p = plan_from_triples(4, 6, &[(0, 1, 2), (2, 0, 2)]);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), -1);
    }

    #[test]
    fn allows_same_side_use_in_separate_rounds() {
        let p = plan_from_triples(4, 6, &[(0, 1, 2), (2, 0, 3)]);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }

    #[test]
    fn ignores_pure_home_or_away_dups() {
        // those belong to home_/away_double_booking
        let p = plan_from_triples(4, 6, &[(0, 1, 2), (0, 2, 2)]);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }
}
