//! # `ColorStyle`

use super::{Color, ColorStyleInherited};

/// Represents color-related styles of a `Component`, with inheritance. `None`
/// for inheritance from parent.
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorStyle {
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

impl ColorStyle {
    /// Inherits this style from another
    pub fn inherit(&self, inherited: ColorStyleInherited) -> ColorStyleInherited {
        ColorStyleInherited {
            foreground: self.foreground.unwrap_or(inherited.foreground),
            background: self.background,
            bold:       self.bold.unwrap_or(inherited.bold),
            italic:     self.italic.unwrap_or(inherited.italic),
            underline:  self.underline,
        }
    }
}
