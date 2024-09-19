// Macro to create a matrix with the following syntax: matrix![1,2,3; 4,5,6; 7,8,9]
#[macro_export]
macro_rules! matrix {
    ( $( $( $x:expr ),* );* ) => {
        {
            let mut v = Vec::new();
            let mut rows = 0;
            let mut oldcols : i32 = -1;
            use nalgebra::DMatrix;

            $(
                let mut cols : i32 = 0;
                rows +=1;
                $(
                    v.push($x as f64);
                    cols+=1;
                )*
                assert!(oldcols == -1 || oldcols == cols, "Matrix must have the same number of columns in each row");
                oldcols = cols;
            )*
            DMatrix::from_row_slice(rows, cols as usize, &v)
        }
    };
}
