use nalgebra::Vector3;
use plotters::coord::Shift;
use plotters::prelude::*;

use crate::trajectories::{APDGSolution, ConvergenceHistory, SimulationParams};

pub mod data_extraction;

pub fn plot_trajectory_3d(
    output: &str,
    title: &str,
    solution: &APDGSolution,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, pos_u, pos_e, pos_n, thrust) = data_extraction::get_trajectory_3d_data(solution);

    let root = BitMapBackend::new(output, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    // axis limits (U, E, N) + padding
    let (u_min, u_max) = min_max(&pos_u).unwrap_or((0.0, 1.0));
    let (e_min, e_max) = min_max(&pos_e).unwrap_or((0.0, 1.0));
    let (n_min, n_max) = min_max(&pos_n).unwrap_or((0.0, 1.0));
    let (u_min, u_max, e_min, e_max, n_min, n_max) =
        add_pad_3d(u_min, u_max, e_min, e_max, n_min, n_max);

    // Up, East, North reference frame
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40).into_font())
        .build_cartesian_3d(u_min..u_max, e_min..e_max, n_min..n_max)?;

    chart.configure_axes().draw()?;

    // trajectory line
    chart
        .draw_series(LineSeries::new(
            pos_u
                .iter()
                .zip(&pos_e)
                .zip(&pos_n)
                .map(|((&u, &e), &n)| (u, e, n)),
            &RED,
        ))?
        .label("Trajectory")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], &RED));

    // thrust vectors
    let max_t = thrust.iter().map(Vector3::norm).fold(0.0, f64::max);
    if max_t > 1e-6 {
        let scale = (u_max - u_min).min(e_max - e_min).min(n_max - n_min) / max_t / 10.0;
        for (((&u, &e), &n), t) in pos_u.iter().zip(&pos_e).zip(&pos_n).zip(&thrust) {
            let end = (u + t[0] * scale, e + t[1] * scale, n + t[2] * scale);
            chart.draw_series(LineSeries::new(vec![(u, e, n), end], &BLUE.mix(0.5)))?;
        }
    }

    root.present()?;
    Ok(())
}

pub fn plot_position_velocity_time(
    output: &str,
    solution: &APDGSolution,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, pos_u, pos_e, pos_n, vel_u, vel_e, vel_n) =
        data_extraction::get_pos_vel_time_data(solution);

    let root = BitMapBackend::new(output, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let cells = root.split_evenly((2, 3));

    single_time_series(&cells[0], &time, &pos_u, "U-Pos (m)", &RED)?;
    single_time_series(&cells[1], &time, &pos_e, "E-Pos (m)", &GREEN)?;
    single_time_series(&cells[2], &time, &pos_n, "N-Pos (m)", &BLUE)?;
    single_time_series(&cells[3], &time, &vel_u, "U-Vel (m/s)", &RED)?;
    single_time_series(&cells[4], &time, &vel_e, "E-Vel (m/s)", &GREEN)?;
    single_time_series(&cells[5], &time, &vel_n, "N-Vel (m/s)", &BLUE)?;

    root.present()?;
    Ok(())
}

pub fn plot_thrust_mass_time(
    output: &str,
    solution: &APDGSolution,
    sim: &SimulationParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, t_mag, t_rate, tilt, az, mass) =
        data_extraction::get_thrust_mass_data(solution, sim);

    let root = BitMapBackend::new(output, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let cells = root.split_evenly((2, 3));

    let time_rate = &time[..t_rate.len()];

    series_bounds(
        &cells[0],
        &time,
        &t_mag,
        "Vac. Thrust (N)",
        sim.t_min_vac,
        sim.t_max_vac,
        &BLUE,
    )?;
    series_bounds(
        &cells[1],
        time_rate,
        &t_rate,
        "Thrust d/dt (N/s)",
        sim.tdot_min,
        sim.tdot_max,
        &MAGENTA,
    )?;
    series_bounds(
        &cells[2],
        &time,
        &tilt,
        "Tilt (deg)",
        0.0,
        sim.theta_max,
        &RED,
    )?;
    single_time_series(&cells[3], &time, &az, "Azimuth (deg)", &CYAN)?;
    series_bounds(
        &cells[4],
        &time,
        &mass,
        "Mass (kg)",
        sim.m_dry,
        sim.m_0,
        &BLACK,
    )?;

    root.present()?;
    Ok(())
}

