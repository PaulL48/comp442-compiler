use std::collections::{HashMap, HashSet};
use crate::symbol::{Symbol, EPSILON_SET};
use log::error;
use std::io::Read;
use std::io::{BufRead, BufReader};


const MAX_FOLLOW_EXPANSIONS: usize = 1000000;
type Sentence = Vec<Symbol>;

#[derive(Debug, PartialEq)]
pub struct Grammar {
    start: Symbol,
    productions: HashMap<Symbol, Vec<Sentence>>,
    follow_sets: HashMap<Symbol, HashSet<Symbol>>,
}

impl Grammar {
    pub fn new(productions: &HashMap<Symbol, Vec<Sentence>>, start_symbol: &Symbol) -> Grammar {
        // Check if a non terminal appears on a RHS and not any LHS
        // Check if any RHS are empty (auto remove with warning)
        if !productions.contains_key(start_symbol) {
            error!("Start symbol {:?} is not defined on the left-hand side of any productions", start_symbol);
            panic!();
        }

        for (producing_symbol, production) in productions {
            for option in production {
                if option.is_empty() {
                    error!("Symbol {:?} has an empty production", producing_symbol);
                    panic!();
                }

                for s in option.iter().filter(|s| matches!(s, Symbol::NonTerminal(_))) {
                    if !productions.contains_key(s) {
                        error!("Non terminal symbol {:?} does not appear on the left hand side of any productions", s);
                        panic!();
                    }
                }
            }
        }

        let follow_sets = follow_sets(productions, start_symbol);

        Grammar {
            productions: productions.clone(),
            start: start_symbol.clone(),
            follow_sets,
        }
    }

    pub fn start(&self) -> &Symbol {
        &self.start
    }

    pub fn productions(&self) -> &HashMap<Symbol, Vec<Sentence>> {
        &self.productions
    }

    pub fn follow(&self, s: &Symbol) -> &HashSet<Symbol> {
        if matches!(s, Symbol::NonTerminal(_)) && !self.productions.contains_key(s) {
            error!("Requested follow set of symbol outside the grammar {:?}", s);
            panic!();
        }
        &self.follow_sets.get(s).unwrap()
    }

    pub fn first(&self, s: &Symbol) -> HashSet<Symbol> {
        first(&self.productions, s)
    }

    pub fn sentence_first(&self, sentence: &Sentence) -> HashSet<Symbol> {
        let mut result = HashSet::new();
        if sentence.is_empty() {
            return result;
        }

        // is_empty guards unwrap of first
        result.extend(first(&self.productions, sentence.first().unwrap()));
        for pair in sentence.windows(2).take_while(|pair| first(&self.productions, &pair[0]).contains(&Symbol::Epsilon)) {
            result.extend(first(&self.productions, &pair[1]));
        }
        result
    }

    pub fn production(&self, non_terminal: &Symbol, option: usize) -> &Sentence {
        &self.productions[non_terminal][option]
    }

