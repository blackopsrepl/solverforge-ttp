use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

/// TODO — describe this entity.
#[planning_entity]
#[derive(Serialize, Deserialize)]
pub struct Game {
    #[planning_id]
    pub id: String,
    pub home_idx: usize,
    pub away_idx: usize,
    pub pair_key: usize,
    pub distances: Vec<i64>,
    // @solverforge:begin entity-variables
    #[planning_variable(value_range_provider = "rounds", allows_unassigned = false)]
    pub round: Option<usize>,
    // @solverforge:end entity-variables
}

impl Game {
    pub fn new(id: impl Into<String>, home_idx: usize, away_idx: usize, pair_key: usize, distances: Vec<i64>) -> Self {
        Self {
            id: id.into(),
            home_idx,
            away_idx,
            pair_key,
            distances,
            // @solverforge:begin entity-variable-init
            round: None,
            // @solverforge:end entity-variable-init
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_construction() {
        let entity = Game::new("test-id", Default::default(), Default::default(), Default::default(), Default::default());
        assert_eq!(entity.id, "test-id");
        let _ = &entity.home_idx;
        let _ = &entity.away_idx;
        let _ = &entity.pair_key;
        let _ = &entity.distances;
    }
}
