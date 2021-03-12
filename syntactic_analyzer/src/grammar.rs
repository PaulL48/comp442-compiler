use lazy_static::lazy_static;
use log::{error, warn};
use regex::Regex;
use std::collections::{hash_map::HashMap, hash_set::HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
    Eos,
}

impl Eq for Symbol {}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TERMINAL_RE: Regex =
                Regex::new("'(?P<value>.*)'").expect("Failed to compile RE");
            static ref NON_TERMINAL_RE: Regex =
                Regex::new("<(?P<value>.*)>").expect("Failed to compile RE");
            static ref EPSILON_RE: Regex = Regex::new("EPSILON").expect("Failed to compile RE");
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
        } else {
            return Err("bad".to_string());
        }
    }
}

lazy_static! {
    static ref EPSILON_SET: HashSet<Symbol> = [Symbol::Epsilon].iter().cloned().collect();
}

pub type Production = Vec<Symbol>;

#[derive(Debug, PartialEq)]
pub struct Grammar {
    start_symbol: Symbol,
    productions: HashMap<Symbol, Vec<Production>>,
}

impl Grammar {
    pub fn new(productions: HashMap<Symbol, Vec<Production>>, start_symbol: Symbol) -> Self {
        Grammar {
            productions,
            start_symbol, 
        }
    }

    pub fn productions(&self) -> &HashMap<Symbol, Vec<Production>> {
        &self.productions
    }

    pub fn start_symbol(&self) -> &Symbol {
        &self.start_symbol
    }

