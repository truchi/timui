//! # `ColorStyleInherit`

use super::{Color, ColorStyle};

/// Represents color-related styles of a `Component`, with inheritance. `None`
/// for inheritance from parent.
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorStyleInherit {
    /// Foreground `Color`, `None` for inheritance
    pub foreground: Option<Color>,
    /// Background `Color`, not inheritable
    pub background: Color,
    /// Bold text, `None` for inheritance
    pub bold:       Option<bool>,
    /// Italic text, `None` for inheritance
    pub italic:     Option<bool>,
    /// Underline text, not inheritable
    pub underline:  bool,
}

// impl ColorStyleInherit {
// /// Inherits this style from a `ColorStyle`
// pub fn inherit(&self, color_style: ColorStyle) -> ColorStyle {
// ColorStyle {
// foreground: self.foreground.unwrap_or(color_style.foreground),
// background: self.background,
// bold:       self.bold.unwrap_or(color_style.bold),
// italic:     self.italic.unwrap_or(color_style.italic),
// underline:  self.underline,
// }
// }
// }
