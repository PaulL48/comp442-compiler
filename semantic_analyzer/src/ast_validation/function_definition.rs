//! Converts a string typed AST into well typed semantic components
//! This amounts to changing non-leaf nodes into structures that provide
//! strong guarantees and names for their contents
//! Generally only those nodes whose children are single variants are defined
//! Ex. function definitions will have children in this order: id, scope, parameter list, function body
//! Whereas a relational operator can have arithmetic operations as either side, or variables, or fcalls, etc..

//! Allow AST nodes to be viewed as a more strongly type version of themselves.
//! Given an AST node such as a function declaration, many things can be wrong with the AST
//! There may be less than 5 children, the variants of the children enum may be incorrect, the names may be incorrect, etc...
//! So this provides a validation that produces a more concrete view into a node or a failure that can be propagated

use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::{FunctionBody, ParameterList};
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct FunctionDefinition<'a> {
    id: &'a str,
    scope: Option<&'a str>,
    parameter_list: ParameterList<'a>,
    return_type: Option<&'a str>,
    function_body: FunctionBody<'a>,
    line: usize,
    column: usize,
}

impl<'a> FunctionDefinition<'a> {
    pub fn view_as(node: &'a Node) -> Result<FunctionDefinition<'a>, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Function definition").has_children(5)?;

        let id = validator.then_string()?;
        let scope = validator.then_optional_string()?;
        let parameter_list = validator.then()?;
        let return_type = validator.then_optional_string()?;
        let function_body = validator.then()?;

        Ok(FunctionDefinition {
            id,
            scope,
            parameter_list,
            return_type,
            function_body,
            line: *node.line(),
            column: *node.column(),
        })
    }
}
