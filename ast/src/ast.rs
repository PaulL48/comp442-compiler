use output_manager::warn_write;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Node {
    node_type: String,
    data: Data,
    line: usize,
    column: usize,

    // Consider adding type
    // consider adding variable name
}

#[derive(Debug, PartialEq)]
pub enum Data {
    Children(Vec<Node>),
    Integer(i64),
    Float(f64),
    String(String),
    Epsilon,
}

impl Node {
    pub fn new(node_type: &str, data: Data, line: usize, column: usize) -> Self {
        Node {
            node_type: node_type.to_string(),
            data,
            line,
            column,
        }
    }

    pub fn data(&self) -> &Data {
        return &self.data;
    }

    pub fn name(&self) -> &String {
        return &self.node_type;
    }

    pub fn data_mut(&mut self) -> &mut Data {
        return &mut self.data;
    }

    pub fn name_mut(&mut self) -> &mut String {
        return &mut self.node_type;
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn column(&self) -> &usize {
        &self.column
    }

    pub fn dft(&self) -> DepthFirstIterator {
        DepthFirstIterator {
            to_visit: vec![self],
        }
    }

    pub fn dot_graph(&self, file: &mut std::fs::File) {
        warn_write(file, "graph file", &format!("digraph a{:p} {{\n", self));

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
            }
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
            }
            _ => (),
        }
    }

    pub fn dot_node_label(&self) -> String {
        let mut label = String::from_str("").unwrap();
        label.push_str(&format!("a{:p}", self));
        match &self.data {
            Data::Float(float) => {
                label.push_str(&format!(
                    r#" [shape=box label="{}\n{}"]"#,
                    self.node_type, float
                ));
            }
            Data::Integer(int) => {
                label.push_str(&format!(
                    r#" [shape=box label="{}\n{}"]"#,
                    self.node_type, int
                ));
            }
            Data::String(s) => {
                let a = s.replace("\"", r#"\""#);
                label.push_str(&format!(
                    r#" [shape=box label="{}\n{}"]"#,
                    self.node_type, a
                ));
            }
            Data::Children(_) => {
                label.push_str(&format!(r#" [shape=ellipse label="{}"]"#, self.node_type));
            }
            Data::Epsilon => {
                label.push_str(&format!(
                    r#" [shape=diamond label="{}\nepsilon"]"#,
                    self.node_type
                ));
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
            }
            _ => (),
        }
        return relations;
    }
}

pub struct DepthFirstIterator<'a> {
    to_visit: Vec<&'a Node>,
}

impl<'a> Iterator for DepthFirstIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.to_visit.pop() {
            Some(current) => {
                match &current {
                    Node {
                        data: Data::Children(children),
                        ..
                    } => {
                        self.to_visit.extend(children.iter().rev());
                    }
                    _ => (),
                }
                Some(current)
            }
            None => None,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use lazy_static::lazy_static;

//     lazy_static! {
//         static ref TEST_AST: Node = Node {
//             node_type: "root".to_string(),
//             data: Data::Children(vec![
//                 Node {
//                     node_type: "n1".to_string(),
//                     data: Data::Children(vec![
//                         Node {
//                             node_type: "leaf1".to_string(),
//                             data: Data::Integer(1)
//                         },
//                         Node {
//                             node_type: "leaf2".to_string(),
//                             data: Data::Float(2f64)
//                         }
//                     ])
//                 },
//                 Node {
//                     node_type: "n2".to_string(),
//                     data: Data::Children(vec![
//                         Node {
//                             node_type: "leaf3".to_string(),
//                             data: Data::String("3".to_string())
//                         },
//                         Node {
//                             node_type: "leaf4".to_string(),
//                             data: Data::Integer(4)
//                         }
//                     ])
//                 },
//             ])
//         };
//     }

//     #[test]
//     fn test_dfs() {
//         let mut dfti = TEST_AST.dft();
//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "root".to_string(),
//                 data: Data::Children(vec![
//                     Node {
//                         node_type: "n1".to_string(),
//                         data: Data::Children(vec![
//                             Node {
//                                 node_type: "leaf1".to_string(),
//                                 data: Data::Integer(1)
//                             },
//                             Node {
//                                 node_type: "leaf2".to_string(),
//                                 data: Data::Float(2f64)
//                             }
//                         ])
//                     },
//                     Node {
//                         node_type: "n2".to_string(),
//                         data: Data::Children(vec![
//                             Node {
//                                 node_type: "leaf3".to_string(),
//                                 data: Data::String("3".to_string())
//                             },
//                             Node {
//                                 node_type: "leaf4".to_string(),
//                                 data: Data::Integer(4)
//                             }
//                         ])
//                     },
//                 ])
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "n1".to_string(),
//                 data: Data::Children(vec![
//                     Node {
//                         node_type: "leaf1".to_string(),
//                         data: Data::Integer(1)
//                     },
//                     Node {
//                         node_type: "leaf2".to_string(),
//                         data: Data::Float(2f64)
//                     }
//                 ])
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "leaf1".to_string(),
//                 data: Data::Integer(1)
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "leaf2".to_string(),
//                 data: Data::Float(2f64)
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "n2".to_string(),
//                 data: Data::Children(vec![
//                     Node {
//                         node_type: "leaf3".to_string(),
//                         data: Data::String("3".to_string())
//                     },
//                     Node {
//                         node_type: "leaf4".to_string(),
//                         data: Data::Integer(4)
//                     }
//                 ])
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "leaf3".to_string(),
//                 data: Data::String("3".to_string())
//             })
//         );

//         assert_eq!(
//             dfti.next(),
//             Some(&Node {
//                 node_type: "leaf4".to_string(),
//                 data: Data::Integer(4)
//             })
//         );

//         assert_eq!(dfti.next(), None);
//     }
// }
