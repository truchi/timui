use lib::component::Component;
use lib::components::char::Char;
// use lib::style::Style;
use lib::ui::UI;
use lib::{children, props};

struct RootProps(u8);
enum RootChildren {
    A(A),
    B(B),
}
struct Root {
    props: RootProps,
    children: RootChildren,
}

struct AProps([char; 4]);
struct AChildren([Char; 4]);
struct A {
    props: AProps,
    children: AChildren,
}

props!(A, AProps);
children!(A, AChildren);

impl Component for A {
    fn ui(&self, _props: &Self::Props, _children: &Self::Children) -> UI {
        UI::from('a')
    }
}

struct BProps(u8);
struct BChildren(u8);
struct B {
    props: BProps,
    children: BChildren,
}

props!(B, BProps);
children!(B, BChildren);
impl Component for B {
    fn ui(&self, _props: &Self::Props, _children: &Self::Children) -> UI {
        UI::from(String::from("b"))
    }
}

fn main() {
    // let v: Components = &[
    // Box::new(A {
    // props: AProps(12),
    // children: AChildren(12),
    // }),
    // Box::new(B {
    // props: BProps(20),
    // children: BChildren(20),
    // }),
    // ];
    //
    // for x in v {
    // println!("{}", x.ui());
    // }
}
