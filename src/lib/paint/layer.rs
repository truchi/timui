use crate::component::{Size, Style};

#[derive(Default, Debug)]
pub struct Cell {
    style: Style,
    c: char,
}

#[derive(Default, Debug)]
pub struct Layer {
    area: Vec<Vec<Cell>>,
}

impl Layer {
    pub fn with_capacity(Size { width, height }: Size<usize>) -> Self {
        let area = Vec::with_capacity(width);

        Self { area }
    }
}
