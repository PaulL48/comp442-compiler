use crate::format_table::FormatTable;
use crate::visibility::Visibility;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Data {
    id: String,
    data_type: String,
    visibility: Visibility,
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
    pub fn new(id: &str, data_type: &str, visibility: &Visibility) -> Self {
        Data {
            id: id.to_string(),
            data_type: data_type.to_string(),
            visibility: *visibility,
        }
    }
}
