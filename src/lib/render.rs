use super::element::Element;
use super::element::ElementObject;
use super::ui::UI;

pub fn render<E>(element: E)
where
    E: Element + 'static,
{
    let element: ElementObject = element.into();
    let ui: UI = element.into();
    // let ui: UI = (&element).into();
    println!("{:#?}", ui);
}
