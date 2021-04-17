// use crate::ast_validation::{NodeValidator, ValidatorError, ViewAs, ParameterList};
// use ast::Node;
// use derive_getters::Getters;
// use crate::symbol_table::Function;

// #[derive(Getters)]
// pub struct FunctionDeclaration<'a> {
//     id: &'a str,
//     parameter_list: ParameterList<'a>,
//     return_type: Option<&'a str>
// }

// impl<'a> std::cmp::PartialEq<Function> for FunctionDeclaration<'a> {
//     fn eq(&self, other: &Function) -> bool {
//         if self.parameter_list.parameters().len() != other.parameter_types().len() {
//             return false;
//         }

//         for (lp, rp) in self.parameter_list().parameters().iter().zip(other.parameter_types().iter()) {
//             if lp.data_type() != rp.data_type() {
//                 return false;
//             }
//         }

//         true
//     }
// }

// impl<'a> ViewAs<'a> for FunctionDeclaration<'a> {
//     fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
//         let mut validator = NodeValidator::new(node, "Function declaration").has_children(3)?;

//         let id = validator.then_string()?;
//         let parameter_list = validator.then()?;
//         let return_type = validator.then_optional_string()?;

//         Ok(FunctionDeclaration {
//             id,
//             parameter_list,
//             return_type
//         })
//     }
// }
