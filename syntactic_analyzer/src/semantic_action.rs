use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use ast::{Node, Data};
use lexical_analyzer::Token;


#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Action {
    /// Create an AST node of a supplied type, with the name of the previous token
    MakeNode(String),

    /// Create an AST node by using n elements from the stack as children
    MakeFamily(usize),

    /// Push the top stack element to the list element underneath it
    MakeSibling,

    /// 
    AdoptChildren,

    /// Rename the top stack element based on the previously encountered token
    Rename
}

lazy_static! {
    static ref MAKE_NODE_RE: Regex = Regex::new("makenode~(?P<type>.*)").expect("Regular expression failed to compile");
    static ref MAKE_FAMILY_RE: Regex = Regex::new("makefamily~(?P<size>.*)").expect("Regular expression failed to compile");
    static ref MAKE_SIBLING_RE: Regex = Regex::new("makesibling").expect("Regular expression failed to compile");
    static ref ADOPT_CHILDREN_RE: Regex = Regex::new("adoptchildren").expect("Regular expression failed to compile");
    static ref RENAME_RE: Regex = Regex::new("rename").expect("Regular expression failed to compile");
}

const INTEGER: &str = "integer";
const FLOAT: &str = "float";
const STRING: &str = "string";
const LIST: &str = "list";

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if MAKE_NODE_RE.is_match(s) {
            let captures = MAKE_NODE_RE.captures(s).unwrap();
            return Ok(Action::MakeNode(captures["type"].to_string()));
        } else if MAKE_FAMILY_RE.is_match(s) {
            let captures = MAKE_FAMILY_RE.captures(s).unwrap();
            let number = captures["size"].to_string().parse().expect("Failed to parse family size in semantic action");
            return Ok(Action::MakeFamily(number));
        } else if MAKE_SIBLING_RE.is_match(s) {
            return Ok(Action::MakeSibling);
        } else if ADOPT_CHILDREN_RE.is_match(s) {
            return Ok(Action::AdoptChildren);
        } else if RENAME_RE.is_match(s) {
            return Ok(Action::Rename);
        } else {
            panic!("Invalid semantic action {}", s);
        }
    }
}

impl Action {
    pub fn execute(&self, semantic_stack: &mut Vec<Node>, previous_token: Token) {
        match self {
            Action::MakeNode(data_type) => self.make_node(semantic_stack, previous_token, data_type),
            Action::MakeFamily(size) => (),
            Action::MakeSibling => (),
            Action::AdoptChildren => (),
            Action::Rename => ()
        }
    }

    fn make_node(&self, semantic_stack: &mut Vec<Node>, previous_token: Token, data_type: &str) {
        // It'll likely be that I'll have to determine the type based on previous_token.token_type
        // since the production is likely to be generic for all numeric expressions

        let data = match data_type {
            INTEGER => {
                let int: i64 = previous_token.lexeme.parse().expect("Could not parse integer");
                Data::Integer(int)
            },
            FLOAT => {
                let float: f64 = previous_token.lexeme.parse().expect("Could not parse float");
                Data::Float(float)
            },
            STRING => {
                Data::String(previous_token.lexeme.clone())
            },
            LIST => {
                Data::Children(vec![])
            }
            _ => {
                panic!("Unrecognized node type {}", data_type);
            }
        };
        semantic_stack.push(Node::new(&previous_token.token_type, data));
    }

    fn make_family(&self, semantic_stack: &mut Vec<Node>, previous_token: Token, size: usize) {
        let mut children = Vec::new();
        for _ in 0..size {
            children.push(semantic_stack.pop().expect(&format!("Expected {} nodes on stack but underflowed", size)));
        }
        children.reverse();
        semantic_stack.push(Node::new(&previous_token.token_type, Data::Children(children)));
    }

    fn make_sibling(&self, semantic_stack: &mut Vec<Node>) {
        let sibling = semantic_stack.pop().expect("Expected a node to create as a sibling for make_sibling action");
        let top = semantic_stack.last_mut().expect("Expected a sibling list after a make_sibling action");
        if let Data::Children(sibling_list) = top.data_mut() {
            sibling_list.push(sibling);
        } else {
            panic!("Expected a sibling list after a make_sibling action");
        }
    }

    fn adopt_children(&self, semantic_stack: &mut Vec<Node>) {

    }

    fn rename(&self, semantic_stack: &mut Vec<Node>, previous_token: Token) {
        let top = semantic_stack.last_mut().expect("Expected a sibling list after a make_sibling action");
        *top.name_mut() = previous_token.token_type;
    }
}


