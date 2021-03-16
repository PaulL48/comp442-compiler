
// Data requirements
// nested node structure
// n children per node
// iterability of children
// Ability to reach leftmost child (start) from any sibling (This could be achieved albeit at a performance cost in a function that peeks children of a node)
// Ability to reach parent from any child (Same as above)

pub struct Ast {
    root: Option<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    node_type: String,
    data: Data,
}

#[derive(Debug, PartialEq)]
pub enum Data {
    Children(Vec<Node>),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            root: None
        }
    }
}

impl Node {
    pub fn new(node_type: &str, data: Data) -> Self {
        Node {
            node_type: node_type.to_string(),
            data
        }
    }

    pub fn data_mut(&mut self) -> &mut Data {
        return &mut self.data;
    }

    pub fn name_mut(&mut self) -> &mut String {
        return &mut self.node_type;
    }
}

