use crate::ast_validation::view_as::ViewAs;
use crate::ast_validation::{node_validator::NodeValidator, ValidatorError, VariableList};
use ast::Node;
use derive_getters::Getters;

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
