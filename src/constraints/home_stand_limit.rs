use crate::constraints::MAX_RUN_LENGTH;
use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD (C1): no home stand longer than `MAX_RUN_LENGTH` consecutive home
/// rounds.
///
/// Home games of a team are exactly the games it hosts; grouping them by team
/// and taking consecutive runs over their round indexes yields its home
/// stands. Each run is penalized by its excess over the limit.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .group_by(
            |game: &Game| game.home_idx,
            solverforge::stream::collector::consecutive_runs(round_run_index),
        )
        .penalize(hard_weight(runs_weight))
        .named("home_stand_limit")
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
    fn allows_three_game_home_stand() {
        // team 0 hosts rounds 0,1,2 (a run of exactly three)
        let p = plan_from_triples(
            4,
            8,
            &[
                (0, 1, 0),
                (0, 2, 1),
                (0, 3, 2),
                (1, 0, 4),
                (1, 3, 5),
                (1, 2, 6),
                (2, 3, 7),
            ],
        );
        assert_eq!((constraint(),).evaluate_all(&p).hard(), 0);
    }

    #[test]
    fn penalizes_four_game_home_stand_by_excess() {
        // team 0 hosts rounds 0..3 -> run of 4, excess 1
        let p = plan_from_triples(
            4,
            8,
            &[
                (0, 1, 0),
                (0, 2, 1),
                (0, 3, 2),
                (0, 1, 3),
                (1, 0, 5),
                (1, 3, 6),
                (1, 2, 7),
                (2, 3, 4),
            ],
        );
        assert_eq!((constraint(),).evaluate_all(&p).hard(), -1);
    }

    #[test]
    fn independent_stands_sum_independently() {
        // six teams: team 0 hosts rounds 0..4 (run of 5, excess 2) while
        // team 1 hosts rounds 5..8 (run of 4, excess 1) -> total excess 3.
        let p = plan_from_triples(
            6,
            10,
            &[
                (0, 1, 0),
                (0, 2, 1),
                (0, 3, 2),
                (0, 4, 3),
                (0, 5, 4),
                (1, 0, 6),
                (1, 2, 5),
                (1, 3, 7),
                (1, 4, 8),
                (2, 3, 9),
            ],
        );
        assert_eq!((constraint(),).evaluate_all(&p).hard(), -3);
    }
}
