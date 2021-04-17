use crate::ast_validation::{FunctionDefinition, FunctionBody, ClassFunctionDeclaration};
use crate::format_table::FormatTable;
use crate::symbol_table::local::Local;
use crate::symbol_table::param::Param;
use crate::symbol_table::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::utils::separated_list;
use crate::visibility::Visibility;
use crate::SemanticError;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;
use output_manager::OutputConfig;
use crate::symbol_table::utils;
use std::collections::HashSet;

// This is a free function or a member function
// with a name that specifies the function
// with an ordered list of type parameters 
// with an optional return type
// where the parameters and return type specify a type transformation
// with optionally a visibility specifier
// A symbol table holding the parameters and declared variables of the function

// Since a member function can be declared
// then there will be a flag to indicate whether the member function is defined

#[derive(Debug, Clone, Default, Getters)]
pub struct Function {
    id: String,
    scope: Option<String>,
    parameter_types: Vec<Param>,
    return_type: Option<String>,
    visibility: Option<Visibility>,
    symbol_table: SymbolTable,
    defined: bool,
    line: usize,
    column: usize,
}

// Right now the goal is:
// Given an AST node, validate and convert it into a Function struct, and add that struct to either the class symbol table or the global symbol table

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function ")?;
        if let Some(visibility) = self.visibility {
            write!(f, "{} ", visibility)?;
        }
        write!(f, "{}{}", self.id, self.signature())
    }
}

impl FormatTable for Function {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut line = format!("{:10}| {:12}| {:34}", "function", self.id, self.signature());
        if let Some(visibility) = self.visibility {
            line.push_str(&format!("| {}", visibility))
        }
        result.push(line);
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        self.id == other.id && self.parameter_types == other.parameter_types
    }
}

impl Function {
    pub fn is_class_member(&self) -> bool {
        self.scope.is_some()
    }

    pub fn is_defined(&self) -> bool {
        self.defined
    }

    // pub fn new(
    //     id: &str,
    //     scope: &Option<&str>,
    //     return_type: &Option<&str>,
    //     visibility: Option<Visibility>,
    // ) -> Self {
    //     let scope = match scope {
    //         None => "".to_string(),
    //         Some(id) => id.to_string(),
    //     };

    //     let return_type = match return_type {
    //         None => "void",
    //         Some(r_type) => r_type,
    //     };

    //     Function {
    //         id: id.to_string(),
    //         parameter_types: Vec::new(),
    //         return_type: return_type.to_string(),
    //         visibility,
    //         symbol_table: SymbolTable::new(id, &Some(scope)),
    //         defined: false,
    //     }
    // }

