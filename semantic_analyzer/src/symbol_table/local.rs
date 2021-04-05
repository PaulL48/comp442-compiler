use crate::format_table::FormatTable;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Local {
    id: String,
    data_type: String,
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
    pub fn new(id: &str, data_type: &str) -> Self {
        Local {
            id: id.to_string(),
            data_type: data_type.to_string(),
        }
    }
}
