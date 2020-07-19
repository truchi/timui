use lib::{
    component::{Component, Elements},
    style::*,
};

#[derive(Debug)]
struct Comp;
impl Component for Comp {
    type Props = ();

    fn style(&self, _: &()) -> Style {
        Style::new()
            .align_content(Center)
            .position(())
            .position(12)
            .position(0.12)
            .position((0.12, Auto))
            .align_content(AlignContent::Center)
    }

    fn children(&self, _: &()) -> Elements {
        vec![]
    }
}

fn main() {
    println!("{}", "💖💖💖");
}
