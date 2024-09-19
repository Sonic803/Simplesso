use nalgebra::DMatrix;
use std::fs::File;
use std::io::Read;

// Genereate a matrix from a file, the file must be in the format: 1,2,3\n 4,5,6\n 7,8,9\n
pub fn matFromFile(path: String) -> DMatrix<f64> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("File {path} does not exist"),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut arr: Vec<f64> = Vec::new();
    let mut rows = 0;
    let mut elements = 0;
    for line in contents.lines() {
        for num in line.split(",") {
            arr.push(num.parse::<f64>().unwrap());
            elements += 1;
        }
        rows += 1;
    }
    let mat = DMatrix::from_row_slice(rows, elements / rows, &arr);
    mat
}
