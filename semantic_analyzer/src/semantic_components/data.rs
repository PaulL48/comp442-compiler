use crate::format_table::FormatTable;
use crate::visibility::Visibility;

#[derive(Debug, Clone)]
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
