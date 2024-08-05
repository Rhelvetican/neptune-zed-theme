mod player;
mod syntax;

use std::collections::HashMap;

pub use player::PlayerColor;
use serde::Serialize;
pub use syntax::SyntaxStyle;

use crate::color::Color;

pub trait IntoStyle {
    fn into_style(self) -> Style;
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Style {
    Color(Color),
    Player(Vec<PlayerColor>),
    Syntax(HashMap<&'static str, SyntaxStyle>),
}

impl<S: IntoStyle> From<S> for Style {
    fn from(value: S) -> Self {
        IntoStyle::into_style(value)
    }
}
