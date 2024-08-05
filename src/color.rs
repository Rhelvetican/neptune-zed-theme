use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::Serialize;

fn to_hex(n: u8) -> String {
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
