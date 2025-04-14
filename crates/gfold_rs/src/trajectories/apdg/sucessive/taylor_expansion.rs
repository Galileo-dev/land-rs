use crate::trajectories::apdg::models::SimulationParams;
use crate::trajectories::APDGSolutionTimeStep;
use autodiff::F;
use good_lp::constraint;
use good_lp::variable;
use good_lp::{Expression, Variable};
use nalgebra::{DVector, Vector3};
use num_traits::Pow;
pub type F64 = F<f64, f64>;

/// Builds a good_lp::Expression representing the first-order Taylor expansion
/// of a given expression around a linearisation point.
///
///  Formula: f_bar + sum( derivative_i * (variable_i - bar_i) )
///
/// Args:
/// * `func`: The function to linearise. accepts a DVector of autodiff::F types.
/// * `vars_and_bars`: A slice of tuples containing a good_lp::Variable and its linearisation point.
///
/// Returns:
/// * `good_lp::Expression` for the Taylor expansion: f_bar + sum(df_dxi * (xi - xi_bar))
pub(super) fn build_taylor_expression<'a>(
    func: impl Fn(&DVector<F64>) -> F64,
    vars_and_bars: &'a [(Variable, f64)],
) -> Expression {
    let num_vars = vars_and_bars.len();

    let bar_values_f64: DVector<f64> =
        DVector::from_iterator(num_vars, vars_and_bars.iter().map(|(_, bar)| *bar));
    let bar_values_cst: DVector<F64> = bar_values_f64.map(F64::cst);
    let f_bar = func(&bar_values_cst).x;

    let mut derivatives: DVector<f64> = DVector::zeros(num_vars);
    for i in 0..num_vars {
        let mut inputs_for_ad = bar_values_cst.clone();
        inputs_for_ad[i] = F64::var(bar_values_f64[i]);
        let df_dxi = func(&inputs_for_ad).dx;
        derivatives[i] = df_dxi;
    }

    let mut taylor_expr = Expression::from(f_bar);

    for i in 0..num_vars {
        let (variable, var_bar) = vars_and_bars[i];
        let derivative = derivatives[i];
        taylor_expr += derivative * (Expression::from(variable) - var_bar);
    }

    taylor_expr
}

#[cfg(test)]
mod tests {
    use super::*;
    use good_lp::{variables, Expression, IntoAffineExpression, Variable};
    use nalgebra::DVector;
    use num_traits::{real::Real, Pow};
    use std::collections::HashMap;

    #[test]
    fn test_taylor_expansion_quadratic() {
        let func = |x: &DVector<F64>| x[0].powi(2) + 2.0 * x[1];

        let mut vars = variables!();
        let x0 = vars.add(variable().min(0));
        let x1: Variable = vars.add(variable().min(0));
        let vars_and_bars = vec![(x0, 1.0), (x1, 2.0)]; // Linearize around (1.0, 2.0)

        let taylor_expr = build_taylor_expression(func, &vars_and_bars);

        let expected_expr = 2.0 * x0 + 2.0 * x1 - 1.0;

        assert!((taylor_expr.constant() - expected_expr.constant()).abs() < 1e-9);
        let taylor_coeffs: HashMap<_, _> = taylor_expr.linear_coefficients().collect();
        let expected_coeffs: HashMap<_, _> = expected_expr.linear_coefficients().collect();
        assert_eq!(taylor_coeffs.len(), expected_coeffs.len());

        assert!(
            (taylor_coeffs.get(&x0).unwrap_or(&0.0) - expected_coeffs.get(&x0).unwrap_or(&0.0))
                .abs()
                < 1e-9
        );
        assert!(
            (taylor_coeffs.get(&x1).unwrap_or(&0.0) - expected_coeffs.get(&x1).unwrap_or(&0.0))
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_taylor_expansion_product() {
        let func = |x: &DVector<F64>| x[0] * x[1];

        let mut vars = variables!();
        let x0 = vars.add(variable().min(0));
        let x1 = vars.add(variable().min(0));
        let vars_and_bars = vec![(x0, 3.0), (x1, 4.0)]; // Linearize around (3.0, 4.0)

        let taylor_expr = build_taylor_expression(func, &vars_and_bars);

        let expected_expr = 4.0 * x0 + 3.0 * x1 - 12.0;

        assert!((taylor_expr.constant() - expected_expr.constant()).abs() < 1e-9);

        let taylor_coeffs: HashMap<_, _> = taylor_expr.linear_coefficients().collect();
        let expected_coeffs: HashMap<_, _> = expected_expr.linear_coefficients().collect();

        assert_eq!(taylor_coeffs.len(), expected_coeffs.len());

        assert!(
            (taylor_coeffs.get(&x0).unwrap_or(&0.0) - expected_coeffs.get(&x0).unwrap_or(&0.0))
                .abs()
                < 1e-9
        );
        assert!(
            (taylor_coeffs.get(&x1).unwrap_or(&0.0) - expected_coeffs.get(&x1).unwrap_or(&0.0))
                .abs()
                < 1e-9
        );
    }
}
