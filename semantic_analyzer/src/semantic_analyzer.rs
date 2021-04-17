use crate::symbol_table;
use crate::symbol_table::symbol_table::SymbolTable;
use output_manager::{OutputConfig, warn_write};
use std::ptr;
use crate::ast_validation;
use crate::ast_validation::ViewAs;

pub struct SemanticAnalysisResults {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalysisResults {
    pub fn new() -> Self {
        SemanticAnalysisResults {
            symbol_table: SymbolTable::new("global"),
        }
    }
}

type Visitor = fn(&ast::Node, &mut SemanticAnalysisResults, &mut OutputConfig, &Vec<String>);
type EndOfPhaseCheck = fn(&mut SemanticAnalysisResults, &mut OutputConfig);

struct Phase {
    visitor: Visitor,
    end_of_phase: EndOfPhaseCheck
}

impl Phase {
    pub fn new(visitor: Visitor, eopc: EndOfPhaseCheck) -> Self {
        Phase {
            visitor,
            end_of_phase: eopc,
        }
    }
}

pub fn analyze(root: &ast::Node, output_config: &mut OutputConfig) -> SemanticAnalysisResults {
    // let phases: Vec<Vec<fn(&ast::Node, &mut SemanticAnalysisResults, &mut OutputConfig, &Vec<String>)>> =
    //     vec![vec![symbol_table::visitor::visit],
    //     vec![type_checking::visit]];

    let phases: Vec<Vec<Phase>> = vec![
        vec![Phase::new(symbol_table::visitor::visit, symbol_table::visitor::end_of_phase)]
    ];

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
                // set_scope(node, main_node, &mut current_scope);


                // Somehow we need to identify when we're in the main block
                // we could check if the pointer to the FuncBody is equal to the third child of the root

                // Somehow the assumption can be made that if the scope 

                (visitor.visitor)(node, &mut results, output_config, &current_scope);
                
            }
            (visitor.end_of_phase)(&mut results, output_config);
        }
    }

    // Check the symbol table for class functions that haven't yet been defined
    // results.symbol_table.check_declared_but_not_defined_functions(output_config);

    // Write results to a file 
    warn_write(&mut output_config.symbol_table_file, &output_config.symbol_table_path, &format!("{}", results.symbol_table));

    results
}

fn set_scope(node: &ast::Node, main_node: &ast::Node, scope_list: &mut Vec<String>) {
    if ptr::eq(main_node, node) {
        scope_list.clear();
        scope_list.push("main".to_owned());
        return;
    }

    match node.name().as_str() {
        "funcDef" => {
            scope_list.clear();
            match ast_validation::FunctionDefinition::view_as(node) {
                Ok(function_declaration) => {
                    let (id, scope) = function_declaration.get_corrected_scoped_id();
                    if let Some(scope) = scope {
                        scope_list.push(scope.to_owned());
                    }
                    scope_list.push(id.to_owned());
                },
                Err(err) => panic!("{}", err)
            }
        },
        "classDecl" => {
            scope_list.clear();
            match ast_validation::ClassDeclaration::view_as(node) {
                Ok(class_declaration) => {
                    scope_list.push(class_declaration.id().to_owned());
                },
                Err(err) => panic!("{}", err)

            }
        },
        // "funcDecl" => {
        //     match ast_validation::FunctionDeclaration::view_as(node) {
        //         Ok(function_declaration) => {
        //             // The list is specifically not cleared since we want
        //             // to preserve the classDecl that preceded this
        //             scope_list.push(function_declaration.id().to_owned());
        //         },
        //         Err(err) => panic!("{}", err)

        //     }
        // },
        _ => ()
    }
}
