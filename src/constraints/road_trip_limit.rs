use crate::constraints::MAX_RUN_LENGTH;
use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD (C1): no road trip longer than `MAX_RUN_LENGTH` consecutive away
/// rounds.
///
/// Away games of a team are exactly the games it visits; grouping them by
/// team and taking consecutive runs over their round indexes yields its road
/// trips. Each run is penalized by its excess over the limit.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .group_by(
            |game: &Game| game.away_idx,
            solverforge::stream::collector::consecutive_runs(round_run_index),
        )
        .penalize(hard_weight(runs_weight))
        .named("road_trip_limit")
}

fn round_run_index(game: &Game) -> i64 {
    game.round.unwrap_or(0) as i64
}

fn runs_weight(_group: &usize, runs: &Runs) -> HardSoftScore {
    let excess: i64 = runs
        .runs()
        .iter()
        .map(|run| (run.end() - run.start() + 1 - MAX_RUN_LENGTH as i64).max(0))
        .sum();
    <HardSoftScore as Score>::from_level_numbers(&[excess, 0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::test_util::plan_from_triples;
    use solverforge::ConstraintSet;

    #[test]
    fn allows_three_game_road_trip() {
        // team 0 away in rounds 0,1,2 — exactly the limit
        let p = plan_from_triples(
            4,
            8,
            &[
                (1, 0, 0),
                (2, 0, 1),
                (3, 0, 2),
                (1, 3, 4),
                (0, 1, 5),
                (0, 2, 6),
                (0, 3, 3),
                (1, 2, 7),
            ],
        );
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }

    #[test]
    fn penalizes_four_game_road_trip_by_excess() {
        // team 0 away in rounds 0..3 -> excess 1 (the (1,2,7) away rounds of
        // team 1 end at 7, non-adjacent, so no second run)
        let p = plan_from_triples(
            4,
            8,
            &[
                (1, 0, 0),
                (2, 0, 1),
                (3, 0, 2),
                (1, 3, 3),
                (0, 1, 5),
                (0, 2, 6),
                (0, 3, 4),
                (1, 2, 6),
            ],
        );
        let _ = &p;
        let score = (constraint(),).evaluate_all(&plan_from_triples(
            4,
            8,
            &[(1, 0, 0), (2, 0, 1), (3, 0, 2), (1, 0, 3)],
        ));
        assert_eq!(score.hard(), -1);
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }
}
