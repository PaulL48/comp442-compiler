//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use ast::{Node, Data};
use crate::SymbolTable;

pub struct State {

}

pub fn visit(node: &Node, current_context: &mut SymbolTable, state: &mut State) {
    match node.name().as_str() {
        "prog" => prog(node, current_context, state),
        "funcDecl" => func_decl(node, current_context, state),
        "classDeclList" => class_list(node, current_context, state),
        "funcDefList" => function_list(node, current_context, state),
        _ => {}
    }
}

fn prog(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data() {
        visit(&children[0], current_results, state);
        visit(&children[1], current_results, state);
        entry_point(&children[2], current_results, state);
    } else {
        panic!();
    }
}

fn class_list(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, current_results, state);
        }
    }
}

fn function_list(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, current_results, state);
        }
    }
}

fn entry_point(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, current_results, state);
        }
    }


}

fn var_list(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, current_results, state);
        }
    }
}

fn var_decl(node: &Node, current_results: &mut SymbolTable, state: &mut State) {
    // one challenge here, we need to create the entry but we also
    // need the name of the enclosing function to prefix the label
}

fn func_decl(node: &Node, context: &mut SymbolTable, state: &mut State) {

}