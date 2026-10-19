solverforge::planning_model! {
    root = "src/domain";

    // @solverforge:begin domain-exports
mod team;
mod round;
mod game;
mod schedule;

pub use team::Team;
pub use round::Round;
pub use game::Game;
pub use schedule::Schedule;
// @solverforge:end domain-exports
}
