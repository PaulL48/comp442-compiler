use crate::ast_validation::view_as::ViewAs;
use crate::ast_validation::{DimensionList, NodeValidator, ValidatorError};
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct Variable<'a> {
    id: &'a str,
    data_type: &'a str,
    dimension_list: DimensionList,
}

impl<'a> ViewAs<'a> for Variable<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Local variable").has_children(3)?;

        let data_type = validator.then_string()?;
        let id = validator.then_string()?;
        let dimension_list = validator.then()?;

        Ok(Variable {
            id,
            data_type,
            dimension_list,
        })
    }
}

impl<'a> Variable<'a> {
    pub fn type_as_symbol_string(&self) -> String {
        let mut result = String::new();
        result.push_str(self.data_type);
        result.push_str(&self.dimension_list.as_symbol_string());
        result
    }
}
