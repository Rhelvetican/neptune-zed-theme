mod player;
mod syntax;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

pub use player::PlayerColor;
use serde::Serialize;
pub use syntax::SyntaxStyle;

use crate::color::Color;

#[derive(Serialize)]
pub struct Theme {
    name: String,
    appearance: Appearance,
    style: HashMap<&'static str, Style>,
}

impl Theme {
    pub fn new(name: String, app: String, style: HashMap<&'static str, Style>) -> Self {
        Self {
            name,
            appearance: app.parse().unwrap_or(Appearance::Dark),
            style,
        }
    }
}

#[derive(Debug)]
pub struct AppErr;

impl Display for AppErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "error parsing appearance from string")
    }
}

impl Error for AppErr {}

#[derive(Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Appearance {
    Light,
    Dark,
}

impl FromStr for Appearance {
    type Err = AppErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "l" | "light" => Ok(Self::Light),
            "d" | "dark" => Ok(Self::Dark),
            _ => Err(AppErr),
        }
    }
}

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
