
//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::{SymbolTable, SymbolTableEntry};
use ast::{Data, Node};
use output_manager::OutputConfig;

pub fn process(
    node: &Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    visit(node, &mut current_results.symbol_table.clone(), &mut State {}, &mut current_results.symbol_table, output)
}

pub struct State {}

// Pass the global context around as a clone
// When a node arrives that mutates a single table, it must replace the

// Pass in a global table and a local table
// when an update is made to the local table
// copy the local table back into the global table

pub fn visit(node: &Node, current_context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // I don't think this needs to traverse the AST again
    // just iterate over the symbol table since it contains type information

    // FOR SETTING THE SIZE OF ALREADY EXISTENT VARIABLES:
    // just iterate over the symbol table

    // For the creation of temporary values in the symbol table
    // Visitor pattern

    match node.name().as_str() {
        "prog" => prog(node, current_context, state, global_table, output),
        "funcDecl" => func_decl(node, current_context, state, global_table, output),
        "classDeclList" => class_list(node, current_context, state, global_table, output),
        "funcDefList" => function_list(node, current_context, state, global_table, output),
        "funcBody" => func_body(node, current_context, state, global_table, output),
        "statBlock" => stat_block(node, current_context, state, global_table, output),
        "assignOp" => assign_op(node, current_context, state, global_table, output),
        "varList" => var_list(node, current_context, state, global_table, output),
        "var" => var(node, current_context, state, global_table, output),
        "dataMember" => data_member(node, current_context, state, global_table, output),
        "addOp" => add_op(node, current_context, state, global_table, output),
        _ => {}
    }
}

fn prog(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data() {
        visit(&children[0], context, state, global_table, output);
        visit(&children[0], context, state, global_table, output);

        if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
            entry_point(&children[2], main.symbol_table_mut(), state, global_table, output);

            *global_table = context.clone();
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn entry_point(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
    // All changes from the various variables being processed should now be in context
    // so find main in the gst and overwrite it with context
}

fn class_list(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn function_list(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_list(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn func_body(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn stat_block(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn assign_op(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_decl(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // one challenge here, we need to create the entry but we also
    // need the name of the enclosing function to prefix the label
    
}

fn var(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn data_member(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {}

fn add_op(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    // Create a new temporary in this context
    let new_temp = context.get_next_temporary();
    // context.add_entry(entry: SymbolTableEntry)
}

fn func_decl(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {}
