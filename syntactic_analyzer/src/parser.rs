use crate::grammar::Grammar;
use crate::parse_table::ParseTable;
use crate::symbol::Symbol;
use lexical_analyzer::{Lex, Token};
use log::{info, trace};
use output_manager::{OutputConfig, warn_write, write_list, write_array};
use std::io::{Seek, Write, SeekFrom};

pub fn parse(lexer: &mut Lex<std::fs::File>, grammar: &Grammar, parse_table: &ParseTable, output_config: &mut OutputConfig) {
    let eos_stack = vec![Symbol::Eos];
    let mut symbol_stack = vec![Symbol::Eos, grammar.start().clone()];
    let mut semantic_stack: Vec<ast::Node> = Vec::new();
    let mut current_token = lexer.next();
    let mut previous_token = current_token.clone();
    // let mut error = false;
    let mut previous_grammar_lhs = Symbol::Eos;

    if let Some(token) = current_token.clone() {
        warn_write(&mut output_config.derivation_file, &output_config.derivation_path, &format!("Processing next token {}\n", token));
    }

    // TODO: If tokens run out, stop the parsing and signal an unexpected end of file
    while symbol_stack != eos_stack {
        output_config.derivation_file.flush().unwrap(); // TODO: Remove before submission
        trace!("Active token: {:?}", current_token);

        if let Some(token) = current_token.clone() {
            if token.line == 6 {
                let i = 0;
            }
        }

        let symbol_stack_top = symbol_stack.last().unwrap().clone();
        let token_symbol = Symbol::from_token(&current_token);
        match &symbol_stack_top {
            Symbol::Terminal(symbol) => {
                if token_symbol == symbol_stack_top {
                    previous_grammar_lhs = symbol_stack_top.clone();
                    symbol_stack.pop();
                    previous_token = current_token;
                    current_token = lexer.next();
                    if let Some(token) = current_token.clone() {
                        warn_write(&mut output_config.derivation_file, &output_config.derivation_path, &format!("Processing next token {}\n", token));
                    }
                    trace!("Processing terminal {:?}", symbol);
                    trace!("Stack: {:?}", symbol_stack);
                } else {
                    // error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack, parse_table, output_config);
                }
            }
            Symbol::NonTerminal(_) => {
                // println!("{:?}, {:?}", symbol_stack_top, token_symbol);
                if parse_table.contains(&symbol_stack_top, &token_symbol) {
                    let option_index = parse_table.get(&symbol_stack_top, &token_symbol);
                    let production = grammar.production(&symbol_stack_top, option_index);
                    // info!(
                    //     "Using production: {:?} -> {:?}",
                    //     symbol_stack_top, production
                    // ); // TODO: Output this to a file
                    warn_write(&mut output_config.derivation_file, &output_config.derivation_path, &format!("{} -> ", symbol_stack_top));
                    write_list(&mut output_config.derivation_file, &output_config.derivation_path, production);
                    previous_grammar_lhs = symbol_stack_top.clone();
                    symbol_stack.pop();
                    symbol_stack.extend(
                        production
                            .iter()
                            .filter(|x| !matches!(x, Symbol::Epsilon))
                            .rev()
                            .cloned(),
                    );
                    warn_write(&mut output_config.derivation_file, &output_config.derivation_path, "Stack: ");
                    write_list(&mut output_config.derivation_file, &output_config.derivation_path, &symbol_stack);
                    trace!("Stack: {:?}", symbol_stack);
                } else {
                    // error = true;
                    skip_errors(grammar, lexer, &mut current_token, &mut symbol_stack, parse_table, output_config);
                }
            }
            Symbol::SemanticAction(action) => {
                action.execute(&mut semantic_stack, previous_token.clone().unwrap(), previous_grammar_lhs.clone());
                info!("Semantic Stack {:?}", semantic_stack);
                previous_grammar_lhs = symbol_stack_top.clone();
                symbol_stack.pop();
            }
            _ => (),
        }
    }

    info!("Exiting parse");
    info!("Semantic stack: {:?}", semantic_stack);
    info!("Symbol stack: {:?}", symbol_stack);
    info!("Current Token: {:?}", current_token);

    if symbol_stack.last().is_none() {
        // Ran out of productions before end of tokens
        warn_write(&mut output_config.syntax_error_file, &output_config.syntax_error_path, &format!("Syntax error: expected end of file, but got {}", current_token.unwrap()));
    } else if !current_token.is_none() || symbol_stack.last() != Some(&Symbol::Eos) {
        // Ran out of file before end of productions
        warn_write(&mut output_config.syntax_error_file, &output_config.syntax_error_path, &format!("Syntax error: unexpected end of file, but was expecting one of "));
        write_array(&mut output_config.syntax_error_file, &output_config.syntax_error_path, &parse_table.table.get(&symbol_stack.last().unwrap().clone()).unwrap().iter().map(|x| x.0).collect());
    } else if !semantic_stack.is_empty() {
        // AST Should be good so print to graph
        let top = semantic_stack.last().unwrap();
        // for disjoint in &semantic_stack {
        //     disjoint.dot_graph(&mut output_config.ast_file);
        // }
        top.dot_graph(&mut output_config.ast_file);
    }
}

fn skip_errors(
    grammar: &Grammar,
    lexer: &mut Lex<std::fs::File>,
    current_token: &mut Option<Token>,
    symbol_stack: &mut Vec<Symbol>,
    parse_table: &ParseTable,
    output_config: &mut OutputConfig,
) {
    let lex_token = current_token.clone().unwrap();
    let mut lookahead = Symbol::from_token(current_token);
    let top = symbol_stack.last().unwrap();
    warn_write(&mut output_config.syntax_error_file, &output_config.syntax_error_path, &format!("Syntax error at line {}, col {}: encountered {}, but was expecting one of ", lex_token.line, lex_token.column, lex_token.lexeme));
    write_array(&mut output_config.syntax_error_file, &output_config.syntax_error_path, &parse_table.table.get(top).unwrap().iter().map(|x| x.0).collect());

    // warn!(
    //     "Syntax error at line {}, col {}",
    //     lex_token.line, lex_token.column
    // );
    trace!("Stack: {:?}", symbol_stack);

    if lookahead == Symbol::Eos || grammar.follow(top).contains(&lookahead) {
        symbol_stack.pop();
        trace!("Stack: {:?}", symbol_stack);
    } else {
        while !(grammar.first(top).contains(&lookahead)
            || grammar.first(top).contains(&Symbol::Epsilon)
                && grammar.follow(top).contains(&lookahead)) && !current_token.is_none()
        {
            *current_token = lexer.next();
            lookahead = Symbol::from_token(current_token);
        }
    }
}
