use crate::ast_validation::class_member::ClassVariable;
use crate::format_table::FormatTable;
use crate::sizes;
use crate::symbol_table::utils;
use crate::visibility::Visibility;
use derive_getters::Getters;
use log::error;
use std::default::Default;
use std::fmt;

// A class member variable

// This is a declaration of a variable name
// with a name that specifier a primitive or compound type
// with zero or more fully specified dimensions
// with a visibility specifier

#[derive(Debug, Clone, Default, Getters)]
pub struct Data {
    id: String,
    visibility: Visibility,

    data_type: String,
    dimension: Vec<i64>,
    bytes: usize,
    line: usize,
    column: usize,
}

impl FormatTable for Data {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:10}| {:10}| {:10}| {:<10}",
            "data",
            self.id,
            self.type_string(),
            self.visibility,
            self.bytes,
        )]
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Member variable {} {} {}",
            self.visibility,
            self.type_string(),
            self.id
        )
    }
}

impl Data {
    pub fn from(class_variable: &ClassVariable) -> Self {
        let mut dimensions = Vec::new();
        for dimension in class_variable.dimension_list().dimensions() {
            match dimension {
                Some(dimension) => dimensions.push(*dimension),
                None => {
                    error!(
                        "Encountered an empty dimension in class member" // This should be a semantic error
                    )
                }
            }
        }

        Data {
            id: class_variable.id().to_string(),
            visibility: *class_variable.visibility(),
            data_type: class_variable.data_type().to_string(),
            dimension: dimensions,
            bytes: 0,
            line: *class_variable.line(),
            column: *class_variable.column(),
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
