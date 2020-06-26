use lib::component::{Component, Elements};
use lib::render::render;
use lib::style::{Color, Percent, Points, Style};

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
            .background(Color::Blue)
            .justify_around()
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            (Comp2, 'a').into(),
            (Comp2, 'b').into(),
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
        Style::new()
            .width(Percent(0.1))
            .height(Percent(1.0))
            .background(Color::Magenta)
            .justify_around()
    }

    fn children(&self, props: &Self::Props) -> Elements {
        vec![
            //
            (Comp3, *props).into(),
            (Comp3, *props).into(),
            // ().into(),
            // 'a'.into(),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp3;
impl Component for Comp3 {
    type Props = char;

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new()
            .width(Points(1.0))
            .height(Points(1.0))
            .background(Color::Red)
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            //
            // <ElementObject as From<char>>::from(*props),
        ]
    }
}

fn main() {
    render((Root, (12, 12)));
    // println!("{}", "💖💖💖");
    // println!("{}", "🎁🎁🎁");
}
