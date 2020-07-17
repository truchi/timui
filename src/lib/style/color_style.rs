//! # `ColorStyle`

use super::Color;
use std::fmt::{Display, Error, Formatter};

/// Represents color-related styles of a `Component`. `None` will inherit.
#[derive(Copy, Clone, Default, Debug)]
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
    pub fn merge(&self, before: &Self) -> Self {
        macro_rules! merge {
            ($prop:ident) => {{
                match (before.$prop, self.$prop) {
                    (Some(b), Some(s)) if b == s => None,
                    _ => self.$prop,
                }
            }};
        }

        Self {
            foreground: merge!(foreground),
            background: merge!(background),
            bold:       merge!(bold),
            italic:     merge!(italic),
            underline:  merge!(underline),
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
