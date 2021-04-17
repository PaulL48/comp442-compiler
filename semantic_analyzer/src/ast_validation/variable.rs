use crate::ast_validation::{DimensionList, NodeValidator, ToSymbol, ValidatorError, ViewAs};
use ast::Node;
use derive_getters::Getters;

use crate::symbol_table::rules;
use crate::symbol_table::{Local, SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use output_manager::OutputConfig;
use std::fmt;

#[derive(Getters)]
pub struct Variable<'a> {
    id: &'a str,
    data_type: &'a str,
    dimension_list: DimensionList,
    line: usize,
    column: usize,
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function parameter {}", self.type_as_symbol_string())
    }
}

impl<'a> ViewAs<'a> for Variable<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Local variable").has_children(3)?;

        let data_type = validator.then_string()?;
        let id = validator.then_string()?;
        let dimension_list = validator.then()?;

        Ok(Variable {
            id,
            data_type,
            dimension_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl ToSymbol for Variable<'_> {
    fn validate_entry(
        &self,
        context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        let matching_entries = context.get_all(self.id());
        rules::id_redefines(
            self.id(),
            &matching_entries,
            self.line(),
            self.column(),
            &self.to_string(),
        )?;
        rules::mandatory_dimensions(&self.dimension_list, self.id())?;
        Ok(())
    }

    fn to_symbol(
        &self,
        _context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        Ok(vec![SymbolTableEntry::Local(Local::from(self))])
    }
}

impl<'a> Variable<'a> {
    pub fn type_as_symbol_string(&self) -> String {
        let mut result = String::new();
        result.push_str(self.data_type);
        result.push_str(&self.dimension_list.as_symbol_string());
        result
    }
}
