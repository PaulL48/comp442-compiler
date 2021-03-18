use ast::{Data, Node};
use lazy_static::lazy_static;
use lexical_analyzer::Token;
use log::{error, trace};
use regex::Regex;
use std::str::FromStr;
use crate::symbol::Symbol;
use std::fmt;

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Action {
    /// Create an AST node of a supplied type, with the name of the previous token
    MakeNode(String, String),

    /// Create an AST node by using n elements from the stack as children
    MakeFamily(usize, String),

    /// Push the top stack element to the list element underneath it
    MakeSibling,

    // ///
    // AdoptChildren,

    // /// Rename the top stack element based on the previously encountered token
    // Rename(String),
}

lazy_static! {
    static ref MAKE_NODE_RE: Regex =
        Regex::new("makenode~(?P<type>.*)~(?P<name>.*)").expect("Regular expression failed to compile");
    static ref MAKE_FAMILY_RE: Regex =
        Regex::new("makefamily~(?P<size>.*)~(?P<name>.*)").expect("Regular expression failed to compile");
    static ref MAKE_SIBLING_RE: Regex =
        Regex::new("makesibling").expect("Regular expression failed to compile");
    static ref ADOPT_CHILDREN_RE: Regex =
        Regex::new("adoptchildren").expect("Regular expression failed to compile");
    static ref RENAME_RE: Regex =
        Regex::new("rename~(?P<name>.*)").expect("Regular expression failed to compile");
}

const INTEGER: &str = "integer";
const FLOAT: &str = "float";
const STRING: &str = "string";
const LIST: &str = "list";
const EPSILON: &str = "epsilon";

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if MAKE_NODE_RE.is_match(s) {
            let captures = MAKE_NODE_RE.captures(s).unwrap();
            let data_type = captures["type"].to_string();
            let name = captures["name"].to_string();
            return Ok(Action::MakeNode(data_type, name));
        } else if MAKE_FAMILY_RE.is_match(s) {
            let captures = MAKE_FAMILY_RE.captures(s).unwrap();
            let number = captures["size"].to_string().parse().unwrap();
            let name = captures["name"].to_string();
            return Ok(Action::MakeFamily(number, name));
        } else if MAKE_SIBLING_RE.is_match(s) {
            return Ok(Action::MakeSibling);
        // } else if ADOPT_CHILDREN_RE.is_match(s) {
        //     // return Ok(Action::AdoptChildren);
        // } else if RENAME_RE.is_match(s) {
        //     let captures = RENAME_RE.captures(s).unwrap();
        //     let name = captures["name"].to_string();
        //     return Ok(Action::Rename(name));
        } else {
            error!("Unrecognized semantic action {}", s);
            panic!();
        }
    }
}

impl Action {
    pub fn execute(&self, semantic_stack: &mut Vec<Node>, previous_token: Token, previous_production: Symbol) {
        match self {
            Action::MakeNode(data_type, name) => {
                self.make_node(semantic_stack, previous_token, data_type, name)
            }
            Action::MakeFamily(size, name) => self.make_family(semantic_stack, previous_token, *size, name),
            Action::MakeSibling => self.make_sibling(semantic_stack),
            // Action::AdoptChildren => self.adopt_children(),
            // Action::Rename(name) => self.rename(semantic_stack, previous_production, name),
        }
    }

    fn make_node(&self, semantic_stack: &mut Vec<Node>, previous_token: Token, data_type: &str, name: &str) {
        // It'll likely be that I'll have to determine the type based on previous_token.token_type
        // since the production is likely to be generic for all numeric expressions

        // if data_type == LIST {
        //     semantic_stack.push(Node::new("anon-list", Data::Children(Vec::new())));
        //     return;
        // }

        let data = match data_type {
            INTEGER => {
                let int = previous_token.lexeme.parse::<i64>();
                if let Err(err) = int {
                    error!(
                        "Failed to parse lexeme {} as integer: {}",
                        previous_token.lexeme, err
                    );
                    return; // TODO: this might have to be removed and delegated to skip_errors to recover from a parse error
                } else {
                    Data::Integer(int.unwrap())
                }
            }
            FLOAT => {
                let float = previous_token.lexeme.parse::<f64>();
                if let Err(err) = float {
                    error!(
                        "Failed to parse lexeme {} as float: {}",
                        previous_token.lexeme, err
                    );
                    return; // TODO: this might have to be removed and delegated to skip_errors to recover from a parse error
                } else {
                    Data::Float(float.unwrap())
                }
            }
            STRING => Data::String(previous_token.lexeme.clone()),
            LIST => Data::Children(Vec::new()),
            EPSILON => Data::Epsilon,
            _ => {
                error!("Unrecognized node type {}", data_type);
                return; // TODO: this might have to be removed and delegated to skip_errors to recover from a parse error
            }
        };
        semantic_stack.push(Node::new(name, data));
    }

    fn make_family(&self, semantic_stack: &mut Vec<Node>, _: Token, size: usize, name: &str) {
        let mut children = Vec::new();
        for _ in 0..size {
            let c = match semantic_stack
                .pop() {
                Some(s) => s,
                None => {
                    trace!("Expected {} nodes on stack but underflowed", size);
                    return;
                }
            };
            
            match &c.data() {
                Data::Children(sub_children) => {
                    if sub_children.is_empty() {
                        children.push(Node::new(c.name(), Data::Epsilon));
                    } else {
                        children.push(c);
                    }
                }
                _ => {
                    children.push(c);
                }
            }

            // children.push(
            //     semantic_stack
            //         .pop()
            //         .expect(&format!("Expected {} nodes on stack but underflowed", size)),
            // );
        }
        children.reverse();
        semantic_stack.push(Node::new(
            name,
            Data::Children(children),
        ));
    }

    fn make_sibling(&self, semantic_stack: &mut Vec<Node>) {
        let sibling = match semantic_stack.pop() {
            None => {
                error!("Expected a node to create as a sibling for make_sibling action");
                return;
            },
            Some(s) => s,
        };

        let top = match semantic_stack.last_mut() {
            None => {
                error!("Expected a sibling list after a make_sibling action");
                return;
            },
            Some(s) => s,
        };
        if let Data::Children(sibling_list) = top.data_mut() {
            sibling_list.push(sibling);
        } else {
            error!("Expected a sibling list after a make_sibling action");
            error!("Node was {:?}", top);            
        }
    }

    // fn adopt_children(&self) {}

    // fn rename(&self, semantic_stack: &mut Vec<Node>, _: Symbol, name: &str) {
    //     let top = semantic_stack
    //         .last_mut()
    //         .expect("Expected a sibling list after a make_sibling action");
    //     *top.name_mut() = name.to_string();
    //     // *top.name_mut() = match previous_production {
    //     //     Symbol::NonTerminal(name) => name,
    //     //     Symbol::Terminal(name) => name,
    //     //     Symbol::Epsilon => "Epsilon".to_string(),
    //     //     Symbol::Eos => "Eos".to_string(),
    //     //     Symbol::SemanticAction(action) => {
    //     //         error!("Renaming node based on semantic action {:?}. This likely isn't what was intended", action);
    //     //         panic!();
    //     //     }
    //     // }
    // }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::MakeNode(data_type, name) => write!(f, "make_node({}, {})", data_type, name),
            Action::MakeFamily(size, name) => write!(f, "make_family({}, {})", size, name),
            Action::MakeSibling => write!(f, "make_sibling"),
            // Action::AdoptChildren => write!(f, "adopt_children"),
            // Action::Rename(name) => write!(f, "rename_node({})", name),
        }
    }
}
