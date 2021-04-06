use crate::ast_validation::function_parameter::FunctionParameter;
use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::view_as::ViewAs;
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct ParameterList<'a> {
    parameters: Vec<FunctionParameter<'a>>,
    line: usize,
    column: usize,
}

impl<'a> ViewAs<'a> for ParameterList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Parameter list");

        let parameters = validator.then_list_of()?;

        Ok(ParameterList { parameters,
        line: *node.line(),
        column: *node.column() })
    }
}

impl<'a> ParameterList<'a> {
    pub fn new(line: usize, column: usize) -> Self {
        ParameterList {
            parameters: Vec::new(),
            line,
            column
        }
    }

    pub fn same_as(&self, string_list: &Vec<String>) -> bool {
        self.parameters.iter().zip(string_list).all(|(lhs, rhs)| lhs.as_symbol_string() == *rhs)
    }
}
