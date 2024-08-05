// Defining colors.

use crate::color::Color;

/// Invisible.
pub const PHOTON: Color = color!(0, 0, 0, 0);

pub mod dark {
    use crate::color::Color;
    /// Dark theme foreground.
    pub const OCEAN: Color = color!(6, 9, 61);
    /// Dark theme background.
    pub const SEA_FLOOR: Color = color!(5, 8, 54);
    /// Dark theme comment.
    pub const DEEP_EMERALD: Color = color!(21, 87, 36);
}

pub mod light {
    use crate::color::Color;
    /// Light theme background.
    pub const QUARTZ: Color = color!(252, 252, 252);
    /// Light theme foreground.
    pub const ASHY_QUARTZ: Color = color!(144, 144, 156);
}

/// Outline color.
pub const ENGRAVE: Color = color!(127, 127, 127);

/// Red.
pub const RUBY: Color = color!(250, 95, 67);
/// Green.
pub const EMERALD: Color = color!(47, 245, 113);
/// Blue.
pub const SAPPHIRE: Color = color!(55, 108, 255);
/// Orange.
pub const CARNELIAN: Color = color!(245, 164, 37);
/// Cyan.
pub const APATITE: Color = color!(58, 247, 175);
