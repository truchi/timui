use lib::component::AnyComponent;
use lib::component::Component;
use lib::render::render;
use lib::style::Style;
use lib::ui::UI;
use lib::{children, component, props, ui};

// impl Component for Root {
// fn ui(&self) -> UI {
// 'c'.into()

// (Box::new(Char { c: 'F' }) as Box<dyn Component>).into()

// vec![
// Box::new(Char { c: 'F' }) as Box<dyn Component>,
// Box::new(Char { c: 'U' }) as Box<dyn Component>,
// Box::new(Char { c: 'C' }) as Box<dyn Component>,
// Box::new(Char { c: 'K' }) as Box<dyn Component>,
// ]
// .into()

// <UI as From<ComponentObject>>::from(Box::new(Char { c: 'F' }))

// <UI as From<ComponentObjectList>>::from(vec![
// Box::new(Char { c: 'F' }),
// Box::new(Char { c: 'U' }),
// Box::new(Char { c: 'C' }),
// Box::new(Char { c: 'K' }),
// ])
// }
// }

component!(
    pub Root ((), ()),
    fn ui |_, _| ui!(Comp, ['F', 'U', 'C', 'K'])
);

component!(
    pub Comp ([char; 4], ()),
    // fn ui |chars, _| ui!(
        // chars.into_iter().map(|c|
            // ui!(Box, *c)
        // ).collect::<Vec<UI>>()
    // )
    fn ui |chars, _| chars.into_iter().map(|c|
        ui!(Char, *c)
    ).collect::<Vec<UI>>().into(),
    // fn ui |chars, _| chars.into_iter().map(|c|
        // Box::new(Char { props: *c, children: () }) as Box<dyn AnyComponent>
    // ).collect::<Vec<_>>().into(),
);

component!(
    pub Char (char, ()),
    fn ui |c, _| c.clone().into(),
);

fn main() {
    println!("done");
    let ui: UI = ui!(Root, ());

    fn print(ui: UI) {
        match ui {
            UI::None => {
                println!("NONE");
            }
            UI::Char(c) => {
                println!("Char {}", c);
            }
            UI::Component(component) => {
                println!("Component");
                print(component.ui());
                println!("/Component");
            }
            UI::Fragment(vec) => {
                println!("Fragment");
                for component in vec {
                    print(component.ui());
                }
                println!("/Fragment");
            }
        }
    }

    print(ui);

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
