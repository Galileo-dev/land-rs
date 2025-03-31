#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A variable in a problem. Use variables to create [expressions](Expression),
/// to express the [objective](ProblemVariables::optimise)
/// and the [Constraints](crate::Constraint) of your model.
///
/// Variables are created using [ProblemVariables::add]. They implement std::ops basic math operations on f64 and i32 values.
///
/// ## Warning
/// `Eq` is implemented on this type, but
/// `v1 == v2` is true only if the two variables represent the same object,
/// not if they have the same definition.
///
/// ```
/// # use clarabel_opt::{variable, variables};
/// let mut vars = variables!();
/// let v1 = vars.add(variable().min(1).max(8));
/// let v2 = vars.add(variable().min(1).max(8));
/// assert_ne!(v1, v2);
///
/// let v1_copy = v1;
/// assert_eq!(v1, v1_copy);
/// ```
#[derive(Debug,
pub struct Variable {
    /// A variable is nothing more than an index into the `variables` field of a ProblemVariables
    /// That's why it can be `Copy`.
    /// All the actual information about the variable (name, type, bounds, ...) is stored in ProblemVariables
    index: usize,
}
