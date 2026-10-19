use crate::domain::{Game, Schedule, Team};
use solverforge::prelude::*;
use solverforge::stream::joiner::equal_bi;
use solverforge::IncrementalConstraint;

/// SOFT (TR): the season starts at home — a team playing away in round 0
/// departs from its home city to that opponent's venue.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join((
            Schedule::teams(),
            equal_bi(
                |game: &Game| Some(game.away_idx),
                |team: &Team| Some(team.idx),
            ),
        ))
        .penalize(travel_weight)
        .named("first_round_departure")
}

fn travel_weight(game: &Game, team: &Team) -> HardSoftScore {
    // game is a road game of `team`; the venue is game.home_idx's city.
    if game.round == Some(0) {
        <HardSoftScore as Score>::from_level_numbers(&[0, team.distances[game.home_idx]])
    } else {
        <HardSoftScore as Score>::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::test_util::plan_with_rows;
    use solverforge::ConstraintSet;

    /// 3 teams: d(0,1)=10, d(0,2)=20, d(1,2)=30.
    fn rows() -> Vec<Vec<i64>> {
        vec![vec![0, 10, 20], vec![10, 0, 30], vec![20, 30, 0]]
    }

    #[test]
    fn penalizes_away_in_round_zero() {
        // team 0 away at round 0 at venue team1: d(0,1) = 10
        let p = plan_with_rows(3, 4, &rows(), &[(1, 0, 0), (0, 1, 1), (2, 0, 2), (0, 2, 3)]);
        assert_eq!((constraint(),).evaluate_all(&p).soft(), -10);
    }

    #[test]
    fn ignores_away_elsewhere() {
        // team 0 away only in rounds 1..3 (never round 0): no departure leg.
        let p = plan_with_rows(3, 4, &rows(), &[(1, 0, 1), (0, 1, 2), (2, 0, 3), (0, 2, 1)]);
        assert_eq!((constraint(),).evaluate_all(&p).soft(), 0);
    }
}
