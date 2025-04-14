use crate::trajectories::apdg::models::SimulationParams;
use crate::trajectories::APDGSolutionTimeStep;
use autodiff::F;
use good_lp::constraint;
use good_lp::variable;
use good_lp::{Expression, Variable};
use nalgebra::Vector3;
type F64 = F<f64, f64>;

/// Builds a good_lp::Expression representing the first-order Taylor expansion
/// of a given expression around a linearization point.
/// Returns:
/// * `good_lp::Expression` for the Taylor expansion: f_bar + sum(df_dxi * (xi - xi_bar))
pub(super) fn build_taylor_expression<'a>(
    func: impl Fn(&[F64]) -> F64,
    vars_and_bars: &'a [(Variable, f64)],
) -> Expression {
    let num_vars = vars_and_bars.len();

    let bar_values_f64: Vec<f64> = vars_and_bars.iter().map(|(_, bar)| *bar).collect();
    let bar_values_f64_cst: Vec<F64> = bar_values_f64.iter().map(|v| F64::cst(*v)).collect();
    let f_bar = func(&bar_values_f64_cst).x;

    let mut derivatives: Vec<f64> = Vec::with_capacity(num_vars);
    for i in 0..num_vars {
        let mut inputs_for_ad: Vec<F64> = bar_values_f64.iter().map(|v| F64::cst(*v)).collect();
        inputs_for_ad[i] = F64::var(bar_values_f64[i]);
        let df_dxi = func(&inputs_for_ad).dx;
        derivatives.push(df_dxi);
    }

    let mut taylor_expr = Expression::from(f_bar);

    for i in 0..num_vars {
        let (variable, var_bar) = vars_and_bars[i];
        let derivative = derivatives[i];
        taylor_expr += derivative * (Expression::from(variable) - var_bar);
    }

    taylor_expr
}
