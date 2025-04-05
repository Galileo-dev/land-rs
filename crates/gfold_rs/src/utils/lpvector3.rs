use good_lp::{constraint, Constraint, Expression, Variable};
use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct LpVector3(Vector3<Variable>);

#[derive(Debug, Clone)]
pub struct LpVector3Expr(pub Vector3<Expression>);

impl LpVector3 {
    pub fn from_vec(vec: Vec<Variable>) -> Self {
        assert_eq!(vec.len(), 3, "Expected a vector of length 3");
        // Convert the Vec into a nalgebra Vector3
        LpVector3(Vector3::new(vec[0].clone(), vec[1].clone(), vec[2].clone()))
    }

    pub fn from_vector(vec: Vector3<Variable>) -> Self {
        LpVector3(vec)
    }

    pub fn eq(self, rhs: LpVector3Expr) -> Vec<Constraint> {
        let mut constraints = Vec::with_capacity(3);
        for i in 0..3 {
            constraints.push(constraint!(self.0[i] == rhs.0[i].clone()));
        }
        constraints
    }
}

impl LpVector3Expr {
    pub fn from_array(arr: [f64; 3]) -> Self {
        LpVector3Expr(Vector3::new(
            Expression::from(arr[0]),
            Expression::from(arr[1]),
            Expression::from(arr[2]),
        ))
    }
}

impl std::ops::Sub for LpVector3 {
    type Output = LpVector3Expr;
    fn sub(self, rhs: LpVector3) -> LpVector3Expr {
        let expr0 = Expression::from(self.0[0]) - Expression::from(rhs.0[0]);
        let expr1 = Expression::from(self.0[1]) - Expression::from(rhs.0[1]);
        let expr2 = Expression::from(self.0[2]) - Expression::from(rhs.0[2]);
        LpVector3Expr(Vector3::new(expr0, expr1, expr2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use good_lp::variables;

    #[test]
    fn test_lpvector3_subtraction_and_eq() {
        let mut vars = variables!();

        let a = vars.add_variable();
        let b = vars.add_variable();
        let c = vars.add_variable();

        let d = vars.add_variable();
        let e = vars.add_variable();
        let f = vars.add_variable();

        let vec1 = Vector3::new(a, b, c);
        let vec2 = Vector3::new(d, e, f);

        let lpvec1 = LpVector3::from_vector(vec1);
        let lpvec2 = LpVector3::from_vector(vec2);

        let lp_expr = lpvec1.clone() - lpvec2.clone();

        let constraints = lpvec1.eq(lp_expr);

        assert_eq!(constraints.len(), 3);
    }
}
