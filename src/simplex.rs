use crate::utils::takeRows;
use nalgebra::DMatrix;

pub enum Solution {
    Optimal(f64, DMatrix<f64>, DMatrix<f64>, Vec<usize>),
    Unbounded,
    Empty,
    Error(String),
}

impl std::fmt::Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Solution::Optimal(val, x, c, B) => write!(
                f,
                "Optimal solution found: \n\nValue: {} \n\ny: {} x: {} B: {:?} \n",
                val, x, c, B
            ),
            Solution::Unbounded => write!(f, "Unbounded solution found"),
            Solution::Empty => write!(f, "The problem has no feasible solution"),
            Solution::Error(s) => write!(f, "Error: {}", s),
        }
    }
}

// Simplex algorithm
pub fn simplex(A: &DMatrix<f64>, b: &DMatrix<f64>, c: &DMatrix<f64>, B: &Vec<usize>) -> Solution {
    // Assert that the input is correct
    assert!(A.nrows() == b.nrows());
    assert!(A.ncols() == c.nrows());
    assert!(A.ncols() == B.len());
    assert!(c.ncols() == 1);
    assert!(b.ncols() == 1);
    for i in 0..B.len() {
        assert!(B[i] < A.nrows() as usize);
    }

    let eqs = A.nrows();
    let vars = A.ncols();

    // Generate N
    let N = &mut vec![0; eqs - vars];
    let mut j = 0;
    for i in 0..eqs {
        if !B.iter().any(|&x| x == i as usize) {
            N[j] = i;
            j += 1;
        }
    }

    let Ab = &takeRows(A, B);
    let An = &takeRows(A, N);
    let bB = &takeRows(b, B);
    let bN = &takeRows(b, N);
    let AbInv: DMatrix<f64>;
    match Ab.clone().try_inverse() {
        Some(inv) => {
            AbInv = inv;
        }
        None => {
            return Solution::Error("Matrix A is not invertible".to_string());
        }
    }
    let W = &-AbInv.clone();

    let yB = -c.transpose() * W;
    let y = &mut DMatrix::<f64>::zeros(A.nrows(), 1);
    for i in 0..vars {
        y[(B[i] as usize, 0)] = yB[(0, i)]
    }
    // If there is a negative component of y, B is not an admissible base
    if y.iter().any(|&x| x < 0.0) {
        return Solution::Error("Base is not admissible".to_string());
    }
    let v = (y.transpose() * b)[(0, 0)];
    let x = &(-W * bB);

    // Check if the Solution is optimal (it is admissible in the primal problem) bn - An * x >= 0
    // If it is not, find the entering index k (Bland)
    let diseq = &(bN - An * x);

    if diseq >= &DMatrix::<f64>::zeros(diseq.nrows(), 1) {
        return Solution::Optimal(v, y.clone(), x.clone(), B.clone());
    }
    let mut k: usize = 0;
    for i in 0..diseq.nrows() {
        if diseq[(i, 0)] < 0.0 {
            k = N[i];
            break;
        }
    }

    // Check if the problem is unbounded
    // If it isn't, find the exiting index h (Bland)

    let mut minr: f64 = std::f64::INFINITY;
    let mut _h: usize = 0;
    let mut hIndex: usize = 0;

    for i in 0..vars {
        let index = B[i];
        let prod = (A.row(k) * W.column(i))[(0, 0)];
        if prod < 0.0 {
            let r = -y[(index, 0)] / prod;
            if r < minr {
                minr = r;
                _h = index;
                hIndex = i;
            }
        }
    }
    if minr == std::f64::INFINITY {
        return Solution::Unbounded;
    }

    let mut newB = B.clone();
    newB[hIndex] = k;
    //sort newB
    newB.sort();
    return simplex(A, b, c, &newB);
}

// Generate the auxiliar problem and solve it
fn auxSimplex(A: &DMatrix<f64>, b: &DMatrix<f64>, c: &DMatrix<f64>) -> Solution {
    // Assert that the input is correct
    assert!(A.nrows() == b.nrows());
    assert!(A.ncols() == c.nrows());
    assert!(c.ncols() == 1);
    assert!(b.ncols() == 1);

    let nrows = A.nrows();
    let ncols = A.ncols();

    // Invert the sign of equations with negative c so that (0,c) is a vertex

    let A = &mut A.clone();
    let c = &mut c.clone();
    for i in 0..c.nrows() {
        if c[(i, 0)] < 0.0 {
            c[(i, 0)] = -c[(i, 0)];
            let _ = A.set_column(i, &-A.column(i));
        }
    }
    //add slack variables to A, set b to 0,1
    let A = &mut A.clone().insert_rows(nrows, ncols, 0.);
    for i in 0..ncols {
        A[(nrows + i, i)] = 1.0;
    }
    let b = &mut DMatrix::<f64>::from_element(nrows + ncols, 1, 0.0);
    for i in nrows..nrows + ncols {
        b[(i, 0)] = 1.0;
    }

    let mut B = Vec::<usize>::new();
    for i in 0..ncols {
        B.push(nrows + i);
    }

    return simplex(A, b, c, &B);
}

// Solve the problem using the simplex algorithm, starting from a feasible base calculated with the auxiliar problem
pub fn simp(A: &DMatrix<f64>, b: &DMatrix<f64>, c: &DMatrix<f64>) -> Solution {
    let aux = auxSimplex(A, b, c);
    //The aux problem has always a Solution if there wasn't an error
    let (val, y, _x, Base) = match aux {
        Solution::Optimal(val, y, x, Base) => (val, y, x, Base),
        Solution::Error(s) => return Solution::Error(s),
        _ => panic!("auxSimplex returned something different from Optimal or Error"),
    };

    //if aux has Solution v != 0, then the original problem is infeasible
    if val != 0.0 {
        return Solution::Empty;
    }

    //trasformare le parti della base con variabili di scarto (degenere) in variabili di y
    let mut B = Base.clone();
    let mut zeri = Vec::<usize>::new();
    for i in 0..A.nrows() {
        if y[(i, 0)] == 0.0 && !B.contains(&i) {
            zeri.push(i);
        }
    }
    for i in 0..B.len() {
        if B[i] >= A.nrows() {
            B[i] = zeri.pop().unwrap();
        }
    }

    return simplex(A, b, c, &B);
}
