use crate::format_table::FormatTable;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use crate::ast_validation::Variable;
use log::error;

#[derive(Debug, Clone, Default, Getters)]
pub struct Local {
    id: String,
    data_type: String,

    actual_type: String,
    dimension: Vec<i64>
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Local variable {} {}", self.data_type, self.id)
    }
}

impl FormatTable for Local {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:12}| {}",
            "local", self.id, self.data_type
        )]
    }
}

impl Local {
    // pub fn new(id: &str, data_type: &str) -> Self {
    //     Local {
    //         id: id.to_string(),
    //         data_type: data_type.to_string(),
    //     }
    // }

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
            data_type: format!("{}{}", variable.data_type().to_string(), variable.dimension_list().as_symbol_string()),
            actual_type: variable.data_type().to_string(),
            dimension: dimensions
        }
    }
}
