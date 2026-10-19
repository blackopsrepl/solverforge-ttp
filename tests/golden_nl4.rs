use solverforge::ConstraintSet;
use ttp::constraints::create_constraints;
use ttp::data::instances;

/// The published Easton–Trick NL4 schedule (2001) is a proven optimum worth
/// 8276. Feeding it through the whole constraint set must reproduce that
/// number exactly: 0 hard violations, soft = -8276 (travel is penalized).
#[test]
fn golden_nl4_scores_published_objective() {
    let plan = instances::golden_nl4();
    assert_eq!(plan.games.iter().filter(|g| g.round.is_some()).count(), 12);
    let score = create_constraints().evaluate_all(&plan);
    assert_eq!(score.hard(), 0, "golden schedule must be hard-feasible");
    assert_eq!(
        score.soft(),
        -8276,
        "golden schedule must reproduce the published objective"
    );
}

/// Any other assignment of the same NL4 games must NOT reproduce the optimum:
/// a smoke test that the score actually moves.
#[test]
fn shuffled_nl4_rounds_change_the_score() {
    let mut plan = instances::golden_nl4();
    plan.games.reverse();
    for game in plan.games.iter_mut() {
        game.round = Some((game.round.unwrap() + 1) % plan.rounds.len());
    }
    let score = create_constraints().evaluate_all(&plan);
    assert!(
        score.soft() != -8276 || score.hard() != 0,
        "a rotated schedule scoring identically would mean dead constraints"
    );
}

/// Loading every embedded instance produces the classic shape.
#[test]
fn instances_have_classic_shape() {
    for id in instances::INSTANCE_IDS {
        let plan = instances::load(id).expect("instance embedded");
        let n = plan.teams.len();
        assert_eq!(plan.games.len(), n * (n - 1), "{id} game count");
        assert_eq!(plan.rounds.len(), 2 * (n - 1), "{id} round count");
        let mut pair_ids = std::collections::HashSet::new();
        for game in &plan.games {
            assert_ne!(game.home_idx, game.away_idx);
            assert_eq!(
                game.pair_key,
                game.home_idx.min(game.away_idx) * n + game.home_idx.max(game.away_idx)
            );
            assert_eq!(game.distances.len(), n);
        }
        assert!(pair_ids.insert(id));
    }
}
