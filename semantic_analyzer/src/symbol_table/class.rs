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
    line: usize,
    column: usize,
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
    pub fn from(class_declaration: &ClassDeclaration) -> Self {
        Class {
            id: class_declaration.id().to_string(),
            symbol_table: SymbolTable::new(class_declaration.id()),
            inheritance_list: class_declaration.inheritance_list().id_list().iter().map(|x| x.to_string()).collect(),
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

    // pub fn add_children(&mut self) {
    //     // This is going to accept the validated AST node
        
    // }

    // pub fn add_symbol_table_entry(
    //     class_declaration: &ClassDeclaration,
    //     global_table: &mut SymbolTable,
    //     output_config: &mut OutputConfig,
    // ) -> Result<(), SemanticError> {
    //     let class = Class::from(class_declaration);
    //     if let Some(entry) = global_table.get(class.id()) {
    //         return Err(SemanticError::new_redefinition(class.line(), class.column(), &class.to_string(), &entry.to_string()));
    //     }

    //     // create a class entry in the global scope
    //     let class = global_table.add_entry(SymbolTableEntry::Class(class));
    //     let class = if let SymbolTableEntry::Class(class) = class {
    //         class
    //     } else {
    //         panic!("Cannot retrieve class entry we just created");
    //     };

    //     class.add_inherited(class_declaration, output_config);
    //     class.add_members(class_declaration, global_table);
    //     Ok(())
    // }

    // fn add_inherited(&mut self, class_declaration: &ClassDeclaration, output_config: &mut OutputConfig) {
    //     let names = HashSet::new();
    //     for inherited in class_declaration.inheritance_list().id_list() {
    //         if names.contains(inherited) {
    //             let error = SemanticError::DuplicateInheritance(format!(
    //                 "{}:{} Identifier \"{}\" is already inherited for the class \"{}\"",
    //                 *class_declaration.inheritance_list().line(),
    //                 *class_declaration.inheritance_list().column(),
    //                 inherited,
    //                 class_declaration.id(),
    //             ));
    //             output_config.add(&error.to_string(), *class_declaration.inheritance_list().line(), *class_declaration.inheritance_list().column());
    //         }
    //         names.insert(inherited);
    //     }
    // }

    // fn add_members(&mut self, class_declaration: &ClassDeclaration, global_table: &mut SymbolTable) -> Result<(), SemanticError> {
    //     for member in class_declaration.member_list().members() {
    //         match member {
    //             ClassMember::FunctionDeclaration(function_declaration) => self.add_function_declaration(function_declaration, global_table)?,
    //             ClassMember::Variable(variable) => self.add_variable(variable)?,
    //         }
    //     }
    //     Ok(())
    // }

    // fn add_function_declaration(&mut self, function_declaration: &ClassFunctionDeclaration, global_table: &mut SymbolTable) -> Result<(), SemanticError> {
    //     // check for redefinitions and overloading
    //     let new_function = Function::from_declaration(function_declaration, self.id());
    //     match self.symbol_table().get(function_declaration.id()) {
    //         Some(entry) => 
    //     }

    //     self.symbol_table.get(function_declaration.id())


    //     let matching_entries = SymbolTable::get_all_inherited(self, function_declaration.id(), global_table);
    //     for entry in matching_entries {
    //         match entry {
    //             SymbolTableEntry::Data(member_variable) => (),
    //             SymbolTableEntry::Function(member_function) => (),
    //             entry => ()
    //         }
    //     }

    //     Ok(())
    // }

    // fn add_variable(&mut self, variable: &ClassVariable) -> Result<(), SemanticError> {

    // }

    // pub fn convert(
    //     validated_node: &ClassDeclaration,
    //     global_table: &mut SymbolTable,
    //     output_config: &mut OutputConfig
    // ) -> Result<(), SemanticError> {
    //     // Create a class entry in the global symbol table
    //     // check global table for an entry with this ID
    //     if global_table.contains(validated_node.id()) {
    //         SemanticError::IdentifierRedefinition(format!(
    //             "{}:{} Identifier \"{}\" is already defined in this scope",
    //             validated_node.line(),
    //             validated_node.column(),
    //             validated_node.id(),
    //         )).write(output_config);
    //     }

    //     let mut active_entry = Class::from(validated_node);
        
    //     let mut inherited_ids = HashSet::new();
    //     for id in validated_node.inheritance_list().id_list() {
    //         if inherited_ids.contains(id) {
    //             SemanticError::DuplicateInheritance(format!(
    //                 "{}:{} Identifier \"{}\" is already inherited for class {}",
    //                 validated_node.inheritance_list().line(),
    //                 validated_node.inheritance_list().column(),
    //                 id,
    //                 validated_node.id(),
    //             )).write(output_config);
    //         }
    //         inherited_ids.insert(id);
    //     }

    //     let inheritance_entry =
    //         SymbolTableEntry::Inherit(Inherit::new(validated_node.inheritance_list().id_list()));
    //     active_entry.symbol_table_mut().add_entry(inheritance_entry);

    //     // TODO: Add name shadowing warnings if
    //     // member variable shares name with inherited class variable
    //     for member in validated_node.member_list().members() {
    //         match member {
    //             ClassMember::FunctionDeclaration(function_declaration) => {
    //                 // if active_entry.symbol_table().contains(function_declaration.id()) {
    //                 //     SemanticError::IdentifierRedefinition(format!(
    //                 //         "{}:{} Identifier \"{}\" is already defined in this scope",
    //                 //         function_declaration.line(),
    //                 //         function_declaration.column(),
    //                 //         function_declaration.id(),
    //                 //     )).write(output_config);
    //                 // }
    //                 // println!("{:?}", function_declaration);

    //                 // Create a Function entry in this class' symbol table
    //                 // Manually search the inherited namespaces for 
    //                 // Given a set of inherited identifiers check if those classes define identical identifiers
    //                 let overriding = active_entry.symbol_table.recursive_get_function_with_signature(function_declaration.id(), function_declaration.parameter_list(), global_table);
    //                 for over in &overriding {
    //                     SemanticError::FunctionOverload(format!(
    //                         "{}:{} Member function \"{}::{}\" is overriding inherited method from {}",
    //                         function_declaration.line(),
    //                         function_declaration.column(),
    //                         validated_node.id(),
    //                         function_declaration.id(),
    //                         over
    //                     )).write(output_config);
    //                 }
    //                 if overriding.len() == 0 {
    //                     let shadowing = active_entry.symbol_table.recursive_get_shadowing(function_declaration.id(), global_table);
    //                     for shadow in shadowing {
    //                         SemanticError::FunctionOverload(format!(
    //                             "{}:{} Member function \"{}::{}\" is shadowing inherited identifier from {}",
    //                             function_declaration.line(),
    //                             function_declaration.column(),
    //                             validated_node.id(),
    //                             function_declaration.id(),
    //                             shadow
    //                         )).write(output_config);
    //                     }
    //                 }
                    
    //                 let function_entry = active_entry.symbol_table_mut().function_can_be_declared(function_declaration.id(), function_declaration.parameter_list(), validated_node.id(), function_declaration.visibility(), &format!("{}::{}", validated_node.id(), function_declaration.id()), function_declaration, output_config)?;
    //                 for parameter in function_declaration.parameter_list().parameters() {
    //                     if function_entry.symbol_table.contains(parameter.id()) {
    //                         SemanticError::IdentifierRedefinition(format!(
    //                             "{}:{} Identifier \"{}\" is already defined in this scope",
    //                             parameter.line(),
    //                             parameter.column(),
    //                             parameter.id(),
    //                         )).write(output_config);
    //                     }
            
    //                     function_entry
    //                         .parameter_types
    //                         .push(parameter.as_symbol_string());
    //                     // let entry =
    //                     //     SymbolTableEntry::Param(Param::new(parameter.id(), &parameter.as_symbol_string()));
    //                     let entry = SymbolTableEntry::Param(Param::from(parameter));
                    
    //                     function_entry.symbol_table.add_entry(entry);
    //                 }
    //             }
    //             ClassMember::Variable(variable) => {
    //                 // Given an identifier and an inheritance list, check recursively if the name shadows 
    //                 let shadowing = active_entry.symbol_table.recursive_get_shadowing(variable.id(), global_table);
    //                 for shadow in shadowing {
    //                     SemanticError::FunctionOverload(format!(
    //                         "{}:{} Member variable \"{}::{}\" is shadowing inherited identifier from {}",
    //                         variable.line(),
    //                         variable.column(),
    //                         validated_node.id(),
    //                         variable.id(),
    //                         shadow
    //                     )).write(output_config);
    //                 }

    //                 if active_entry.symbol_table().contains(variable.id()) {
    //                     SemanticError::IdentifierRedefinition(format!(
    //                         "{}:{} Identifier \"{}\" is already defined in this scope",
    //                         variable.line(),
    //                         variable.column(),
    //                         variable.id(),
    //                     )).write(output_config);
    //                 }


    //                 // Create a variable enrty in this class' symbol table
    //                 // let entry = SymbolTableEntry::Data(Data::new(
    //                 //     variable.id(),
    //                 //     variable.data_type(),
    //                 //     variable.visibility(),
    //                 // ));
    //                 let entry = SymbolTableEntry::Data(Data::from(variable));
    //                 active_entry.symbol_table_mut().add_entry(entry);
    //             }
    //         }
    //     }

    //     global_table.add_entry(SymbolTableEntry::Class(active_entry));

    //     Ok(())
    // }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {}", self.id)
    }
}
