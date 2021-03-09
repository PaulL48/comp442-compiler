use std::collections::{hash_map::HashMap, hash_set::HashSet};
use lazy_static::lazy_static;
use std::hash::Hash;
use std::vec::Vec;

#[derive(Debug, PartialEq, Hash, Clone)]
enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
}

impl Eq for Symbol {}

lazy_static! {
    static ref EPSILON_SET: HashSet<Symbol> = [Symbol::Epsilon].iter().cloned().collect();
}

type Production = Vec<Symbol>;

#[derive(Debug)]
struct Grammar {
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    start_symbol: Symbol,
    productions: HashMap<Symbol, Vec<Production>>,
    // start symbol
}

impl Grammar {
    fn new(productions: HashMap<Symbol, Vec<Production>>, start_symbol: Symbol) -> Self {


        // 
        // scan all the symbols and symbols in the productions
        let mut terminals = HashSet::new();
        let mut non_terminals = HashSet::new();

        for (symbol, production) in &productions {
            match symbol {
                Symbol::NonTerminal(_) => non_terminals.insert(symbol.clone()),
                Symbol::Terminal(_) => terminals.insert(symbol.clone()),
                Symbol::Epsilon => panic!("Epsilon cannot appear on the LHS of a production"),
            };

            for option in production {
                for symbol in option {
                    match symbol {
                        Symbol::NonTerminal(_) => non_terminals.insert(symbol.clone()),
                        Symbol::Terminal(_) => terminals.insert(symbol.clone()),
                        _ => false,
                    };
                }
            }
        }

        Grammar {
            terminals,
            non_terminals,
            productions,
            start_symbol,
        }
    }
}

struct FirstAndFollowSets {

}

fn first<'a>(grammar: &'a Grammar, a: &'a Symbol,) -> HashSet<Symbol> {
    first_internal(grammar, a, &mut HashSet::new())
}

