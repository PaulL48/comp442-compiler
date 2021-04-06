use crate::ast_validation::FunctionDefinition;
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

#[derive(Debug, Clone, Default, Getters)]
pub struct Function {
    id: String,
    parameter_types: Vec<String>,
    return_type: String,
    visibility: Option<Visibility>,
    pub symbol_table: SymbolTable,
    pub defined: bool,
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

impl Function {
    pub fn new(
        id: &str,
        scope: &Option<&str>,
        return_type: &Option<&str>,
        visibility: Option<Visibility>,
    ) -> Self {
        let scope = match scope {
            None => "".to_string(),
            Some(id) => id.to_string(),
        };

        let return_type = match return_type {
            None => "void",
            Some(r_type) => r_type,
        };

        Function {
            id: id.to_string(),
            parameter_types: Vec::new(),
            return_type: return_type.to_string(),
            visibility,
            symbol_table: SymbolTable::new(id, &Some(scope)),
            defined: false,
        }
    }

    fn signature(&self) -> String {
        format!(
            "({}): {}",
            separated_list(&self.parameter_types, ", "),
            self.return_type
        )
    }

    // fn full_signature(&self) -> String {
    //     let mut result = String::new();
    //     if let Some(visibility) = self.visibility {
    //         result.push_str(&format!("{} ", visibility));
    //     }

    //     if let Some(scope) = self.scope {
    //         result.push_str(&format!("{}::{}", self.id, scope));
    //     } else {
    //         result.push_str(&format!("{}", self.id));
    //     }

    //     result.push_str(&self.signature());

    //     result
    // }

    // When we encounter a funcDecl node we either have to create a new symbol
    // table, or retrieve the symbol table from the class and fill it in
    pub fn convert(
        validated_node: &FunctionDefinition,
        global_table: &mut SymbolTable,
        output_config: &mut OutputConfig
    ) -> Result<(), SemanticError> {
        let active_entry = Function::get_or_create_function_entry(validated_node, global_table, output_config)?;

        for parameter in validated_node.parameter_list().parameters() {
            if active_entry.symbol_table.contains(parameter.id()) {
                SemanticError::IdentifierRedefinition(format!(
                    "{}:{} Identifier \"{}\" is already defined in this scope",
                    parameter.line(),
                    parameter.column(),
                    parameter.id(),
                )).write(output_config);
            }

            active_entry
                .parameter_types
                .push(parameter.as_symbol_string());
            let entry =
                SymbolTableEntry::Param(Param::new(parameter.id(), &parameter.as_symbol_string()));
            active_entry.symbol_table.add_entry(entry);
        }

        // The next step would be to populate the local variables of the function
        for local_variable in validated_node
            .function_body()
            .local_variable_list()
            .variables()
        {
            if active_entry.symbol_table.contains(local_variable.id()) {
                SemanticError::IdentifierRedefinition(format!(
                    "{}:{} Identifier \"{}\" is already defined in this scope",
                    local_variable.line(),
                    local_variable.column(),
                    local_variable.id(),
                )).write(output_config);
            }

            let entry = SymbolTableEntry::Local(Local::new(
                local_variable.id(),
                &local_variable.type_as_symbol_string(),
            ));
            active_entry.symbol_table.add_entry(entry);
        }

        active_entry.defined = true;

        Ok(())
    }