    pub fn from_reader<R: Read>(stream: R) -> std::io::Result<Self> {
        let buf_reader = BufReader::new(stream);
        let mut productions: HashMap<Symbol, Vec<Sentence>> = HashMap::new();
        let mut first = true;
        let mut start_symbol = Symbol::Eos;

        for line in buf_reader.lines() {
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

        Ok(Grammar::new(&productions, &start_symbol))
    }
}

fn first(productions: &HashMap<Symbol, Vec<Sentence>>, s: &Symbol) -> HashSet<Symbol>{
    if matches!(s, Symbol::NonTerminal(_)) && !productions.contains_key(s) {
        error!("Requested first set of symbol outside the grammar {:?}", s);
        panic!();
    }
    first_internal(productions, s, &mut HashSet::new())
}

fn first_internal(productions: &HashMap<Symbol, Vec<Sentence>>, s: &Symbol, visited: &mut HashSet<Symbol>) -> HashSet<Symbol> {
    if matches!(s, Symbol::NonTerminal(_)) && !productions.contains_key(s) {
        error!("Requested first set of non terminal outside the grammar {:?}", s);
        panic!();
    }
    
    let mut result = HashSet::new();
    if visited.contains(s) {
        return result;
    }

    match s {
        Symbol::NonTerminal(_) => {
            result.extend(non_terminal_first(productions, s, visited));
        },
        other => {result.insert(other.clone());},
    }

    result
}

fn non_terminal_first(productions: &HashMap<Symbol, Vec<Sentence>>, s: &Symbol, visited: &mut HashSet<Symbol>) -> HashSet<Symbol> {
    let mut result = HashSet::new();
    for option in productions.get(s).unwrap() {
        result.extend(
            &first_internal(productions, option.first().unwrap(), visited) - &EPSILON_SET
        );
        let mut all_epsilons = true;
        for pair in option.windows(2) {
            if first(productions, &pair[0]).contains(&Symbol::Epsilon) {
                result.extend(first_internal(productions, &pair[1], visited))
            } else {
                all_epsilons = false;
                break;
            }
        }
        if all_epsilons && first(productions, option.last().unwrap()).contains(&Symbol::Epsilon) {
            result.insert(Symbol::Epsilon);
        }
    }
    result
}

fn follow_sets(productions: &HashMap<Symbol, Vec<Sentence>>, start_symbol: &Symbol) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut follow_sets = HashMap::new();
    for (symbol, _) in productions {
        follow_sets.insert(symbol.clone(), unexpanded_follow(productions, start_symbol, symbol));
    }
    expand_follow(&follow_sets)
}

fn unexpanded_follow(productions: &HashMap<Symbol, Vec<Sentence>>, start_symbol: &Symbol, s: &Symbol) -> HashSet<Symbol> {
    let mut result = HashSet::new();
    if s == start_symbol {
        result.insert(Symbol::Eos);
    }

    for (producing_symbol, production) in productions {
        for option in production {
            for symbol_pair in option.windows(2) {
                if symbol_pair[0] == *s {
                    if first(productions, &symbol_pair[1]).contains(&Symbol::Epsilon) {
                        result.extend(&first(productions, &symbol_pair[1]) - &EPSILON_SET);
                        result.insert(producing_symbol.clone());
                    } else {
                        result.insert(symbol_pair[1].clone());
                    }
                }
            }

            // Unwrap guarded by empty RHS check in new
            if s == option.last().unwrap() {
                result.insert(producing_symbol.clone());
            }
        }
    }
    result
}

fn expand_follow(follow_sets: &HashMap<Symbol, HashSet<Symbol>>) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut current = follow_sets.clone();
    let mut next = expand_follow_once(&current);
    let mut iterations = 0;
    while current != next && iterations < MAX_FOLLOW_EXPANSIONS {
        current = next;
        next = expand_follow_once(&current);
        iterations += 1;
    }
    next
}

