use log::error;
use std::collections::{HashMap, HashSet};
use maplit::hashset;

struct Sentence {
    symbols: Vec<Symbol>,
}

impl Sentence {
    ///  Iterator over the symbols that are not Actions
    fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter().filter(|x| !matches!(x, Symbol::Action(_)))
    }

    /// Iterator over all the symbols, including Actions
    fn iter_aug(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter()
    }

    /// First symbol of sentence not including semantic actions
    fn first(&self) -> &Symbol {
        match self.iter().collect::<Vec<_>>().first() {
            Some(symbol) => symbol,
            None => {
                error!("Sentence is empty");
                panic!();
            }
        }
    }
}

struct Productions {
    start_symbol: Symbol,
    all_symbols: HashSet<Symbol>, // This assists in determining membership
    productions: HashMap<Symbol, Vec<Sentence>>,
}

struct Grammar {
    symbol_names: HashSet<String>,
    productions: HashMap<Symbol, Vec<Sentence>>,
}

impl Productions {
    pub fn contains(&self, symbol: &Symbol) -> bool {
        self.all_symbols.contains(symbol)
    }

    pub fn get_options(&self, symbol: &Symbol) -> &Vec<Sentence> {
        if !self.contains(symbol) {
            error!("Requested production options for symbol outside of grammar: {:?}", symbol);
            panic!();
        } else if !matches!(symbol, Symbol::NonTerminal(_)) {
            error!("Requested production options for a non NonTerminal symbol: {:?}", symbol);
            panic!();
        }

        &self.productions.get(symbol).unwrap()
    }

    pub fn first(&self, symbol: &Symbol) -> HashSet<Symbol> {
        if !self.contains(symbol) {
            error!("Requested first set for symbol outside of grammar: {:?}", symbol);
            panic!();
        }

        match symbol {
            Symbol::Terminal(_) => {
                hashset![symbol]
            },
            Symbol::Epsilon => {
                hashset![symbol]
            },
            Symbol::Eos => {
                hashset![symbol]
            },
            Symbol::Action(_) => {
                error!("Request first set of semantic action: {:?}", symbol);
                panic!();
            }
            Symbol::NonTerminal(_) => {
                let result = HashSet::new();
                for option in self.get_options(symbol) {
                    result.extend(self.first(option.first()));
                    for symbol_pair in option.iter().collect().windows(2) {
                        if self.first(symbol_pair.0).contains(&Symbol::Epsilon) {
                            result.extend(self.first(symbol_pair.1));
                        }
                    }
                }
                result
            }
        }
    }

    pub fn sentence_first(&self, sentence: &[Symbol]) -> HashSet<Symbol> {
        let result = HashSet::new();
        if sentence.is_empty() {
            
        }
    }

    pub fn follow(&self, symbol: &Symbol) -> HashSet<Symbol> {

    }

    fn unexpanded_follow(&self, symbol: &Symbol) -> HashSet<Symbol> {


        let result = HashSet::new();
        if *symbol == self.start_symbol {
            result.insert(*symbol);
        }
        for (producing_symbol, options) in self.productions {
            for option in options {
                match option.iter().position(|x| x == symbol) {
                    Some(position) => {

                    },
                    _ => ()
                }



                for symbol in option.iter().collect::<Vec<_>>().windows(2) {

                }
            }
        }
        result
    }
}

// follow requires the entire set of productions
fn follow_sets() {
    
}

// The whole purpose of the grammar is to support the query of the
// first and follow sets, as well as the parse_table
// So the structure should be oriented around the algorithm

// The second purpose of the grammar is to cache the first and follow sets
// and make then easy to look up

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
enum Symbol {
    // The lifetime of the symbols is tied to the grammar
    NonTerminal(String),
    Terminal(String),
    Action(Action),
    Epsilon,
    Eos
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
enum Action {
    MakeNode,
    MakeFamily,
    MakeSibling,
}

fn make_node() {}
fn make_family() {}
fn make_sibling() {}