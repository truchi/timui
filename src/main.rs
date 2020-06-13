use lib::component::Component;
use lib::element::Elements;
use lib::render::render;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Root;
pub type RootProps = (u16, u16);
pub type RootChildren = ();

impl Component for Root {
    type Props = RootProps;

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            Rc::new(String::from("ROOT")).into(),
            Comp2.into(),
            (Comp2, '2').into(),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp2;
impl Component for Comp2 {
    type Props = char;

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            Rc::new(String::from("COMP 2")).into(),
            Comp3.into(),
            (Comp3, 'a').into(),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp3;
impl Component for Comp3 {
    type Props = char;

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            Rc::new(String::from("COMP 3")).into(),
            Rc::new(String::from("a")).into(),
            Rc::new(String::from("b")).into(),
        ]
    }
}

fn main() {
    render((Root, (12, 12)));
    println!("{}{}", '💖', '🎁');
}
