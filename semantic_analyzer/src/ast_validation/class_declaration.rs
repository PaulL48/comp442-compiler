use crate::ast_validation::{ViewAs, ValidatorError, NodeValidator, InheritanceList, ClassMemberList};
use derive_getters::Getters;
use ast::Node;

#[derive(Getters)]
pub struct ClassDeclaration<'a> {
    id: &'a str,
    inheritance_list: InheritanceList<'a>,
    member_list: ClassMemberList<'a>,
    line: usize,
    column: usize,
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




