use crate::ast_validation::{
    ClassMemberList, InheritanceList, NodeValidator, ToSymbol, ValidatorError, ViewAs,
};
use crate::symbol_table::{Class, SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;

use std::fmt;

#[derive(Getters)]
pub struct ClassDeclaration<'a> {
    id: &'a str,
    inheritance_list: InheritanceList<'a>,
    member_list: ClassMemberList<'a>,
    line: usize,
    column: usize,
}

impl<'a> fmt::Display for ClassDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {}", self.id())
    }
}

impl<'a> ViewAs<'a> for ClassDeclaration<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Class declaration").has_children(3)?;

        let id = validator.then_string()?;
        let inheritance_list = validator.then()?;
        let member_list = validator.then()?;

        Ok(ClassDeclaration {
            id,
            inheritance_list,
            member_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl<'a> ToSymbol for ClassDeclaration<'a> {
    fn validate_entry(
        &self,
        context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        if let Some(entry) = context.get(self.id()) {
            return Err(SemanticError::new_redefinition(
                self.line(),
                self.column(),
                &self.to_string(),
                &entry.to_string(),
            ));
        }
        Ok(())
    }

    fn to_symbol(
        &self,
        _context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let mut new_entry = Class::from(self);
        self.inheritance_list
            .validate_entry(new_entry.symbol_table(), output)?;
        let inheritance_entries = self
            .inheritance_list
            .to_symbol(new_entry.symbol_table(), output)?;
        new_entry.symbol_table_mut().extend(inheritance_entries);

        self.member_list
            .validate_entry(new_entry.symbol_table(), output)?;
        let member_list = self
            .member_list
            .to_symbol(new_entry.symbol_table(), output)?;

        new_entry.symbol_table_mut().extend(member_list);

        Ok(vec![SymbolTableEntry::Class(new_entry)])
    }
}
