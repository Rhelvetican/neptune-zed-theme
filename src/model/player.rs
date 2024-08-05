use serde::Serialize;

use crate::{color::Color, pallette::PHOTON};

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
