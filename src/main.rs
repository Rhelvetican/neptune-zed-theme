#[macro_use]
mod color;
mod model;
mod pallette;

use std::collections::HashMap;

use color::Color;
use jsonutils::file::write_json;
use model::{PlayerColor, SyntaxStyle};

const DIST: &str = "./themes/neptune_theme.json";

fn main() {
    let mut theme = HashMap::<String, Color>::new();
    theme.insert(format!("skibidi_{}", 72), color!(72, 255, 172));
    write_json(DIST, theme).unwrap();
}
