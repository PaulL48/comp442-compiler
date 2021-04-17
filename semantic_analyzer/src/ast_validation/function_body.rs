use crate::ast_validation::{node_validator::NodeValidator, ValidatorError, VariableList, ViewAs, ToSymbol};
use ast::Node;
use derive_getters::Getters;

use crate::symbol_table::{SymbolTable, SymbolTableEntry, Class};
use output_manager::OutputConfig;
use crate::SemanticError;
use crate::symbol_table::rules;
use std::fmt;


#[derive(Getters)]
pub struct FunctionBody<'a> {
    local_variable_list: VariableList<'a>,
    statement_list: &'a Node,
    line: usize,
    column: usize,
}

impl<'a> ViewAs<'a> for FunctionBody<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Function body").has_children(2)?;

        let local_variable_list = validator.then_optional()?;
        let statement_list = validator.then_node()?;

        let local_variable_list = match local_variable_list {
            Some(list) => list,
            None => VariableList::new()
        };

        Ok(FunctionBody {
            local_variable_list,
            statement_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl ToSymbol for FunctionBody<'_> {
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        Ok(())
    }
    
    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        Ok(self.local_variable_list.to_validated_symbol(context, output)?)
    }
}
