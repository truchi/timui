//! # `Color`

/// `Color`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Transparent,
    Magenta,
    Red,
    // Rgb { r: u8, g: u8, b: u8 },
    // Rgba { r: u8, g: u8, b: u8, a: u8 },
    White,
    Yellow,
}

impl Default for Color {
    /// `Color::Transparent`
    fn default() -> Self {
        Self::Transparent
    }
}

derive_colors!(
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow,
);
