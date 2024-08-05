use serde::Serialize;

use crate::{color::Color, pallette::PHOTON};

use super::{IntoStyle, Style};

#[derive(Serialize)]
pub struct PlayerColor {
    cursor: Color,
    selection: Color,
    background: Color,
}

impl Default for PlayerColor {
    fn default() -> Self {
        Self {
            cursor: PHOTON,
            selection: PHOTON,
            background: PHOTON,
        }
    }
}

impl IntoStyle for Vec<PlayerColor> {
    fn into_style(self) -> Style {
        Style::Player(self)
    }
}

impl PlayerColor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_cursor(&mut self, cursor: Color) {
        self.cursor = cursor
    }

    pub fn set_selection(&mut self, selection: Color) {
        self.selection = selection
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.background = bg
    }
}
