use nalgebra::constraint;

use super::flatten::fast_flatten_vecs;

struct CscMatrixBuilder {
    /// Indicates the row index of the corresponding element in `nzval`
    rowval: Vec<Vec<usize>>,
    /// All non-zero values in the matrix, in column-major order
    nzval: Vec<Vec<f64>>,
    n_rows: usize,
    n_cols: usize,
}

impl CscMatrixBuilder {
    fn new(n_cols: usize) -> Self {
        Self {
            rowval: vec![Vec::new(); n_cols],
            nzval: vec![Vec::new(); n_cols],
            n_rows: 0,
            n_cols,
        }
    }
    fn add_row(&mut self, row: constraint) {
        for (var, value) in row.linear_coefficients() {
            self.rowval[var.index()].push(self.n_rows);
            self.nzval[var.index()].push(value);
        }
        self.n_rows += 1;
    }
    fn build(self) -> clarabel::algebra::CscMatrix {
        let mut colptr = Vec::with_capacity(self.n_cols + 1);
        colptr.push(0);
        for col in &self.rowval {
            colptr.push(colptr.last().unwrap() + col.len());
        }
        clarabel::algebra::CscMatrix::new(
            self.n_rows,
            self.n_cols,
            colptr,
            fast_flatten_vecs(self.rowval),
            fast_flatten_vecs(self.nzval),
        )
    }
}
