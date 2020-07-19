use super::Color;
use std::fmt::{Display, Error, Formatter};

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
    /// Sets fields to None if the values are identical to the ones found in
    /// `before`
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

macro_rules! str {
    ($self:ident foreground) => {{
        $self.foreground.map_or("", |c| c.fg_str())
    }};
    ($self:ident background) => {{
        $self.background.map_or("", |c| c.bg_str())
    }};
    ($self:ident $str:ident) => {{
        $self.$str.map_or("", |x| str!(impl x $str))
    }};
    (impl $prop:ident bold) => {{ str!(impl $prop, Bold, NoBold) }};
    (impl $prop:ident italic) => {{ str!(impl $prop, Italic, NoItalic) }};
    (impl $prop:ident underline) => {{ str!(impl $prop, Underline, NoUnderline) }};
    (impl $prop:ident, $Attr:ident, $NoAttr:ident) => {{
        if $prop { str!(impl impl $Attr) } else { str!(impl impl $NoAttr) }
    }};
    (impl impl $Attr:ident) => {{
        <termion::style::$Attr as AsRef<str>>::as_ref(&termion::style::$Attr)
    }};
}

impl Display for ColorStyle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}{}{}",
            str!(self foreground),
            str!(self background),
            str!(self bold),
            str!(self italic),
            str!(self underline),
        )
    }
}

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::{Color, ColorStyle};
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

    #[test]
    fn display() {
        // All None
        {
            let color_style = ColorStyle {
                foreground: None,
                background: None,
                bold:       None,
                italic:     None,
                underline:  None,
            };

            assert_eq!(str!(color_style foreground), "", "None displays nothing");
            assert_eq!(str!(color_style background), "", "None displays nothing");
            assert_eq!(str!(color_style bold), "", "None displays nothing");
            assert_eq!(str!(color_style italic), "", "None displays nothing");
            assert_eq!(str!(color_style underline), "", "None displays nothing");
        }

        use termion::style::{Bold, Italic, NoBold, NoItalic, NoUnderline, Underline};
        let bold = <Bold as AsRef<str>>::as_ref(&Bold);
        let italic = <Italic as AsRef<str>>::as_ref(&Italic);
        let underline = <Underline as AsRef<str>>::as_ref(&Underline);
        let no_bold = <NoBold as AsRef<str>>::as_ref(&NoBold);
        let no_italic = <NoItalic as AsRef<str>>::as_ref(&NoItalic);
        let no_underline = <NoUnderline as AsRef<str>>::as_ref(&NoUnderline);

        // All Some(true)
        {
            let color_style = ColorStyle {
                foreground: Some(Color::Black),
                background: Some(Color::White),
                bold:       Some(true),
                italic:     Some(true),
                underline:  Some(true),
            };

            assert_eq!(
                str!(color_style foreground),
                Color::Black.fg_str(),
                "Black foreground"
            );
            assert_eq!(
                str!(color_style background),
                Color::White.bg_str(),
                "White background"
            );
            assert_eq!(str!(color_style bold), bold, "Bold");
            assert_eq!(str!(color_style italic), italic, "Italic");
            assert_eq!(str!(color_style underline), underline, "Underline");
        }

        // All Some(false)
        {
            let color_style = ColorStyle {
                foreground: Some(Color::Red),
                background: Some(Color::Green),
                bold:       Some(false),
                italic:     Some(false),
                underline:  Some(false),
            };

            assert_eq!(
                str!(color_style foreground),
                Color::Red.fg_str(),
                "Red foreground"
            );
            assert_eq!(
                str!(color_style background),
                Color::Green.bg_str(),
                "Green background"
            );
            assert_eq!(str!(color_style bold), no_bold, "Not bold");
            assert_eq!(str!(color_style italic), no_italic, "Not italic");
            assert_eq!(str!(color_style underline), no_underline, "Not underline");
        }
    }
}
