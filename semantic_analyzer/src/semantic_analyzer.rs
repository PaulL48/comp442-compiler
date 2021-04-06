use crate::symbol_table;
use crate::symbol_table::symbol_table::SymbolTable;
use crate::type_checking;
use output_manager::{OutputConfig, warn_write};
use std::ptr;

pub struct SemanticAnalysisResults {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalysisResults {
    pub fn new() -> Self {
        SemanticAnalysisResults {
            symbol_table: SymbolTable::new("global", &None),
        }
    }
}

pub fn analyze(root: &ast::Node, output_config: &mut OutputConfig) -> SemanticAnalysisResults {
    let phases: Vec<Vec<fn(&ast::Node, &mut SemanticAnalysisResults, &mut OutputConfig, &Vec<String>)>> =
        vec![vec![symbol_table::visitor::visit],
        vec![type_checking::visit]];

    let mut results: SemanticAnalysisResults = SemanticAnalysisResults::new();
    let mut current_scope = Vec::new();

    let main_node = match root.data() {
        ast::Data::Children(children) => {
            &children[2]
        }
        _ => {
            panic!("No main in AST");
        }
    };

    for phase in phases {
        for visitor in phase {
            for node in root.dft() {
                // Scan for particular nodes to adjust current_scope
                match node.name().as_str() {
                    "funcDef" => {
                        // if it has a scope,
                        match node.data() {
                            ast::Data::Children(children) => {
                                match children[0].data() {
                                    ast::Data::String(id) => {
                                        current_scope = vec![id.clone()]
                                    },
                                    _ => ()
                                }
                                match children[1].data() {
                                    ast::Data::String(scope) => {
                                        current_scope.push(scope.clone());
                                    },
                                    _ => (),
                                }
                            }
                            _ => ()
                        }
                    }
                    _=>()
                }

                if ptr::eq(main_node, node) {
                    current_scope = vec!["main".to_string()];
                }


                // Somehow we need to identify when we're in the main block
                // we could check if the pointer to the FuncBody is equal to the third child of the root

                // Somehow the assumption can be made that if the scope 

                visitor(node, &mut results, output_config, &current_scope);
            }
        }
    }

    // Check the symbol table for class functions that haven't yet been defined
    results.symbol_table.check_declared_but_not_defined_functions(output_config);

    // Write results to a file 
    warn_write(&mut output_config.symbol_table_file, &output_config.symbol_table_path, &format!("{}", results.symbol_table));

    results
}
