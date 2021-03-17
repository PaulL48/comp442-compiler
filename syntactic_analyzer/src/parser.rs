use crate::grammar::Grammar;
use crate::parse_table::ParseTable;
use crate::symbol::Symbol;
use lexical_analyzer::{Lex, Token};

pub fn parse(lexer: &mut Lex<std::fs::File>, grammar: &Grammar, parse_table: &ParseTable) {
    let eos_stack = vec![Symbol::Eos];
    let mut symbol_stack = vec![Symbol::Eos, grammar.start().clone()];
    let mut semantic_stack = Vec::new();
    let mut current_token = lexer.next();
    let mut previous_token = current_token.clone();
    let mut error = false;
    let mut previous_grammar_lhs = Symbol::Eos;

    println!("Parse table {:?}", parse_table);
    println!("Starting Stack: {:?}", symbol_stack);
    println!("Starting token {:?}", current_token);

    while symbol_stack != eos_stack {
        let symbol_stack_top = symbol_stack.last().unwrap().clone();
        let token_symbol = Symbol::from_token(&current_token);
        match &symbol_stack_top {
            Symbol::Terminal(symbol) => {
                if token_symbol == symbol_stack_top {
                    previous_grammar_lhs = symbol_stack_top.clone();
                    symbol_stack.pop();
                    previous_token = current_token;
                    current_token = lexer.next();
                    println!("Processing terminal {:?}", symbol);
                    println!("Stack: {:?}", symbol_stack);
                } else {
                    error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack)
                }
            }
            Symbol::NonTerminal(_) => {
                println!("{:?}, {:?}", symbol_stack_top, token_symbol);
                if parse_table.contains(&symbol_stack_top, &token_symbol) {
                    let option_index = parse_table.get(&symbol_stack_top, &token_symbol);
                    let production = grammar.production(&symbol_stack_top, option_index);
                    println!(
                        "Using production: {:?} -> {:?}",
                        symbol_stack_top, production
                    );
                    previous_grammar_lhs = symbol_stack_top.clone();
                    symbol_stack.pop();
                    symbol_stack.extend(
                        production
                            .iter()
                            .filter(|x| !matches!(x, Symbol::Epsilon))
                            .rev()
                            .cloned(),
                    );
                    println!("Stack: {:?}", symbol_stack);
                } else {
                    error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack)
                }
            }
            Symbol::SemanticAction(action) => {
                action.execute(&mut semantic_stack, previous_token.clone().unwrap(), previous_grammar_lhs.clone());
                println!("Semantic Stack {:?}", semantic_stack);
                previous_grammar_lhs = symbol_stack_top.clone();
                symbol_stack.pop();
            }
            _ => (),
        }
    }

    match current_token {
        Some(_) => println!("Error - end"),
        _ => (),
    }
}

fn skip_errors(
    grammar: &Grammar,
    lexer: &mut Lex<std::fs::File>,
    current_token: &mut Option<Token>,
    symbol_stack: &mut Vec<Symbol>,
) {
    let lex_token = current_token.clone().unwrap();
    let mut lookahead = Symbol::from_token(current_token);
    let top = symbol_stack.last().unwrap();
    println!(
        "Syntax error at line {}, col {}",
        lex_token.line, lex_token.column
    );
    println!("Stack: {:?}", symbol_stack);

    if lookahead == Symbol::Eos || grammar.follow(top).contains(&lookahead) {
        symbol_stack.pop();
        println!("Stack: {:?}", symbol_stack);
    } else {
        while !(grammar.first(top).contains(&lookahead)
            || grammar.first(top).contains(&Symbol::Epsilon)
                && grammar.follow(top).contains(&lookahead))
        {
            *current_token = lexer.next();
            lookahead = Symbol::from_token(current_token);
        }
    }
}
