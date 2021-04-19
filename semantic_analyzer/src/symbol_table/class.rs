use crate::ast_validation::ClassDeclaration;
use crate::format_table::FormatTable;
use crate::symbol_table::SymbolTable;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;

// A class is:
// An identifier that names a new compound type type
// A list of types that name other compound types
// a nested symbol table that holds member functions and variables

#[derive(Debug, Clone, Default, Getters)]
pub struct Class {
    id: String,
    inheritance_list: Vec<String>,
    symbol_table: SymbolTable,
    bytes: usize,
    line: usize,
    column: usize,
}

impl FormatTable for Class {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = vec![format!("class | {}", self.id)];
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}

impl Class {
    pub fn from(class_declaration: &ClassDeclaration) -> Self {
        Class {
            id: class_declaration.id().to_string(),
            symbol_table: SymbolTable::new(class_declaration.id()),
            inheritance_list: class_declaration
                .inheritance_list()
                .id_list()
                .iter()
                .map(|x| x.to_string())
                .collect(),
            bytes: 0,
            line: *class_declaration.line(),
            column: *class_declaration.column(),
        }
    }

    

    pub fn resultant_type(&self) -> &str {
        self.id()
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {}", self.id)
    }
}
