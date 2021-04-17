//! Given the current symbol table and a potential new entry these functions encode
//! the checks for various conditions of validity and invalidity
use crate::ast_validation::{DimensionList, ParameterList};
use crate::symbol_table::symbol_table::SymbolTableEntry;
use crate::symbol_table::Function;
use crate::SemanticError;
use output_manager::OutputConfig;

pub fn function_redefines(
    id: &str,
    parameter_list: &ParameterList,
    entries: &Vec<&SymbolTableEntry>,
    line: &usize,
    column: &usize,
    string_repr: &str,
) -> Result<(), SemanticError> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id()
                    && parameter_list == function.parameter_types()
                    && function.is_defined()
                {
                    return Err(SemanticError::new_redefinition(
                        line,
                        column,
                        string_repr,
                        &function.to_string(),
                    ));
                }
            }
            non_function => {
                return Err(SemanticError::new_redefinition(
                    line,
                    column,
                    string_repr,
                    &non_function.to_string(),
                ))
            }
        }
    }

    Ok(())
}

pub fn id_redefines(
    new_id: &str,
    entries: &Vec<&SymbolTableEntry>,
    line: &usize,
    column: &usize,
    string_repr: &str,
) -> Result<(), SemanticError> {
    for entry in entries {
        if let Some(id) = entry.id() {
            if id == new_id {
                return Err(SemanticError::new_redefinition(
                    line,
                    column,
                    string_repr,
                    &entry.to_string(),
                ));
            }
        }
    }
    Ok(())
}

pub fn get_exact<'a>(
    id: &str,
    parameter_list: &ParameterList,
    entries: &'a Vec<&'a SymbolTableEntry>,
) -> Option<&'a Function> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id()
                    && parameter_list == function.parameter_types()
                    && !function.is_defined()
                {
                    return Some(function);
                }
            }
            _ => (),
        }
    }
    None
}

pub fn get_exact_clone<'a>(
    id: &str,
    parameter_list: &ParameterList,
    entries: &Vec<&SymbolTableEntry>,
) -> Option<Function> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id()
                    && parameter_list == function.parameter_types()
                    && !function.is_defined()
                {
                    return Some(function.clone());
                }
            }
            _ => (),
        }
    }
    None
}

pub fn warn_overloading_function(
    id: &str,
    parameter_list: &ParameterList,
    entries: &Vec<&SymbolTableEntry>,
    line: &usize,
    column: &usize,
    output: &mut OutputConfig,
) {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id() && parameter_list != function.parameter_types() {
                    let err = SemanticError::new_overload(line, column, id);
                    output.add(&err.to_string(), err.line(), err.col());
                }
            }
            _ => (),
        }
    }
}

pub fn mandatory_dimensions(dimension_list: &DimensionList, id: &str) -> Result<(), SemanticError> {
    for dimension in dimension_list.dimensions() {
        if let None = dimension {
            return Err(SemanticError::new_missing_dimension(
                dimension_list.line(),
                dimension_list.column(),
                id,
            ));
        }
    }
    Ok(())
}
