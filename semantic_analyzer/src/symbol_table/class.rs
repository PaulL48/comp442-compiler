use crate::format_table::FormatTable;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use crate::ast_validation::{ClassDeclaration, ClassMember};
use crate::SemanticError;
use crate::symbol_table::{Data, Function, Inherit};

#[derive(Debug, Clone, Default, Getters)]
pub struct Class {
    id: String,
    symbol_table: SymbolTable,
}

impl FormatTable for Class {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("class | {}", self.id));
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}

impl Class {
    pub fn new(id: &str) -> Self {
        Class {
            id: id.to_string(),
            symbol_table: SymbolTable::new(id, &None)
        }
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    pub fn convert(validated_node: &ClassDeclaration, global_table: &mut SymbolTable) -> Result<(), SemanticError> {
        // Create a class entry in the global symbol table
            // check global table for an entry with this ID
        if global_table.contains(validated_node.id()) {
            return Err(SemanticError::IdentifierRedefinition(format!(
                "{}:{} Identifier \"{}\" is already defined in this scope",
                validated_node.line(),
                validated_node.column(),
                validated_node.id(),
            )))
        }

        let mut active_entry = Class::new(validated_node.id());

        let inheritance_entry = SymbolTableEntry::Inherit(Inherit::new(validated_node.inheritance_list().id_list()));
        active_entry.symbol_table_mut().add_entry(inheritance_entry);

        // For each variable create a data member
        // For each func decl create a symbol table entry
        for member in validated_node.member_list().members() {
            match member {
                ClassMember::FunctionDeclaration(function_declaration) => {
                    // Create a Function entry in this class' symbol table
                    let entry = SymbolTableEntry::Function(Function::new(
                        function_declaration.id(),
                        &Some(validated_node.id()),
                        &Some(function_declaration.return_type()),
                        Some(*function_declaration.visibility())
                    ));
                    active_entry.symbol_table_mut().add_entry(entry);
                },
                ClassMember::Variable(variable) => {
                    // Create a variable enrty in this class' symbol table
                    let entry = SymbolTableEntry::Data(Data::new(variable.id(), variable.data_type(), variable.visibility()));
                    active_entry.symbol_table_mut().add_entry(entry);
                }
            }
        }

        global_table.add_entry(SymbolTableEntry::Class(active_entry));

        Ok(())
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {}", self.id)
    }
}
