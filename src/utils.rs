use nalgebra::DMatrix;

// Given a matrix A and a vector of rows, return a new matrix with only the rows in the vector
pub fn takeRows(A: &DMatrix<f64>, rows: &Vec<usize>) -> DMatrix<f64> {
    let mut res = DMatrix::<f64>::zeros(rows.len(), A.ncols());
    for i in 0..rows.len() {
        res.set_row(i, &A.row(rows[i]));
    }
    res
}

/*
pub fn pmat(m: &DMatrix<f64>) {
    let mut s = String::new();
    let rows = m.nrows();
    let cols = m.ncols();
    s.push_str("\n┌");

    for _col in 0..cols + 1 {
        s.push_str("\t");
    }
    s.push_str("┐");
    for row in 0..rows {
        s.push_str("\n│\t");
        for col in 0..cols {
            s.push_str(&format!("{:.1}\t", m[(row, col)]));
        }
        s.push_str("│");
    }
    s.push_str("\n└");
    for _col in 0..cols + 1 {
        s.push_str("\t");
    }
    s.push_str("┘");
    println!("{}", s);
}
 */
