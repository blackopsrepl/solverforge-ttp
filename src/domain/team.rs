use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

/// TODO — describe this fact.
#[problem_fact]
#[derive(Serialize, Deserialize)]
pub struct Team {
    #[planning_id]
    pub id: String,
    pub name: String,
    pub idx: usize,
    pub distances: Vec<i64>,
    pub num_rounds: usize,
}

impl Team {
    pub fn new(id: impl Into<String>, name: impl Into<String>, idx: usize, distances: Vec<i64>, num_rounds: usize) -> Self {
        Self { id: id.into(), name: name.into(), idx, distances, num_rounds }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_construction() {
        let fact = Team::new("test-id", "test", Default::default(), Default::default(), Default::default());
        assert_eq!(fact.id, "test-id");
        assert_eq!(fact.name, "test");
        let _ = &fact.idx;
        let _ = &fact.distances;
        let _ = &fact.num_rounds;
    }
}
