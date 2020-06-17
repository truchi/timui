use super::component::{Element, ElementObject};
use super::layout::{Number, Size};
use super::ui::UI;
use termion::terminal_size;

pub fn render<E: Element + 'static>(element: E) {
    let (width, height) = terminal_size().unwrap();
    let size = Size {
        width: Number::Defined(width as f32),
        height: Number::Defined(height as f32),
    };
    let element: ElementObject = element.into();
    let ui: UI = element.into();
    println!("{:#?}", ui);
    ui.compute_layout(size);

    dbg!(size);
    dbg!(ui.root.layout_node(|x| x.layout()));
    for child in ui.root.borrow().children.iter() {
        dbg!(child.layout_node(|x| x.layout()));
    }
}
