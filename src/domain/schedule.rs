use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

// @solverforge:begin solution-imports
use super::Team;
use super::Round;
use super::Game;
// @solverforge:end solution-imports

#[planning_solution(
    constraints = "crate::constraints::create_constraints",
    solver_toml = "../../solver.toml",
    config = "configure_solver"
)]
#[derive(Serialize, Deserialize)]
pub struct Schedule {
    // @solverforge:begin solution-collections
    #[problem_fact_collection]
    pub teams: Vec<Team>,
    #[problem_fact_collection]
    pub rounds: Vec<Round>,
    #[planning_entity_collection]
    pub games: Vec<Game>,
    // @solverforge:end solution-collections
    #[planning_score]
    pub score: Option<HardSoftScore>,
    /// Wall-clock solve budget supplied with each job, in seconds.
    #[serde(default = "default_duration_seconds")]
    pub duration_seconds: u64,
}

impl Schedule {
    #[rustfmt::skip]
    pub fn new(
        // @solverforge:begin solution-constructor-params
        teams: Vec<Team>,
        rounds: Vec<Round>,
        games: Vec<Game>,
        // @solverforge:end solution-constructor-params
    ) -> Self {
        Self {
            // @solverforge:begin solution-constructor-init
            teams,
            rounds,
            games,
            // @solverforge:end solution-constructor-init
            score: None,
            duration_seconds: default_duration_seconds(),
        }
    }
}

pub fn default_duration_seconds() -> u64 {
    30
}

/// Per-plan solve budget hook: each job's wall-clock termination comes from
/// the posted plan, not from the shared `solver.toml`.
fn configure_solver(plan: &Schedule, config: solverforge::SolverConfig) -> solverforge::SolverConfig {
    config.with_termination_seconds(plan.duration_seconds)
}
