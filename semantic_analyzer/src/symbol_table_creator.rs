// dispatch based on node type
use crate::semantic_analyzer::SemanticAnalysisResults;
use crate::symbol_table::symbol_table::SymbolTable;
use ast;

/// Called in the traversal of the tree
pub fn visit(node: &ast::Node, current_result: &mut SemanticAnalysisResults) {
    let mut symbol_table = SymbolTable {
        name: "global".to_string(),
        ..Default::default()
    };
    match node.name().as_str() {
        "prog" => prog(node, current_result),
        "funcDef" => func_def(node, current_result),
        "classDecl" => class_decl(node, current_result),

        _ => (),
    }
}

pub fn prog(node: &ast::Node, results: &mut SemanticAnalysisResults) {}

pub fn func_def(node: &ast::Node, results: &mut SemanticAnalysisResults) {
    // right now, the goal is:
    // transform an AST funcDef node into a symbol table

    // if the scope spec is specified
    //      get the class entry it is referring to
    //      if the
    if let ast::Data::Children(children) = node.data() {
        // node order is: id, scopeSpec, fparamList, type, funcBody
        if children.len() != 5 {
            // Incorrect number of children, malformed node
            return;
        }

        // There's some validation that's going to be boilerplate for each node handler
        // the enum -> Data type
        // the nature of the data children.len

        match children[1].data() {
            ast::Data::Epsilon => {}
            ast::Data::String(parent_class) => {}
            _ => {} // malformed node
        }

        if let ast::Data::Epsilon = children[1].data() {
        } else {
        }
    } else {
        // The node is supposedly a funcDef node but lacks children
        // the AST is malformed at this node
    }
}

pub fn class_decl(node: &ast::Node, results: &mut SemanticAnalysisResults) {}

// fn visit_internal(symbol_table: &mut SymbolTable, node: &ast::Node) {

// }

// As we enter a particular function we can even swap the symbol table that we're passing down in the hierarchy

// fn prog(symbol_table: &mut SymbolTable, node: &ast::Node) {
//     // enter the second node
//     if let ast::Data::Children(children) = node.data() {
//         func_def_list(symbol_table, &children[1]);
//     } else {
//         // bad. This means the AST is malformed
//     }
// }

// fn func_def_list(symbol_table: &mut SymbolTable, node: &ast::Node) {
//     // Children are all funcDefs
//     if let ast::Data::Children(children) = node.data() {
//         for def_node in children {
//             let mut func_symbol_table: SymbolTable = Default::default();
//             func_def(&mut func_symbol_table, def_node);
//             println!("{}", func_symbol_table);
//         }
//     } else {
//         // bad
//     }

// }

// fn func_def(symbol_table: &mut SymbolTable, node: &ast::Node) {

// }

// fn class_decl(symbol_table: &mut SymbolTable, node: &ast::Node) {}

// fn class_decl_list(symbol_table: &mut SymbolTable, node: &ast::Node) {
// }

// fn plus(symbol_table: &mut SymbolTable, node: &ast::Node) {}
// fn minus(symbol_table: &mut SymbolTable, node: &ast::Node) {}
// fn or(symbol_table: &mut SymbolTable, node: &ast::Node) {}

// fn add_op(symbol_table: &mut SymbolTable, node: &ast::Node) {}
// fn assign_op(symbol_table: &mut SymbolTable, node: &ast::Node) {}

// In this case the "leaves" of concern are reflected in the elements of the symbol table
// class_decl, func_decl,

// During the table traversal there will never be more than one -ELEMENT-
// being created, it's always in progress constructing either
// a function or a class
// the one exception is that the function of a class decl will be encountered later
// but by this point we have a parent::child accessible in the id and scopespec of the func_decl
// so we could have a get_subtable(id, scope_list)
// this would perform the search for a class if the scope list is non-empty and for the internal funtion.
// This would also work when you're