fn expand_follow_once(follow_sets: &HashMap<Symbol, HashSet<Symbol>>) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut result = follow_sets.clone();
    for (symbol, follow_set) in follow_sets {
        let non_terminals = follow_set
            .iter()
            .filter(|x| matches!(x, Symbol::NonTerminal(_)))
            .cloned();
        for non_terminal in non_terminals {
            result.get_mut(symbol).unwrap().remove(&non_terminal);
            if non_terminal != * symbol {
                result.get_mut(symbol).unwrap().extend(follow_sets.get(&non_terminal).unwrap().iter().cloned());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use maplit::{hashmap, hashset};

    lazy_static! {
        static ref TEST_GRAMMAR: Grammar = Grammar::new(
            &hashmap!{
                Symbol::NonTerminal("E".to_string()) => vec![vec![
                    Symbol::NonTerminal("T".to_string()),
                    Symbol::NonTerminal("E'".to_string()),
                ]],
                Symbol::NonTerminal("E'".to_string()) => vec![
                    vec![
                        Symbol::Terminal("+".to_string()),
                        Symbol::NonTerminal("T".to_string()),
                        Symbol::NonTerminal("E'".to_string()),
                    ], 
                    vec![Symbol::Epsilon]
                ],
                Symbol::NonTerminal("T".to_string()) => vec![vec![
                    Symbol::NonTerminal("F".to_string()),
                    Symbol::NonTerminal("T'".to_string()),
                ]],
                Symbol::NonTerminal("T'".to_string()) => vec![
                    vec![
                            Symbol::Terminal("*".to_string()),
                            Symbol::NonTerminal("F".to_string()),
                            Symbol::NonTerminal("T'".to_string()),
                    ],
                    vec![Symbol::Epsilon],
                ],
                Symbol::NonTerminal("F".to_string()) => vec![
                    vec![Symbol::Terminal("0".to_string())],
                    vec![Symbol::Terminal("1".to_string())],
                    vec![
                        Symbol::Terminal("(".to_string()),
                        Symbol::NonTerminal("E".to_string()),
                        Symbol::Terminal(")".to_string()),
                    ],
                ]
            },
            &Symbol::NonTerminal("E".to_string()),
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
            hashset!{
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string())
            }
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("E'".to_string())),
            hashset!{Symbol::Terminal("+".to_string()), Symbol::Epsilon}
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("T".to_string())),
            hashset!{
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string())
            }
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("T'".to_string())),
            hashset!{Symbol::Terminal("*".to_string()), Symbol::Epsilon}
        );

        assert_eq!(
            TEST_GRAMMAR.first(&Symbol::NonTerminal("F".to_string())),
            hashset!{
                Symbol::Terminal("0".to_string()),
                Symbol::Terminal("1".to_string()),
                Symbol::Terminal("(".to_string()),
            }
        );
    }

    #[test]
    fn test_follow() {
        assert_eq!(
            unexpanded_follow(TEST_GRAMMAR.productions(), TEST_GRAMMAR.start(), &Symbol::NonTerminal("E".to_string())),
            hashset!{Symbol::Eos, Symbol::Terminal(")".to_string())}
        );

        assert_eq!(
            unexpanded_follow(TEST_GRAMMAR.productions(), TEST_GRAMMAR.start(), &Symbol::NonTerminal("E'".to_string())),
            hashset!{
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            }
        );

        assert_eq!(
            unexpanded_follow(TEST_GRAMMAR.productions(), TEST_GRAMMAR.start(), &Symbol::NonTerminal("T".to_string())),
            hashset!{
                Symbol::Terminal("+".to_string()),
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            }
        );

        assert_eq!(
            unexpanded_follow(TEST_GRAMMAR.productions(), TEST_GRAMMAR.start(), &Symbol::NonTerminal("T'".to_string())),
            hashset!{
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            }
        );

        assert_eq!(
            unexpanded_follow(TEST_GRAMMAR.productions(), TEST_GRAMMAR.start(), &Symbol::NonTerminal("F".to_string())),
            hashset!{
                Symbol::Terminal("*".to_string()),
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            }
        )
    }

    #[test]
    fn test_follow_sets() {
        assert_eq!(
            *TEST_GRAMMAR.follow(&Symbol::NonTerminal("E".to_string())),
            hashset!{
                Symbol::Eos, Symbol::Terminal(")".to_string())
            }
        );

        assert_eq!(
            *TEST_GRAMMAR.follow(&Symbol::NonTerminal("E'".to_string())),
            hashset!{Symbol::Eos, Symbol::Terminal(")".to_string())}
        );

        assert_eq!(
            *TEST_GRAMMAR.follow(&Symbol::NonTerminal("T".to_string())),
            hashset!{
                Symbol::Eos,
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal(")".to_string())
            }
        );

        assert_eq!(
            *TEST_GRAMMAR.follow(&Symbol::NonTerminal("T'".to_string())),
            hashset!{
                Symbol::Eos,
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal(")".to_string())
            }
        );

        assert_eq!(
            *TEST_GRAMMAR.follow(&Symbol::NonTerminal("F".to_string())),
            hashset!{
                Symbol::Terminal("*".to_string()),
                Symbol::Terminal("+".to_string()),
                Symbol::Eos,
                Symbol::Terminal(")".to_string())
            }
        );
    }
}