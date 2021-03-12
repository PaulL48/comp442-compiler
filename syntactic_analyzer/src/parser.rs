// The amalgamation of all state and algorithms for parsing a program

// what is required for a parse
// grammar
// parse table
// lexer

use lexical_analyzer::{Lex, Token};
use crate::symbol::Symbol;
use crate::grammar2::Grammar;
use crate::parse_table2::ParseTable;

pub fn parse(lexer: &mut Lex<std::fs::File>, grammar: &Grammar, parse_table: &ParseTable) {
    let eos_stack = vec![Symbol::Eos];
    let mut symbol_stack = vec![Symbol::Eos, grammar.start().clone()];
    let mut current_token = lexer.next();
    let mut error = false;

    while symbol_stack != eos_stack {
        let symbol_stack_top = symbol_stack.last().unwrap().clone();
        let token_symbol = Symbol::from_token(&current_token);
        match &symbol_stack_top {
            Symbol::Terminal(symbol) => {
                if token_symbol == symbol_stack_top {
                    symbol_stack.pop();
                    current_token = lexer.next();
                    println!("Processing terminal {:?}", symbol);
                    println!("Stack: {:?}", symbol_stack);
                } else {
                    error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack)
                }
            },
            Symbol::NonTerminal(_) => {
                if parse_table.contains(&symbol_stack_top, &token_symbol) {
                    let option_index = parse_table.get(&symbol_stack_top, &token_symbol);
                    let production = grammar.production(&symbol_stack_top, option_index);
                    println!("Using production: {:?} -> {:?}", symbol_stack_top, production);
                    symbol_stack.pop();
                    symbol_stack.extend(production.iter().filter(|x| !matches!(x, Symbol::Epsilon)).rev().cloned());
                    println!("Stack: {:?}", symbol_stack);
                } else {
                    error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack)
                }
            },
            _ => (),
        }
    }

    match current_token {
        Some(_) => println!("Error - end"),
        _ => ()
    }
}

fn skip_errors(grammar: &Grammar, lexer: &mut Lex<std::fs::File>, current_token: &mut Option<Token>, symbol_stack: &mut Vec<Symbol>) {
    let lex_token = current_token.clone().unwrap();
    let mut lookahead = Symbol::from_token(current_token);
    let top = symbol_stack.last().unwrap();
    println!("Syntax error at line {}, col {}", lex_token.line, lex_token.column);
    println!("Stack: {:?}", symbol_stack);

    if lookahead == Symbol::Eos || grammar.follow(top).contains(&lookahead) {
        symbol_stack.pop();
        println!("Stack: {:?}", symbol_stack);
    } else {
        while !(grammar.first(top).contains(&lookahead) ||
                grammar.first(top).contains(&Symbol::Epsilon) &&
                grammar.follow(top).contains(&lookahead))
        {
            *current_token = lexer.next();
            lookahead = Symbol::from_token(current_token);
        }
    }
}

