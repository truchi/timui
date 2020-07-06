//! # `ColorStyleInherited`

use super::Color;

/// Represents color-related styles of a `Component`
#[derive(Clone, Copy, Debug)]
pub struct ColorStyleInherited {
    /// Foreground `Color`
    pub foreground: Color,
    /// Background `Color`
    pub background: Color,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underline text
    pub underline: bool,
}

impl Default for ColorStyleInherited {
    /// `Color::White` foreground, `Color::Black` background
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
