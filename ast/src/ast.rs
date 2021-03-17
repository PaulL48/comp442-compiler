use std::str::FromStr;
use output_manager::warn_write;

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
        Ast { root: None }
    }
}

impl Node {
    pub fn new(node_type: &str, data: Data) -> Self {
        Node {
            node_type: node_type.to_string(),
            data,
        }
    }

    pub fn data_mut(&mut self) -> &mut Data {
        return &mut self.data;
    }

    pub fn name_mut(&mut self) -> &mut String {
        return &mut self.node_type;
    }

    pub fn dot_graph(&self, file: &mut std::fs::File) {
        warn_write(file, "graph file", "digraph A {\n");

        // accumulate all labels
        self.dot_node_label_rec(file);

        self.dot_node_relation_rec(file);

        warn_write(file, "graph file", "}");
    }

    pub fn dot_node_label_rec(&self, file: &mut std::fs::File) {
        // this node's label and if it has children add them as well
        warn_write(file, "graph file", &self.dot_node_label());
        match &self.data {
            Data::Children(children) => {
                for child in children {
                    child.dot_node_label_rec(file);
                }
            },
            _ => (),
        }
    }

    pub fn dot_node_relation_rec(&self, file: &mut std::fs::File) {
        // this node's label and if it has children add them as well
        warn_write(file, "graph file", &self.dot_relations());
        match &self.data {
            Data::Children(children) => {
                for child in children {
                    child.dot_node_relation_rec(file);
                }
            },
            _ => (),
        }
    }

    pub fn dot_node_label(&self) -> String {
        let mut label = String::from_str("").unwrap();
        label.push_str(&format!("a{:p}", self));
        match &self.data {
            Data::Float(float) => {
                label.push_str(&format!(r#" [shape=box label="{}\n{}"]"#, self.node_type, float));
            },
            Data::Integer(int) => {
                label.push_str(&format!(r#" [shape=box label="{}\n{}"]"#, self.node_type, int));
            },
            Data::String(s) => {
                label.push_str(&format!(r#" [shape=box label="{}\n{}"]"#, self.node_type, s));
            },
            Data::Children(_) => {
                label.push_str(&format!(r#" [shape=ellipse label="{}"]"#, self.node_type));
            }
        }
        label.push_str("\n");
        label
    }

    pub fn dot_relations(&self) -> String {
        let mut relations = String::from_str("").unwrap();
        match &self.data {
            Data::Children(children) => {
                for child in children {
                    relations.push_str(&format!("a{:p} -> a{:p}\n", self, child));
                }
            },
            _ => (),
        }
        return relations;
    }
}
