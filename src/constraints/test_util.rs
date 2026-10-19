use crate::domain::{Game, Round, Schedule, Team};

/// Builds a plan whose games carry zero distance rows; for constraints that
/// never read distances. See [`plan_with_rows`] for real distance data.
pub(crate) fn plan_from_triples(
    n: usize,
    n_rounds: usize,
    games: &[(usize, usize, usize)],
) -> Schedule {
    let rows = vec![vec![0; n]; n];
    plan_with_rows(n, n_rounds, &rows, games)
}

/// Builds a test plan from (home_idx, away_idx, round_idx) triples where
/// `distance_rows[t]` is team t's distance matrix row. Each `Game` carries
/// the distance row of its HOST team (its venue is the host's city).
pub(crate) fn plan_with_rows(
    n: usize,
    n_rounds: usize,
    distance_rows: &[Vec<i64>],
    games: &[(usize, usize, usize)],
) -> Schedule {
    let teams = (0..n)
        .map(|i| {
            Team::new(
                format!("t-{}", i),
                format!("team-{}", i),
                i,
                distance_rows[i].clone(),
                n_rounds,
            )
        })
        .collect();
    let rounds = (0..n_rounds)
        .map(|i| Round::new(format!("r-{}", i), format!("round {}", i), i))
        .collect();
    let scheduled = games
        .iter()
        .enumerate()
        .map(|(gi, &(h, a, _))| {
            Game::new(
                format!("g-{}", gi),
                h,
                a,
                h.min(a) * n + h.max(a),
                distance_rows[h].clone(),
            )
        })
        .collect();
    let mut s = Schedule::new(teams, rounds, scheduled);
    for (&(_, _, r), g) in games.iter().zip(s.games.iter_mut()) {
        g.round = Some(r);
    }
    s
}
