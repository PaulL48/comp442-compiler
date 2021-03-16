use crate::semantic_action::Action;
use lazy_static::lazy_static;
use lexical_analyzer::Token;
use log::error;
use maplit::hashset;
use regex::Regex;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

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

lazy_static! {
    pub static ref EPSILON_SET: HashSet<Symbol> = hashset! {Symbol::Epsilon};
}

impl Eq for Symbol {}

impl Symbol {
    pub fn from_token(token: &Option<Token>) -> Symbol {
        match token {
            Some(token) => return Symbol::Terminal(token.token_type.clone()),
            // Some(token) => return Symbol::Terminal(token.lexeme.clone()),
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
            static ref ACTION_RE: Regex =
                Regex::new("@(?P<action>.*)@").expect("Failed to compile RE");
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
        } else if ACTION_RE.is_match(s) {
            let captures = ACTION_RE.captures(s).unwrap();
            return Ok(Symbol::SemanticAction(
                captures["action"].parse().expect("Failed to parse action"),
            ));
        } else {
            error!("Unexpected symbol in grammar {:?}", s);
            panic!();
        }
    }
}
