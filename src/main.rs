#![allow(non_snake_case)]

// extern crate time;
// use time::Instant;

mod args;
mod files;
mod matrixMacro;
mod simplex;
mod utils;

//use simplex::simplex;
use files::matFromFile;
use simplex::simp;

use args::SimplexArgs;
use clap::Parser;

fn main() {
    let args = SimplexArgs::parse();
    let path = args.path;

    let A = matFromFile(path.clone() + "/A.txt");
    let b = matFromFile(path.clone() + "/b.txt");
    let c = matFromFile(path.clone() + "/c.txt");

    let sol = simp(&A, &b, &c);

    println!("{:?}", sol);
}
