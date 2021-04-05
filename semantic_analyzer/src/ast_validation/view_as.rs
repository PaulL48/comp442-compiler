use crate::ast_validation::node_validator::ValidatorError;
use ast::Node;

pub trait ViewAs<'a>: Sized {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError>;
}
