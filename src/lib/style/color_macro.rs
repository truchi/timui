//! # Macro for `Color` methods

/// Macro for `Color` methods
macro_rules! derive_colors {
    ($($color:ident,)*) => {
        impl Color {
            /// Returns the foreground escape sequence for this `Color`
            pub fn fg_str(&self) -> &'static str {
                use termion::color;
                match self {
                    Self::Transparent => "",
                    $(Self::$color => color::$color.fg_str(),)*
                }
            }

            /// Returns the background escape sequence for this `Color`
            pub fn bg_str(&self) -> &'static str {
                use termion::color;
                match self {
                    Self::Transparent => "",
                    $(Self::$color => color::$color.bg_str(),)*
                }
            }
        }
    };
}
