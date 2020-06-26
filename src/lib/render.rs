use crate::component::{Element, ElementObject};
use crate::style::{Defined, Size};
use crate::ui::UI;
use termion::terminal_size;

pub fn render<E: Element + 'static>(element: E) {
    let (width, height) = terminal_size().unwrap();
    let element: ElementObject = element.into();
    let mut ui: UI = element.into();
    // println!("{:#?}", ui);

    ui.render(width as usize, height as usize);

    // dbg!(size);
    // dbg!(ui.root.layout_node(|x| x.layout()));
    // for child in ui.root.borrow().children.iter() {
    // dbg!(child.layout_node(|x| x.layout()));
    // }
}
