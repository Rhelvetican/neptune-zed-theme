use crate::{color::Color, pallette::PHOTON};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SyntaxStyle {
    color: Color,
    font_style: Option<String>,
    font_weight: Option<u16>,
}

impl Default for SyntaxStyle {
    fn default() -> Self {
        Self {
            color: PHOTON,
            font_style: None,
            font_weight: None,
        }
    }
}

impl SyntaxStyle {
    /// Get the default variant.
    pub fn new() -> Self {
        Self::default()
    }

    /// Change the color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    /// Change font style.
    /// Empty strings are converted to
    pub fn set_font_style(&mut self, style: String) {
        if style.is_empty() {
            self.font_style = None
        } else {
            self.font_style = Some(style)
        }
    }

    pub fn set_font_weight(&mut self, weight: u16) {
        if weight == 0 {
            self.font_weight = None
        } else {
            self.font_weight = Some(weight)
        }
    }
}
