/// A convex optimisation problem modeler that is easy to use, performant with large problems, and well-typed.
/// Aimed at modeling convex optimisation problems based on [good_lp](https://docs.rs/good_lp/latest/good_lp/)
/// The main difference is support of discretization and second order cone constraints
mod constraints;
mod expression;
mod flatten;
mod matrix;
