use crate::ast_validation::{NodeValidator, ValidatorError, Variable, ViewAs, ToSymbol};
use ast::Node;
use derive_getters::Getters;

use crate::symbol_table::{SymbolTable, SymbolTableEntry, Class};
use output_manager::OutputConfig;
use crate::SemanticError;
use crate::symbol_table::rules;


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
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        Ok(())
    
    }
    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
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
            variables: Vec::new()
        }
    }
}
