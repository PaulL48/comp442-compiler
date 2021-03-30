use crate::format_table::FormatTable;
use crate::utils::separated_list;
use std::default::Default;

#[derive(Debug, Clone, Default)]
pub struct Inherit {
    names: Vec<String>,
}

impl FormatTable for Inherit {
    fn lines(&self, _: usize) -> Vec<String> {
        if self.names.is_empty() {
            vec![format!("{:10}| none", "inherit")]
        } else {
            vec![format!("{:10}| {}", "inherit", separated_list(&self.names, ", "))]
        }
    }
}
