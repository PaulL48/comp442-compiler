use crate::ast_validation::{NodeValidator, ToSymbol, ValidatorError, ViewAs};
use crate::symbol_table::{Inherit, SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;
use std::collections::HashSet;

#[derive(Getters)]
pub struct InheritanceList<'a> {
    id_list: Vec<&'a str>,
    line: usize,
    column: usize,
}

impl<'a> ViewAs<'a> for InheritanceList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Inheritance list");

        let id_list = validator.then_list_of_strings()?;

        Ok(InheritanceList {
            id_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl<'a> ToSymbol for InheritanceList<'a> {
    fn validate_entry(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        let mut names = HashSet::new();
        for id in &self.id_list {
            if names.contains(id) {
                let err = SemanticError::new_duplicate_inheritance(
                    self.line(),
                    self.column(),
                    &context.name,
                    id,
                );
                output.add(&err.to_string(), err.line(), err.col());
            }
            names.insert(id);
        }
        Ok(())
    }

    fn to_symbol(
        &self,
        _context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        Ok(vec![SymbolTableEntry::Inherit(Inherit::new(&self.id_list))])
    }
}
