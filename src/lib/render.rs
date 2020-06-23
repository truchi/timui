use crate::component::{Element, ElementObject};
use crate::style::{Defined, Size};
use crate::ui::UI;
use termion::terminal_size;

pub fn render<E: Element + 'static>(element: E) {
    let (width, height) = terminal_size().unwrap();
    let size = Size {
        width: Defined(width as f32),
        height: Defined(height as f32),
    };
    // let size = Size {
    // width: Number::Defined(100.0),
    // height: Number::Defined(100.0),
    // };
    let element: ElementObject = element.into();
    let mut ui: UI = element.into();
    // println!("{:#?}", ui);

    ui.render(size);

    // dbg!(size);
    // dbg!(ui.root.layout_node(|x| x.layout()));
    // for child in ui.root.borrow().children.iter() {
    // dbg!(child.layout_node(|x| x.layout()));
    // }
}
