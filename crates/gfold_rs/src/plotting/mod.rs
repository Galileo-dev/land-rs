use bon::builder;
use nalgebra::Vector3;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::prelude::*;

use crate::trajectories::{APDGSolution, ConvergenceHistory, SimulationParams};

const BASE_WIDTH: u32 = 1024 * 3;
const ASPECT_RATIO_STANDARD: f64 = 4.0 / 3.0;
const ASPECT_RATIO_WIDE: f64 = 8.0 / 3.0;

pub mod data_extraction;

/// Scale a vector of values by a factor
fn scale(values: &[f64], factor: f64) -> Vec<f64> {
    values.iter().map(|v| v * factor).collect()
}

pub fn plot_trajectory_3d(
    output: &str,
    title: &str,
    solution: &APDGSolution,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, pos_u, pos_e, pos_n, thrust) = data_extraction::get_trajectory_3d_data(solution);

    let aspect_ratio = ASPECT_RATIO_STANDARD;
    let width = BASE_WIDTH;
    let height = (width as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // axis limits (U, E, N)
    let (u_min, u_max) = min_max(&pos_u).unwrap_or((0.0, 1.0));
    let (e_min, e_max) = min_max(&pos_e).unwrap_or((0.0, 1.0));
    let (n_min, n_max) = min_max(&pos_n).unwrap_or((0.0, 1.0));

    // Up, East, North reference frame
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 60).into_font())
        .build_cartesian_3d(e_min..e_max, u_min..u_max, n_min..n_max)?;

    chart
        .configure_axes()
        .label_style(("sans-serif", 40).into_font())
        .draw()?;

    // trajectory line
    chart.draw_series(LineSeries::new(
        pos_e
            .iter()
            .zip(&pos_n)
            .zip(&pos_u)
            .map(|((&e, &n), &u)| (e, u, n)),
        RED.stroke_width(5),
    ))?;

    // thrust vectors
    let max_t = thrust.iter().map(Vector3::norm).fold(0.0, f64::max);
    if max_t > 1e-6 {
        let scale = (e_max - e_min).min(n_max - n_min).min(u_max - u_min) / max_t / 5.0;

        for (((&e, &n), &u), t) in pos_e.iter().zip(&pos_n).zip(&pos_u).zip(&thrust) {
            let start = (e, u, n);
            let end = (e + t[1] * scale, u + t[0] * scale, n + t[2] * scale);
            chart.draw_series(LineSeries::new(
                vec![start, end],
                BLUE.mix(0.5).stroke_width(3),
            ))?;
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

    let aspect_ratio = ASPECT_RATIO_STANDARD;
    let width = BASE_WIDTH;
    let height = (width as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let cells = root.split_evenly((2, 3));

    single_time_series()
        .chart_builder(build_chart(&cells[0])?)
        .t(&time)
        .y(&pos_u)
        .caption("U-Pos (m)")
        .col(&RED)
        .x_label("Time (s)")
        .y_label("Position (m)")
        .call()?;
    single_time_series()
        .chart_builder(build_chart(&cells[1])?)
        .t(&time)
        .y(&pos_e)
        .caption("E-Pos (m)")
        .col(&GREEN)
        .x_label("Time (s)")
        .y_label("Position (m)")
        .call()?;
    single_time_series()
        .chart_builder(build_chart(&cells[2])?)
        .t(&time)
        .y(&pos_n)
        .caption("N-Pos (m)")
        .col(&BLUE)
        .x_label("Time (s)")
        .y_label("Position (m)")
        .call()?;
    single_time_series()
        .chart_builder(build_chart(&cells[3])?)
        .t(&time)
        .y(&vel_u)
        .caption("U-Vel (m/s)")
        .col(&RED)
        .x_label("Time (s)")
        .y_label("Velocity (m/s)")
        .call()?;
    single_time_series()
        .chart_builder(build_chart(&cells[4])?)
        .t(&time)
        .y(&vel_e)
        .caption("E-Vel (m/s)")
        .col(&GREEN)
        .x_label("Time (s)")
        .y_label("Velocity (m/s)")
        .call()?;
    single_time_series()
        .chart_builder(build_chart(&cells[5])?)
        .t(&time)
        .y(&vel_n)
        .caption("N-Vel (m/s)")
        .col(&BLUE)
        .x_label("Time (s)")
        .y_label("Velocity (m/s)")
        .call()?;

    root.present()?;
    Ok(())
}

pub fn plot_thrust_time(
    output: &str,
    solution: &APDGSolution,
    sim: &SimulationParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, t_mag, t_rate, tilt, az, mass) =
        data_extraction::get_thrust_mass_data(solution, sim);

    let aspect_ratio = ASPECT_RATIO_STANDARD;
    let width = BASE_WIDTH;
    let height = (width as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let cells = root.split_evenly((2, 2));

    let time_rate = &time[..t_rate.len()];

    // Scale data to kN and kN/s
    let t_mag_kn = scale(&t_mag, 1e-3);
    let t_min_vac_kn = sim.t_min_vac * 1e-3;
    let t_max_vac_kn = sim.t_max_vac * 1e-3;
    let t_rate_kn = scale(&t_rate, 1e-3);
    let tdot_min_kn = sim.tdot_min * 1e-3;
    let tdot_max_kn = sim.tdot_max * 1e-3;

    single_time_series()
        .chart_builder(build_chart(&cells[0])?)
        .t(&time)
        .y(&t_mag_kn)
        .caption("Vac. Thrust (kN)")
        .col(&BLUE)
        .lb(t_min_vac_kn)
        .ub(t_max_vac_kn)
        .x_label("Time (s)")
        .y_label("Thrust (kN)")
        .call()?;

    let mut rate_chart = single_time_series()
        .chart_builder(build_chart(&cells[1])?)
        .t(time_rate)
        .y(&t_rate_kn)
        .caption("Thrust d/dt (kN/s)")
        .col(&MAGENTA)
        .lb(tdot_min_kn)
        .ub(tdot_max_kn)
        .x_label("Time (s)")
        .y_label("Thrust Rate (kN/s)")
        .call()?;

    add_horizontal_dashed_line(&mut rate_chart, 0.0, &BLACK, 5, 5)?;

    single_time_series()
        .chart_builder(build_chart(&cells[2])?)
        .t(&time)
        .y(&tilt)
        .caption("Tilt (deg)")
        .col(&RED)
        .lb(0.0)
        .ub(sim.theta_max)
        .x_label("Time (s)")
        .y_label("Tilt Angle (deg)")
        .call()?;

    single_time_series()
        .chart_builder(build_chart(&cells[3])?)
        .t(&time)
        .y(&az)
        .caption("Azimuth (deg)")
        .col(&CYAN)
        .x_label("Time (s)")
        .y_label("Azimuth (deg)")
        .call()?;

    root.present()?;
    Ok(())
}

pub fn plot_mass_time(
    output: &str,
    solution: &APDGSolution,
    sim: &SimulationParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let (time, t_mag, t_rate, tilt, az, mass) =
        data_extraction::get_thrust_mass_data(solution, sim);

    let aspect_ratio = ASPECT_RATIO_WIDE;
    let width = BASE_WIDTH;
    let height = (width as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mass_t = scale(&mass, 1e-3);
    let m0_t = sim.m_0 * 1e-3;
    let mdry_t = sim.m_dry * 1e-3;

    let mut chart = single_time_series()
        .chart_builder(build_chart(&root)?)
        .t(&time)
        .y(&mass_t)
        .col(&BLACK)
        .lb(mdry_t)
        .ub(m0_t)
        .x_label("Time (s)")
        .y_label("Mass (x1000 kg)")
        .call()?;

    add_horizontal_dashed_line(&mut chart, m0_t, &BLACK, 20, 20)?;
    add_horizontal_dashed_line(&mut chart, mdry_t, &BLACK, 20, 20)?;
    root.present()?;
    Ok(())
}

/// Function to find the min and max value in a vec
fn min_max(v: &[f64]) -> Option<(f64, f64)> {
    v.iter().fold(None, |acc, &x| match acc {
        None => Some((x, x)),
        Some((lo, hi)) => Some((lo.min(x), hi.max(x))),
    })
}

fn build_chart<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
) -> Result<ChartBuilder<DB>, Box<dyn std::error::Error>> {
    let mut chart_builder = ChartBuilder::on(area);

    Ok(chart_builder)
}

#[builder]
fn single_time_series<DB: DrawingBackend, 'a>(
    mut chart_builder: ChartBuilder<'a, '_, DB>,
    t: &[f64],
    y: &[f64],
    caption: Option<&str>,
    col: &RGBColor,
    lb: Option<f64>,
    ub: Option<f64>,
    x_label: &str,
    y_label: &str,
) -> Result<
    ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    Box<dyn std::error::Error>,
>
where
    DB::ErrorType: std::error::Error + 'static,
{
    if t.is_empty() || y.is_empty() {
        let mut empty_chart = chart_builder
            .margin(50)
            .set_label_area_size(LabelAreaPosition::Left, 120)
            .set_label_area_size(LabelAreaPosition::Bottom, 80)
            .build_cartesian_2d(0f64..1.0, 0.0..1.0)?;

        empty_chart.configure_mesh().draw()?;
        return Ok(empty_chart);
    }

    let (y_min, y_max) = min_max(y).unwrap_or((-1.0, 1.0));

    let y_low = match lb {
        Some(lb_val) => y_min.min(lb_val),
        None => y_min,
    };

    let y_high = match ub {
        Some(ub_val) => y_max.max(ub_val),
        None => y_max,
    };

    if let Some(cap) = caption {
        chart_builder.caption(cap, ("sans-serif", 40));
    }

    let pad = 1.0;
    let y_low = y_low - pad;
    let y_high = y_high + pad;

    let mut chart = chart_builder
        .margin(50)
        .set_label_area_size(LabelAreaPosition::Left, 120)
        .set_label_area_size(LabelAreaPosition::Bottom, 80)
        .build_cartesian_2d(0f64..t.last().copied().unwrap_or(1.0), (y_low)..(y_high))?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", 40).into_font())
        .y_desc(y_label)
        .x_desc(x_label)
        .light_line_style(WHITE)
        .x_label_formatter(&|v| {
            if (v.fract()).abs() < 1e-6 {
                format!("{}", v.round() as i32)
            } else {
                format!("{:.1}", v)
            }
        })
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    chart.draw_series(LineSeries::new(
        t.iter().zip(y).map(|(&x, &y)| (x, y)),
        col.stroke_width(3),
    ))?;

    Ok(chart)
}

/// Add a horizontal dashed line to a chart
fn add_horizontal_dashed_line<'a, DB>(
    chart: &mut ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    y_value: f64,
    color: &'static RGBColor,
    dash_px: u32,
    gap_px: u32,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB: DrawingBackend + 'a,
    DB::ErrorType: std::error::Error + 'static,
{
    use plotters::style::ShapeStyle;

    let x_range = chart.x_range();
    let (x_start, x_end) = (x_range.start, x_range.end);

    let area = chart.plotting_area();
    let px_width = area.dim_in_pixel().0 as f64;
    let units_per_pixel = (x_end - x_start).abs() / px_width.max(f64::EPSILON);
    let dash_len = dash_px as f64 * units_per_pixel;
    let gap_len = gap_px as f64 * units_per_pixel;

    let mut x: f64 = x_start;
    while x < x_end {
        let dash_end = (x + dash_len).min(x_end);
        chart.draw_series(LineSeries::new(
            vec![(x, y_value), (dash_end, y_value)],
            ShapeStyle {
                color: color.to_rgba(),
                filled: false,
                stroke_width: 2,
            },
        ))?;
        x = dash_end + gap_len;
    }

    Ok(())
}

fn present_chart<'a, DB: DrawingBackend + 'a>(
    chart: ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<(), Box<dyn std::error::Error>>
where
    DB::ErrorType: std::error::Error + 'static,
{
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

    let aspect_ratio = ASPECT_RATIO_STANDARD;
    let width = BASE_WIDTH;
    let height = (BASE_WIDTH as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let cells = root.split_evenly((3, 1));

    let iters: Vec<f64> = (1..=hist.len()).map(|i| i as f64).collect();

    single_time_series()
        .chart_builder(build_chart(&cells[0])?)
        .t(&iters)
        .y(&hist.pos)
        .caption("Position Convergence (log10 dpos)")
        .col(&RED)
        .x_label("Iteration")
        .y_label("log10 Position Error")
        .call()?;

    single_time_series()
        .chart_builder(build_chart(&cells[1])?)
        .t(&iters)
        .y(&hist.vel)
        .caption("Velocity Convergence (log10 dvel)")
        .col(&GREEN)
        .x_label("Iteration")
        .y_label("log10 Velocity Error")
        .call()?;

    single_time_series()
        .chart_builder(build_chart(&cells[2])?)
        .t(&iters)
        .y(&hist.thrust)
        .caption("Thrust Convergence (log10 dT)")
        .col(&BLUE)
        .x_label("Iteration")
        .y_label("log10 Thrust Error")
        .call()?;

    root.present()?;
    Ok(())
}

pub fn plot_relaxation_convergence(
    output: &str,
    hist: &ConvergenceHistory,
) -> Result<(), Box<dyn std::error::Error>> {
    if hist.pos.is_empty() {
        eprintln!("No convergence data to plot");
        return Ok(());
    }

    let aspect_ratio = ASPECT_RATIO_STANDARD;
    let width = BASE_WIDTH;
    let height = (width as f64 / aspect_ratio).round() as u32;

    let root = BitMapBackend::new(output, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    const LOG_EPSILON: f64 = 1e-10; // Prevent a log10(0) error

    // Log Max ||aR||
    let max_aR = hist.aR.iter().fold(0.0, |max, &v| v.abs().max(max));

    let iters: Vec<f64> = (1..=hist.len()).map(|i| i as f64).collect();
    let (min, max) = (0.0, max_aR);

    single_time_series()
        .chart_builder(build_chart(&root)?)
        .t(&iters)
        .y(&hist.aR)
        .caption("Relaxation Term Convergence ||aR||")
        .col(&RED)
        .lb(min)
        .ub(max)
        .x_label("Iteration")
        .y_label("Max ||aR||")
        .call()?;

    root.present()?;
    Ok(())
}
