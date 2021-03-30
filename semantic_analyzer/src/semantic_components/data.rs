use crate::format_table::FormatTable;
use crate::visibility::Visibility;
use std::default::Default;

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub name: String,
    pub data_type: String,
    pub visibility: Visibility,
}

impl FormatTable for Data {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!("{:10}| {:12}| {:34}| {}", "data", self.name, self.data_type, self.visibility)]
    }
}

impl Data {
    fn new(name: &str, data_type: &str, visibility: &Visibility) -> Self {
        Data {
            name: name.to_string(),
            data_type: data_type.to_string(),
            visibility: *visibility
        }
    }
}
