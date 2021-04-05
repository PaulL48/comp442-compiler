use crate::ast_validation::{NodeValidator, ValidatorError, ViewAs};
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct InheritanceList<'a> {
    id_list: Vec<&'a str>,
}

impl<'a> ViewAs<'a> for InheritanceList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Inheritance list");

        let id_list = validator.then_list_of_strings()?;

        Ok(InheritanceList { id_list })
    }
}
