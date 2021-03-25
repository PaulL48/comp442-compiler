use crate::format_table::FormatTable;
use crate::symbol_table::SymbolTable;

#[derive(Debug, Clone)]
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
