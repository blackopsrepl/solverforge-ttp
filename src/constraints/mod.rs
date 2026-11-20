//! Constraint definitions.
//!
//! Add constraint modules with `solverforge generate constraint ...`.

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

mod assemble;
mod run_length;
#[cfg(test)]
pub(crate) mod test_util;

pub use assemble::create_constraints;
pub use run_length::MAX_RUN_LENGTH;
