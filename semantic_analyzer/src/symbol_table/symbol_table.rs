use crate::format_table::FormatTable;
use crate::symbol_table::*;
use std::default::Default;
use std::fmt;
use crate::SemanticError;
use output_manager::OutputConfig;
use crate::ast_validation::{FunctionDefinition, ParameterList};
use crate::ast_validation::class_member::ClassFunctionDeclaration;
use crate::Visibility;
use crate::symbol_table::Class;
use std::collections::HashSet;
use maplit::hashset;

#[derive(Debug, Clone)]
pub enum SymbolTableEntry {
    Class(class::Class),
    Function(function::Function),
    Inherit(inherit::Inherit),
    Param(param::Param),
    Local(local::Local),
    Data(data::Data),
}

pub trait TableEntryGenerator<T> {
    fn add_symbol_table_entry(validated_node: &T, global_table: &mut SymbolTable, current_scope: &Vec<String>, output_config: &mut OutputConfig) -> Result<(), SemanticError>;
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

    // TODO: re-implement

    // The non-array type
    // pub fn base_type(&self) -> &str {
    //     match self {
    //         SymbolTableEntry::Class(class) => class.id(),
    //         SymbolTableEntry::Function(function) => &function.return_type().unwrap_or("void".to_owned()),
    //         SymbolTableEntry::Inherit(_) => panic!("Getting resultant type of inheritance list"),
    //         SymbolTableEntry::Param(param) => param.actual_type(),
    //         SymbolTableEntry::Local(local) => local.actual_type(),
    //         SymbolTableEntry::Data(data) => data.actual_type(),
    //     }
    // }
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

/// Helper type for inheritance based searches
/// Determines whether the initial supplied class is searched
pub enum Search {
    Inclusive,
    Exclusive
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
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}

use crate::symbol_table::rules;

impl Extend<SymbolTableEntry> for SymbolTable {
    fn extend<T: IntoIterator<Item=SymbolTableEntry>>(&mut self, iter: T) {
        for elem in iter {
            self.add_entry(elem);
        }
    }
}

impl SymbolTable {
    /// Add an entry to the symbol table and return a mutable reference to it
    pub fn add_entry(&mut self, entry: SymbolTableEntry) -> &mut SymbolTableEntry {
        self.values.push(entry);
        self.values.last_mut().unwrap()
    }

    // pub fn add_class(&mut self, class: Class, output: &mut OutputConfig) -> Result<&mut Class, SemanticError>  {
    //     rules::class();
    //     todo!();
    // }

    // pub fn validated_add(&mut self, entry: SymbolTableEntry, output: &mut OutputConfig) -> Result<(), SemanticError> {
    //     match entry {
    //         SymbolTableEntry::Function(function) => {
    //             if function.is_class_member() {
    //                 // This is the only case where the entry ALREADY exists in the symbol table
    //                 // So we need to get that and act on it
    //                 let active_entry = rules::member_function()?;
    //                 active_entry.add_children();
    //                 return Ok(());
    //             } else if function.is_defined() {
    //                 rules::free_function(self, &function, output)?;
    //             } else {
    //                 rules::function_declaration();
    //                 return Ok(());
    //             }
    //         },
    //         SymbolTableEntry::Class(class) => rules::class(),
    //         SymbolTableEntry::Data(data) => rules::member_variable(),
    //         SymbolTableEntry::Inherit(inherit) => rules::inherit(),
    //         SymbolTableEntry::Local(local) => rules::local(),
    //         SymbolTableEntry::Param(param) => rules::param(),
    //     };

    //     match self.add_entry(entry) {
    //         SymbolTableEntry::Class(class) => class.add_children(),
    //         SymbolTableEntry::Function(function) => function.add_children(),
    //         _ => (),
    //     }

    //     Ok(())
    // }

