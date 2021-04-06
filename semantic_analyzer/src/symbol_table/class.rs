use crate::ast_validation::{ClassDeclaration, ClassMember};
use crate::format_table::FormatTable;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::symbol_table::{Data, Function, Inherit};
use crate::SemanticError;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use std::collections::HashSet;
use output_manager::OutputConfig;

#[derive(Debug, Clone, Default, Getters)]
pub struct Class {
    id: String,
    symbol_table: SymbolTable,
}

impl FormatTable for Class {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = vec![
            format!("class | {}", self.id)
        ];
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
            symbol_table: SymbolTable::new(id, &None),
        }
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    pub fn convert(
        validated_node: &ClassDeclaration,
        global_table: &mut SymbolTable,
        output_config: &mut OutputConfig
    ) -> Result<(), SemanticError> {
        // Create a class entry in the global symbol table
        // check global table for an entry with this ID
        if global_table.contains(validated_node.id()) {
            SemanticError::IdentifierRedefinition(format!(
                "{}:{} Identifier \"{}\" is already defined in this scope",
                validated_node.line(),
                validated_node.column(),
                validated_node.id(),
            )).write(output_config);
        }

        let mut active_entry = Class::new(validated_node.id());
        
        let mut inherited_ids = HashSet::new();
        for id in validated_node.inheritance_list().id_list() {
            if inherited_ids.contains(id) {
                SemanticError::DuplicateInheritance(format!(
                    "{}:{} Identifier \"{}\" is already inherited for class {}",
                    validated_node.inheritance_list().line(),
                    validated_node.inheritance_list().column(),
                    id,
                    validated_node.id(),
                )).write(output_config);
            }
            inherited_ids.insert(id);
        }

        let inheritance_entry =
            SymbolTableEntry::Inherit(Inherit::new(validated_node.inheritance_list().id_list()));
        active_entry.symbol_table_mut().add_entry(inheritance_entry);

        // TODO: Add name shadowing warnings if
        // member variable shares name with inherited class variable
        for member in validated_node.member_list().members() {
            match member {
                ClassMember::FunctionDeclaration(function_declaration) => {
                    if active_entry.symbol_table().contains(function_declaration.id()) {
                        SemanticError::IdentifierRedefinition(format!(
                            "{}:{} Identifier \"{}\" is already defined in this scope",
                            function_declaration.line(),
                            function_declaration.column(),
                            function_declaration.id(),
                        )).write(output_config);
                    }

                    // Create a Function entry in this class' symbol table
                    let entry = SymbolTableEntry::Function(Function::new(
                        function_declaration.id(),
                        &Some(validated_node.id()),
                        function_declaration.return_type(),
                        Some(*function_declaration.visibility()),
                    ));
                    active_entry.symbol_table_mut().add_entry(entry);
                }
                ClassMember::Variable(variable) => {
                    if active_entry.symbol_table().contains(variable.id()) {
                        SemanticError::IdentifierRedefinition(format!(
                            "{}:{} Identifier \"{}\" is already defined in this scope",
                            variable.line(),
                            variable.column(),
                            variable.id(),
                        )).write(output_config);
                    }

                    // Create a variable enrty in this class' symbol table
                    let entry = SymbolTableEntry::Data(Data::new(
                        variable.id(),
                        variable.data_type(),
                        variable.visibility(),
                    ));
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
