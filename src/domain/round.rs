use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

/// TODO — describe this fact.
#[problem_fact]
#[derive(Serialize, Deserialize)]
pub struct Round {
    #[planning_id]
    pub id: String,
    pub name: String,
    pub index: usize,
}

impl Round {
    pub fn new(id: impl Into<String>, name: impl Into<String>, index: usize) -> Self {
        Self { id: id.into(), name: name.into(), index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_construction() {
        let fact = Round::new("test-id", "test", Default::default());
        assert_eq!(fact.id, "test-id");
        assert_eq!(fact.name, "test");
        let _ = &fact.index;
    }
}
