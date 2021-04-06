use crate::ast_validation::FunctionBody;
use crate::symbol_table::function::Function;
use crate::symbol_table::local::Local;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use output_manager::OutputConfig;

pub fn convert(
    validated_node: &FunctionBody,
    global_table: &mut SymbolTable,
    output_config: &mut OutputConfig
) -> Result<(), SemanticError> {
    // It is a syntactic error to have two defintions of main, but this is a backup
    if global_table.contains("main") {
        SemanticError::IdentifierRedefinition(format!(
            "{}:{} Identifier \"{}\" is already defined in this scope",
            validated_node.line(),
            validated_node.column(),
            "main",
        )).write(output_config);
    }

    let mut entrypoint = Function::new("main", &None, &None, None);

    for local_variable in validated_node.local_variable_list().variables() {
        let entry = SymbolTableEntry::Local(Local::new(
            local_variable.id(),
            &local_variable.type_as_symbol_string(),
        ));
        entrypoint.symbol_table.add_entry(entry);
    }

    global_table.add_entry(SymbolTableEntry::Function(entrypoint));
    Ok(())
}
