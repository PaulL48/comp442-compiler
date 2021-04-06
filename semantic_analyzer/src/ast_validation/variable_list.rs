//! Handles parameter lists and local variable lists

use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::view_as::ViewAs;
use crate::ast_validation::Variable;
use ast::Node;
use derive_getters::Getters;

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

impl<'a> VariableList<'a> {
    pub fn new() -> Self {
        VariableList {
            variables: Vec::new()
        }
    }
}
