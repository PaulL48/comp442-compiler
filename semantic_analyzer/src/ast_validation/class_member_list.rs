use crate::ast_validation::{NodeValidator, ValidatorError, ViewAs, ClassMember};
use ast::Node;
use derive_getters::Getters;

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

        Ok(ClassMemberList {
            members
        })
    }
}
