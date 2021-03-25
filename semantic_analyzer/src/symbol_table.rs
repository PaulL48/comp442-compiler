use crate::format_table::{FormatTable};
use crate::semantic_components::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SymbolTableEntry {
    Class(class::Class),
    Function(function::Function),
    Inherit(inherit::Inherit),
    Param(param::Param),
    Local(local::Local),
    Data(data::Data),
}

impl FormatTable for SymbolTableEntry {
    fn lines(&self, width: usize) -> Vec<String> {
        match self {
            SymbolTableEntry::Class(c) => c.lines(width),
            SymbolTableEntry::Function(f) => f.lines(width),
            SymbolTableEntry::Inherit(i) => i.lines(width),
            SymbolTableEntry::Param(p) => p.lines(width),
            SymbolTableEntry::Local(l) => l.lines(width),
            SymbolTableEntry::Data(d) => d.lines(width)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub name: String,
    pub values: Vec<SymbolTableEntry>,
    pub parent_scopes: Vec<String>,
}

impl FormatTable for SymbolTable {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = vec![
            self.header_bar(width),
            format!("| {:1$}  |", self.title(), width - 5),
            self.header_bar(width)
        ];
        result.extend(self.values.iter().flat_map(|x| x.lines(width)).map(|x| format!("| {:1$}  |", x, width - 5)));
        result.push(self.header_bar(width));
        result
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in self.lines(83) {
            write!(f, "{}\n", l)?;
        }
        Ok(())
    }
}

impl SymbolTable {
    fn title(&self) -> String {
        let mut title = "".to_string();
        for parent_scope in &self.parent_scopes {
            title.push_str(parent_scope);
            title.push_str("::");
        }
        format!("table: {}{}", title, self.name)
    }

    fn header_bar(&self, table_width: usize) -> String {
        format!("{:=<1$}", "", table_width)
    }
}
