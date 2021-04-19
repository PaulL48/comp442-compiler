//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::SymbolTable;
use ast::{Data, Node};
use output_manager::OutputConfig;

pub fn process(
    node: &Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    visit(node, &mut current_results.symbol_table, &mut State {})
}

pub struct State {}

pub fn visit(node: &Node, current_context: &mut SymbolTable, state: &mut State) {
    match node.name().as_str() {
        "prog" => prog(node, current_context, state),
        "funcDecl" => func_decl(node, current_context, state),
        "classDeclList" => class_list(node, current_context, state),
        "funcDefList" => function_list(node, current_context, state),
        "funcBody" => func_body(node, current_context, state),
        "statBlock" => stat_block(node, current_context, state),
        "assignOp" => assign_op(node, current_context, state),
        "varList" => var_list(node, current_context, state),
        "var" => var(node, current_context, state),
        "dataMember" => data_member(node, current_context, state),
        "addOp" => add_op(node, current_context, state),
        _ => {}
    }
}

fn prog(node: &Node, context: &mut SymbolTable, state: &mut State) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data() {
        visit(&children[0], context, state);
        visit(&children[1], context, state);
        entry_point(&children[2], context, state);
    } else {
        panic!();
    }
}

fn class_list(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn function_list(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn entry_point(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn var_list(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn func_body(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn stat_block(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn assign_op(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn var_decl(node: &Node, context: &mut SymbolTable, state: &mut State) {
    // one challenge here, we need to create the entry but we also
    // need the name of the enclosing function to prefix the label
}

fn var(node: &Node, context: &mut SymbolTable, state: &mut State) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state);
        }
    }
}

fn data_member(node: &Node, context: &mut SymbolTable, state: &mut State) {}

fn add_op(node: &Node, context: &mut SymbolTable, state: &mut State) {}

fn func_decl(node: &Node, context: &mut SymbolTable, state: &mut State) {}
