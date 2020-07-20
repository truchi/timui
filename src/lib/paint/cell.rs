use crate::style::{Color, ColorStyle};
use std::fmt::{Display, Error, Formatter};

/// A terminal `Cell`. Holds a `char` and `ColorStyle`.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Cell {
    /// The `char`
    char:  char,
    /// Styles
    style: ColorStyle,
}

impl Cell {
    /// Creates a `Cell`
    pub fn new(char: char, style: ColorStyle) -> Self {
        Self { char, style }
    }

    /// Creates a default `Cell` with `background` color
    pub fn with_background(background: Color) -> Self {
        Self {
            char:  ' ',
            style: ColorStyle::with_background(background),
        }
    }

    pub fn merge(below: &Self, above: &Self) -> Self {
        if above.has_background() {
            return *above;
        }

        if above.has_foreground() {
            let mut merged = *above;
            merged.style.background = below.style.background;

            return merged;
        }

        *below
    }

    pub fn has_foreground(&self) -> bool {
        self.char != ' ' && self.style.foreground.is_some()
    }

    pub fn has_background(&self) -> bool {
        self.style.background.is_some()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}", self.style, self.char)
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
    fn has_foreground() {
        assert_eq!(
            Cell {
                char:  ' ',
                style: ColorStyle {
                    foreground: None,
                    ..Default::default()
                },
            }
            .has_foreground(),
            false
        );
        assert_eq!(
            Cell {
                char:  'a',
                style: ColorStyle {
                    foreground: None,
                    ..Default::default()
                },
            }
            .has_foreground(),
            false
        );
        assert_eq!(
            Cell {
                char:  ' ',
                style: ColorStyle {
                    foreground: Some(Color::Red),
                    ..Default::default()
                },
            }
            .has_foreground(),
            false
        );
        assert_eq!(
            Cell {
                char:  'a',
                style: ColorStyle {
                    foreground: Some(Color::Green),
                    ..Default::default()
                },
            }
            .has_foreground(),
            true
        );
    }

    #[test]
    fn has_background() {
        assert_eq!(
            Cell {
                char:  'a',
                style: ColorStyle {
                    background: Some(Color::Green),
                    ..Default::default()
                },
            }
            .has_background(),
            true
        );
        assert_eq!(
            Cell {
                char:  'b',
                style: ColorStyle {
                    background: None,
                    ..Default::default()
                },
            }
            .has_background(),
            false
        );
    }

    #[test]
    fn merge() {
        let below = Cell {
            char:  'z',
            style: ColorStyle {
                foreground: Some(Color::Red),
                background: Some(Color::Green),
                ..Default::default()
            },
        };
        let has_background = Cell {
            char:  ' ',
            style: ColorStyle {
                background: Some(Color::Green),
                ..Default::default()
            },
        };

        assert_eq!(
            Cell::merge(&below, &has_background),
            has_background,
            "Merged is same as above when above has background"
        );

        let has_foreground = Cell {
            char:  'r',
            style: ColorStyle {
                foreground: Some(Color::Blue),
                ..Default::default()
            },
        };

        assert_eq!(
            Cell::merge(&below, &has_foreground).style.background,
            below.style.background,
            "Merged has below's background when above has no background"
        );
        assert_eq!(
            {
                let mut merged = Cell::merge(&below, &has_foreground);
                merged.style.background = has_foreground.style.background;

                merged
            },
            has_foreground,
            "Merged is same as above (except for background) when above has no background"
        );

        let default: Cell = Default::default();

        assert_eq!(
            Cell::merge(&below, &default),
            below,
            "Merged is same as below when above has neither foreground nor background"
        );
    }
}
