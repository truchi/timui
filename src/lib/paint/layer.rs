use crate::style::Color;

#[derive(Default)]
pub struct Cell {
    foreground: Color,
    background: Color,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    c: char,
}

#[derive(Default)]
pub struct Surface {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    vec: Vec<Cell>,
}

#[derive(Default)]
pub struct Uniform {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    cell: Cell,
}

pub enum Layer {
    Uniform(Uniform),
    Surface(Surface),
}

impl Default for Layer {
    fn default() -> Self {
        Self::Uniform(Default::default())
    }
}

impl Layer {}
