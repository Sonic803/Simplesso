use clap::Parser;
const ABOUT: &str = "
An implementation of the simplex algorithm in Rust, it solves problems of the form:

⎰ min b'*y
| y'*A = c 
⎱ y >= 0

and when a solution exists it solves also:

⎰ max c'*x
⎱ A*x <= b";

#[derive(Parser, Debug)]
#[clap(author="Antonio Ciociola", version="1.0.0", about=ABOUT)]
pub struct SimplexArgs {
    /// The path to the folder where are stored the files A.txt, b.txt, c.txt
    //#[arg(short, long)]
    pub path: String,
}
