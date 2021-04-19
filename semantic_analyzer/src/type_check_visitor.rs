
//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::{SymbolTable, SymbolTableEntry};
use ast::{Data, Node};
use output_manager::OutputConfig;
use crate::SemanticError;
use log::info;

pub fn process(
    node: &mut Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    info!("Starting type check");
    visit(node, &mut current_results.symbol_table.clone(), &mut State {}, &mut current_results.symbol_table, output)
}

pub struct State {}

// Pass the global context around as a clone
// When a node arrives that mutates a single table, it must replace the

// Pass in a global table and a local table
// when an update is made to the local table
// copy the local table back into the global table

pub fn visit(node: &mut Node, current_context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
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
        "mulOp" => mul_op(node, current_context, state, global_table, output),
        "intfactor" => intfactor(node, current_context, state, global_table, output),
        "floatfactor" => floatfactor(node, current_context, state, global_table, output),
        "stringfactor" => stringfactor(node, current_context, state, global_table, output),
        _ => {}
    }
}

fn prog(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data_mut() {
        visit(&mut children[0], context, state, global_table, output);
        visit(&mut children[0], context, state, global_table, output);

        if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
            entry_point(&mut children[2], main.symbol_table_mut(), state, global_table, output);

            *global_table = context.clone();
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn entry_point(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
    // All changes from the various variables being processed should now be in context
    // so find main in the gst and overwrite it with context
}

fn class_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn function_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn func_body(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn stat_block(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn assign_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_decl(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // one challenge here, we need to create the entry but we also
    // need the name of the enclosing function to prefix the label
    
}

fn var(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn data_member(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {}

fn add_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        let lht = if let Some(d_type) = children[0].data_type() {
            d_type
        } else {
            panic!();
        };

        let rht = if let Some(d_type) = children[2].data_type() {
            d_type
        } else {
            panic!();
        };

        if lht != rht {
            let err = SemanticError::new_type_error(node.line(), node.column(), &lht, &rht);
            output.add(&err.to_string(), err.line(), err.col());
            node.set_type("error-type");
            return;
        } else {
            node.set_type(&lht);
        }
    }
}

fn mul_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        let lht = if let Some(d_type) = children[0].data_type() {
            d_type
        } else {
            panic!();
        };

        let rht = if let Some(d_type) = children[2].data_type() {
            d_type
        } else {
            panic!();
        };

        if lht != rht {
            let err = SemanticError::new_type_error(node.line(), node.column(), &lht, &rht);
            output.add(&err.to_string(), err.line(), err.col());
            node.set_type("error-type");
            return;
        } else {
            node.set_type(&lht);
        }
    }
}


fn func_decl(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {}

fn intfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    node.set_type("integer");
}

fn floatfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    node.set_type("float");
}

fn stringfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    node.set_type("stringlit");
}



// UTILITY FUNCTIONS
fn binary_types_match(lh: &Node, rh: &Node) {

}