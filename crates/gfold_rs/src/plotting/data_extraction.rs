use crate::trajectories::{APDGSolution, SimulationParams};
use nalgebra::Vector3;

pub fn get_trajectory_3d_data(
    solution: &APDGSolution,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<Vector3<f64>>) {
    let time = time_vector(solution);
    let pos_u: Vec<f64> = solution.steps().iter().map(|s| s.r[0]).collect();
    let pos_e: Vec<f64> = solution.steps().iter().map(|s| s.r[1]).collect();
    let pos_n: Vec<f64> = solution.steps().iter().map(|s| s.r[2]).collect();
    let thrust_vectors: Vec<Vector3<f64>> = solution.steps().iter().map(|s| s.t).collect();
    (time, pos_u, pos_e, pos_n, thrust_vectors)
}

pub fn get_pos_vel_time_data(
    solution: &APDGSolution,
) -> (
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
) {
    let time = time_vector(solution);
    let pos_u: Vec<f64> = solution.steps().iter().map(|s| s.r[0]).collect();
    let pos_e: Vec<f64> = solution.steps().iter().map(|s| s.r[1]).collect();
    let pos_n: Vec<f64> = solution.steps().iter().map(|s| s.r[2]).collect();
    let vel_u: Vec<f64> = solution.steps().iter().map(|s| s.v[0]).collect();
    let vel_e: Vec<f64> = solution.steps().iter().map(|s| s.v[1]).collect();
    let vel_n: Vec<f64> = solution.steps().iter().map(|s| s.v[2]).collect();
    (time, pos_u, pos_e, pos_n, vel_u, vel_e, vel_n)
}

/// Calculates the tilt and azimuth angle in degrees
fn calculate_angles(thrust_vector: &Vector3<f64>, up_vector: &Vector3<f64>) -> (f64, f64) {
    let thrust_norm = thrust_vector.norm();
    if thrust_norm < 1e-6 {
        return (0.0, 0.0);
    }
    let normalized_thrust = thrust_vector / thrust_norm;
    let cos_tilt = normalized_thrust.dot(up_vector).max(-1.0).min(1.0);
    let tilt_angle_deg = cos_tilt.acos().to_degrees();
    let azimuth_deg = thrust_vector[2].atan2(thrust_vector[1]).to_degrees();
    (tilt_angle_deg, azimuth_deg)
}

pub fn get_thrust_mass_data(
    solution: &APDGSolution,
    params: &SimulationParams,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let time = time_vector(solution);
    let thrust_mag: Vec<f64> = solution.steps().iter().map(|s| s.gamma).collect();
    let mass: Vec<f64> = solution.steps().iter().map(|s| s.m).collect();
    let thrust_rate: Vec<f64> = if solution.num_steps() < 2 {
        Vec::new()
    } else {
        (0..solution.num_steps() - 1)
            .map(|k| (solution.steps()[k + 1].gamma - solution.steps()[k].gamma) / solution.dt())
            .collect()
    };
    let angles: Vec<(f64, f64)> = solution
        .steps()
        .iter()
        .map(|s| calculate_angles(&s.t, &params.e_hat_up))
        .collect();
    let tilt_angle: Vec<f64> = angles.iter().map(|(tilt, _)| *tilt).collect();
    let azimuth_angle: Vec<f64> = angles.iter().map(|(_, az)| *az).collect();

    (
        time,
        thrust_mag,
        thrust_rate,
        tilt_angle,
        azimuth_angle,
        mass,
    )
}

/// Calculate the T- time for each step
fn time_vector(solution: &APDGSolution) -> Vec<f64> {
    (0..solution.num_steps())
        .map(|k| k as f64 * solution.dt())
        .collect()
}
