use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::view_as::ViewAs;
use ast::Node;
use derive_getters::Getters;

// The produced view should be easily identified as equal against
#[derive(Debug, Clone, Getters)]
pub struct DimensionList {
    dimensions: Vec<Option<i64>>,
    line: usize,
    column: usize,
}

impl<'a> ViewAs<'a> for DimensionList {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Dimension list");

        let dimensions = validator.then_list_of_optional_ints()?;

        Ok(DimensionList {
            dimensions,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl DimensionList {
    pub fn as_symbol_string(&self) -> String {
        let mut result = String::new();
        for dimension in self.dimensions() {
            if let Some(int) = dimension {
                result.push_str(&format!("[{}]", int));
            } else {
                result.push_str("[]");
            }
        }
        result
    }
}
