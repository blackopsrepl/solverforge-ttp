use crate::domain::{Game, Schedule};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD (C2, no-repeater): the two meetings of the same team pair must be at
/// least two rounds apart — a game (i,j) may not be immediately followed by
/// game (j,i).
///
/// The self-join keys on the pair's canonical id, so each unordered meeting
/// pair is visited exactly once; the weight tests the round gap.
pub fn constraint() -> impl IncrementalConstraint<Schedule, HardSoftScore> {
    ConstraintFactory::<Schedule, HardSoftScore>::new()
        .for_each(Schedule::games())
        .join(joiner::equal(pair_join_key))
        .penalize(hard_weight(pair_weight))
        .named("pair_no_repeater")
}

fn pair_join_key(game: &Game) -> usize {
    game.pair_key
}

fn pair_weight(left: &Game, right: &Game) -> HardSoftScore {
    match (left.round, right.round) {
        (Some(l), Some(r)) => {
            let gap = (l as i64 - r as i64).abs();
            if gap <= 1 {
                <HardSoftScore as Score>::one_hard()
            } else {
                <HardSoftScore as Score>::zero()
            }
        }
        _ => <HardSoftScore as Score>::zero(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Round, Team};
    use solverforge::ConstraintSet;

    /// Builds a 4-team plan with empty distances from explicit
    /// (home_idx, away_idx, round_idx) triples; `n` sizes the pair key space.
    fn plan(n: usize, n_rounds: usize, games: &[(usize, usize, usize)]) -> Schedule {
        let teams = (0..n)
            .map(|i| {
                Team::new(
                    format!("t-{}", i),
                    format!("team-{}", i),
                    i,
                    vec![],
                    n_rounds,
                )
            })
            .collect();
        let rounds = (0..n_rounds)
            .map(|i| Round::new(format!("r-{}", i), format!("round {}", i), i))
            .collect();
        let scheduled = games
            .iter()
            .map(|&(h, a, _)| {
                Game::new(
                    format!("g-{}-{}", h, a),
                    h,
                    a,
                    h.min(a) * n + h.max(a),
                    vec![],
                )
            })
            .collect();
        let mut s = Schedule::new(teams, rounds, scheduled);
        for (&(_, _, r), g) in games.iter().zip(s.games.iter_mut()) {
            g.round = Some(r);
        }
        s
    }

    fn score_of(n: usize, n_rounds: usize, games: &[(usize, usize, usize)]) -> HardSoftScore {
        let p = plan(n, n_rounds, games);
        (constraint(),).evaluate_all(&p)
    }

    #[test]
    fn ignores_unassigned() {
        assert_eq!(score_of(4, 6, &[]).hard(), 0);
    }

    #[test]
    fn penalizes_back_to_back_repeaters() {
        // (0,1) in round 2, (1,0) in round 3: gap 1 -> one violation
        assert_eq!(score_of(4, 6, &[(0, 1, 2), (1, 0, 3)]).hard(), -1);
    }

    #[test]
    fn penalizes_same_round_pair() {
        assert_eq!(score_of(4, 6, &[(0, 1, 2), (1, 0, 2)]).hard(), -1);
    }

    #[test]
    fn allows_pair_two_rounds_apart() {
        assert_eq!(score_of(4, 6, &[(0, 1, 2), (1, 0, 4)]).hard(), 0);
    }

    #[test]
    fn unrelated_pairs_never_violate() {
        assert_eq!(score_of(4, 6, &[(0, 1, 2), (2, 0, 3)]).hard(), 0);
    }
}
