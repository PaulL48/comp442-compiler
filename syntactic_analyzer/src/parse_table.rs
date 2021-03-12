// // Building the parsing table
// // from building the parse table slide in SyntaxII
// // Generating the parse table:
// // The parse table rows contain non-terminal, columns contain terminals
// // table[E, '+']
// // The order that the rules are processed is not important
// // Prof says: "Lookout for first(TE'). if T can be epsilon then it will include first(E')"
// //            but in response first(TE') was used, but wouldn't first(E) account for that?!? No if E has other options then it is larger
// //            so we'll need to make a first() that accepts a vector of symbols
// //            I think this issue will also affect the current use of the first and follow
// /*
//     for every rule in the grammar (that is of the form A -> X)
//         try step 2 and step 3
    
//     Step 2.
//         for all terminals (t) included in first(X)
//             add A -> X to table[A, t]

//     Step 3.
//         if epsilon is in first(X)
//             for all terminals (t) in follow(A)
//                 add A -> X to table[A, t]

//     Step 4.
//         After all elements are filled, empty table locations are filled with errors

// */

// // What is a parse table
// // a doubly indexable hash map
// // table[NonTerminal, Terminal]
// // so that's a hash map of hash maps

// use std::collections::HashMap;
// use crate::grammar::{Grammar, Symbol};
// use lexical_analyzer::{Lexer, Lex, Token};

// #[derive(Debug, PartialEq)]
// pub struct ParseTable {
//     start_symbol: Symbol,
//     table: HashMap<Symbol, HashMap<Symbol,  usize>>,
// }

// impl ParseTable {
//     pub fn from_grammar(grammar: &Grammar) -> Self {
//         let follow_sets = grammar.follow_sets();
//         let mut table: HashMap<Symbol, HashMap<Symbol, usize>> = HashMap::new();
        
//         for (symbol, productions) in grammar.productions() {
//             println!("Processing symbol: {:?}", symbol);
//             table.insert(symbol.clone(), HashMap::new());
//             for (index, option) in productions.iter().enumerate() {
//                 let first_set = grammar.sentence_first(option);
//                 for terminal in first_set.iter().filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos)) {
//                     table.get_mut(&symbol).unwrap().insert(terminal.clone(), index);
//                 }

//                 if first_set.contains(&Symbol::Epsilon) {
//                     for terminal in follow_sets[symbol].iter().filter(|x| matches!(x, Symbol::Terminal(_)) || matches!(x, Symbol::Eos)) {
//                         table.get_mut(&symbol).unwrap().insert(terminal.clone(), index);
//                     }
//                 }
//             }
//         }

//         ParseTable {
//             table,
//             start_symbol: grammar.start_symbol().clone()
//         }
//     }

//     pub fn parse(&self, grammar: &Grammar, lexer: Lexer, path: &str) {
//         // what are the really bad things in here:
//         // naming
//         // dealing with a (the token stream)
//         // repetitions of match a.Clone
//         // 

//         let mut symbol_stack = vec![];
//         let mut lexer = lexer.lex(path);
//         let mut a = lexer.next();
//         let mut error = false;
//         symbol_stack.push(Symbol::Eos);
//         symbol_stack.push(self.start_symbol.clone());

//         // Once the iterator starts yielding none,
//         // the Symbol must be eos
        
//         while *symbol_stack.last().unwrap() != Symbol::Eos {
//             let x = symbol_stack.last().unwrap().clone();
//             match x.clone() {
//                 Symbol::Terminal(token) => {
//                     let b = a.clone();
//                     if *token == b.unwrap().unwrap().token_type {
//                         symbol_stack.pop();
//                         a = lexer.next(); 
//                         println!("Processing terminal {:?}", token);
//                         println!("stack: {:?}", symbol_stack);
//                     } else {
//                         error = true;

//                         let t = match a.clone() {
//                             Some(Ok(lex_result)) => Symbol::Terminal(lex_result.token_type),
//                             None => Symbol::Eos,
//                             _ => panic!("Lexing error"), 
//                         };
//                         self.skip_error(grammar, &mut lexer, &t, &mut symbol_stack, &mut a);
//                     }
//                 },
//                 Symbol::NonTerminal(token) => {
//                     let t = match a.clone() {
//                         Some(Ok(lex_result)) => Symbol::Terminal(lex_result.token_type),
//                         None => Symbol::Eos,
//                         _ => panic!("Lexing error"), 
//                     };
//                     // let t = Symbol::Terminal(b.unwrap().unwrap().token_type.clone());
//                     if self.table.contains_key(&x) && self.table[&x].contains_key(&t) {
//                         println!("Using productions: {:?} -> {:?}", x, &grammar.productions()[&x][self.table[&x][&t]]);
//                         symbol_stack.pop();
//                         let production = &grammar.productions()[&x][self.table[&x][&t]];
//                         symbol_stack.extend(production.iter().cloned().rev().filter(|x| !matches!(x, Symbol::Epsilon)));
//                         println!("stack: {:?}", symbol_stack);
//                     } else {
//                         error = true;
//                         self.skip_error(grammar, &mut lexer, &t, &mut symbol_stack, &mut a);
//                     }
//                 },
//                 _ => ()
//             }
//         }

