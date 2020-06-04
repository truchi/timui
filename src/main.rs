use lib::render::render;
use lib::view::View;
use lib::{component, element, view};

#[derive(Default, Debug)]
pub struct Root;
component!(
    Root, (), (),
    fn view |_, _| view![
        (Word, vec!['F', 'U', 'C', 'K'], ())
        (char, 'C',)
    ],
);

#[derive(Default, Debug)]
pub struct Word;
component!(
    Word, Vec<char>, (),
    fn view |chars, _| View::List(chars.iter().map(|c| element!(char, *c)).collect()),
);

#[derive(Default, Debug)]
pub struct Footer;
component!(
    Footer, (), (),
    fn view |_, _| 'a'.into(),
);

fn main() {
    render(element![Root, ()]);
    println!("DONE!");
}