    /// Return the symbol table of the function represented by the validated node.
    /// If the validated node has Some(scope) this assumes the symbol table already exists
    /// within the class named id as a Function.
    /// If the scope is None this will create a new Function element in the global symbol table
    fn get_or_create_function_entry<'a>(
        validated_node: &FunctionDefinition,
        global_table: &'a mut SymbolTable,
        output_config: &mut OutputConfig
    ) -> Result<&'a mut Function, SemanticError> {
        match validated_node.scope() {
            Some(scope) => {
                match global_table.get_mut(validated_node.id()) {
                    // Valid class scope
                    Some(SymbolTableEntry::Class(class)) => {
                        // Something needs to change here
                        // match class.symbol_table().function_can_be_defined(scope, &validated_node.parameter_list(), &format!("{}::{}", validated_node.id(), scope)) {
                        //     Ok(_) => {

                        //     },
                        //     Err(err) => return Err(err)
                        // }

                        return class.symbol_table_mut().function_can_be_defined(scope, &validated_node.parameter_list(), &format!("{}::{}", validated_node.id(), scope), validated_node, output_config);
                        // Past here the function can be defined by getting the entry from the symbol table

                        // if !class.symbol_table().function_can_be_defined(scope, &validated_node.parameter_list()) {

                        // }

                        // but now we need to get the nested function symbol table
                        // match class.symbol_table_mut().get_mut(scope) {
                        //     // Valid class scope, declared member function
                        //     Some(SymbolTableEntry::Function(function)) => {
                        //         // TODO: Add member function overloading
                        //         // What is overloading in the context of symbol table creation?
                        //         // it means when getting/checking for an existing symbol table we must check the signature of the functions
                        //         if !validated_node.parameter_list().same_as(function.parameter_types()) {

                        //         }

                        //         if class.symbol_table().function_can_be_defined(scope, )
                                

                        //         if function.defined {
                        //             return Err(SemanticError::IdentifierRedefinition(format!(
                        //                 "{}:{} Function \"{}\" is already defined for class scope {}",
                        //                 validated_node.line(),
                        //                 validated_node.column(),
                        //                 scope,
                        //                 validated_node.id(),
                        //             )))
                        //         }

                        //         Ok(function)
                        //     },
                        //     // Valid class scope, identifier is not a member function
                        //     Some(entry) => {
                        //         return Err(SemanticError::IdentifierIsNotAMemberFunction(format!(
                        //             "{}:{} Scope identifier {}::{} names a \"{}\" and not a member function",
                        //             validated_node.line(),
                        //             validated_node.column(),
                        //             validated_node.id(),
                        //             scope,
                        //             entry
                        //         )))
                        //     },
                        //     // Valid class scope, undeclared identifier
                        //     None => {
                        //         return Err(SemanticError::UndefinedIdentifier(format!(
                        //             "{}:{} Definition provided for undeclared member function {}::{}",
                        //             validated_node.line(),
                        //             validated_node.column(),
                        //             validated_node.id(),
                        //             scope,
                        //         )))
                        //     }
                        // }
                    }
                    // Scope identifier exists but is not a class
                    Some(entry) => {
                        return Err(SemanticError::InvalidScopeIdentifier(format!(
                            "{}:{} Scope identifier {} names a \"{}\", and not a class",
                            // "Definition provided for undeclared class members {}::{} at {}:{}",
                            validated_node.line(),
                            validated_node.column(),
                            validated_node.id(),
                            entry
                        )));
                    }
                    // Scope identifier does not exist
                    None => {
                        return Err(SemanticError::UndefinedIdentifier(format!(
                            "{}:{} Class identifier {} does not exist in this scope",
                            validated_node.line(),
                            validated_node.column(),
                            validated_node.id(),
                        )))
                    } // Scope is specifying an undefined class
                }
            }
            // Free function
            None => {
                return global_table.function_can_be_defined(validated_node.id(), validated_node.parameter_list(), validated_node.id(), validated_node, output_config);

                // A free function can be defined if it is different in its parameter list than all the functions in the global scope of the same identifier
                // if !global_table.function_can_be_defined(validated_node.id(), validated_node.parameter_list()) {
                //     return Err(SemanticError::IdentifierRedefinition(format!(
                //         "{}:{} Identifier \"{}\" is already defined in this scope",
                //         validated_node.line(),
                //         validated_node.column(),
                //         validated_node.id(),
                //     )))
                // }

                // if global_table.contains(validated_node.id()) {
                //     // ADD function overloading

                //     return Err(SemanticError::IdentifierRedefinition(format!(
                //         "{}:{} Identifier \"{}\" is already defined in this scope",
                //         validated_node.line(),
                //         validated_node.column(),
                //         validated_node.id(),
                //     )))
                // }

                // let mut f = Function::new(
                //     validated_node.id(),
                //     validated_node.scope(),
                //     validated_node.return_type(),
                //     None,
                // );

                // if let SymbolTableEntry::Function(f) =
                //     global_table.add_entry(SymbolTableEntry::Function(f))
                // {
                //     Ok(f)
                // } else {
                //     panic!("Free function was just created in symbol table and cannot be accessed");
                // }
            }
        }
    }
}
