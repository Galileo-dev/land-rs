
```rust
/// The [clarabel](https://oxfordcontrol.github.io/ClarabelDocs/stable/) solver,
/// to be used with [UnsolvedProblem::using].
pub fn clarabel(to_solve: UnsolvedProblem) -> ClarabelProblem {
    let UnsolvedProblem {
        objective,
        direction,
        variables,
    } = to_solve;
    let coef = if direction == ObjectiveDirection::Maximisation {
        -1.
    } else {
        1.
    };
    let mut objective_vector = vec![0.; variables.len()];
    for (var, obj) in objective.linear_coefficients() {
        objective_vector[var.index()] = obj * coef;
    }
    let constraints_matrix_builder = CscMatrixBuilder::new(variables.len());
    let mut settings = DefaultSettingsBuilder::default();
    settings.verbose(false).tol_feas(1e-9);
    let mut p = ClarabelProblem {
        objective: objective_vector,
        constraints_matrix_builder,
        constraint_values: Vec::new(),
        variables: variables.len(),
        settings,
        cones: Vec::new(),
    };
    // add trivial constraints embedded in the variable definitions
    for (var, def) in variables.iter_variables_with_def() {
        if def.is_integer {
            panic!("Clarabel doesn't support integer variables")
        }
        if def.min != f64::NEG_INFINITY {
            p.add_constraint(var >> def.min);
        }
        if def.max != f64::INFINITY {
            p.add_constraint(var << def.max);
        }
    }
    p
}

/// A clarabel model
pub struct ClarabelProblem {
    constraints_matrix_builder: CscMatrixBuilder,
    constraint_values: Vec<f64>,
    objective: Vec<f64>,
    variables: usize,
    settings: DefaultSettingsBuilder<f64>,
    cones: Vec<SupportedConeT<f64>>,
}

impl ClarabelProblem {
    /// Access the problem settings
    pub fn settings(&mut self) -> &mut DefaultSettingsBuilder<f64> {
        &mut self.settings
    }

    /// Convert the problem into a clarabel solver
    pub fn into_solver(self) -> DefaultSolver<f64> {
        let settings = self.settings.build().expect("Invalid clarabel settings");
        let quadratic_objective = &CscMatrix::zeros((self.variables, self.variables));
        let objective = &self.objective;
        let constraints = &self.constraints_matrix_builder.build();
        let constraint_values = &self.constraint_values;
        let cones = &self.cones;
        DefaultSolver::new(
            quadratic_objective,
            objective,
            constraints,
            constraint_values,
            cones,
            settings,
        )
    }
}

impl SolverModel for ClarabelProblem {
    type Solution = ClarabelSolution;
    type Error = ResolutionError;

    fn solve(self) -> Result<Self::Solution, Self::Error> {
        let mut solver = self.into_solver();
        solver.solve();
        match solver.solution.status {
            e @ (SolverStatus::PrimalInfeasible | SolverStatus::AlmostPrimalInfeasible) => {
                eprintln!("Clarabel error: {:?}", e);
                Err(ResolutionError::Infeasible)
            }
            SolverStatus::Solved
            | SolverStatus::AlmostSolved
            | SolverStatus::AlmostDualInfeasible
            | SolverStatus::DualInfeasible => Ok(ClarabelSolution {
                solution: solver.solution,
            }),
            SolverStatus::Unsolved => Err(ResolutionError::Other("Unsolved")),
            SolverStatus::MaxIterations => Err(ResolutionError::Other("Max iterations reached")),
            SolverStatus::MaxTime => Err(ResolutionError::Other("Time limit reached")),
            SolverStatus::NumericalError => Err(ResolutionError::Other("Numerical error")),
            SolverStatus::InsufficientProgress => Err(ResolutionError::Other("No progress")),
        }
    }

    fn add_constraint(&mut self, constraint: Constraint) -> ConstraintReference {
        match constraint.kind {
            ConstraintType::Equality => {
                let expr = constraint.linear_expression();
                let row_linear = expr.linear.clone();
                let row_const = expr.constant;
                self.constraints_matrix_builder.add_row(row_linear);
                self.constraint_values.push(-row_const);
                // Merged into the ZeroCone
                match self.cones.last_mut() {
                    Some(SupportedConeT::ZeroConeT(ref mut size)) => {
                        *size += 1;
                    }
                    _ => self.cones.push(
                      SupportedConeT::ZeroConeT(1)
                      ),
                }
                ConstraintReference {
                    index: self.constraint_values.len() - 1,
                }
            }

            ConstraintType::LessThanOrEqual => {
                let expr: &crate::Expression = constraint.linear_expression();
                let row_linear = expr.linear.clone();
                let row_const = expr.constant;
                self.constraints_matrix_builder.add_row(row_linear);
                self.constraint_values.push(-row_const);
                // Merged into NonnegativeCone
                match self.cones.last_mut() {
                    Some(SupportedConeT::NonnegativeConeT(ref mut size)) => {
                        *size += 1;
                    }
                    _ => self.cones.push(
                      SupportedConeT::NonnegativeConeT(1)
                      ),
                }
                ConstraintReference {
                    index: self.constraint_values.len() - 1,
                }
            }

            ConstraintType::SecondOrderCone => {
                // Clarabel standard form Ax + s = b, s in
                //SOC means s = b - Ax, s_0 >= ||s_1...||.
                // We need s_0 = t = Ct*x + dt and s_i = x_i = Coff_xi*x + dxi.
                // The constraint.expressions vector is in the form:
                // [x1, ..., xn, t].
                // Process t (last element) first
                let dim = constraint.expressions.len();
                let start_row = self.constraint_values.len();
                for expr in constraint.expressions.iter().rev() {
                    let row_const = expr.constant;
                    self.constraints_matrix_builder
                    .add_row_from_iter(
                        expr.linear
                            .coefficients
                            .iter()
                            .map(|(&var, &coeff)| (var, -coeff)),
                    );
                    self.constraint_values.push(row_const);
                }

                self.cones.push(
                  SupportedConeT::SecondOrderConeT(dim)
                  );
                ConstraintReference { index: start_row }
            }
        }
    }

    fn name() -> &'static str {
        "Clarabel"
    }
}
```
