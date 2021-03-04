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

#[derive(Debug, PartialEq, Hash, Clone)]
enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
}

impl Eq for Symbol {}

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
struct Grammar {
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    productions: HashMap<Symbol, Vec<Production>>,
}

impl Grammar {
    fn new(productions: HashMap<Symbol, Vec<Production>>) -> Self {
        // scan all the symbols and symbols in the productions
        let mut terminals = HashSet::new();
        let mut non_terminals = HashSet::new();

        for (symbol, production) in &productions {
            match symbol {
                Symbol::NonTerminal(_) => non_terminals.insert(symbol.clone()),
                Symbol::Terminal(_) => terminals.insert(symbol.clone()),
                Symbol::Epsilon => panic!("Epsilon cannot appear on the LHS of a production")
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
        }
    }
}

// productions:
/// {
///     A: [["0"], ["1"], ["(", "E", ")"]]
/// }

// So now iterating every production for every non-terminal would be O(n * m')
// n is the number of non-terminals; m' is the number of productions for a given non-terminal

// So how about the first set
// (Note: it would seem that caching first(A) would be wise)
fn first(grammar: &Grammar, a: &Symbol) -> HashSet<Symbol> {
    let mut result = HashSet::new();
    if grammar.terminals.contains(a) || *a == Symbol::Epsilon {
        result.insert(a.clone());
    } else if grammar.non_terminals.contains(a) {
        for production in grammar.productions.get(a) {
            for option in production {
                println!("recusing on {:?} from {:?}", option[0], a);
                let mut f = first(grammar, &option[0]);
                f.remove(&Symbol::Epsilon);
                result.extend(f);
                for symbol in option
                    .iter()
                    .skip(1)
                    .take_while(|symbol| {println!("rec on {:?} from {:?}", symbol, a); return first(grammar, symbol).contains(&Symbol::Epsilon)})
                {
                    println!("Recursing on {:?} from {:?}", symbol ,a);
                    result.extend(first(grammar, symbol));
                }
            }
        }
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_ok() {

        let grammar = Grammar::new([
            (Symbol::NonTerminal("E".to_string()),  vec![vec![Symbol::NonTerminal("T".to_string()), Symbol::NonTerminal("E'".to_string())]]),
            (Symbol::NonTerminal("E'".to_string()), vec![vec![Symbol::Terminal("+".to_string()), Symbol::NonTerminal("T".to_string()), Symbol::NonTerminal("E'".to_string())], vec![Symbol::Epsilon]]),
            (Symbol::NonTerminal("T".to_string()),  vec![vec![Symbol::NonTerminal("F".to_string()), Symbol::NonTerminal("T'".to_string())]]),
            (Symbol::NonTerminal("T'".to_string()), vec![vec![Symbol::Terminal("*".to_string()), Symbol::NonTerminal("F".to_string()), Symbol::NonTerminal("T'".to_string())], vec![Symbol::Epsilon]]),
            (Symbol::NonTerminal("F".to_string()),  vec![vec![Symbol::Terminal("0".to_string())], vec![Symbol::Terminal("1".to_string())], vec![Symbol::Terminal("(".to_string()), Symbol::NonTerminal("E".to_string()), Symbol::Terminal(")".to_string())]]),
        ].iter().cloned().collect());

        println!("{:?}", grammar.productions);
        println!("{:?}", grammar.non_terminals);
        println!("{:?}", grammar.terminals);

        assert_eq!(first(&grammar, &Symbol::NonTerminal("E".to_string())), [Symbol::Terminal("0".to_string()), Symbol::Terminal("1".to_string()), Symbol::Terminal("(".to_string())].iter().cloned().collect::<HashSet<Symbol>>());
    }
}