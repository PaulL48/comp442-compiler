use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use maplit::hashset;
use lazy_static::lazy_static;
use std::str::FromStr;
use regex::Regex;
use log::error;
use lexical_analyzer::Token;
use ast::{Node, Data};
use maplit::hashmap;

// From what I can tell the semantic action will
// either create a node or amalgamate n node elements

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    SemanticAction(Action),
    Epsilon,
    Eos,
}

// The consumption of a node is verified by a label.
// ex. A translation expects a T'i1 and produces a E's
// So semantic actions either translate a node (pop, push), add a new node, or consume nodes to create a new one
// pub enum Action {
//     Create(String), // 
//     Translate(String, String), // src, dest

// }

// There seems to be three possible choices in what an action does
// create a node
// translate a node label
// group nodes
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Action {
    Create(String, String), // Determines the literal used
    Group(String, usize),
    MakeSibling,
}

// In the grammar

// The creation of a group of nodes involves n elements of the stack,
// so the creation of the nodes must be parametrized in the number of elements
// and the elements must be able to hold one or more 

// #[derive(Debug, PartialEq, Hash, Clone)]
// pub enum 

lazy_static! {
    pub static ref EPSILON_SET: HashSet<Symbol> = hashset!{Symbol::Epsilon};
}

impl Eq for Symbol {}

impl Symbol {
    pub fn from_token(token: &Option<Token>) -> Symbol {
        match token {
            // Some(token) => return Symbol::Terminal(token.token_type.clone()),
            Some(token) => return Symbol::Terminal(token.lexeme.clone()),
            None => return Symbol::Eos,
        }
    }
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TERMINAL_RE: Regex =
                Regex::new("'(?P<value>.*)'").expect("Failed to compile RE");
            static ref NON_TERMINAL_RE: Regex =
                Regex::new("<(?P<value>.*)>").expect("Failed to compile RE");
            static ref EPSILON_RE: Regex = Regex::new("EPSILON").expect("Failed to compile RE");
            static ref CREATE_ACTION_RE: Regex = Regex::new("@(?P<action>create~[a-zA-Z]+~[a-zA-Z]+)@").expect("Failed to compile RE");
            static ref GROUP_ACTION_RE: Regex = Regex::new("@(?P<action>group~[0-9]+~.*)@").expect("Failed to compile RE");
        }

        // Nested unwraps() are safe due to is_match guard
        if NON_TERMINAL_RE.is_match(s) {
            let captures = NON_TERMINAL_RE.captures(s).unwrap();
            return Ok(Symbol::NonTerminal(captures["value"].to_string()));
        } else if TERMINAL_RE.is_match(s) {
            let captures = TERMINAL_RE.captures(s).unwrap();
            return Ok(Symbol::Terminal(captures["value"].to_string()));
        } else if EPSILON_RE.is_match(s) {
            return Ok(Symbol::Epsilon);
        } else if CREATE_ACTION_RE.is_match(s) {
            let captures = CREATE_ACTION_RE.captures(s).unwrap();
            let components = captures["action"].split("~").map(|x| x.trim()).collect::<Vec<_>>();
            let create_type = components[1];
            let create_name = components[2];
            match components[1] {
                "integer" => {},
                "float" => {},
                "string" => {},
                _ => {
                    error!("Semantic action \"create\" had malformed data type");
                    panic!();
                }
            }
            return Ok(Symbol::SemanticAction(Action::Create(create_type.to_string(), create_name.to_string())));
        } else if GROUP_ACTION_RE.is_match(s) {
            let captures = GROUP_ACTION_RE.captures(s).unwrap();
            let components = captures["action"].split("~").map(|x| x.trim()).collect::<Vec<_>>();
            let group_size = components[1];
            let name = components[2];
            let group_size: usize = group_size.parse().expect("Failed to parse grammar");
            return Ok(Symbol::SemanticAction(Action::Group(name.to_string(), group_size)));
        } else {
            error!("Unexpected symbol in grammar {:?}", s);
            panic!();
        }
    }
}

impl Action {


    pub fn execute_action(&self, semantic_stack: &mut Vec<Node>, previous_token: Token) {
        match self {
            Action::Create(data_type, name) => self.create(semantic_stack, previous_token, data_type, name),
            Action::Group(name, count) => self.group(semantic_stack, previous_token, count, name),
            Action::MakeSibling => self.make_sibling(semantic_stack, previous_token,)
        }
    }

    fn create(&self, semantic_stack: &mut Vec<Node>, previous_token: Token, data_type: &str, name: &str) {
        let new_node;
        if data_type == "integer" {
            let parsed_number: i64 = previous_token.lexeme.parse().expect("Could not parse digit");
            new_node = Node::new(name, Data::Integer(parsed_number));
        } else if data_type == "float" {
            let parsed_float: f64 = previous_token.lexeme.parse().expect("Could not parse digit");
            new_node = Node::new(name, Data::Float(parsed_float));
        } else if data_type == "string" {
            new_node = Node::new(name, Data::String(previous_token.lexeme));
        } else {
            error!("Semantic action create has invalid type");
            panic!();
        }

        semantic_stack.push(new_node);
    }

    fn group(&self, semantic_stack: &mut Vec<Node>, _: Token, count: &usize, name: &str) {
        let mut children = Vec::new();
        for _ in 0..*count {
            children.push(semantic_stack.pop().unwrap());
        }
        children.reverse();
        let new_node = Node::new(name, Data::Children(children));
        semantic_stack.push(new_node);
    }

    fn make_sibling(&self, semantic_stack: &mut Vec<Node>, previous_token: Token) {
        // Pop a node from the stack and make it part of the sibling list right below it
        let new_sibling = semantic_stack.pop().expect("Expected a node to create as a sibling for make_sibling action");
        let top = semantic_stack.last_mut().expect("Expected a sibling list after a make_sibling action");
        if let Data::Children(sibling_list) = top.data_mut() {
            sibling_list.push(new_sibling);
        } else {
            panic!("Expected a sibling list after a make_sibling action");
        }
    }
}

// An action can be
// make node
// make sibling
// adopt children
// make family