fn min_max(v: &[f64]) -> Option<(f64, f64)> {
    v.iter().fold(None, |acc, &x| match acc {
        None => Some((x, x)),
        Some((lo, hi)) => Some((lo.min(x), hi.max(x))),
    })
}

fn add_pad_3d(
    u0: f64,
    u1: f64,
    e0: f64,
    e1: f64,
    n0: f64,
    n1: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let pad = |rng: f64| {
        if rng.abs() < 1e-6 {
            0.1
        } else {
            rng * 0.1
        }
    };
    (
        u0 - pad(u1 - u0),
        u1 + pad(u1 - u0),
        e0 - pad(e1 - e0),
        e1 + pad(e1 - e0),
        n0 - pad(n1 - n0),
        n1 + pad(n1 - n0),
    )
}

fn single_time_series<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    t: &[f64],
    y: &[f64],
    caption: &str,
    col: &RGBColor,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB::ErrorType: std::error::Error + 'static,
{
    if t.is_empty() || y.is_empty() {
        return Ok(());
    }
    let (y0, y1) = min_max(y).unwrap_or((-1.0, 1.0));
    let pad = if (y1 - y0).abs() < 1e-6 {
        0.1
    } else {
        (y1 - y0) * 0.1
    };
    let mut chart = ChartBuilder::on(area)
        .caption(caption, ("sans-serif", 12))
        .margin(5)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(
            0f64..t.last().copied().unwrap_or(1.0),
            (y0 - pad)..(y1 + pad),
        )?;
    chart.configure_mesh().y_labels(5).x_labels(3).draw()?;
    chart.draw_series(LineSeries::new(t.iter().zip(y).map(|(&x, &y)| (x, y)), col))?;
    Ok(())
}

fn series_bounds<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    t: &[f64],
    y: &[f64],
    caption: &str,
    lb: f64,
    ub: f64,
    col: &RGBColor,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB::ErrorType: std::error::Error + 'static,
{
    if t.is_empty() || y.is_empty() {
        return Ok(());
    }
    let (y0, y1) = min_max(y).unwrap_or((-1.0, 1.0));
    let y_low = y0.min(lb);
    let y_high = y1.max(ub);
    let pad = if (y_high - y_low).abs() < 1e-6 {
        0.1
    } else {
        (y_high - y_low) * 0.1
    };
    let mut chart = ChartBuilder::on(area)
        .caption(caption, ("sans-serif", 12))
        .margin(5)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(
            0f64..t.last().copied().unwrap_or(1.0),
            (y_low - pad)..(y_high + pad),
        )?;
    chart.configure_mesh().y_labels(5).x_labels(3).draw()?;

    let t_end = *t.last().unwrap_or(&0.0);
    chart.draw_series(LineSeries::new(
        vec![(0.0, lb), (t_end, lb)],
        &BLACK.mix(0.5),
    ))?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, ub), (t_end, ub)],
        &BLACK.mix(0.5),
    ))?;
    chart.draw_series(LineSeries::new(t.iter().zip(y).map(|(&x, &y)| (x, y)), col))?;
    Ok(())
}

pub fn plot_convergence(
    output: &str,
    hist: &ConvergenceHistory,
) -> Result<(), Box<dyn std::error::Error>> {
    if hist.pos.is_empty() {
        eprintln!("No convergence data to plot");
        return Ok(());
    }
    let root = BitMapBackend::new(output, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min, max) = [&hist.pos, &hist.vel, &hist.thrust]
        .iter()
        .flat_map(|v| v.iter())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), &v| {
            (mn.min(v), mx.max(v))
        });
    let pad = if (max - min).abs() < 1e-6 {
        0.1
    } else {
        (max - min) * 0.1
    };

    let mut chart = ChartBuilder::on(&root)
        .caption("SC Convergence", ("sans-serif", 20))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0usize..(hist.len() + 1), (min - pad)..(max + pad))?;

    chart.configure_mesh().draw()?;

    let iters = 1..=hist.len();
    chart
        .draw_series(LineSeries::new(
            iters.clone().zip(&hist.pos).map(|(i, &v)| (i, v)),
            &RED,
        ))?
        .label("log10 dpos")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(
            iters.clone().zip(&hist.vel).map(|(i, &v)| (i, v)),
            &GREEN,
        ))?
        .label("log10 dvel")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart
        .draw_series(LineSeries::new(
            iters.zip(&hist.thrust).map(|(i, &v)| (i, v)),
            &BLUE,
        ))?
        .label("log10 dT")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
