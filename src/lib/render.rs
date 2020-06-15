use super::element::Element;
use super::element::ElementObject;
use super::ui::UI;

pub fn render<E: Element + 'static>(element: E) {
    let element: ElementObject = element.into();
    let ui: UI = element.into();
    println!("{:#?}", ui);
}
