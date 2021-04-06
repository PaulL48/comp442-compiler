use crate::format_table::FormatTable;
use crate::symbol_table::*;
use std::default::Default;
use std::fmt;
use crate::SemanticError;
use output_manager::OutputConfig;
use crate::ast_validation::{FunctionDefinition, ParameterList};
use crate::ast_validation::class_member::ClassFunctionDeclaration;
use crate::Visibility;

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
            writeln!(f, "{}", l)?;
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
        false
    }

    fn get_undefined_function_in_scope(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_definition: &FunctionDefinition, output_manager: &mut OutputConfig) -> Result<Option<&mut Function>, SemanticError> {
        for entry in &mut self.values {
            match entry {
                SymbolTableEntry::Function(function) => {
                    if function.id() == id {
                        println!("FOUND MATCHING FUNCTION ENTRY: {:?}", function);

                        if parameters.same_as(&function.parameter_types()) {
                            if function.defined {
                                match &self.scope {
                                    Some(scope) => {
                                        return Err(SemanticError::IdentifierRedefinition(format!(
                                            "{}:{} Function \"{}\" is already defined for the scope {}",
                                            parameters.line(),
                                            parameters.column(),
                                            function_name,
                                            scope
                                        )));
                                    },
                                    None => {
                                        return Err(SemanticError::IdentifierRedefinition(format!(
                                            "{}:{} Function \"{}\" is already defined",
                                            parameters.line(),
                                            parameters.column(),
                                            function_name,
                                        )));
                                    }
                                };
                            } else {
                                return Ok(Some(function));
                            }
                        } else {
                            // This means the function is overloading another function
                            SemanticError::FunctionOverload(format!(
                                "{}:{} Function provides an overloaded signature for \"{}\"",
                                function_definition.line(),
                                function_definition.column(),
                                id
                            )).write(output_manager);
                        }
                    }
                },
                entry => {
                    if let Some(entry_id) = entry.id() {
                        if entry_id == id {
                            return Err(SemanticError::IdentifierRedefinition(format!(
                                "{}:{} Identifier \"{}\" is already defined and names \"{}\"",
                                parameters.line(),
                                parameters.column(),
                                function_name,
                                entry
                            )));        
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn get_undefined_function_in_class_scope(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_declaration: &ClassFunctionDeclaration, output_manager: &mut OutputConfig) -> Result<Option<&mut Function>, SemanticError> {
        for entry in &mut self.values {
            match entry {
                SymbolTableEntry::Function(function) => {


                    if function.id() == id {
                        println!("FOUND MATCHING FUNCTION ENTRY: {:?}", function);

                        if parameters.same_as(&function.parameter_types()) {
                            if function.defined {
                                match &self.scope {
                                    Some(scope) => {
                                        return Err(SemanticError::IdentifierRedefinition(format!(
                                            "{}:{} Function \"{}\" is already defined for the scope {}",
                                            parameters.line(),
                                            parameters.column(),
                                            function_name,
                                            scope
                                        )));
                                    },
                                    None => {
                                        return Err(SemanticError::IdentifierRedefinition(format!(
                                            "{}:{} Function \"{}\" is already defined",
                                            parameters.line(),
                                            parameters.column(),
                                            function_name,
                                        )));
                                    }
                                };
                            } else {
                                return Ok(Some(function));
                            }
                        } else {
                            // This means the function is overloading another function
                            SemanticError::FunctionOverload(format!(
                                "{}:{} Function provides an overloaded signature for \"{}\"",
                                function_declaration.line(),
                                function_declaration.column(),
                                id
                            )).write(output_manager);
                        }
                    }
                },
                entry => {
                    if let Some(entry_id) = entry.id() {
                        if entry_id == id {
                            return Err(SemanticError::IdentifierRedefinition(format!(
                                "{}:{} Identifier \"{}\" is already defined and names \"{}\"",
                                parameters.line(),
                                parameters.column(),
                                function_name,
                                entry
                            )));        
                        }
                    }
                }
            }
        }

        Ok(None)
    }


    pub fn function_can_be_defined(&mut self, id: &str, parameters: &ParameterList, function_name: &str, function_definition: &FunctionDefinition, output_config: &mut OutputConfig) -> Result<& mut Function, SemanticError> {
        // We can use the supplied function_definition to see if it is in the global scope or not
        match function_definition.scope() {
            Some(_) => {
                println!("Found definition for function {}", id);
                let result = self.get_undefined_function_in_scope(id, parameters, function_name, function_definition, output_config)?;
                match result {
                    Some(f) => {return Ok(f);},
                    None => {
                        return Err(SemanticError::DefinedButNotDeclared(format!(
                            "{}:{} Definition provided for undeclared member function {}",
                            function_definition.line(),
                            function_definition.column(),
                            function_name
                        )));
                    }
                }
            },
            None => {
                let result = self.get_undefined_function_in_scope(id, parameters, function_name, function_definition, output_config)?;
                // here the result mean different things
                // Some is bad
                // None is good
                match result {
                    Some(_) => {
                        // This means there's a declared by not defined function in the global scope...
                        panic!("there's a declared by not defined function in the global scope...?");
                    },
                    None => {
                        let f = Function::new(
                            function_definition.id(),
                            function_definition.scope(),
                            function_definition.return_type(),
                            None
                        );
                        if let SymbolTableEntry::Function(f) = self.add_entry(SymbolTableEntry::Function(f)) {
                            return Ok(f);
                        } else {
                            panic!("Free function was just created in symbol table and cannot be accessed");
                        }
                    }
                }
            }
        }
    }

    pub fn function_can_be_declared(&mut self, id: &str, parameters: &ParameterList, scope: &str, visibility: &Visibility, function_name: &str, function_declaration: &ClassFunctionDeclaration, output_config: &mut OutputConfig) -> Result<&mut Function, SemanticError> {
        // We know we are in a class scoped symbol table
        let result = self.get_undefined_function_in_class_scope(id, parameters, function_name, function_declaration, output_config)?;
        match result {
            Some(f) => {panic!("Function is trying to be declared but is already declared")},
            None => {
                
            }
        }

        let f = Function::new(
            function_declaration.id(),
            &Some(scope),
            function_declaration.return_type(),
            Some(*visibility),
        );
        if let SymbolTableEntry::Function(f) = self.add_entry(SymbolTableEntry::Function(f)) {
            return Ok(f);
        } else {
            panic!("Free function was just created in symbol table and cannot be accessed");
        }
    }

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

    pub fn check_declared_but_not_defined_functions(&self, output_config: &mut OutputConfig) {
        for entry in &self.values {
            if let SymbolTableEntry::Class(class) = entry {
                for class_entry in &class.symbol_table().values {
                    if let SymbolTableEntry::Function(function) = class_entry {
                        if !function.defined {
                            SemanticError::DeclaredButNotDefined(format!(
                                "No definition for declared member function {}::{}{}",
                                class.id(),
                                function.id(),
                                function.signature()
                            )).write(output_config);
                        }
                    }
                }
            }
        }
    }
}

