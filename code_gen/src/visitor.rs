use ast::{Data, Node};
use output_manager::OutputConfig;
use semantic_analyzer::SymbolTable;

pub fn visit(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
    match node.name().as_str() {
        "prog" => prog(node, context, output),
        _ => {}
    }
}

fn prog(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data() {
        class_list(&children[0], context, output);
        function_list(&children[1], context, output);
        entry_point(&children[2], context, output);
    } else {
        panic!();
    }

    // if let Data::Children(children) = node.data() {
    //     for child in children {
    //         visit(child, current_results);
    //     }
    // }
}

fn class_list(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {}

fn function_list(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {}

fn entry_point(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, output);
        }
    }
}

fn var_list(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, output);
        }
    }
}

fn var_decl(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
    // one challenge here, we need to create the entry but we also
    // need the name of the enclosing function to prefix the label
}
