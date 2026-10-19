/* Constraint definitions.

Add constraint modules with `solverforge generate constraint ...`.
The neutral shell starts with an empty constraint set. */

use crate::domain::Schedule;
use solverforge::prelude::*;

pub use self::assemble::create_constraints;

/// TTP(3): the canonical maximal home stand / road trip length (Easton,
/// Nemhauser, Trick 2001).
pub const MAX_RUN_LENGTH: usize = 3;

#[cfg(test)]
pub(crate) mod test_util;

// @solverforge:begin constraint-modules
mod away_double_booking;
mod cross_side_double_booking;
mod first_round_departure;
mod home_double_booking;
mod home_stand_limit;
mod last_round_return;
mod pair_no_repeater;
mod road_trip_limit;
mod travel_away_to_away;
mod travel_away_to_home;
mod travel_home_to_away;
// @solverforge:end constraint-modules

mod assemble {
    use super::*;

    pub fn create_constraints() -> impl ConstraintSet<Schedule, HardSoftScore> {
        // Alphabetical to match rustfmt's reorder_modules on the module block.
        // @solverforge:begin constraint-calls
        (
            away_double_booking::constraint(),
            cross_side_double_booking::constraint(),
            first_round_departure::constraint(),
            home_double_booking::constraint(),
            home_stand_limit::constraint(),
            last_round_return::constraint(),
            pair_no_repeater::constraint(),
            road_trip_limit::constraint(),
            travel_away_to_away::constraint(),
            travel_away_to_home::constraint(),
            travel_home_to_away::constraint(),
        )
        // @solverforge:end constraint-calls
    }
}
