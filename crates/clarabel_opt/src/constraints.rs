use bon::Builder;

#[derive(Debug, Clone, Copy)]
struct Equality {}

#[derive(Debug, Clone, Copy)]
struct Inequality {
    slack: f64,
}
#[derive(Debug, Clone, Copy)]
struct SecondOrderCone {
    cone_dimension: usize,
}

#[derive(Debug, Clone, Copy)]
enum ConstraintKind {
    EqualityConstraint(Equality),
    InequalityConstraint(Inequality),
    SecondOrderConeConstraint(SecondOrderCone),
}

#[derive(Builder)]
struct Constraints<T>
where
    T: Into<ConstraintKind>,
{
    terms: Vec<(usize, f64)>,
    rhs: f64,
    #[builder(into)]
    kind: T,
}

#[cfg(test)]
mod tests {
    use super::{Constraints, Equality};

    #[test]
    fn test_builder() {
        let constraints = Constraints::builder().terms(bon::vec![(1, 1.0)]).kind();
    }
}
