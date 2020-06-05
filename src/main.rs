use lib::element::Element;
use lib::element::ElementObject;
use lib::render::render;
use lib::view::View;
use lib::{component, element, view};

#[derive(Default, Debug)]
pub struct Root;
component!(
    Root, (), (),
    fn view |_, _| view![
        (Word, vec!['F', 'U', 'C', 'K'], vec![Element::new(Footer::default(), 'a', ())])
        (char, 'C',)
    ],
);

#[derive(Default, Debug)]
pub struct Word;
component!(
    Word, Vec<char>, Vec<Element<Footer>>,
    fn view |chars, footers| View::List(
        chars.iter().map(|c| element!(char, *c))
            .chain(footers.into_iter().map(|footer| footer.over_props(|_| 'b').into()))
            .collect::<Vec<ElementObject>>()
    )
);

#[derive(Default, Debug)]
pub struct Footer;
component!(
    Footer, char, (),
    fn view |c, _| c.into(),
);

fn main() {
    render(element![Root, ()]);
    println!("DONE!");
}
