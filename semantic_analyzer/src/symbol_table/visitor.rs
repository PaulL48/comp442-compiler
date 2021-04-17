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

pub fn end_of_phase(
    current_data: &mut SemanticAnalysisResults,
    output_config: &mut OutputConfig,
) {

    // check for inheritance problems
    //      - Check for cyclic inheritance
    //      - warn for overloads of functions higher in the class hierarchy
    //      - warn for overrides of functions higher in the class hierarchy
    //      - warn for shadowed functions and variables higher in the class hierarchy
    //
    for entry in &current_data.symbol_table.values {
        if let SymbolTableEntry::Class(class) = entry {
            if class.symbol_table().inherit_list_has_cycles(&current_data.symbol_table) {
                let err = SemanticError::new_cyclic_inheritance(class.line(), class.column(), &class.to_string());
                output_config.add(&err.to_string(), err.line(), err.col());
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
        let entry = if let Some(SymbolTableEntry::Function(function)) = entry.pop() {
            function
        } else {
            panic!("entry generated from a function definition should be a function");
        };

        // Because we copied the declaration we already have and filled it with more data
        // we need to get the class entry and replace the entry for the function
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
