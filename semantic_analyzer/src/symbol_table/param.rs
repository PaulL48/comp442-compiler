use crate::ast_validation::FunctionParameter;
use crate::format_table::FormatTable;
use crate::semantic_error::SemanticError;
use crate::symbol_table::function::Function;
use crate::symbol_table::symbol_table::SymbolTable;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Param {
    id: String,
    data_type: String,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parameter {} {}", self.data_type, self.id)
    }
}

impl FormatTable for Param {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:12}| {}",
            "param", self.id, self.data_type
        )]
    }
}

impl Param {
    pub fn new(id: &str, data_type: &str) -> Self {
        Param {
            id: id.to_string(),
            data_type: data_type.to_string(),
        }
    }
}

// impl Param {
//     pub fn convert(validated_node: &FunctionParameter, function_entry: &mut Function) -> Result<(), SemanticError> {
//         // We need to reach further down to get the string value of the dimension list
//         // for the datatype

//     }
// }