    /// Returns a new function with manually set id and return type
    pub fn create_main(validated_node: &FunctionBody) -> Self {
        Function {
            id: "main".to_owned(),
            scope: None,
            parameter_types: Vec::new(),
            return_type: None,
            visibility: None,
            symbol_table: SymbolTable::new("main"),
            defined: true,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn from_definition(validated_node: &FunctionDefinition) -> Self {
        let (id, scope) = validated_node.get_corrected_scoped_id();

        Function {
            id: id.to_string(),
            scope: scope.map(|s| s.to_string()),
            parameter_types: validated_node.parameter_list().parameters().iter().map(|x| Param::from(x)).collect(),
            return_type: validated_node.return_type().map(|t| t.to_string()),
            visibility: None, // Free functions have no visibility
            symbol_table: SymbolTable::scoped_new(validated_node.id(), scope),
            defined: true,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn from_declaration(validated_node: &ClassFunctionDeclaration, parent_class: &str) -> Self {        
        Function {
            id: validated_node.id().to_string(),
            scope: Some(parent_class.to_string()),
            parameter_types: validated_node.parameter_list().parameters().iter().map(|x| Param::from(x)).collect(),
            return_type: validated_node.return_type().map(|t| t.to_string()),
            visibility: Some(*validated_node.visibility()),
            symbol_table: SymbolTable::scoped_new(validated_node.id(), Some(parent_class)),
            defined: false,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn return_type_as_string(&self) -> String {
        match &self.return_type {
            Some(return_type) => return_type.clone(),
            None => "void".to_owned(),
        }
    }

    pub fn signature(&self) -> String {
        let mut result = String::new();
        result.push_str("(");
        let parameter_list_string = self.parameter_types.iter().map(|p| p.type_string()).collect::<Vec<_>>().join(",");
        result.push_str(&parameter_list_string);
        result.push(')');
        result
    }

    pub fn signature_eq(lhs: &Function, rhs: &Function) -> bool {
        lhs.id == rhs.id && lhs.parameter_types == rhs.parameter_types && lhs.return_type == rhs.return_type
    }

    // pub fn is_unique(&self, similar_functions: &[Function]) -> bool {
    //     for function in similar_functions {
    //         if Function::signature_eq(self, function) {
    //             return true;
    //         }
    //     }
    //     false
    // }

    /// When an AST function definition node is encountered, perform some semantic checks and add this
    /// function to the program symbol table
    // pub fn add_symbol_table_entry(
    //     function_definition: &FunctionDefinition,
    //     global_table: &mut SymbolTable,
    //     output_config: &mut OutputConfig,
    // ) -> Result<(), SemanticError> {
    //     let (id, scope) = function_definition.get_corrected_scoped_id();

    //     let mut active_entry = if let Some(scope) = scope {
    //         Function::get_class_function_declaration(function_definition, global_table, output_config)?
    //     } else {
    //         let mut active_entry = Function::create_free_function(function_definition, global_table, output_config)?;
    //         active_entry.add_parameters(output_config);
    //         active_entry
    //     };

    //     active_entry.add_local_variables(function_definition.function_body(), output_config);

    //     Ok(())
    // }

    // pub fn add_children(&mut self) {}

    // // pub fn add_declared_symbol_table_entry(
    // //     function_declaration: &ClassFunctionDeclaration,
    // //     global_table: &mut SymbolTable,
    // //     output_config: &mut OutputConfig
    // // ) -> Result<(), SemanticError> {

    // // }


    // /// Get a declared but not defined function entry from the specified class
    // /// Checks for redefinition within the class scope and defined but not declared
    // fn get_class_function_declaration<'a>(function_definition: &FunctionDefinition, global_table: &'a mut SymbolTable, output_config: &mut OutputConfig) -> Result<&'a mut Function, SemanticError> {
    //     // We must try to get the function that matches the signature
        
    //     let (id, scope) = function_definition.get_corrected_scoped_id();
    //     let scope = scope.expect("This should only be invoked if the function has a scope");

    //     // Find the class in the global scope
    //     let class = if let Some(SymbolTableEntry::Class(class)) = global_table.get(scope) {
    //         class
    //     } else {
    //         return Err(SemanticError::new_defined_not_declared(function_definition.line(), function_definition.column(), id, scope));
    //     };

    //     // let candidates = class.symbol
    //     // global_table.verify_free_function()

    //     // Find the declaration of the function
    //     let declaration = match class.symbol_table().get(id) {
    //         Some(SymbolTableEntry::Function(declaration)) => declaration,
    //         Some(entry) => { return Err(SemanticError::new_redefinition(function_definition.line(), function_definition.column(), &format!("{}::{}", scope, id), &entry.to_string())) },
    //         None => { return Err(SemanticError::new_defined_not_declared(function_definition.line(), function_definition.column(), id, scope)) }
    //     };

    //     if *declaration.defined() {
    //         return Err(SemanticError::new_redefinition(function_definition.line(), function_definition.column(), id, scope))
    //     }

    //     Ok(&mut declaration)
    // }

    // // fn verify_free_function_definition()

    // /// Add the free function to the global symbol table and return it
    // /// Checks for redefinition within the global scope and overloading
    // fn create_free_function<'a>(function_definition: &FunctionDefinition, global_table: &'a mut SymbolTable, output_config: &mut OutputConfig) -> Result<&'a mut Function, SemanticError> {
    //     let new_function = Function::from_definition(function_definition);
    //     let matching_functions = Vec::new();


    //     // for inheritance hierarchies the process to check is the same in each scope but must be performed 
    //     // for each of the parents

    //     // Check for non-function redefinitions
    //     for entry in global_table.get_all(function_definition.id()) {
    //         match entry {
    //             SymbolTableEntry::Function(existing_function) => {
    //                 if new_function == *existing_function {
    //                     return Err(SemanticError::new_redefinition(
    //                         new_function.line(), new_function.column(), &new_function.to_string(), 
    //                         &entry.to_string()
    //                     ));
    //                 }
    //                 matching_functions.push(existing_function);
    //             },
    //             existing_non_function => {
    //                 return Err(SemanticError::new_redefinition(
    //                     new_function.line(), new_function.column(), &new_function.to_string(), 
    //                     &existing_non_function.to_string()
    //                 ));
    //             }
    //         }
    //     }

    //     // Check for function overloading
    //     if !matching_functions.is_empty() {
    //         output_config.add(&SemanticError::new_overload(new_function.line(), new_function.column(), new_function.id()).to_string(), *new_function.line(), *new_function.column());
    //     }

    //     if let SymbolTableEntry::Function(f) = global_table.add_entry(SymbolTableEntry::Function(new_function)) {
    //         return Ok(f);
    //     } else {
    //         panic!("Function entry just created, but is not confirmed as a function entry");
    //     }
    // }

    // fn add_parameters(&mut self, output_config: &mut OutputConfig) {
    //     for parameter in self.parameter_types {
    //         if let Some(entry) = self.symbol_table.get(parameter.id()) {
    //             output_config.add(&SemanticError::new_redefinition(parameter.line(), parameter.column(), &parameter.to_string(), &entry.to_string()).to_string(), *parameter.line(), *parameter.column());
    //         } else {
    //             self.symbol_table.add_entry(SymbolTableEntry::Param(parameter));
    //         }
    //     }
    // }

    // pub fn add_local_variables(&mut self, function_body: &FunctionBody, output_config: &mut OutputConfig) {
    //     for local_variable in function_body
    //         .local_variable_list()
    //         .variables() 
    //     {
    //         let local = Local::from(local_variable);
    //         if let Some(entry) = self.symbol_table.get(local_variable.id()) {
    //             output_config.add(&SemanticError::new_redefinition(local.line(), local.column(), &local.to_string(), &entry.to_string()).to_string(), *local.line(), *local.column());
    //         } else {
    //             self.symbol_table.add_entry(SymbolTableEntry::Local(local));
    //         }
    //     }
    // }



    // When we encounter a funcDecl node we either have to create a new symbol
    // table, or retrieve the symbol table from the class and fill it in
    // pub fn convert(
    //     validated_node: &FunctionDefinition,
    //     global_table: &mut SymbolTable,
    //     output_config: &mut OutputConfig
    // ) -> Result<(), SemanticError> {
    //     let active_entry = Function::get_or_create_function_entry(validated_node, global_table, output_config)?;
    //     // There are two cases:
    //     // This is a free function. Meaning no symbol table entry yet
    //     // This is a predeclared function



    //     // TODO: This is likely broken
    //     // if the parameters are filled already (this is a pre-declared function, don't mess with it)
    //     if active_entry.parameter_types.len() == 0 {
    //         for parameter in validated_node.parameter_list().parameters() {
    //             if active_entry.symbol_table.contains(parameter.id()) {
    //                 SemanticError::IdentifierRedefinition(format!(
    //                     "{}:{} Identifier \"{}\" is already defined in this scope",
    //                     parameter.line(),
    //                     parameter.column(),
    //                     parameter.id(),
    //                 )).write(output_config);
    //             }

    //             active_entry
    //                 .parameter_types
    //                 .push(parameter.as_symbol_string());
    //             // let entry =
    //             //     SymbolTableEntry::Param(Param::new(parameter.id(), &parameter.as_symbol_string()));
    //             let entry = SymbolTableEntry::Param(Param::from(parameter));

    //             active_entry.symbol_table.add_entry(entry);
    //         }
    
    //     }

    //     // The next step would be to populate the local variables of the function
    //     for local_variable in validated_node
    //         .function_body()
    //         .local_variable_list()
    //         .variables()
    //     {
    //         if active_entry.symbol_table.contains(local_variable.id()) {
    //             SemanticError::IdentifierRedefinition(format!(
    //                 "{}:{} Identifier \"{}\" is already defined in this scope",
    //                 local_variable.line(),
    //                 local_variable.column(),
    //                 local_variable.id(),
    //             )).write(output_config);
    //         }

    //         // let entry = SymbolTableEntry::Local(Local::new(
    //         //     local_variable.id(),
    //         //     &local_variable.type_as_symbol_string(),
    //         // ));
    //         let entry = SymbolTableEntry::Local(Local::from(local_variable));
    //         active_entry.symbol_table.add_entry(entry);
    //     }

    //     active_entry.defined = true;

    //     Ok(())
    // }

    /// Return the symbol table of the function represented by the validated node.
    /// If the validated node has Some(scope) this assumes the symbol table already exists
    /// within the class named id as a Function.
    /// If the scope is None this will create a new Function element in the global symbol table
    // fn get_or_create_function_entry<'a>(
    //     validated_node: &FunctionDefinition,
    //     global_table: &'a mut SymbolTable,
    //     output_config: &mut OutputConfig
    // ) -> Result<&'a mut Function, SemanticError> {
    //     match validated_node.scope() {
    //         Some(scope) => {
    //             match global_table.get_mut(validated_node.id()) {
    //                 // Valid class scope
    //                 Some(SymbolTableEntry::Class(class)) => {
    //                     return class.symbol_table_mut().function_can_be_defined(scope, &validated_node.parameter_list(), &format!("{}::{}", validated_node.id(), scope), validated_node, output_config);
    //                 }
    //                 // Scope identifier exists but is not a class
    //                 Some(entry) => {
    //                     return Err(SemanticError::InvalidScopeIdentifier(format!(
    //                         "{}:{} Scope identifier {} names a \"{}\", and not a class",
    //                         // "Definition provided for undeclared class members {}::{} at {}:{}",
    //                         validated_node.line(),
    //                         validated_node.column(),
    //                         validated_node.id(),
    //                         entry
    //                     )));
    //                 }
    //                 // Scope identifier does not exist
    //                 None => {
    //                     return Err(SemanticError::UndefinedIdentifier(format!(
    //                         "{}:{} Class identifier {} does not exist in this scope",
    //                         validated_node.line(),
    //                         validated_node.column(),
    //                         validated_node.id(),
    //                     )))
    //                 } // Scope is specifying an undefined class
    //             }
    //         }
    //         // Free function
    //         None => {
    //             return global_table.function_can_be_defined(validated_node.id(), validated_node.parameter_list(), validated_node.id(), validated_node, output_config);
    //         }
    //     }
    // }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}
