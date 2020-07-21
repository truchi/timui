//! # `Canvas`

use super::{Cell, Layer, Rect};
use crate::style::{Color, ColorStyle};
use std::{
    fmt::{Display, Error, Formatter},
    rc::Rc,
};

/// A positioned rectangle of `Cell`s
#[derive(Default, Debug)]
pub struct Canvas {
    /// The `Rect` bounds of the `Layer`
    rect: Rect,
    /// The `Cell`s filling the `Layer`
    vec:  Vec<Cell>,
}

impl Layer for Canvas {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn get(&self, x: u16, y: u16) -> &Cell {
        debug(self.rect, x, y);
        self.vec
            .get((self.rect.w * y + x) as usize)
            .expect("x and/or y out of bounds")
    }

    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        debug(self.rect, x, y);
        self.vec
            .get_mut((self.rect.w * y + x) as usize)
            .expect("x and/or y out of bounds")
    }
}

impl Canvas {
    /// Creates a `Canvas` filled with `color`ed background `Cell`s
    pub fn with_background(rect: impl Into<Rect>, color: Color) -> Self {
        let rect = rect.into();
        let mut vec = Vec::with_capacity((rect.w * rect.h) as usize);

        for _ in 0..(rect.w * rect.h) {
            vec.push(Cell::with_background(color));
        }

        Self { rect, vec }
    }

    /// Merges `above` above this `Canvas`
    pub fn above(&mut self, above: &impl Layer) {
        let Rect { x, y, w, h } = self.rect.clip(&above.rect());
        let dx = x - self.rect.x;
        let dy = y - self.rect.y;

        for i in 0..w {
            for j in 0..h {
                let cell = self.get_mut(i + dx, j + dy);
                *cell = Cell::merge(cell, &above.get(i, j));
            }
        }
    }

    /// Merges `below` below this `Canvas`
    pub fn below(&mut self, below: &impl Layer) {
        let Rect { x, y, w, h } = self.rect.clip(&below.rect());
        let dx = x - self.rect.x;
        let dy = y - self.rect.y;

        for i in 0..w {
            for j in 0..h {
                let cell = self.get_mut(i + dx, j + dy);
                *cell = Cell::merge(&below.get(i, j), cell);
            }
        }
    }
}

impl<T: Into<Rect>> From<(T, Cell)> for Canvas {
    fn from((rect, cell): (T, Cell)) -> Self {
        let empty = Cell::new(' ', cell.style);
        let rect = rect.into();
        let width = rect.w as usize;
        let height = rect.h as usize;
        let size = width * height;

        let mut vec = Vec::with_capacity(size);
        if size > 0 {
            vec.push(cell);
            (1..size).for_each(|_| vec.push(empty));
        }

        Self { rect, vec }
    }
}

impl<T: Into<Rect>> From<(T, ColorStyle, Rc<String>)> for Canvas {
    fn from((rect, style, string): (T, ColorStyle, Rc<String>)) -> Self {
        let empty = Cell::new(' ', style);
        let rect = rect.into();
        let width = rect.w as usize;
        let height = rect.w as usize;
        let size = width * height;
        let strs = textwrap::Wrapper::new(width)
            .break_words(false)
            .wrap(&string[..]);
        let lines = strs.len();

        let mut vec = Vec::with_capacity(size);
        for str in strs.iter() {
            let len = str.len();
            let cells = str
                .chars() // TODO Not "real" chars
                .map(|char| Cell::new(char, style));

            cells.for_each(|cell| vec.push(cell));
            (len..width).for_each(|_| vec.push(empty));
        }

        (width * lines..size).for_each(|_| vec.push(empty));

        Self { rect, vec }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Ok(for cell in self.vec.iter() {
            cell.fmt(f).expect("Cell format error");
        })
    }
}

fn debug(rect: Rect, x: u16, y: u16) {
    debug_assert!(x >= rect.x, "`x` ({}) must be >= `rect.x` ({})", x, rect.x);
    debug_assert!(
        x <= rect.x + rect.w,
        "`x` ({}) must be <= `rect.x + rect.w` ({})",
        x,
        rect.x + rect.w
    );
    debug_assert!(y >= rect.y, "`y` ({}) must be >= `rect.y` ({})", y, rect.y);
    debug_assert!(
        y <= rect.y + rect.h,
        "`y` ({}) must be <= `rect.y + rect.h` ({})",
        y,
        rect.y + rect.h
    );
}

// ========================================================================= //
//                                   TESTS                                   //
// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    fn canvas() -> (Vec<Cell>, Canvas, Canvas) {
        let bottom = Cell::new(' ', ColorStyle {
            background: Some(Color::Black),
            ..Default::default()
        });
        let red_background = Cell::new(' ', ColorStyle {
            background: Some(Color::Red),
            ..Default::default()
        });
        let green_foreground = Cell::new('a', ColorStyle {
            foreground: Some(Color::Green),
            ..Default::default()
        });
        let default = Default::default();

        let expected = vec![
            red_background,
            {
                let mut cell = *&green_foreground;
                cell.style.background = Some(Color::Black);
                cell
            },
            bottom,
            bottom,
        ];

        (
            expected,
            Canvas {
                rect: (0, 0, 4, 1).into(),
                vec:  [bottom; 4].into(),
            },
            Canvas {
                rect: (0, 0, 3, 1).into(),
                vec:  vec![red_background, green_foreground, default],
            },
        )
    }

    #[test]
    fn above() {
        let (expected, mut below, above) = canvas();

        assert_eq!(
            {
                below.above(&above);
                below.vec
            },
            expected
        );
    }

    #[test]
    fn below() {
        let (expected, below, mut above) = canvas();

        assert_eq!(
            {
                above.below(&below);
                above.vec
            },
            &expected[..3]
        );
    }
}