    pub fn from_reader<R: Read>(stream: R) -> std::io::Result<Self> {
        let buf_reader = BufReader::new(stream);
        let mut productions: HashMap<Symbol, Vec<Production>> = HashMap::new();
        let mut start_symbol = Symbol::Eos;
        let mut first = true;

        for (_, line) in buf_reader.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    error!("Error while reading stream: {}", err);
                    panic!();
                }
            };

            if line.len() == 0 {
                continue;
            }

            // Split the line at the equals
            let parts: Vec<&str> = line.split("::=").map(|x| x.trim()).collect();
            if parts.len() > 1 {
                let lhs = parts[0];
                let rhs = parts[1]
                    .split(" ")
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty());
                let lhs_symbol;

                match lhs.parse::<Symbol>() {
                    Ok(symbol) => {
                        if productions.contains_key(&symbol) {
                            productions.get_mut(&symbol).unwrap().push(vec![]);
                        }

                        productions.entry(symbol.clone()).or_insert(vec![vec![]]);
                        if first {
                            start_symbol = symbol.clone();
                            first = false;
                        }
                        lhs_symbol = symbol.clone();
                    }
                    _ => {
                        error!("Failed to parse {}", lhs);
                        lhs_symbol = Symbol::Epsilon.clone();
                        continue;
                    }
                }

                for component in rhs {
                    match component.parse::<Symbol>() {
                        Ok(symbol) => {
                            // TODO: Address these unwraps

                            productions
                                .get_mut(&lhs_symbol)
                                .unwrap()
                                .last_mut()
                                .unwrap()
                                .push(symbol);
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(Grammar {
            productions,
            start_symbol,
        })
    }

    pub fn sentence_first(&self, sentence: &[Symbol]) -> HashSet<Symbol> {
        let mut result = HashSet::new();
        if sentence.is_empty() {
            return result;
        }
        // is_empty guard allows unwrap of first()
        result.extend(self.first(sentence.first().unwrap()));
        for pair in sentence
            .windows(2)
            .take_while(|pair| self.first(&pair[0]).contains(&Symbol::Epsilon))
        {
            result.extend(self.first(&pair[1]));
        }
        return result;
    }

    pub fn first(&self, a: &Symbol) -> HashSet<Symbol> {
        self.first_internal(a, &mut HashSet::new())
    }

    fn first_internal<'a>(
        &'a self,
        a: &'a Symbol,
        visited: &mut HashSet<&'a Symbol>,
    ) -> HashSet<Symbol> {
        let mut result = HashSet::new();
        if visited.contains(a) {
            return result;
        }

        match a {
            Symbol::NonTerminal(_) => {
                visited.insert(a);
                // TODO: make this more robust be returning result
                for option in self
                    .productions
                    .get(a)
                    .expect(&format!("Symbol {:?} does not exist within the grammar", a))
                {
                    result.extend(
                        &self.first_internal(option.first().unwrap(), visited) - &EPSILON_SET,
                    );
                    let mut all_epsilons = true;
                    for p in option.windows(2) {
                        if self
                            .first_internal(&p[0], visited)
                            .contains(&Symbol::Epsilon)
                        {
                            result.extend(self.first_internal(&p[1], visited));
                        } else {
                            all_epsilons = false;
                            break;
                        }
                    }

                    if all_epsilons
                        && self
                            .first_internal(option.last().unwrap(), visited)
                            .contains(&Symbol::Epsilon)
                    {
                        result.insert(Symbol::Epsilon);
                    }
                }
            }
            terminal_or_epsilon => {
                result.insert(terminal_or_epsilon.clone());
            }
        }
        return result;
    }

    pub fn follow_sets(&self) -> HashMap<Symbol, HashSet<Symbol>> {
        let mut follow_sets = HashMap::new();
        for (symbol, _) in &self.productions {
            follow_sets.insert(symbol.clone(), self.unexpanded_follow(&symbol));
        }

        Grammar::expand_follow_sets(&follow_sets)
    }

    fn unexpanded_follow(&self, a: &Symbol) -> HashSet<Symbol> {
        let mut result = HashSet::new();
        if *a == self.start_symbol {
            result.insert(Symbol::Eos);
        }

        for (producing_symbol, production) in &self.productions {
            for option in production {
                for symbol_pair in option.windows(2) {
                    if symbol_pair[0] == *a {
                        if self.first(&symbol_pair[1]).contains(&Symbol::Epsilon) {
                            result.extend(&self.first(&symbol_pair[1]) - &EPSILON_SET);
                            result.insert(producing_symbol.clone());
                        } else {
                            result.insert(symbol_pair[1].clone());
                        }
                    }
                }

                // Rule 2
                if a == option.last().unwrap() {
                    result.insert(producing_symbol.clone());
                }
            }
        }
        result
    }

    const MAX_EXPANSION_DEPTH: i32 = 100000;

    fn expand_follow_sets_once(
        follow_sets: &HashMap<Symbol, HashSet<Symbol>>,
    ) -> HashMap<Symbol, HashSet<Symbol>> {
        let mut result = follow_sets.clone();
        for (symbol, follow_set) in follow_sets {
            let non_terminals: Vec<Symbol> = follow_set
                .iter()
                .cloned()
                .filter(|x| matches!(x, Symbol::NonTerminal(_)))
                .collect();
            for non_terminal in non_terminals {
                result.get_mut(symbol).unwrap().remove(&non_terminal);
                if non_terminal != *symbol {
                    result
                        .get_mut(symbol)
                        .unwrap()
                        .extend(follow_sets.get(&non_terminal).unwrap().iter().cloned());
                }
            }
        }
        result
    }

    fn expand_follow_sets(
        follow_sets: &HashMap<Symbol, HashSet<Symbol>>,
    ) -> HashMap<Symbol, HashSet<Symbol>> {
        let mut current = follow_sets.clone();
        let mut next = Grammar::expand_follow_sets_once(&current);
        let mut iterations = 0;
        while current != next && iterations < Grammar::MAX_EXPANSION_DEPTH {
            current = next;
            next = Grammar::expand_follow_sets_once(&current);
            iterations += 1;
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_GRAMMAR: Grammar = Grammar::new(
            [
                (
                    Symbol::NonTerminal("E".to_string()),
                    vec![vec![
                        Symbol::NonTerminal("T".to_string()),
                        Symbol::NonTerminal("E'".to_string()),
                    ]],
                ),
                (
                    Symbol::NonTerminal("E'".to_string()),
                    vec![
                        vec![
                            Symbol::Terminal("+".to_string()),
                            Symbol::NonTerminal("T".to_string()),
                            Symbol::NonTerminal("E'".to_string()),
                        ],
                        vec![Symbol::Epsilon],
                    ],
                ),
                (
                    Symbol::NonTerminal("T".to_string()),
                    vec![vec![
                        Symbol::NonTerminal("F".to_string()),
                        Symbol::NonTerminal("T'".to_string()),
                    ]],
                ),
                (
                    Symbol::NonTerminal("T'".to_string()),
                    vec![
                        vec![
                            Symbol::Terminal("*".to_string()),
                            Symbol::NonTerminal("F".to_string()),
                            Symbol::NonTerminal("T'".to_string()),
                        ],
                        vec![Symbol::Epsilon],
                    ],
                ),
                (
                    Symbol::NonTerminal("F".to_string()),
                    vec![
                        vec![Symbol::Terminal("0".to_string())],
                        vec![Symbol::Terminal("1".to_string())],
                        vec![
                            Symbol::Terminal("(".to_string()),
                            Symbol::NonTerminal("E".to_string()),
                            Symbol::Terminal(")".to_string()),
                        ],
                    ],
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            Symbol::NonTerminal("E".to_string()),
        );
    }

    #[test]
    fn test_grammar_from_stream() {
        let string = r#"
            <E> ::= <T> <E'>
            <E'> ::= '+' <T> <E'>
            <E'> ::= EPSILON
            <T> ::= <F> <T'>
            <T'> ::= '*' <F> <T'>
            <T'> ::= EPSILON
            <F> ::= '0'
            <F> ::= '1'
            <F> ::= '(' <E> ')'
        "#
        .as_bytes();
        let expected: &Grammar = &TEST_GRAMMAR;

        let grammar = Grammar::from_reader(string).unwrap();
        assert_eq!(grammar, *expected);
    }

    #[test]
    fn test_first_ok() {
        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("E".to_string())),
            [
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string())
            ]
            .iter()
            .cloned()
            .collect::<HashSet<Symbol>>()
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("E'".to_string())),
            [Symbol::Terminal("+".to_string()), Symbol::Epsilon,]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("T".to_string())),
            [
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string())
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("T'".to_string())),
            [Symbol::Terminal("*".to_string()), Symbol::Epsilon,]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("F".to_string())),
            [
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_follow() {
        assert_eq!(
            TEST_GRAMMAR.unexpanded_follow(&Symbol::NonTerminal("E".to_string())),
            [Symbol::Eos, Symbol::Terminal(")".to_string())]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.unexpanded_follow(&Symbol::NonTerminal("E'".to_string())),
            [
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.unexpanded_follow(&Symbol::NonTerminal("T".to_string())),
            [
                Symbol::Terminal("+".to_string()),
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.unexpanded_follow(&Symbol::NonTerminal("T'".to_string())),
            [
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            TEST_GRAMMAR.unexpanded_follow(&Symbol::NonTerminal("F".to_string())),
            [
                Symbol::Terminal("*".to_string()),
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_expand_follow_sets() {
        let mut follow_sets = HashMap::new();
        for (symbol, _) in &TEST_GRAMMAR.productions {
            follow_sets.insert(symbol.clone(), TEST_GRAMMAR.unexpanded_follow(&symbol));
        }

        let follow_sets = Grammar::expand_follow_sets(&follow_sets);

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("E".to_string())],
            [Symbol::Eos, Symbol::Terminal(")".to_string())]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("E'".to_string())],
            [Symbol::Eos, Symbol::Terminal(")".to_string())]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("T".to_string())],
            [
                Symbol::Eos,
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal(")".to_string())
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("T'".to_string())],
            [
                Symbol::Eos,
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal(")".to_string())
            ]
            .iter()
            .cloned()
            .collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("F".to_string())],
            [
                Symbol::Terminal("*".to_string()),
                Symbol::Terminal("+".to_string()),
                Symbol::Eos,
                Symbol::Terminal(")".to_string())
            ]
            .iter()
            .cloned()
            .collect()
        );
    }
}
