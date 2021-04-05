use crate::ast_validation::view_as::ViewAs;
use crate::ast_validation::{FunctionBody, NodeValidator, ValidatorError};
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct ProgramRoot<'a> {
    _class_declaration_list: &'a Node,
    _function_definition_list: &'a Node,
    main: FunctionBody<'a>,
}

impl<'a> ViewAs<'a> for ProgramRoot<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Program root").has_children(3)?;

        let _class_declaration_list = validator.then_node()?;
        let _function_definition_list = validator.then_node()?;
        let main = validator.then()?;

        Ok(ProgramRoot {
            _class_declaration_list,
            _function_definition_list,
            main,
        })
    }
}
