use crate::ast_validation::Variable;
use crate::format_table::FormatTable;
use crate::symbol_table::utils;
use derive_getters::Getters;
use log::error;
use std::default::Default;
use std::fmt;
use crate::sizes;

// A variable declared in a function scope
// A name that identifies a variable
// A data type that names a primitive or compound type
// Zero or more fully specified dimensions

#[derive(Debug, Clone, Default, Getters)]
pub struct Local {
    id: String,
    data_type: String,
    dimension: Vec<i64>,
    bytes: usize,
    line: usize,
    column: usize,
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Local variable {} {}", self.type_string(), self.id)
    }
}

impl FormatTable for Local {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:10}| {:10}| {:<10}",
            "local",
            self.id,
            self.type_string(),
            self.bytes,
        )]
    }
}

impl Local {
    pub fn from(variable: &Variable) -> Self {
        let mut dimensions = Vec::new();
        for dimension in variable.dimension_list().dimensions() {
            match dimension {
                Some(dimension) => dimensions.push(*dimension),
                None => {
                    error!(
                        "Encountered an empty dimension in class member" // This should be a semantic error
                    )
                }
            }
        }

        Local {
            id: variable.id().to_string(),
            data_type: variable.data_type().to_string(),
            dimension: dimensions,
            bytes: 0,
            line: *variable.line(),
            column: *variable.column(),
        }
    }

    pub fn type_string(&self) -> String {
        utils::type_string(&self.data_type, &self.dimension)
    }

    pub fn computed_size(&mut self) -> usize {
        let size = sizes::size_of(&self.data_type, &self.dimension);
        self.bytes = size;
        size
    }

}
