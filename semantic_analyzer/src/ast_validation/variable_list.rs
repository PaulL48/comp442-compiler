use crate::ast_validation::{NodeValidator, ToSymbol, ValidatorError, Variable, ViewAs};
use ast::Node;
use derive_getters::Getters;

use crate::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use output_manager::OutputConfig;

#[derive(Getters)]
pub struct VariableList<'a> {
    variables: Vec<Variable<'a>>,
}

impl<'a> ViewAs<'a> for VariableList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Local variable list");

        let variables = validator.then_list_of()?;

        Ok(VariableList { variables })
    }
}

impl ToSymbol for VariableList<'_> {
    fn validate_entry(
        &self,
        _context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
    fn to_symbol(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let mut results = Vec::new();
        for variable in &self.variables {
            variable.validate_entry(context, output)?;
            results.extend(variable.to_symbol(context, output)?);
        }
        Ok(results)
    }
}

impl<'a> VariableList<'a> {
    pub fn new() -> Self {
        VariableList {
            variables: Vec::new(),
        }
    }
}
