use color::to_hex;

#[macro_use]
mod color;
mod model;
mod pallette;

const DIST: &str = "./themes";

fn main() {
    let col = color!(72, 255, 255);
    println!("{}", col)
}
