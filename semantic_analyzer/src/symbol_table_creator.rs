// dispatch based on node type
use ast;
use crate::symbol_table::SymbolTable;


pub fn visit(node: &ast::Node) {
    let mut symbol_table = SymbolTable {name: "global".to_string(), .. Default::default()};
    match node.name().as_str() {
        "prog" => prog(&mut symbol_table, node),
        "funcDef" => func_def(&mut symbol_table, node),
        "classDecl" => class_decl(&mut symbol_table, node),

        _ => ()
    }
}

// fn visit_internal(symbol_table: &mut SymbolTable, node: &ast::Node) {

// }

// As we enter a particular function we can even swap the symbol table that we're passing down in the hierarchy


fn prog(symbol_table: &mut SymbolTable, node: &ast::Node) {
    // enter the second node
    if let ast::Data::Children(children) = node.data() {
        func_def_list(symbol_table, &children[1]);
    } else {
        // bad. This means the AST is malformed
    }
}

fn func_def_list(symbol_table: &mut SymbolTable, node: &ast::Node) {
    // Children are all funcDefs
    if let ast::Data::Children(children) = node.data() {
        for def_node in children {
            let mut func_symbol_table: SymbolTable = Default::default();
            func_def(&mut func_symbol_table, def_node);
            println!("{}", func_symbol_table);
        }
    } else {
        // bad
    }

}

fn func_def(symbol_table: &mut SymbolTable, node: &ast::Node) {

}

fn class_decl(symbol_table: &mut SymbolTable, node: &ast::Node) {}

fn class_decl_list(symbol_table: &mut SymbolTable, node: &ast::Node) {
}



fn plus(symbol_table: &mut SymbolTable, node: &ast::Node) {}
fn minus(symbol_table: &mut SymbolTable, node: &ast::Node) {}
fn or(symbol_table: &mut SymbolTable, node: &ast::Node) {}

fn add_op(symbol_table: &mut SymbolTable, node: &ast::Node) {}
fn assign_op(symbol_table: &mut SymbolTable, node: &ast::Node) {}

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