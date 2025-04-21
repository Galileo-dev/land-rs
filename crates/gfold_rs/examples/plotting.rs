//! Minimal driver demonstrating the new plotting helpers.

use gfold_rs::{
    plotting::*,
    trajectories::{APDGProblemSolver, Settings},
};

fn main() {
    let settings = Settings::builder().build();
    let mut solver = APDGProblemSolver::default();

    let (sol, hist) = solver.solve(&settings).unwrap();

    plot_trajectory_3d("trajectory_chart.png", "U-E-N Trajectory", &sol).unwrap();

    plot_position_velocity_time("pos_vel_chart.png", &sol, settings.simulation_settings()).unwrap();

    plot_thrust_mass_time(
        "thrust_mass_chart.png",
        &sol,
        settings.simulation_settings(),
    )
    .unwrap();

    plot_convergence("convergence_chart.png", &hist).unwrap();

    plot_relaxation_convergence("relaxation_convergence_chart.png", &hist).unwrap();
}
