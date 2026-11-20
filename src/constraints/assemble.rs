//! Assembles the project's constraint set.

use crate::domain::Schedule;
use solverforge::prelude::*;

/// Builds the constraint set the solver scores every solution against.
pub fn create_constraints() -> impl ConstraintSet<Schedule, HardSoftScore> {
    // @solverforge:begin constraint-calls
    (
        super::away_double_booking::constraint(),
        super::cross_side_double_booking::constraint(),
        super::first_round_departure::constraint(),
        super::home_double_booking::constraint(),
        super::home_stand_limit::constraint(),
        super::last_round_return::constraint(),
        super::pair_no_repeater::constraint(),
        super::road_trip_limit::constraint(),
        super::travel_away_to_away::constraint(),
        super::travel_away_to_home::constraint(),
        super::travel_home_to_away::constraint(),
    )
    // @solverforge:end constraint-calls
}
