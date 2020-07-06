use lib::{
    component::{Component, ElementObject, Elements},
    render::render,
    style::{Color, Dimension, Percent, Points, Style},
};
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Root;
pub type RootProps = u16;
pub type RootChildren = ();

impl Component for Root {
    type Props = RootProps;

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new()
            .width(Percent(1.0))
            .height(Percent(1.0))
            // .background(Color::Blue)
            .justify_around()
        // .none() // no effect ???
    }

    fn children(&self, props: &Self::Props) -> Elements {
        vec![(Wrap, *props).into()]
    }
}

#[derive(Default, Debug)]
pub struct Wrap;
impl Component for Wrap {
    type Props = u16;

    fn style(&self, props: &Self::Props) -> Style {
        Style::new()
            .width(Points(*props as f32))
            .height(Percent(1.0))
            .background(Color::Blue)
            .justify_around()
    }

    fn children(&self, _props: &Self::Props) -> Elements {
        vec![
            (Comp2, (Color::Red, 0.2)).into(),
            (Comp2, (Color::Green, 0.3)).into(),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp2;
impl Component for Comp2 {
    type Props = (Color, f32);

    fn style(&self, props: &Self::Props) -> Style {
        Style::new()
            .width(Percent(0.5))
            .height(Percent(props.1))
            .background(props.0)
            .justify_around()
    }

    fn children(&self, props: &Self::Props) -> Elements {
        vec![
            //
            (Comp3, 'a').into(),
            Comp4.into(),
            /* (Comp3, *props).into(),
             * ().into(),
             * 'a'.into(), */
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp3;
impl Component for Comp3 {
    type Props = char;

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new()
            // .width(Dimension::Auto)
            // .height(Dimension::Auto)
            .width(Percent(0.4))
            .height(Percent(0.5))
            .bold()
            .underline()
            .italic()
            .background(Color::Red)
    }

    fn children(&self, props: &Self::Props) -> Elements {
        vec![
            //
            <ElementObject as From<char>>::from(*props),
        ]
    }
}

#[derive(Default, Debug)]
pub struct Comp4;
impl Component for Comp4 {
    type Props = ();

    fn style(&self, _props: &Self::Props) -> Style {
        Style::new()
            // .width(Dimension::Auto)
            // .height(Dimension::Auto)
            .width(Percent(0.4))
            .height(Percent(1.0))
            .bold()
            .underline()
            .italic()
            .background(Color::Red)
    }

    fn children(&self, props: &Self::Props) -> Elements {
        vec![
            Rc::new("Memory safety without garbage collection. Concurrency without data races. Zero-cost abstractions.".to_string()).into(),
        ]
    }
}

fn main() {
    let text = "textwrap: a small library for wrapping text.";

    println!(
        "{}",
        textwrap::Wrapper::new(1).break_words(false).fill(text)
    );

    // for i in (0..=151) {
    // let j = 151 - i;
    // println!("{}", j);
    // std::thread::sleep(std::time::Duration::from_millis(200));
    // render((Root, j));
    // }
    // render((Root, 12));
    // println!("{}", "💖💖💖");
    // println!("{}", "🎁🎁🎁");
}
