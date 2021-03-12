use std::collections::HashMap;
use crate::symbol::Symbol;
use crate::grammar2::Grammar;

pub struct ParseTable {
    table: HashMap<Symbol, HashMap<Symbol,  usize>>,
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
            table.insert(symbol.clone(), HashMap::new());
            for (index, option) in production.iter().enumerate() {
                let first_set = grammar.sentence_first(option);
                for terminal in first_set.iter().filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos)) {
                    table.get_mut(&symbol).unwrap().insert(terminal.clone(), index);
                }

                if first_set.contains(&Symbol::Epsilon) {
                    for terminal in grammar.follow(symbol).iter().filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos)) {
                        table.get_mut(&symbol).unwrap().insert(terminal.clone(), index);
                    }
                }
            }
        }

        ParseTable {
            table
        }
    }

    pub fn contains(&self, non_terminal: &Symbol, terminal: &Symbol) -> bool {
        self.table.contains_key(non_terminal) && self.table[non_terminal].contains_key(terminal)
    }

    pub fn get(&self, non_terminal: &Symbol, terminal: &Symbol) -> usize {
        self.table[non_terminal][terminal]
    }
}
