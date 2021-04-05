use crate::ast_validation::{DimensionList, NodeValidator, ParameterList, ValidatorError, ViewAs};
use crate::Visibility;
use ast::Node;
use derive_getters::Getters;

#[derive(Getters)]
pub struct ClassFunctionDeclaration<'a> {
    visibility: Visibility,
    id: &'a str,
    parameter_list: ParameterList<'a>,
    return_type: &'a str,
}

#[derive(Getters)]
pub struct ClassVariable<'a> {
    visibility: Visibility,
    data_type: &'a str,
    id: &'a str,
    dimension_list: DimensionList,
}

pub enum ClassMember<'a> {
    FunctionDeclaration(ClassFunctionDeclaration<'a>),
    Variable(ClassVariable<'a>),
}

impl<'a> ViewAs<'a> for ClassMember<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        // This gives visibility + [varDecl | funcDecl]
        let mut validator = NodeValidator::new(node, "Class member").has_children(2)?;
        let visibility = validator.then_optional_string()?;
        let interior_node = validator.then_node()?;

        let visibility = match visibility {
            Some(s) => {
                match s {
                    "public" => Visibility::Public,
                    "private" => Visibility::Private,
                    _ => {
                        // syntactic analysis should prevent this from happening
                        return Err(ValidatorError::MalformedAst(format!(
                            "Visibility node should be \"public\" or \"private\", found \"{}\"",
                            s
                        )));
                    }
                }
            }
            None => Visibility::Private,
        };

        // This is the only node I'm aware of that needs to disambiguate based on the
        // node name itself
        match interior_node.name().as_str() {
            "varDecl" => {
                // the internal varDecl will generate a 'data' symbol table entry
                let mut internal_validator =
                    NodeValidator::new(interior_node, "Class variable").has_children(3)?;

                let data_type = internal_validator.then_string()?;
                let id = internal_validator.then_string()?;
                let dimension_list = internal_validator.then()?;

                return Ok(ClassMember::Variable(ClassVariable {
                    visibility,
                    data_type,
                    id,
                    dimension_list,
                }));
            }
            "funcDecl" => {
                let mut internal_validator =
                    NodeValidator::new(interior_node, "Class variable").has_children(3)?;

                let id = internal_validator.then_string()?;
                let parameter_list = internal_validator.then()?;
                let return_type = internal_validator.then_string()?;

                return Ok(ClassMember::FunctionDeclaration(ClassFunctionDeclaration {
                    visibility,
                    id,
                    parameter_list,
                    return_type,
                }));
            }
            _ => {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node requires second child to be a varDecl or a funcDecl, found {:?}",
                    node.name(),
                    interior_node,
                )))
            }
        }
    }
}
