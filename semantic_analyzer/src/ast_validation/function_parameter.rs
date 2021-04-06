use crate::ast_validation::dimension_list::DimensionList;
use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::view_as::ViewAs;
use ast::Node;
use derive_getters::Getters;

#[derive(Getters, Debug)]
pub struct FunctionParameter<'a> {
    id: &'a str,
    data_type: &'a str,
    dimension_list: DimensionList,
    line: usize,
    column: usize,
}

impl<'a> ViewAs<'a> for FunctionParameter<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Function parameter").has_children(3)?;

        let data_type = validator.then_string()?;
        let id = validator.then_string()?;
        let dimension_list = validator.then()?;

        Ok(FunctionParameter {
            id,
            data_type,
            dimension_list,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl<'a> FunctionParameter<'a> {
    pub fn as_symbol_string(&self) -> String {
        let mut result = String::new();
        result.push_str(self.data_type);
        result.push_str(&self.dimension_list.as_symbol_string());
        result
    }
}
