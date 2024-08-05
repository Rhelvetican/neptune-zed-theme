use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PlayerColor {
    cursor: String,
    selection: String,
    background: String,
}
