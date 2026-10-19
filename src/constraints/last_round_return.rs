use crate::domain::{Game, Schedule, Team};
use solverforge::prelude::*;
use solverforge::stream::joiner::equal_bi;
use solverforge::IncrementalConstraint;

/// SOFT (TR): the season ends at home — a team playing away in the final
/// round returns from that opponent's venue to its home city. The final round
/// index is each team's `num_rounds - 1` (compact 2RR: 2n - 2 for all teams).
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
        .named("last_round_return")
}

fn travel_weight(game: &Game, team: &Team) -> HardSoftScore {
    if game.round == Some(team.num_rounds.saturating_sub(1)) {
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
    fn cr_rows() -> Vec<Vec<i64>> {
        vec![vec![0, 10, 20], vec![10, 0, 30], vec![20, 30, 0]]
    }

    #[test]
    fn penalizes_away_in_last_round() {
        // team 0 away at round 3 (= num_rounds-1) at venue team2: d(0,2) = 20
        let p = plan_with_rows(
            3,
            4,
            &cr_rows(),
            &[(1, 0, 0), (0, 1, 1), (2, 0, 2), (0, 2, 3)],
        );
        assert_eq!((constraint(),).evaluate_all(&p).soft(), -20);
    }

    #[test]
    fn ignores_away_elsewhere() {
        // num_rounds = 4, so the return leg only fires for round 3; this plan
        // has team 0 away at rounds 0,1 only.
        let p = plan_with_rows(
            3,
            4,
            &cr_rows(),
            &[(1, 0, 0), (0, 1, 1), (0, 2, 2), (2, 0, 1)],
        );
        assert_eq!((constraint(),).evaluate_all(&p).soft(), 0);
    }
}
