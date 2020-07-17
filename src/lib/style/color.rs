//! # `Color`

/// `Color`
///
/// Represents a terminal color. Defaults to `Transparent`. Convertible to
/// terminal escape sequence thanks to `termion`. No alpha (for now).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    /// Transparent,
    Transparent,
    /// Black,
    Black,
    /// Blue,
    Blue,
    /// Cyan,
    Cyan,
    /// Green,
    Green,
    /// Magenta,
    Magenta,
    /// Red,
    Red,
    /// White,
    White,
    /// Yellow,
    Yellow,
    /// Light black,
    LightBlack,
    /// Light blue,
    LightBlue,
    /// Light cyan,
    LightCyan,
    /// Light green,
    LightGreen,
    /// Light magenta,
    LightMagenta,
    /// Light red,
    LightRed,
    /// Light white,
    LightWhite,
    /// Light yellow,
    LightYellow,
    /* Rgb { r: u8, g: u8, b: u8 },
     * Rgba { r: u8, g: u8, b: u8, a: u8 }, */
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
