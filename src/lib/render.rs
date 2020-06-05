use super::element::ElementObject;
use super::ui::UI;

pub fn render(element: ElementObject) {
    let ui: UI = element.into();
    println!("{:#?}", ui);
}
