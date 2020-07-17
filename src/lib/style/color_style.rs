//! # `ColorStyle`

use super::Color;

/// Represents color-related styles of a `Component`
#[derive(Clone, Copy, Default, Debug)]
pub struct ColorStyle {
    /// Foreground `Color`
    pub foreground: Color,
    /// Background `Color`
    pub background: Color,
    /// Bold text
    pub bold:       bool,
    /// Italic text
    pub italic:     bool,
    /// Underline text
    pub underline:  bool,
}