fn first_internal<'a>(
    grammar: &'a Grammar,
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
            for option in grammar.productions.get(a).expect(&format!("Symbol {:?} does not exist within the grammar", a)) {
                result.extend(&first_internal(grammar, option.first().unwrap(), visited) - &EPSILON_SET);
                let mut all_epsilons = true;
                for p in option.windows(2) {
                    if first_internal(grammar, &p[0], visited).contains(&Symbol::Epsilon) {
                        result.extend(first_internal(grammar, &p[1], visited));
                    } else {
                        all_epsilons = false;
                        break;
                    }
                }

                if all_epsilons
                    && first_internal(grammar, option.last().unwrap(), visited)
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

/*
    Follow sets of example grammar ($ is eos)

    follow(E)  -> $ )
    follow(E') -> $ )
    follow(T)  -> + $ )
    follow(T') -> + $ )
    follow(F)  -> * + $ )
*/

// What we can do is that in the result of the follow set, terminals are terminals
// nonterminals are placeholders for their follow sets



fn follow(grammar: &Grammar, a: &Symbol) -> HashSet<Symbol> {
    // A must be non-terminal

    // If the symbol is the start symbol
    //      follow(A) includes eos
    let mut result = HashSet::new();
    if *a == grammar.start_symbol {
        result.insert(Symbol::Terminal("$".to_string()));
    }

    // match a {
    //     Symbol::Terminal(_) => {
    //         result.insert(a.clone());
    //         return result;
    //     },
    //     _ => ()
    // }

    for (producing_symbol, production) in &grammar.productions {
        for option in production {
            for symbol_pair in option.windows(2) {
                if symbol_pair[0] == *a {
                    if first(grammar, &symbol_pair[1]).contains(&Symbol::Epsilon) {
                        result.extend(&first(grammar, &symbol_pair[1]) - &EPSILON_SET);
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

    // If there is a production B -> XAY in productions (X can be nothing)
    //      follow(A) includes first(Y) - EPSILON

    // If there is a production B -> XAY in productions and Y can produce epsilon (X can be nothing)
    // (Note "Y" can produce epsilon is just a bad way of writing first(Y) includes epsilon)
    //      follow(A) includes follow(B)

    // Prof seems to go through rules and exhaust them for the grammar before moving on
    // ie. find all start symbols
    // find all for "If there is a production B -> XAY in productions (X can be nothing)"
    // find all for "If there is a production B -> XAY in productions and Y can produce epsilon (X can be nothing)"

    // The last rule, follow(A) includes follow(B) is affected by the order that the productions are processed
    // so the professor suggests that the inclusion relation must be applied repeatedly until the sets stop changing

}

/*
    follow_set = {
        E = {NonTerminal("E'"), Terminal("$")},
        E' = {NonTerminal("E'")}
    }
*/

fn expand_follow_sets_once(follow_sets: &HashMap<Symbol, HashSet<Symbol>>) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut result = follow_sets.clone();
    for (symbol, follow_set) in follow_sets {
        let non_terminals: Vec<Symbol> = follow_set.iter().cloned().filter(|x| matches!(x, Symbol::NonTerminal(_))).collect();
        for non_terminal in non_terminals {
            result.get_mut(symbol).unwrap().remove(&non_terminal);
            if non_terminal != *symbol {
                result.get_mut(symbol).unwrap().extend(follow_sets.get(&non_terminal).unwrap().iter().cloned());
            }
        }
    }
    result
}

const MAX_EXPANSION_DEPTH: i32 = 100000;

fn expand_follow_sets(follow_sets: &HashMap<Symbol, HashSet<Symbol>>) -> HashMap<Symbol, HashSet<Symbol>> {
    let mut current = follow_sets.clone();
    let mut next = expand_follow_sets_once(&current);
    let mut iterations = 0;
    while current != next && iterations < MAX_EXPANSION_DEPTH {
        current = next;
        next = expand_follow_sets_once(&current);
        iterations += 1;
    }
    next
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
    fn test_first_ok() {
        assert_eq!(
            first(
                &TEST_GRAMMAR,
                &Symbol::NonTerminal("E".to_string())
            ),
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
            first(
                &TEST_GRAMMAR,
                &Symbol::NonTerminal("E'".to_string())
            ),
            [Symbol::Terminal("+".to_string()), Symbol::Epsilon,]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            first(
                &TEST_GRAMMAR,
                &Symbol::NonTerminal("T".to_string())
            ),
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
            first(
                &TEST_GRAMMAR,
                &Symbol::NonTerminal("T'".to_string())
            ),
            [Symbol::Terminal("*".to_string()), Symbol::Epsilon,]
                .iter()
                .cloned()
                .collect()
        );

        assert_eq!(
            first(
                &TEST_GRAMMAR,
                &Symbol::NonTerminal("F".to_string())
            ),
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
            follow(&TEST_GRAMMAR, &Symbol::NonTerminal("E".to_string())),
            [
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow(&TEST_GRAMMAR, &Symbol::NonTerminal("E'".to_string())),
            [
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow(&TEST_GRAMMAR, &Symbol::NonTerminal("T".to_string())),
            [
                Symbol::Terminal("+".to_string()),
                Symbol::NonTerminal("E".to_string()),
                Symbol::NonTerminal("E'".to_string()),
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow(&TEST_GRAMMAR, &Symbol::NonTerminal("T'".to_string())),
            [
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            ].iter().cloned().collect()
        );
        
        assert_eq!(
            follow(&TEST_GRAMMAR, &Symbol::NonTerminal("F".to_string())),
            [
                Symbol::Terminal("*".to_string()),
                Symbol::NonTerminal("T".to_string()),
                Symbol::NonTerminal("T'".to_string()),
            ].iter().cloned().collect()
        );
    }

    #[test]
    fn test_expand_follow_sets() {
        let mut follow_sets = HashMap::new();
        for (symbol, _) in &TEST_GRAMMAR.productions {
            follow_sets.insert(symbol.clone(), follow(&TEST_GRAMMAR, &symbol));
        }

        let follow_sets = expand_follow_sets(&follow_sets);


        assert_eq!(
            follow_sets[&Symbol::NonTerminal("E".to_string())],
            [
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("E'".to_string())],
            [
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("T".to_string())],
            [
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );

        assert_eq!(
            follow_sets[&Symbol::NonTerminal("T'".to_string())],
            [
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );
        
        assert_eq!(
            follow_sets[&Symbol::NonTerminal("F".to_string())],
            [
                Symbol::Terminal("*".to_string()),
                Symbol::Terminal("+".to_string()),
                Symbol::Terminal("$".to_string()),
                Symbol::Terminal(")".to_string())
            ].iter().cloned().collect()
        );
    }
}
