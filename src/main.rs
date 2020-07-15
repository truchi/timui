use lib::component::{Component, Elements};

#[derive(Debug)]
struct Comp;
impl Component for Comp {
    type Props = ();

    fn children(&self, _: &Self::Props) -> Elements {
        vec![().into()]
    }
}

fn main() {
    println!("{}", "💖💖💖");
}
