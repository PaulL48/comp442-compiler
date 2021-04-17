// //! Set of tools to determine the type of a given node
// use ast::{Node, Data};
// use crate::SymbolTable;
// use crate::SemanticError;
// use output_manager::OutputConfig;

// pub fn get_type<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     match node.name().as_str() {
//         "var" => var(node, symbol_table, current_scope, output_config),
//         "factor" => factor(node, symbol_table, current_scope, output_config),
//         "dataMember" => data_member(node, symbol_table, current_scope, None, output_config),
//         "addOp" => binary_op(node, symbol_table, current_scope, output_config),
//         "mulOp" => binary_op(node, symbol_table, current_scope, output_config),
//         _ => {panic!("Trying to get type of {}", node.name())}
//     }
// }

// pub fn get_context_type<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, previous_type: Option<&'a str>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     match node.name().as_str() {
//         "dataMember" => data_member(node, symbol_table, current_scope, previous_type, output_config),
//         "id" => id(node, symbol_table, current_scope, previous_type, output_config),
//         "fCall" => function_call(node, symbol_table, current_scope, previous_type, output_config),
//         _ => {panic!("Malformed AST");}
//     }
// }

// pub fn var<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     if let Data::Children(children) = node.data() {
//         let mut current_type: &str = get_context_type(&children[0], symbol_table, current_scope, None, output_config)?;
//         for child in children.iter().skip(1) {
//             current_type = get_context_type(child, symbol_table, current_scope, Some(current_type), output_config)?;
//         }
//         return Ok(current_type);
//     }
//     panic!("Malformed AST");

// }

// pub fn factor<'a>(node: &'a Node, _: &SymbolTable, current_scope: &Vec<String>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     // can be extracted directly from the node itself
//     match node.data() {
//         Data::Float(f) => Ok("float"),
//         Data::Integer(i) => Ok("integer"),
//         Data::String(s) => Ok("string"),
//         _ => {
//             panic!("Malformed AST");
//         }
//     }
// }

// pub fn function_call<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, previous_type: Option<&'a str>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     match node.data() {
//         Data::Children(children) => {
//             return id(&children[0], symbol_table, current_scope, previous_type, output_config);
//         },
//         _ => {
//             panic!("Malformed AST");
//         }
//     }
// }

// pub fn data_member<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, previous_type: Option<&'a str>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     // if the previous type is none, we use the name in the context of the scope to
//     // a = make_linear(...).a
//     // a = LINEAR.a
//     match node.data() {
//         Data::Children(children) => {
//             return id(&children[0], symbol_table, current_scope, previous_type, output_config);
//         },
//         _ => {
//             panic!("Malformed AST");
//         }
//     }
// }

// pub fn id<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, previous_type: Option<&'a str>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {
//     let new_scope = match previous_type {
//         Some(data_type) => {
//             let c = vec![data_type.to_string()];
//             c
//         },
//         None => current_scope.clone()
//     };

//     match node.data() {
//         Data::String(s) => {
//             let a = match previous_type {
//                 Some(_) => symbol_table.get_scoped(s, &new_scope, symbol_table),
//                 None => symbol_table.get_scoped_alt(s, &new_scope, symbol_table)
//             };

//             if let Some(entry) = a {
//                 return Ok(entry.base_type());
//             } else {
//                 return Err(SemanticError::UndefinedIdentifier(format!(
//                     "{}:{} Identifier \"{}\" is undefined",
//                     node.line(),
//                     node.column(),
//                     s
//                 )));
//             }
//         },
//         _ => {panic!("Malformed AST");}
//     }
// }

// pub fn binary_op<'a>(node: &'a Node, symbol_table: &'a SymbolTable, current_scope: &Vec<String>, output_config: &mut OutputConfig) -> Result<&'a str, SemanticError> {    
//     match node.data() {
//         Data::Children(children) => {
//             let lhs =  get_type(&children[0], symbol_table, current_scope, output_config)?;
//             let rhs = get_type(&children[2], symbol_table, current_scope, output_config)?;
//             if lhs == rhs {
//                 return Ok(lhs);
//             } else {
//                 return Err(SemanticError::BinaryMismatchedTypes(format!(
//                     "{}:{} Mismatched types in binary operator, found {} and {}",
//                     node.line(),
//                     node.column(),
//                     lhs,
//                     rhs,
//                 )))
//             }
//         },
//         _ => {
//             panic!("Malformed AST");
//         }
//     }
// }