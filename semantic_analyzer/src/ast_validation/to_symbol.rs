use crate::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use output_manager::OutputConfig;

pub trait ToSymbol {
    /// Checks for any semantic errors or warnings that would come from
    /// adding the validated AST node to the supplied symbol table.
    /// If a corrupting change is detected, the function returns
    fn validate_entry(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<(), SemanticError>;

    /// Generates the symbol table entry for this validated node
    fn to_symbol(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError>;

    fn to_validated_symbol(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        self.validate_entry(context, output)?;
        self.to_symbol(context, output)
    }
}
