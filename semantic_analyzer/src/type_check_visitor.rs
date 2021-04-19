
//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::{SymbolTable, SymbolTableEntry};
use ast::{Data, Node};
use output_manager::OutputConfig;
use crate::SemanticError;
use log::info;

const INTEGER: &str = "integer";
const FLOAT: &str = "float";
const STRING: &str = "string";

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
        "type" => type_node(node, current_context, state, global_table, output),
        "varDecl" => var_decl(node, current_context, state, global_table, output),
        "id" => id(node, current_context, state, global_table, output),
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
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[1], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn var_decl(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        // for child in children {
        //     visit(child, context, state, global_table, output);
        // }

        // Manual processing of children to handle the correct context of dimlist
        visit(&mut children[0], context, state, global_table, output);
        visit(&mut children[1], context, state, global_table, output);
        
    }

    // TODO: More validation may have to be done here to verify dimension list
}

fn var(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
        
        if children.len() > 1 {
            panic!("My assumption was wrong");
        }

        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn data_member(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // This is where we would have to search not just the current context but also
    // if this was a class the inherited contexts

    // This assuming that the dataMember is an assignable value and on the lhs of an equal sign
    // match context.get(id: &str)
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
        
        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn add_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    let line = *node.line();
    let col = *node.column();
    
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn mul_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    let line = *node.line();
    let col = *node.column();
    
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
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

fn type_node(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::String(variable_type) = node.data() {
        match variable_type.as_str() {
            INTEGER | FLOAT | STRING => (), // OK its a primitive
            user_defined_type => {
                // signal an error if a class doesn't exist with the name
                if let None = global_table.get(user_defined_type) {
                    let err = SemanticError::new_undefined_type(node.line(), node.column(), user_defined_type);
                    output.add(&err.to_string(), err.line(), err.col());
                }
            } 
        }
    }
}

fn mandatory_dimlist(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // The list is in a mandatory context (a declaration or a datamember)
    // This means if it has any dimensions, they must be defined

}

fn id(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // Fetch the type from the context and set the node to the type
    // TODO: Note that this will need changing once classes are introduced

    if let Data::String(id) = node.data() {
        match context.get(id) {
            Some(SymbolTableEntry::Local(local)) => {
                node.set_type(local.data_type());
                // TODO: Dimensions
            },
            Some(SymbolTableEntry::Param(parameter)) => {
                node.set_type(parameter.data_type());
                // TODO: Dimensions
            },
            Some(entry) => panic!("Id \"{}\" is colliding with something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
            None => {
                let err = SemanticError::new_undefined_identifier(node.line(), node.column(), id);
                output.add(&err.to_string(), err.line(), err.col());
                node.set_type("error-type");
            }
        }


    }
}


fn check_binary_types(lhs: &Node, rhs: &Node, output: &mut OutputConfig, line: usize, col: usize) -> Result<String, ()> {
    let lht = if let Some(d_type) = lhs.data_type() {
        d_type
    } else {
        panic!();
    };

    let rht = if let Some(d_type) = rhs.data_type() {
        d_type
    } else {
        panic!();
    };

    if lht != rht {
        let err = SemanticError::new_binary_type_error(&line, &col, &lht, &rht);
        output.add(&err.to_string(), err.line(), err.col());
        Err(())
    } else {
        Ok(lht)
    }
}
