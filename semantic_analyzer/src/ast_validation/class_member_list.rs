use crate::ast_validation::{ClassMember, NodeValidator, ToSymbol, ValidatorError, ViewAs};
use crate::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;

// Each member can be either a function declaration or a
// variable declaration
#[derive(Getters)]
pub struct ClassMemberList<'a> {
    members: Vec<ClassMember<'a>>,
}

impl<'a> ViewAs<'a> for ClassMemberList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Class member list");

        let members = validator.then_list_of()?;

        Ok(ClassMemberList { members })
    }
}

impl<'a> ToSymbol for ClassMemberList<'a> {
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
        for member in &self.members {
            results.extend(member.to_validated_symbol(context, output)?);
        }
        Ok(results)
    }
}
