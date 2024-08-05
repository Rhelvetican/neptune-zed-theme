use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::Serialize;

use crate::model::{IntoStyle, Style};

#[derive(Serialize)]
#[serde(untagged)]
pub enum Color {
    Rgb(Rgb),
    Rgba(Rgba),
}

impl IntoStyle for Color {
    fn into_style(self) -> Style {
        Style::Color(self)
    }
}

impl Color {
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }

    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba(Rgba::new(r, g, b, a))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Color::Rgb(rgb) => rgb.fmt(f),
            Color::Rgba(rgba) => rgba.fmt(f),
        }
    }
}

pub fn to_hex(n: u8) -> String {
    let tmp = format!("{:#04x}", n);
    String::from(&tmp[2..])
}

/// Struct used to represent Rbg color.
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "#{}{}{}", to_hex(self.r), to_hex(self.g), to_hex(self.b))
    }
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Struct used to represent Rbga color.
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "#{}{}{}{}",
            to_hex(self.r),
            to_hex(self.g),
            to_hex(self.b),
            to_hex(self.a)
        )
    }
}

impl Serialize for Rgba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Macro to make a color.
#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::color::Color::new_rgb($r as u8, $g as u8, $b as u8)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        $crate::color::Color::new_rgba($r as u8, $g as u8, $b as u8, $a as u8)
    };
}
