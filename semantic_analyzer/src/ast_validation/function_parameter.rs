use crate::ast_validation::{DimensionList, NodeValidator, ValidatorError, ToSymbol, ViewAs};
use ast::Node;
use derive_getters::Getters;
use crate::symbol_table::{SymbolTable, SymbolTableEntry, Param};
use output_manager::OutputConfig;
use crate::SemanticError;
use crate::symbol_table::rules;
use std::fmt;

#[derive(Getters, Debug)]
pub struct FunctionParameter<'a> {
    id: &'a str,
    data_type: &'a str,
    dimension_list: DimensionList,
    line: usize,
    column: usize,
}

impl fmt::Display for FunctionParameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_symbol_string())
    }
}

impl<'a> ViewAs<'a> for FunctionParameter<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Function parameter").has_children(3)?;

        let data_type = validator.then_string()?;
        let id = validator.then_string()?;
        let dimension_list = validator.then()?;

        Ok(FunctionParameter {
            id,
            data_type,
            dimension_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl<'a> ToSymbol for FunctionParameter<'a> {
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        let matching_entries = context.get_all(self.id());
        rules::id_redefines(self.id(), &matching_entries, self.line(), self.column(), &self.to_string())?;
        // parameters are the one place dimensions are not required
        Ok(())
    }
    
    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        Ok(vec![SymbolTableEntry::Param(Param::from(self))])
    }
}

impl<'a> FunctionParameter<'a> {
    pub fn as_symbol_string(&self) -> String {
        let mut result = String::new();
        result.push_str(self.data_type);
        result.push_str(&self.dimension_list.as_symbol_string());
        result
    }
}