//         match a.clone() {
//             None => (),
//             _ => println!("Error - end"),
//         }
//     }

//     fn skip_error(&self, grammar: &Grammar, lexer: &mut Lex<std::fs::File>, current: &Symbol, symbol_stack: &mut Vec<Symbol>, error_token: &mut Option<Result<Token, lexical_analyzer::lexer::LexingError>>) {
//         let mut lookahead = current.clone();
//         let follow_sets = grammar.follow_sets();
//         let temp = error_token.clone().unwrap().unwrap();
//         println!("Syntax error at line {}, col {}", temp.line, temp.column);
//         println!("Stack: {:?}", symbol_stack);
//         if lookahead == Symbol::Eos || follow_sets[symbol_stack.last().unwrap()].contains(&lookahead) {
//             symbol_stack.pop();
//             println!("Stack: {:?}", symbol_stack);
//         } else {

//             // lookahead in first(top) or epsilon in first(top) and lookahead in follow(top)
//             while !(grammar.first(symbol_stack.last().unwrap()).contains(&lookahead) || 
//                   grammar.first(symbol_stack.last().unwrap()).contains(&Symbol::Epsilon) && 
//                   follow_sets[symbol_stack.last().unwrap()].contains(&lookahead))
//             {
//                 // consider conversion from Option<Result<Token>> to Symbol
//                 *error_token = lexer.next();
//                 match error_token.clone() {
//                     Some(Ok(token)) => {lookahead = Symbol::Terminal(token.token_type)},
//                     None => {lookahead = Symbol::Eos},
//                     _ => panic!("Lexing error"),
//                 }
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use lazy_static::lazy_static;
//     use maplit::{hashmap, hashset};

//     lazy_static! {
//         static ref TEST_GRAMMAR: Grammar = Grammar::new(
//             hashmap!{
//                 Symbol::NonTerminal("E".to_string()) => vec![vec![
//                     Symbol::NonTerminal("T".to_string()),
//                     Symbol::NonTerminal("E'".to_string()),
//                 ]],
//                 Symbol::NonTerminal("E'".to_string()) => vec![
//                     vec![
//                         Symbol::Terminal("+".to_string()),
//                         Symbol::NonTerminal("T".to_string()),
//                         Symbol::NonTerminal("E'".to_string()),
//                     ], 
//                     vec![Symbol::Epsilon]
//                 ],
//                 Symbol::NonTerminal("T".to_string()) => vec![vec![
//                     Symbol::NonTerminal("F".to_string()),
//                     Symbol::NonTerminal("T'".to_string()),
//                 ]],
//                 Symbol::NonTerminal("T'".to_string()) => vec![
//                     vec![
//                             Symbol::Terminal("*".to_string()),
//                             Symbol::NonTerminal("F".to_string()),
//                             Symbol::NonTerminal("T'".to_string()),
//                     ],
//                     vec![Symbol::Epsilon],
//                 ],
//                 Symbol::NonTerminal("F".to_string()) => vec![
//                     vec![Symbol::Terminal("0".to_string())],
//                     vec![Symbol::Terminal("1".to_string())],
//                     vec![
//                         Symbol::Terminal("(".to_string()),
//                         Symbol::NonTerminal("E".to_string()),
//                         Symbol::Terminal(")".to_string()),
//                     ],
//                 ]
//             },
//             Symbol::NonTerminal("E".to_string()),
//         );
//     }

//     #[test]
//     fn test_parse_table_from_grammar() {
//         assert_eq!(
//             ParseTable::from_grammar(&TEST_GRAMMAR).table,
//             hashmap!{
//                 Symbol::NonTerminal("E".to_string()) => hashmap!{
//                     Symbol::Terminal("0".to_string()) => 0,
//                     Symbol::Terminal("1".to_string()) => 0,
//                     Symbol::Terminal("(".to_string()) => 0,
//                 },
//                 Symbol::NonTerminal("E'".to_string()) => hashmap!{
//                     Symbol::Terminal(")".to_string()) => 1,
//                     Symbol::Terminal("+".to_string()) => 0,
//                     Symbol::Eos => 1,
//                 },
//                 Symbol::NonTerminal("T".to_string()) => hashmap!{
//                     Symbol::Terminal("0".to_string()) => 0,
//                     Symbol::Terminal("1".to_string()) => 0,
//                     Symbol::Terminal("(".to_string()) => 0,
//                 },
//                 Symbol::NonTerminal("T'".to_string()) => hashmap!{
//                     Symbol::Terminal(")".to_string()) => 1,
//                     Symbol::Terminal("+".to_string()) => 1,
//                     Symbol::Terminal("*".to_string()) => 0,
//                     Symbol::Eos => 1,
//                 },
//                 Symbol::NonTerminal("F".to_string()) => hashmap!{
//                     Symbol::Terminal("0".to_string()) => 0,
//                     Symbol::Terminal("1".to_string()) => 1,
//                     Symbol::Terminal("(".to_string()) => 2,
//                 }
//             }
//         );
//     }

//     #[test]
//     fn test_parse() {

//     }
// }
