use crate::format_table::FormatTable;
use crate::symbol_table::SymbolTable;
use std::default::Default;

#[derive(Debug, Clone, Default)]
pub struct Class {
    pub name: String,
    pub symbol_table: SymbolTable,
}

impl FormatTable for Class {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("class | {}", self.name));
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}

impl Class {
    pub fn new(name: &str, parent_scopes: &[String]) -> Self {
        Class {
            name: name.to_string(),
            symbol_table: SymbolTable::new(name, parent_scopes),
        }
    }
}
