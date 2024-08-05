#[macro_use]
mod color;

mod model;
mod pallette;

use std::collections::HashMap;

mod dark;
mod light;

mod csnt;
mod util;

use dark::{set_dark_player_colors, set_dark_syntax_colors};
use light::{set_light_player_colors, set_light_syntax_colors};

use model::{IntoStyle, Style, Theme, ZedTheme};
use util::write_theme;

const DIST: &str = "./themes/neptune_theme.json";

fn main() {
    let mut theme = ZedTheme::default();
    theme.add_theme(create_dark_theme());
    theme.add_theme(create_light_theme());

    write_theme(DIST, theme);
}

fn create_dark_theme() -> Theme {
    let syntaxes = set_dark_syntax_colors();
    let players = set_dark_player_colors();
    let mut style = HashMap::<&str, Style>::new();

    style.insert("players", players.into_style());
    style.insert("syntax", syntaxes.into_style());

    Theme::new(String::from("Neptune Dark"), String::from("dark"), style)
}

fn create_light_theme() -> Theme {
    let syntaxes = set_light_syntax_colors();
    let players = set_light_player_colors();
    let mut style = HashMap::<&str, Style>::new();

    style.insert("players", players.into_style());
    style.insert("syntax", syntaxes.into_style());

    Theme::new(String::from("Neptune Light"), String::from("light"), style)
}
