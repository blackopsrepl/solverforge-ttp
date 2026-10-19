//! Classic Easton–Nemhauser–Trick National League TTP(3) instances, served
//! by the RobinX repository (Van Bulck et al.). The files embedded here are
//! the `NLx` distance matrices converted from the repository's XML format to
//! JSON at commit time; see the README for provenance and best-known bounds.

use crate::domain::{Game, Round, Schedule, Team};

const NL4: &str = include_str!("instances/NL4.json");
const NL6: &str = include_str!("instances/NL6.json");
const NL8: &str = include_str!("instances/NL8.json");
const NL10: &str = include_str!("instances/NL10.json");
const NL12: &str = include_str!("instances/NL12.json");
const NL14: &str = include_str!("instances/NL14.json");
const NL16: &str = include_str!("instances/NL16.json");

pub const INSTANCE_IDS: [&str; 7] = ["NL4", "NL6", "NL8", "NL10", "NL12", "NL14", "NL16"];

pub const DEFAULT_INSTANCE_ID: &str = "NL4";

/// The published Easton–Trick schedule for NL4: (home_idx, away_idx, round).
/// Its reported value is the proven optimum 8276 (see README References).
pub const GOLDEN_NL4: &[(usize, usize, usize)] = &[
    (0, 1, 1),
    (0, 2, 0),
    (0, 3, 2),
    (1, 0, 4),
    (1, 2, 5),
    (1, 3, 0),
    (2, 0, 3),
    (2, 1, 2),
    (2, 3, 1),
    (3, 0, 5),
    (3, 1, 3),
    (3, 2, 4),
];

#[derive(serde::Deserialize)]
struct InstanceFile {
    teams: Vec<String>,
    distances: Vec<Vec<i64>>,
}

fn raw(id: &str) -> Option<&'static str> {
    match id {
        "NL4" => Some(NL4),
        "NL6" => Some(NL6),
        "NL8" => Some(NL8),
        "NL10" => Some(NL10),
        "NL12" => Some(NL12),
        "NL14" => Some(NL14),
        "NL16" => Some(NL16),
        _ => None,
    }
}

/// Applies the published golden schedule to a freshly loaded NL4 plan.
pub fn golden_nl4() -> Schedule {
    let mut plan = load("NL4").expect("NL4 is embedded");
    for (&(home, away, round), game) in GOLDEN_NL4.iter().zip(plan.games.iter_mut()) {
        assert_eq!((game.home_idx, game.away_idx), (home, away));
        game.round = Some(round);
    }
    plan
}

/// Loads an embedded benchmark instance as a (fully unassigned) plan.
pub fn load(id: &str) -> Option<Schedule> {
    let file: InstanceFile =
        serde_json::from_str(raw(id)?).expect("embedded instance JSON must parse");
    Some(build(&file))
}

fn build(file: &InstanceFile) -> Schedule {
    let n = file.teams.len();
    let num_rounds = 2 * (n - 1); // compact 2RR
    let teams = (0..n)
        .map(|t| Team {
            id: format!("team-{}", file.teams[t]),
            name: file.teams[t].clone(),
            idx: t,
            distances: file.distances[t].clone(),
            num_rounds,
        })
        .collect::<Vec<_>>();
    let rounds = (0..num_rounds)
        .map(|r| Round {
            id: format!("round-{r}"),
            name: format!("Round {}", r + 1),
            index: r,
        })
        .collect::<Vec<_>>();
    let mut games = Vec::with_capacity(n * (n - 1));
    for home in 0..n {
        for away in 0..n {
            if home != away {
                games.push(Game {
                    id: format!("game-{}-{}", file.teams[home], file.teams[away]),
                    home_idx: home,
                    away_idx: away,
                    pair_key: home.min(away) * n + home.max(away),
                    distances: file.distances[home].clone(),
                    round: None,
                });
            }
        }
    }
    Schedule::new(teams, rounds, games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_instance_builds_a_well_formed_plan() {
        for id in INSTANCE_IDS {
            let plan = load(id).unwrap_or_else(|| panic!("embedded {id} must load"));
            let n = plan.teams.len();
            assert_eq!(plan.rounds.len(), 2 * (n - 1));
            assert_eq!(plan.games.len(), n * (n - 1));
            assert!(!plan.teams.is_empty());
        }
    }
}
