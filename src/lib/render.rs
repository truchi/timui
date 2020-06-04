use super::element::ElementObject;
use super::ui::UI;

pub fn render(element: ElementObject) {
    println!("{:#?}", element);

    let ui: UI = element.into();
    println!("{:#?}", ui);
}
