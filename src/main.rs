use lib::component::{Component, Elements};
use lib::render::render;
use lib::style::{Percent, Style};

#[derive(Default, Debug)]
pub struct Root;
pub type RootProps = (u16, u16);
pub type RootChildren = ();

impl Component for Root {
    type Props = RootProps;

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new()
            .width(Percent(1.0))
            .height(Percent(1.0))
            .justify_center()
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            Comp2.into(),
            // ().into(),
            // 'a'.into(),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp2;
impl Component for Comp2 {
    type Props = char;

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new().width(Percent(0.1)).height(Percent(0.1))
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![Comp3.into(), ().into(), 'a'.into()]
    }
}

#[derive(Default, Debug)]
pub struct Comp3;
impl Component for Comp3 {
    type Props = char;

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            String::from("COMP 3").into(),
            String::from("a").into(),
            String::from("b").into(),
        ]
    }
}

fn main() {
    render((Root, (12, 12)));
    println!("{}", "💖💖💖\n🎁🎁🎁");
}