    // fn raw_add_class(&mut self, entry: Class) -> &mut Class {
    //     // self.values.push(SymbolTableEntry::)
    //     todo!();
    // }
/*
    fn add_______<T: ToEntry>(&mut self, node: &T) {
        node.validate_addition()
        // For classes this requires some access to the other classes
        // ie. access to the global table
    }

    fn add_child____<T: ToEntry>(&mut self, node: &T, global_table: &SymbolTable) {
        
    }
*/
    /// Return the title string of this symbol table
    fn title(&self) -> String {
        let mut title = "".to_string();
        if let Some(scope) = &self.scope {
            title.push_str(&scope);
            title.push_str("::");
        }
        format!("table: {}{}", title, self.name)
    }

    /// Return the line of "=" that makes up the horizontal rules in the table string
    fn header_bar(&self, table_width: usize) -> String {
        format!("{:=<1$}", "", table_width)
    }

    pub fn new(name: &str) -> Self {
        SymbolTable {
            name: name.to_string(),
            scope: None,
            values: Vec::new(),
        }
    }

    pub fn scoped_new(name: &str, scope: Option<&str>) -> Self {
        SymbolTable {
            name: name.to_string(),
            scope: scope.map(|x| x.to_string()),
            values: Vec::new()
        }
    }

    pub fn get(&self, id: &str) -> Option<&SymbolTableEntry> {
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return Some(entry);
                }
            }
        }
        None
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

    /// Return the list of all entries in this table with the supplied identifier
    pub fn get_all(&self, id: &str) -> Vec<&SymbolTableEntry> {
        let mut result = Vec::new();
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    result.push(entry);
                }
            }
        }
        result
    }

    pub fn get_all_mut(&mut self, id: &str) -> Vec<&mut SymbolTableEntry> {
        let mut result = Vec::new();
        for entry in &mut self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    result.push(entry);
                }
            }
        }
        result
    }

    pub fn replace_class_function_declaration(&mut self, definition: Function) {
        if let Some(scope) = definition.scope() {
            // Should have a scope
            if let Some(SymbolTableEntry::Class(class)) = self.get_mut(scope) {
                class.symbol_table_mut().replace_declaration(definition);
            }
        }
    }

    fn replace_declaration(&mut self, definition: Function) {
        let position = self.values.iter().position(|x| {
            if let SymbolTableEntry::Function(function) = x {
                return *function == definition;
            }
            false
        });

        if let Some(position) = position {
            self.values[position] = SymbolTableEntry::Function(definition);
        } else {
            panic!("Asking to replace nonexistent declaration");
        }
    }

    /// Get the matching function directly
    // pub fn get_matching_function(&self, id: &str, parameters: &ParameterList) -> &mut Function {
    //     for entry in &self.values {
    //         if let Some(SymbolTableEntry::Function(function)) = entry {
    //             if function.id() == id && parameters.parameters == function.parameter_types {

    //             }
    //         }
    //     }
    // }

    /// Return the list of entries that share the identifier from the supplied class' inheritance hierarchy
    /// The purpose of this is to allow a check at the definition of a class member for shadowing, and overriding
    /// Checking for redefinition and overloading should be done with get_all
    pub fn get_all_inherited<'a>(class: &'a Class, id: &str, global_table: &'a SymbolTable) -> Vec<&'a SymbolTableEntry> {
        let mut result = Vec::new();
        let mut visited = hashset!{class.id().clone()};

        // search the inherited classes recursively
        for inherited in class.inheritance_list() {
            if let Some(SymbolTableEntry::Class(class)) = global_table.get(inherited) {
                if visited.contains(class.id()) {
                    continue;
                    // Signal a cyclic inheritance
                }
                result.extend(class.symbol_table().get_all_inherited_aux(class, id, global_table, &mut visited))
            }
        }
        result
    }

    /// Implementation of get_all_inherited
    fn get_all_inherited_aux<'a>(&'a self, class: &'a Class, id: &str, global_table: &'a SymbolTable, visited: &mut HashSet<String>) -> Vec<&'a SymbolTableEntry> {
        let mut result = Vec::new();
        
        // search the current class
        result.extend(class.symbol_table().get_all(id));
        visited.insert(class.id().clone());

        // search the inherited classes recursively
        for inherited in class.inheritance_list() {
            if let Some(SymbolTableEntry::Class(class)) = global_table.get(inherited) {
                if visited.contains(class.id()) {
                    continue;
                    // Signal a cyclic inheritance
                }

                result.extend(class.symbol_table().get_all_inherited_aux(class, id, global_table, visited))
            }
        }

        result
    }

    // pub fn get_scoped_alt<'a>(&'a self, id: &str, scope: &Vec<String>, global_table: &'a SymbolTable) -> Option<&SymbolTableEntry> {
    //     // If it has no previous type that means that the identifier is either defined locally (within the function)
    //     // or defined the owning class
    //     // or defined in any of the parents of the owning class

    //     // If it has a previous type, then that previous type is the only context, it must be a class
    //     if scope.len() == 2 {
    //         if let Some(entry) = self.get(&scope[0]) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 if let Some(entry) = class.symbol_table().get(&scope[1]) {
    //                     if let SymbolTableEntry::Function(function) = entry {
    //                         match function.symbol_table().get(id) {
    //                             Some(entry) => return Some(entry),
    //                             None => ()
    //                         }
    //                     }
    //                 }

    //                 if let Some(entry) = class.symbol_table().get(id) {
    //                     return Some(entry);
    //                 }

    //                 if let Some(entry) = self.get_first_in_inheritance(class.inheritance_list(), id, global_table) {
    //                     return Some(entry)
    //                 }
                
                
    //             }
    //         }

    //         return global_table.get(id);
    //     } else if scope.len() == 1 {
    //         if let Some(entry) = self.get(&scope[0]) {
    //             if let SymbolTableEntry::Function(function) = entry {
    //                 return function.symbol_table().get(id);
    //             }
    //         }
    //     } // else if: Bad, there's no sense in getter a global symbol
    //     None

    // }

    // pub fn get_scoped<'a>(&'a self, id: &str, scope: &Vec<String>, global_table: &'a SymbolTable) -> Option<&SymbolTableEntry> {
    //     // scope is <class>::<function> or just <function>
    //     if scope.len() == 2 {
    //         if let Some(entry) = self.get(&scope[0]) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 if let Some(entry) = class.symbol_table().get(&scope[1]) {
    //                     if let SymbolTableEntry::Function(function) = entry {
    //                         match function.symbol_table().get(id) {
    //                             Some(entry) => return Some(entry),
    //                             None => ()
    //                         }
    //                     }
    //                 }

    //                 if let Some(entry) = class.symbol_table().get(id) {
    //                     return Some(entry);
    //                 }

    //                 return self.get_first_in_inheritance(class.inheritance_list(), id, global_table)
    //             }
    //         }
    //     } else if scope.len() == 1 {
    //         if let Some(entry) = self.get(&scope[0]) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 return class.symbol_table().get(id);
    //             }
    //         }
    //     } // else if: Bad, there's no sense in getter a global symbol
    //     None
    // }

    // pub fn get_first_in_inheritance<'a>(&'a self, inheritance_list: &Vec<String>, id: &str, global_table: &'a SymbolTable) -> Option<&SymbolTableEntry> {
    //     // Scan through the current layer of inheritance
    //     for inherit in inheritance_list {
    //         if let Some(entry) = global_table.get(inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 if let Some(inner_entry) = class.symbol_table().get(id) {
    //                     return Some(inner_entry);
    //                 }
    //             }
    //         }
    //     }

    //     // If it's still not found return the recursive result for each identifier
    //     for inherit in inheritance_list {
    //         if let Some(entry) = global_table.get(inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 if let Some(inner_entry) = self.get_first_in_inheritance(class.inheritance_list(), id, global_table) {
    //                     return Some(inner_entry);
    //                 }
    //             }
    //         }
    //     }

    //     None
    // }

    // pub fn get_function_with_signature(&self, id: &str, parameter_list: &ParameterList) -> Option<&Function> {
    //     for entry in &self.values {
    //         match entry {
    //             SymbolTableEntry::Function(function) => {
    //                 if function.id() == id && parameter_list.same_as(&function.parameter_types) {
    //                     return Some(function);
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }
    //     None
    // }

    // pub fn recursive_get_shadowing(&self, id: &str, global_table: &SymbolTable) -> Vec<String> {
    //     let mut result = Vec::new();

    //     for inherit in self.collect_inherits() {
    //         if let Some(entry) = global_table.get(&inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 result.extend(class.symbol_table().recursive_get_shadowing_aux(id, global_table));
    //             }
    //         }
    //     }
    //     result
    // }

    // fn recursive_get_shadowing_aux(&self, id: &str, global_table: &SymbolTable) -> Vec<String> {
    //     let mut result = Vec::new();

    //     for entry in &self.values {
    //         if let Some(entry_id) = entry.id() {
    //             if entry_id == id {
    //                 result.push(self.name.clone());
    //             }
    //         }
    //     }

    //     for inherit in self.collect_inherits() {
    //         if let Some(entry) = global_table.get(&inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 result.extend(class.symbol_table().recursive_get_shadowing_aux(id, global_table));
    //             }
    //         }
    //     }
    //     result
    // }

    // pub fn recursive_get_function_with_signature(&self, id: &str, parameter_list: &ParameterList, global_table: &SymbolTable) -> Vec<String> {
    //     let mut result = Vec::new();
    //     for inherit in self.collect_inherits() {
    //         if let Some(entry) = global_table.get(&inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 result.extend(class.symbol_table().recursive_get_function_with_signature_aux(id, parameter_list, global_table));
    //             }
    //         }
    //     }
    //     result
    // }

    // /// Return all the parent classes that provide a declaration for the signature
    // fn recursive_get_function_with_signature_aux(&self, id: &str, parameter_list: &ParameterList, global_table: &SymbolTable) -> Vec<String> {
    //     let mut result = Vec::new();

    //     for entry in &self.values {
    //         match entry {
    //             SymbolTableEntry::Function(function) => {
    //                 if function.id() == id && parameter_list.same_as(&function.parameter_types) {
    //                     result.push(self.name.clone());
    //                     break;
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }

    //     for inherit in self.collect_inherits() {
    //         if let Some(entry) = global_table.get(&inherit) {
    //             if let SymbolTableEntry::Class(class) = entry {
    //                 result.extend(class.symbol_table().recursive_get_function_with_signature_aux(id, parameter_list, global_table));
    //             }
    //         }
    //     }
    //     result
    // }

    pub fn inherit_list_has_cycles(&self, global_table:&SymbolTable) -> bool {
        self.inherit_list_has_cycles_aux(&self.collect_inherits(), global_table, &mut Vec::new())
    }

    fn inherit_list_has_cycles_aux(&self, inherit_list: &Vec<String>, global_table:&SymbolTable, visited: &mut Vec<String>) -> bool {
        for inherit in inherit_list {
            if visited.contains(&inherit.to_string()) {
                return true;
            }
        }

        visited.extend(inherit_list.iter().map(|x| x.to_string()));
        for inherit in inherit_list {
            // get the class, get the inherit list and call again
            if let Some(entry) = global_table.get(inherit) {
                if let SymbolTableEntry::Class(class) = entry {
                    if class.symbol_table().inherit_list_has_cycles_aux(&class.symbol_table().collect_inherits(), global_table, visited) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Return all inherit entries
    fn collect_inherits(&self) -> Vec<String> {
        for entry in &self.values {
            if let SymbolTableEntry::Inherit(inherit) = entry {
                return inherit.names().clone();
            }
        }
        return Vec::new();
    }

    pub fn contains(&self, id: &str) -> bool {
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return true;
                }
            }
        }
        false
    }

    // fn get_undefined_function_in_scope(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_definition: &FunctionDefinition, output_manager: &mut OutputConfig) -> Result<Option<&mut Function>, SemanticError> {
    //     for entry in &mut self.values {
    //         match entry {
    //             SymbolTableEntry::Function(function) => {
    //                 if function.id() == id {
    //                     // println!("FOUND MATCHING FUNCTION ENTRY: {:?}", function);

    //                     if parameters.same_as(&function.parameter_types()) {
    //                         if function.defined {
    //                             match &self.scope {
    //                                 Some(scope) => {
    //                                     return Err(SemanticError::IdentifierRedefinition(format!(
    //                                         "{}:{} Function \"{}\" is already defined for the scope {}",
    //                                         parameters.line(),
    //                                         parameters.column(),
    //                                         function_name,
    //                                         scope
    //                                     )));
    //                                 },
    //                                 None => {
    //                                     return Err(SemanticError::IdentifierRedefinition(format!(
    //                                         "{}:{} Function \"{}\" is already defined",
    //                                         parameters.line(),
    //                                         parameters.column(),
    //                                         function_name,
    //                                     )));
    //                                 }
    //                             };
    //                         } else {
    //                             return Ok(Some(function));
    //                         }
    //                     } else {
    //                         // This means the function is overloading another function
    //                         SemanticError::FunctionOverload(format!(
    //                             "{}:{} Function provides an overloaded signature for \"{}\"",
    //                             function_definition.line(),
    //                             function_definition.column(),
    //                             id
    //                         )).write(output_manager);
    //                     }
    //                 }
    //             },
    //             entry => {
    //                 if let Some(entry_id) = entry.id() {
    //                     if entry_id == id {
    //                         return Err(SemanticError::IdentifierRedefinition(format!(
    //                             "{}:{} Identifier \"{}\" is already defined and names \"{}\"",
    //                             parameters.line(),
    //                             parameters.column(),
    //                             function_name,
    //                             entry
    //                         )));        
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(None)
    // }

    // fn get_undefined_function_in_class_scope(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_declaration: &ClassFunctionDeclaration, output_manager: &mut OutputConfig) -> Result<Option<&mut Function>, SemanticError> {
    //     for entry in &mut self.values {
    //         match entry {
    //             SymbolTableEntry::Function(function) => {


    //                 if function.id() == id {
    //                     // println!("FOUND MATCHING FUNCTION ENTRY: {:?}", function);

    //                     if parameters.same_as(&function.parameter_types()) {
    //                         if function.defined {
    //                             match &self.scope {
    //                                 Some(scope) => {
    //                                     return Err(SemanticError::IdentifierRedefinition(format!(
    //                                         "{}:{} Function \"{}\" is already defined for the scope {}",
    //                                         parameters.line(),
    //                                         parameters.column(),
    //                                         function_name,
    //                                         scope
    //                                     )));
    //                                 },
    //                                 None => {
    //                                     return Err(SemanticError::IdentifierRedefinition(format!(
    //                                         "{}:{} Function \"{}\" is already defined",
    //                                         parameters.line(),
    //                                         parameters.column(),
    //                                         function_name,
    //                                     )));
    //                                 }
    //                             };
    //                         } else {
    //                             return Ok(Some(function));
    //                         }
    //                     } else {
    //                         // This means the function is overloading another function
    //                         SemanticError::FunctionOverload(format!(
    //                             "{}:{} Function provides an overloaded signature for \"{}\"",
    //                             function_declaration.line(),
    //                             function_declaration.column(),
    //                             id
    //                         )).write(output_manager);
    //                     }
    //                 }
    //             },
    //             entry => {
    //                 if let Some(entry_id) = entry.id() {
    //                     if entry_id == id {
    //                         return Err(SemanticError::IdentifierRedefinition(format!(
    //                             "{}:{} Identifier \"{}\" is already defined and names \"{}\"",
    //                             parameters.line(),
    //                             parameters.column(),
    //                             function_name,
    //                             entry
    //                         )));        
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(None)
    // }


    // pub fn function_can_be_defined(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_definition: &FunctionDefinition, output_config: &mut OutputConfig) -> Result<& mut Function, SemanticError> {
    //     // We can use the supplied function_definition to see if it is in the global scope or not
    //     match function_definition.scope() {
    //         Some(_) => {
    //             // println!("Found definition for function {}", id);
    //             let result = self.get_undefined_function_in_scope(id, parameters, function_name, function_definition, output_config)?;
    //             match result {
    //                 Some(f) => {return Ok(f);},
    //                 None => {
    //                     return Err(SemanticError::DefinedButNotDeclared(format!(
    //                         "{}:{} Definition provided for undeclared member function {}",
    //                         function_definition.line(),
    //                         function_definition.column(),
    //                         function_name
    //                     )));
    //                 }
    //             }
    //         },
    //         None => {
    //             let result = self.get_undefined_function_in_scope(id, parameters, function_name, function_definition, output_config)?;
    //             // here the result mean different things
    //             // Some is bad
    //             // None is good
    //             match result {
    //                 Some(_) => {
    //                     // This means there's a declared by not defined function in the global scope...
    //                     panic!("there's a declared by not defined function in the global scope...?");
    //                 },
    //                 None => {
    //                     let f = Function::new(
    //                         function_definition.id(),
    //                         function_definition.scope(),
    //                         function_definition.return_type(),
    //                         None,
    //                         function_definition.line(),
    //                         function_definition.column(),
    //                     );

    //                     let f = Function::from(
    //                         function_definition,
    //                         None,
    //                         function
    //                     );
    //                     if let SymbolTableEntry::Function(f) = self.add_entry(SymbolTableEntry::Function(f)) {
    //                         return Ok(f);
    //                     } else {
    //                         panic!("Free function was just created in symbol table and cannot be accessed");
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub fn function_can_be_declared(&mut self, id: &str, parameters: &ParameterList, scope: &str, visibility: &Visibility, function_name: &str, function_declaration: &ClassFunctionDeclaration, output_config: &mut OutputConfig) -> Result<&mut Function, SemanticError> {
    //     // We know we are in a class scoped symbol table
    //     let result = self.get_undefined_function_in_class_scope(id, parameters, function_name, function_declaration, output_config)?;
    //     match result {
    //         Some(_) => {panic!("Function is trying to be declared but is already declared")},
    //         None => {
                
    //         }
    //     }

    //     let f = Function::new(
    //         function_declaration.id(),
    //         &Some(scope),
    //         function_declaration.return_type(),
    //         Some(*visibility),
    //     );
    //     if let SymbolTableEntry::Function(f) = self.add_entry(SymbolTableEntry::Function(f)) {
    //         return Ok(f);
    //     } else {
    //         panic!("Free function was just created in symbol table and cannot be accessed");
    //     }
    // }

    // pub fn function_is_overloading(&self, id: &str, parameters: &ParameterList) -> bool {
    //     for entry in &self.values {
    //         match entry {
    //             SymbolTableEntry::Function(function) => {
    //                 if function.id() == id && !parameters.same_as(&function.parameter_types()) {
    //                     return true;
    //                 }
    //             },
    //             _ => ()
    //         }
    //     }
    //     return false;
    // }

    // pub fn check_declared_but_not_defined_functions(&self, output_config: &mut OutputConfig) {
    //     for entry in &self.values {
    //         if let SymbolTableEntry::Class(class) = entry {
    //             for class_entry in &class.symbol_table().values {
    //                 if let SymbolTableEntry::Function(function) = class_entry {
    //                     if !function.defined {
    //                         SemanticError::DeclaredButNotDefined(format!(
    //                             "No definition for declared member function {}::{}{}",
    //                             class.id(),
    //                             function.id(),
    //                             function.signature()
    //                         )).write(output_config);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

