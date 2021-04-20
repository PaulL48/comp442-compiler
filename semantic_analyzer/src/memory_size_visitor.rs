//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
// use crate::{SymbolTable, SymbolTableEntry};
use ast::Node;

use output_manager::OutputConfig;

pub fn process(
    _node: &Node,
    current_results: &mut SemanticAnalysisResults,
    _output: &mut OutputConfig,
) {
    // we can just sum the elements of a symbol table

    for element in current_results.symbol_table.values.iter_mut() {
        element.computed_size();
    }
    // info!("Starting memory size check");
    // visit(node, &mut current_results.symbol_table.clone(), &mut State {}, &mut current_results.symbol_table)
}

// pub struct State {}

// // Pass the global context around as a clone
// // When a node arrives that mutates a single table, it must replace the

// // Pass in a global table and a local table
// // when an update is made to the local table
// // copy the local table back into the global table

// pub fn visit(
//     node: &Node,
//     current_context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     // I don't think this needs to traverse the AST again
//     // just iterate over the symbol table since it contains type information

//     // FOR SETTING THE SIZE OF ALREADY EXISTENT VARIABLES:
//     // just iterate over the symbol table

//     // For the creation of temporary values in the symbol table
//     // Visitor pattern

//     match node.name().as_str() {
//         "prog" => prog(node, current_context, state, global_table),
//         "funcDecl" => func_decl(node, current_context, state, global_table),
//         "classDeclList" => class_list(node, current_context, state, global_table),
//         "funcDefList" => function_list(node, current_context, state, global_table),
//         "funcBody" => func_body(node, current_context, state, global_table),
//         "statBlock" => stat_block(node, current_context, state, global_table),
//         "assignOp" => assign_op(node, current_context, state, global_table),
//         "varList" => var_list(node, current_context, state, global_table),
//         "var" => var(node, current_context, state, global_table),
//         "dataMember" => data_member(node, current_context, state, global_table),
//         "addOp" => add_op(node, current_context, state, global_table),
//         _ => {}
//     }
// }

// fn prog(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable) {
//     // Here we'll explicitly invoke the individual children
//     if let Data::Children(children) = node.data() {
//         visit(&children[0], context, state, global_table);
//         visit(&children[0], context, state, global_table);

//         if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
//             entry_point(&children[2], main.symbol_table_mut(), state, global_table);

//             *global_table = context.clone();
//         } else {
//             panic!();
//         }
//     } else {
//         panic!();
//     }
// }

// fn entry_point(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
//     // All changes from the various variables being processed should now be in context
//     // so find main in the gst and overwrite it with context
// }

// fn class_list(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn function_list(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn var_list(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn func_body(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn stat_block(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn assign_op(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn var_decl(
//     _node: &Node,
//     _context: &mut SymbolTable,
//     _state: &mut State,
//     _global_table: &mut SymbolTable,
// ) {
//     // one challenge here, we need to create the entry but we also
//     // need the name of the enclosing function to prefix the label
// }

// fn var(node: &Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }
// }

// fn data_member(
//     _node: &Node,
//     _context: &mut SymbolTable,
//     _state: &mut State,
//     _global_table: &mut SymbolTable,
// ) {
// }

// fn add_op(
//     node: &Node,
//     context: &mut SymbolTable,
//     state: &mut State,
//     global_table: &mut SymbolTable,
// ) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, state, global_table);
//         }
//     }

//     // Create a new temporary in this context
//     let _new_temp = context.get_next_temporary();
//     // context.add_entry(entry: SymbolTableEntry)
// }

// fn func_decl(
//     _node: &Node,
//     _context: &mut SymbolTable,
//     _state: &mut State,
//     _global_table: &mut SymbolTable,
// ) {
// }
