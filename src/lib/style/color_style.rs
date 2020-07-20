use super::Color;
use std::fmt::{Display, Error, Formatter};
use termion::style::*;

/// Visual (non-layout) styles of a `Component`.
///
/// `None` will inherit.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct ColorStyle {
    /// Foreground `Color`
    pub foreground: Option<Color>,
    /// Background `Color`
    pub background: Option<Color>,
    /// Bold text
    pub bold:       Option<bool>,
    /// Italic text
    pub italic:     Option<bool>,
    /// Underline text
    pub underline:  Option<bool>,
}

impl ColorStyle {
    /// Creates a default `ColorStyle` with `background` color
    pub fn with_background(background: Color) -> Self {
        Self {
            background: Some(background),
            ..Default::default()
        }
    }

    /// Sets fields to None if the values are identical to the corresponding
    /// ones in `before`
    pub fn simplify(&self, before: &Self) -> Self {
        macro_rules! simplify {
            ($prop:ident) => {{
                match (before.$prop, self.$prop) {
                    (Some(b), Some(s)) if b == s => None,
                    _ => self.$prop,
                }
            }};
        }

        Self {
            foreground: simplify!(foreground),
            background: simplify!(background),
            bold:       simplify!(bold),
            italic:     simplify!(italic),
            underline:  simplify!(underline),
        }
    }
}

macro_rules! strs {
    ($prop:ident : $Attr:ident, $NoAttr:ident) => {
        fn $prop(color_style: &ColorStyle) -> &'static str {
            match color_style.$prop {
                Some(true) => <$Attr as AsRef<str>>::as_ref(&$Attr),
                Some(false) => <$NoAttr as AsRef<str>>::as_ref(&$NoAttr),
                None => "",
            }
        }
    };
}

strs!(bold: Bold, NoBold);
strs!(italic: Italic, NoItalic);
strs!(underline: Underline, NoUnderline);

impl Display for ColorStyle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}{}{}",
            self.foreground.map_or("", |c| c.fg_str()),
            self.background.map_or("", |c| c.bg_str()),
            bold(self),
            italic(self),
            underline(self),
        )
    }
}

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn simplify() {
        let color_style_before = ColorStyle {
            foreground: None,
            background: Some(Color::Black),
            bold:       Some(true),
            italic:     Some(true),
            underline:  None,
        };
        let color_style = ColorStyle {
            foreground: None,
            background: Some(Color::Black),
            bold:       Some(false),
            italic:     None,
            underline:  Some(true),
        };
        // Same as color_style...
        let expected = ColorStyle {
            foreground: None,
            background: None, // ...except for simplification here
            bold:       Some(false),
            italic:     None,
            underline:  Some(true),
        };

        assert_eq!(
            color_style.simplify(&color_style_before),
            expected,
            "Simplifies background"
        );
    }
}
