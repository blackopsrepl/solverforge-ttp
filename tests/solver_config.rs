use solverforge::SolverConfig;

/// The runtime rejects unknown keys in solver.toml and fails every job at
/// start; load the shipped file the way the solver does.
#[test]
fn shipped_solver_toml_loads() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/solver.toml");
    SolverConfig::from_toml_file(path).expect("solver.toml must load");
}
