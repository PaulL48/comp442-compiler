use lazy_static::lazy_static;
use std::collections::{hash_map::HashMap, hash_set::HashSet};
use std::hash::Hash;
use std::vec::Vec;

#[derive(Debug, PartialEq, Hash, Clone)]
enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
    Eos,
}

impl Eq for Symbol {}

lazy_static! {
    static ref EPSILON_SET: HashSet<Symbol> = [Symbol::Epsilon].iter().cloned().collect();
}

type Production = Vec<Symbol>;

#[derive(Debug)]
struct Grammar {
    // terminals: HashSet<Symbol>,
    // non_terminals: HashSet<Symbol>,
    start_symbol: Symbol,
    productions: HashMap<Symbol, Vec<Production>>,
    // start symbol
}

impl Grammar {
    fn new(productions: HashMap<Symbol, Vec<Production>>, start_symbol: Symbol) -> Self {
        Grammar {
            productions,
            start_symbol,
        }
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
