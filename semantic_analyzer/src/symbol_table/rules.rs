//! Given the current symbol table and a potential new entry these functions encode
//! the checks for various conditions of validity and invalidity
use crate::SemanticError;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::symbol_table::{Function};
use output_manager::OutputConfig;
use crate::ast_validation::{FunctionDefinition, DimensionList, ParameterList};

/// Signal any necessary warnings or 
// pub fn free_function<'a>(
//     global_table: &SymbolTable,
//     function: &Function,
//     output: &mut OutputConfig,
// ) -> Result<(), SemanticError> {
//     let common_entries = global_table.get_all(function.id());
//     function_redefines(function, &common_entries)?;
//     warn_overloading_function(function, &common_entries, output);
//     Ok(())
// }

/// Validate the incoming member function definition
/// Return the symbol table entry that already existed due to the declaration
pub fn member_function<'a>() -> Result<&'a mut Function, SemanticError> {
    todo!();
}

pub fn function_declaration() {}

/*
Should each validation be in the element or in the symbol table 
*/

pub fn member_variable() {}
pub fn parameter() {}


pub fn local() {}

pub fn class() {}

pub fn inherit() {}

pub fn param() {}

fn _function_redefines(new_function: &Function, entries: &Vec<&SymbolTableEntry>) -> Result<(), SemanticError> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if function == new_function {
                    return Err(SemanticError::new_redefinition(new_function.line(), new_function.column(), &new_function.to_string(), &function.to_string()));
                }
            },
            non_function => return Err(SemanticError::new_redefinition(
                new_function.line(), new_function.column(), &new_function.to_string(), 
                &non_function.to_string()
            ))
        }
    }

    Ok(())
}

pub fn function_redefines(id: &str, parameter_list: &ParameterList, entries: &Vec<&SymbolTableEntry>, line: &usize, column: &usize, string_repr: &str) -> Result<(), SemanticError> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id() && parameter_list == function.parameter_types() && function.is_defined() {
                    return Err(SemanticError::new_redefinition(line, column, string_repr, &function.to_string()));
                }
            },
            non_function => return Err(SemanticError::new_redefinition(
                line, column, string_repr, 
                &non_function.to_string()
            ))
        }
    }

    Ok(())
}

pub fn id_redefines(new_id: &str, entries: &Vec<&SymbolTableEntry>, line: &usize, column: &usize, string_repr: &str) -> Result<(), SemanticError> {
    for entry in entries {
        if let Some(id) = entry.id() {
            if id == new_id {
                return Err(SemanticError::new_redefinition(line, column, string_repr, &entry.to_string()));
            }
        }
    }
    Ok(())
}

pub fn get_exact<'a>(id: &str, parameter_list: &ParameterList, entries: &'a Vec<&'a SymbolTableEntry>) -> Option<&'a Function> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id() && parameter_list == function.parameter_types() && !function.is_defined() {
                    return Some(function);
                }
            },
            _ => ()
        }
    }
    None
}

pub fn get_exact_mut<'a>(id: &str, parameter_list: &ParameterList, entries: &'a mut Vec<&'a mut SymbolTableEntry>) -> Option<&'a mut Function> {
    for entry in entries.iter_mut() {
        match entry {
            SymbolTableEntry::Function(ref mut function) => {
                if id == function.id() && parameter_list == function.parameter_types() && !function.is_defined() {
                    return Some(function);
                }
            },
            _ => ()
        }
    }
    None
}

pub fn get_exact_clone<'a>(id: &str, parameter_list: &ParameterList, entries: &Vec<&SymbolTableEntry>) -> Option<Function> {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id() && parameter_list == function.parameter_types() && !function.is_defined() {
                    return Some(function.clone());
                }
            },
            _ => ()
        }
    }
    None
}

fn _warn_overloading_function(new_function: &Function, entries: &Vec<&SymbolTableEntry>, output: &mut OutputConfig) {
    if !entries.is_empty() {
        let err = SemanticError::new_overload(new_function.line(), new_function.column(), new_function.id());
        output.add(&err.to_string(), err.line(), err.col());
    }
}

pub fn warn_overloading_function(id: &str, parameter_list: &ParameterList, entries: &Vec<&SymbolTableEntry>, line: &usize, column: &usize, output: &mut OutputConfig) {
    for entry in entries {
        match entry {
            SymbolTableEntry::Function(function) => {
                if id == function.id() && parameter_list != function.parameter_types() {
                    let err = SemanticError::new_overload(line, column, id);
                    output.add(&err.to_string(), err.line(), err.col());
                }
            },
            _ => (),
        }
    }
}

pub fn mandatory_dimensions(dimension_list: &DimensionList, id: &str) -> Result<(), SemanticError> {
    for dimension in dimension_list.dimensions() {
        if let None = dimension {
            return Err(SemanticError::new_missing_dimension(dimension_list.line(), dimension_list.column(), id));
        }
    }
    Ok(())
}