//! Given an AST node, build a symbol table

use crate::ast_validation::{ClassDeclaration, FunctionDefinition, ProgramRoot, ToSymbol, ViewAs};

use crate::semantic_analyzer::SemanticAnalysisResults;
use crate::semantic_error::SemanticError;

use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};

use ast::Node;
use output_manager::OutputConfig;

pub fn visit(
    node: &Node,
    current_data: &mut SemanticAnalysisResults,
    output_config: &mut OutputConfig,
    _: &Vec<String>,
) {
    let result = match node.name().as_str() {
        "prog" => program_root(node, &mut current_data.symbol_table, output_config),
        "funcDef" => function_definition(node, &mut current_data.symbol_table, output_config),
        "classDecl" => class_declaration(node, &mut current_data.symbol_table, output_config),
        _ => Ok(()),
    };
    buffer_any_message(result, output_config);
}

pub fn end_of_phase(current_data: &mut SemanticAnalysisResults, output_config: &mut OutputConfig) {
    // check for inheritance problems
    //      - Check for cyclic inheritance
    //      - warn for overloads of functions higher in the class hierarchy
    //      - warn for overrides of functions higher in the class hierarchy
    //      - warn for shadowed functions and variables higher in the class hierarchy
    //
    for entry in &current_data.symbol_table.values {
        if let SymbolTableEntry::Class(class) = entry {
            if class
                .symbol_table()
                .inherit_list_has_cycles(&current_data.symbol_table)
            {
                let err = SemanticError::new_cyclic_inheritance(
                    class.line(),
                    class.column(),
                    &class.to_string(),
                );
                output_config.add(&err.to_string(), err.line(), err.col());
                continue;
            }

            for class_entry in &class.symbol_table().values {
                match class_entry {
                    SymbolTableEntry::Inherit(inherit_list) => {
                        for inherit in inherit_list.names() {
                            match current_data.symbol_table.get(inherit) {
                                Some(SymbolTableEntry::Class(_)) => (), // Ok
                                Some(other) => {
                                    let err = SemanticError::new_incorrect_type(
                                        *inherit_list.line(),
                                        *inherit_list.column(),
                                        &other.to_string(),
                                        "a class type",
                                    );
                                    output_config.add(&err.to_string(), err.line(), err.col());
                                } // Not Ok
                                None => {
                                    let err = SemanticError::new_undefined_type(
                                        inherit_list.line(),
                                        inherit_list.column(),
                                        inherit,
                                    );
                                    output_config.add(&err.to_string(), err.line(), err.col());
                                } // Not Ok
                            }
                        }
                    }
                    SymbolTableEntry::Function(function) => {
                        if !function.is_defined() {
                            let err = SemanticError::new_declared_but_not_defined(
                                function.line(),
                                function.column(),
                                &format!(
                                    "{}::{}",
                                    function
                                        .scope()
                                        .clone()
                                        .expect("Class member missing scope"),
                                    function.id()
                                ),
                            );
                            output_config.add(&err.to_string(), err.line(), err.col());
                        }

                        let matches = SymbolTable::get_all_inherited(
                            class,
                            function.id(),
                            &current_data.symbol_table,
                        );
                        for matching_entry in matches {
                            match matching_entry {
                                SymbolTableEntry::Function(matching_function) => {
                                    if matching_function == function {
                                        let err = SemanticError::new_override(
                                            function.line(),
                                            function.column(),
                                            &function.to_string(),
                                        );
                                        output_config.add(&err.to_string(), err.line(), err.col());
                                    }
                                }
                                SymbolTableEntry::Data(matching_variable) => {
                                    let err = SemanticError::new_shadowing(
                                        function.line(),
                                        function.column(),
                                        &function.to_string(),
                                        &matching_variable.to_string(),
                                    );
                                    output_config.add(&err.to_string(), err.line(), err.col());
                                }
                                _ => (),
                            }
                        }
                    }
                    SymbolTableEntry::Data(variable) => {
                        let matches = SymbolTable::get_all_inherited(
                            class,
                            variable.id(),
                            &current_data.symbol_table,
                        );
                        for matching_entry in matches {
                            match matching_entry {
                                SymbolTableEntry::Function(matching_function) => {
                                    let err = SemanticError::new_shadowing(
                                        variable.line(),
                                        variable.column(),
                                        &variable.to_string(),
                                        &matching_function.to_string(),
                                    );
                                    output_config.add(&err.to_string(), err.line(), err.col());
                                }
                                SymbolTableEntry::Data(matching_variable) => {
                                    let err = SemanticError::new_shadowing(
                                        variable.line(),
                                        variable.column(),
                                        &variable.to_string(),
                                        &matching_variable.to_string(),
                                    );
                                    output_config.add(&err.to_string(), err.line(), err.col());
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // check for declared but not defined functions
}

pub fn program_root(
    node: &ast::Node,
    global_table: &mut SymbolTable,
    output_config: &mut OutputConfig,
) -> Result<(), SemanticError> {
    let view: ProgramRoot = ViewAs::try_view_as(node);
    let entry = view.to_validated_symbol(global_table, output_config)?;
    global_table.extend(entry);
    Ok(())
}

pub fn function_definition(
    node: &ast::Node,
    global_table: &mut SymbolTable,
    output_config: &mut OutputConfig,
) -> Result<(), SemanticError> {
    let view: FunctionDefinition = ViewAs::try_view_as(node);
    let mut entry = view.to_validated_symbol(global_table, output_config)?;

    let (_id, scope) = view.get_corrected_scoped_id();
    if let Some(_) = scope {
        let mut entry = if let Some(SymbolTableEntry::Function(function)) = entry.pop() {
            function
        } else {
            panic!("entry generated from a function definition should be a function");
        };

        // Because we copied the declaration we already have and filled it with more data
        // we need to get the class entry and replace the entry for the function
        entry.set_defined();
        global_table.replace_class_function_declaration(entry);
    } else {
        global_table.extend(entry);
    }

    Ok(())
}

pub fn class_declaration(
    node: &ast::Node,
    global_table: &mut SymbolTable,
    output_config: &mut OutputConfig,
) -> Result<(), SemanticError> {
    let view: ClassDeclaration = ViewAs::try_view_as(node);
    let entry = view.to_validated_symbol(global_table, output_config)?;
    global_table.extend(entry);
    Ok(())
}

fn buffer_any_message(result: Result<(), SemanticError>, output: &mut OutputConfig) {
    if let Err(err) = result {
        output.add(&err.to_string(), err.line(), err.col());
    }
}
