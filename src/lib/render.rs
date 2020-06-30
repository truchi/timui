use crate::component::{Element, ElementObject};
use crate::ui::UI;
use termion::terminal_size;

pub fn render<E: Element + 'static>(element: E) {
    let (width, height) = terminal_size().unwrap();
    let element: ElementObject = element.into();
    let mut ui: UI = element.into();

    ui.render(width as usize, height as usize - 5);
}
