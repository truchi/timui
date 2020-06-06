type ElementObject = Box<dyn Element>;
trait Element {
    fn attrs(&self) -> i8 {
        12
    }
    fn children(&self) -> Vec<Child>;
}

enum Child {
    String(String),
    Node(ElementObject),
}

struct World;
impl Element for World {
    fn children(&self) -> Vec<Child> {
        vec![Child::String("world!".to_string())]
    }
}

struct Hello;
impl Element for Hello {
    fn children(&self) -> Vec<Child> {
        vec![
            Child::String("Hello".to_string()),
            Child::Node(Box::new(World)),
        ]
    }
}

struct Tree<'tree, T> {
    value: T,
    parent: Option<&'tree Tree<'tree, T>>,
    children: Vec<Tree<'tree, T>>,
}

enum Node {
    String(String),
    Node(i8),
}

type UI<'ui> = Tree<'ui, Node>;
type Parent<'ui> = Option<&'ui UI<'ui>>;

impl<'ui> From<(Parent<'ui>, ElementObject)> for UI<'ui> {
    fn from((parent, element): (Parent<'ui>, ElementObject)) -> Self {
        let (value, children) = match elements {
            Child::String(s) => (Node::String(s), vec![]),
            Child::Node(element) => (Node::Node(element.attrs()), element.children()),
        };
    }
}

impl<'ui> From<ElementObject> for UI<'ui> {
    fn from(element: ElementObject) -> Self {
        let root = Tree {
            value: Node::String("I have to create a fake node here".to_string()), // Not cool bc allocations
            parent: None,
            children: vec![],
        };
        let attrs = element.attrs();
        let children = element.children();

        root
    }
}

fn main() {
    println!("{}", '🎁');
}
