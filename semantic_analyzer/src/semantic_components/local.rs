use crate::format_table::FormatTable;

#[derive(Debug, Clone)]
pub struct Local {
    pub name: String,
    pub data_type: String,
}

impl FormatTable for Local {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!("{:10}| {:12}| {}", "local", self.name, self.data_type)]
    }
}
