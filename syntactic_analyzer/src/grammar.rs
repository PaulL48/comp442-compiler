// generating first and follows sets
// first:

/*
// Where A is a terminal or non-terminal
first(A) {
    if (A in Terminals) || (A == EPSILON)
        first(A) includes A
    else if (A in Nonterminals) && There-exists (A -> S1 S2 S3 ... Sk) in productions such that S1...Sk in (Nonterminals U terminals)
        first(A) includes (first(S1) - {EPSILON})
        if there exists zero or more i < k && (EPSILON in first(S1), first(S2), ..., first(Sk))
            first(A) includes first(Si+1)


}



a better rendition
first(A) {
    if (A in Terminals) || (A == EPSILON)
        first(A) includes A
    else if (A in Nonterminals)
        for each production (A -> S1 S2 ... Sk)
            // 2.1
            first(A) includes (first(S1) - {EPSILON})

            // 2.1 start
            // Find the longest series of epsilons starting from S1
            i = 0
            for (; i < k; ++i)
                if !EPSILON in first(Si)
                    break

            // 2.3
            // If every S can be epsilon, A can be epsilon
            if i == k
                first(A) includes EPSILON

            // Now since every S up to Si can be epsilon, A can start with the first of S1 to Si
            for (; i >= 0; --i)
                first(A) includes first(Si)

            // 2.1 end
}


*/

// Initial goals create an internal representation of a grammar that provides
// economy of computing the first and follow sets

// example grammar
/*
    E  -> TE'
    E' -> +TE' | EPSILON
    T  -> FT'
    T' -> *FT' | EPSILON
    F  -> 0 | 1 | (E)
*/

// Formatted for parsing <> delimit non-terminals, '' delimit terminals
/*
    <E>  -> <T> <E'>
    <E'> -> '+' <T> <E'> | EPSILON
    <T>  -> <F> <T'>
    <T'> -> '*' <F> <T'> | EPSILON
    <F>  -> '0' | '1' | '(' <E> ')'
*/

// Each symbol can have one or more production, each production is a chain of terminals and non-terminals

use std::hash::Hash;
use lazy_static::lazy_static;

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

// With this structure we would have production as Vec<Symbol> and a grammar as a Vec<production>
/*
type Production = std::Vec<Symbol>;
struct Grammar {
    // terminals: std::Vec<Symbol>
    // non-terminals: std::Vec<Symbol>
    productions: std::Vec<Production>,
}
*/
// This would mean O(n) for every terminal U non-terminal to find their productions
// So to iterate every production for every non-terminal would be O(n * m)
// n is number of non-terminals; m is total number of productions
// use std::collections::{hash_map::HashMap, hash_set::HashSet};
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::vec::Vec;

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

// productions:
/// {
///     A: [["0"], ["1"], ["(", "E", ")"]]
/// }

// So now iterating every production for every non-terminal would be O(n * m')
// n is the number of non-terminals; m' is the number of productions for a given non-terminal

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
}
