use crate::format_table::FormatTable;
use crate::visibility::Visibility;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use crate::ast_validation::class_member::ClassVariable;
use log::error;

// A class member variable

#[derive(Debug, Clone, Default, Getters)]
pub struct Data {
    id: String,
    data_type: String,
    visibility: Visibility,

    actual_type: String,
    dimension: Vec<i64>,
}

impl FormatTable for Data {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:12}| {:34}| {}",
            "data", self.id, self.data_type, self.visibility
        )]
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Member variable {} {} {}",
            self.visibility, self.data_type, self.id
        )
    }
}

impl Data {
    // pub fn new(id: &str, data_type: &str, visibility: &Visibility) -> Self {
    //     Data {
    //         id: id.to_string(),
    //         data_type: data_type.to_string(),
    //         visibility: *visibility,
    //         actual_type: "".to_string(),
    //         dimension: Vec::new()
    //     }
    // }

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
            data_type: format!("{}{}", class_variable.data_type().to_string(), class_variable.dimension_list().as_symbol_string()),
            visibility: *class_variable.visibility(),

            actual_type: class_variable.data_type().to_string(),
            dimension: dimensions
        }
    } 
}
