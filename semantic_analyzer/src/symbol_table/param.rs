use crate::format_table::FormatTable;
use crate::ast_validation::FunctionParameter;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use output_manager::OutputConfig;

use crate::symbol_table::{utils, SymbolTable};

// This is the entry in a function symbol table for a parameter of the function
// an identifier that specifies a variable
// a data type that names a primitive or compound type
// Either: A list of zero or more dimension specifiers, or a list of zero or more NONEs

// Generally speaking there are statements and declarations
// A declaration introduces: A compound type, a variable declaration, a parameter, a member variable
// a statement uses the elements of the previous declarations

#[derive(Debug, Clone, Default, Getters)]
pub struct Param {
    id: String,
    data_type: String,
    dimension: Vec<Option<i64>>,
    line: usize,
    column: usize,
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
            "param", self.id, self.type_string()
        )]
    }
}

impl PartialEq for Param {
    fn eq(&self, other: &Param) -> bool {
        self.data_type == other.data_type
    }
}

impl PartialEq<FunctionParameter<'_>> for Param  {
    fn eq(&self, other: &FunctionParameter) -> bool {
        self.data_type == other.data_type()
    }
}

impl Param {
    pub fn from(function_parameter: &FunctionParameter) -> Self {
        Param {
            id: function_parameter.id().to_string(),
            data_type: function_parameter.data_type().to_string(),
            dimension: function_parameter.dimension_list().dimensions().clone(),
            line: *function_parameter.line(),
            column: *function_parameter.column(),
        }
    }

    pub fn type_string(&self) -> String {
        utils::parameter_type_string(&self.data_type, &self.dimension)
    }
}
