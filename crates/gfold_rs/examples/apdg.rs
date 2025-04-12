use gfold_rs::trajectories::{APDGProblemSolver, Settings};

fn main() {
    let settings = Settings::builder().build();

    let mut apdg_problem = APDGProblemSolver::default();

    apdg_problem
        .solve(&settings)
        .expect("Failed to solve the APDG problem");
}
