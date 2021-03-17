use crate::grammar::Grammar;
use crate::symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseTable {
    table: HashMap<Symbol, HashMap<Symbol, usize>>,
}

impl ParseTable {
    pub fn new() -> Self {
        ParseTable {
            table: HashMap::new(),
        }
    }

    pub fn from_grammar(grammar: &Grammar) -> Self {
        let mut table: HashMap<Symbol, HashMap<Symbol, usize>> = HashMap::new();

        for (symbol, production) in grammar.productions() {
            println!("Processing {:?}", symbol);
            table.insert(symbol.clone(), HashMap::new());
            for (index, option) in production.iter().enumerate() {
                println!("Processing RHS {:?}", option);
                let first_set = grammar.sentence_first(option);
                for terminal in first_set
                    .iter()
                    .filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos))
                {
                    table
                        .get_mut(&symbol)
                        .unwrap()
                        .insert(terminal.clone(), index);
                }

                if first_set.contains(&Symbol::Epsilon) {
                    for terminal in grammar
                        .follow(symbol)
                        .iter()
                        .filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos))
                    {
                        table
                            .get_mut(&symbol)
                            .unwrap()
                            .insert(terminal.clone(), index);
                    }
                }
            }
        }

        ParseTable { table }
    }

    pub fn contains(&self, non_terminal: &Symbol, terminal: &Symbol) -> bool {
        self.table.contains_key(non_terminal) && self.table[non_terminal].contains_key(terminal)
    }

    pub fn get(&self, non_terminal: &Symbol, terminal: &Symbol) -> usize {
        self.table[non_terminal][terminal]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use maplit::{hashmap, hashset};

    lazy_static! {
        static ref TEST_GRAMMAR: Grammar = Grammar::new(
            &hashmap! {
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
    fn test_parse_table() {
        let parse_table = ParseTable::from_grammar(&TEST_GRAMMAR);
        assert_eq!(
            hashmap! {
                Symbol::NonTerminal("E".to_string()) => hashmap! {
                    Symbol::Terminal("0".to_string()) => 0,
                    Symbol::Terminal("1".to_string()) => 0,
                    Symbol::Terminal("(".to_string()) => 0
                },
                Symbol::NonTerminal("E'".to_string()) => hashmap! {
                    Symbol::Terminal(")".to_string()) => 1,
                    Symbol::Terminal("+".to_string()) => 0,
                    Symbol::Eos => 1
                },
                Symbol::NonTerminal("T".to_string()) => hashmap! {
                    Symbol::Terminal("0".to_string()) => 0,
                    Symbol::Terminal("1".to_string()) => 0,
                    Symbol::Terminal("(".to_string()) => 0
                },
                Symbol::NonTerminal("T'".to_string()) => hashmap! {
                    Symbol::Terminal(")".to_string()) => 1,
                    Symbol::Terminal("+".to_string()) => 1,
                    Symbol::Terminal("*".to_string()) => 0,
                    Symbol::Eos => 1
                },
                Symbol::NonTerminal("F".to_string()) => hashmap! {
                    Symbol::Terminal("0".to_string()) => 0,
                    Symbol::Terminal("1".to_string()) => 1,
                    Symbol::Terminal("(".to_string()) => 2
                }
            },
            parse_table.table
        );
    }
}
