use crate::format_table::FormatTable;
use std::default::Default;

#[derive(Debug, Clone, Default)]
pub struct Param {
    name: String,
    data_type: String,
}

impl FormatTable for Param {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!("{:10}| {:12}| {}", "param", self.name, self.data_type)]
    }
}