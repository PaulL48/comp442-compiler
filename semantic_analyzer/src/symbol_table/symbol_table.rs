use crate::format_table::FormatTable;
use crate::symbol_table::*;
use std::default::Default;
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

impl fmt::Display for SymbolTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolTableEntry::Class(class) => class.fmt(f),
            SymbolTableEntry::Function(function) => function.fmt(f),
            SymbolTableEntry::Inherit(inherit) => inherit.fmt(f),
            SymbolTableEntry::Param(param) => param.fmt(f),
            SymbolTableEntry::Local(local) => local.fmt(f),
            SymbolTableEntry::Data(data) => data.fmt(f),
        }
    }
}

impl Default for SymbolTableEntry {
    fn default() -> Self {
        SymbolTableEntry::Local(local::Local::default())
    }
}

impl SymbolTableEntry {
    pub fn id(&self) -> Option<&str> {
        match self {
            SymbolTableEntry::Class(class) => Some(class.id()),
            SymbolTableEntry::Function(function) => Some(function.id()),
            SymbolTableEntry::Inherit(_) => None,
            SymbolTableEntry::Param(param) => Some(param.id()),
            SymbolTableEntry::Local(local) => Some(local.id()),
            SymbolTableEntry::Data(data) => Some(data.id()),
        }
    }
}

impl FormatTable for SymbolTableEntry {
    fn lines(&self, width: usize) -> Vec<String> {
        match self {
            SymbolTableEntry::Class(c) => c.lines(width),
            SymbolTableEntry::Function(f) => f.lines(width),
            SymbolTableEntry::Inherit(i) => i.lines(width),
            SymbolTableEntry::Param(p) => p.lines(width),
            SymbolTableEntry::Local(l) => l.lines(width),
            SymbolTableEntry::Data(d) => d.lines(width),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub name: String,
    pub values: Vec<SymbolTableEntry>,
    pub scope: Option<String>,
}

impl FormatTable for SymbolTable {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = vec![
            self.header_bar(width),
            format!("| {:1$}  |", self.title(), width - 5),
            self.header_bar(width),
        ];
        result.extend(
            self.values
                .iter()
                .flat_map(|x| x.lines(width))
                .map(|x| format!("| {:1$}  |", x, width - 5)),
        );
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
    pub fn add_entry(&mut self, entry: SymbolTableEntry) -> &mut SymbolTableEntry {
        self.values.push(entry);
        self.values.last_mut().unwrap()
    }

    fn title(&self) -> String {
        let mut title = "".to_string();
        if let Some(scope) = &self.scope {
            title.push_str(&scope);
            title.push_str("::");
        }
        format!("table: {}{}", title, self.name)
    }

    fn header_bar(&self, table_width: usize) -> String {
        format!("{:=<1$}", "", table_width)
    }

    pub fn new(name: &str, scope: &Option<String>) -> Self {
        SymbolTable {
            name: name.to_string(),
            scope: scope.clone(),
            values: Vec::new(),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SymbolTableEntry> {
        for entry in &mut self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    pub fn get(&mut self, id: &str) -> Option<&SymbolTableEntry> {
        for entry in &mut self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    pub fn contains(&self, id: &str) -> bool {
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return true;
                }
            }
        }
        return false;
    }
}
