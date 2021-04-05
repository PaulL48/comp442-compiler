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

#[derive(Debug, Clone, Default, Getters)]
pub struct Function {
    id: String,
    parameter_types: Vec<String>,
    return_type: String,
    visibility: Option<Visibility>,
    pub symbol_table: SymbolTable,
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
        match self.visibility {
            Some(visibility) => line.push_str(&format!("| {}", visibility)),
            _ => (),
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
    ) -> Result<(), SemanticError> {
        // TODO: Identify duplicate function definitions
        let active_entry = Function::get_or_create_function_entry(validated_node, global_table)?;

        for parameter in validated_node.parameter_list().parameters() {
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
            let entry = SymbolTableEntry::Local(Local::new(
                local_variable.id(),
                &local_variable.type_as_symbol_string(),
            ));
            active_entry.symbol_table.add_entry(entry);
        }

        Ok(())
    }

    /// Return the symbol table of the function represented by the validated node.
    /// If the validated node has Some(scope) this assumes the symbol table already exists
    /// within the class named id as a Function.
    /// If the scope is None this will create a new Function element in the global symbol table
    fn get_or_create_function_entry<'a>(
        validated_node: &FunctionDefinition,
        global_table: &'a mut SymbolTable,
    ) -> Result<&'a mut Function, SemanticError> {
        match validated_node.scope() {
            Some(scope) => {
                match global_table.get_mut(validated_node.id()) {
                    // Valid class scope
                    Some(SymbolTableEntry::Class(class)) => {
                        // but now we need to get the nested function symbol table
                        match class.symbol_table_mut().get_mut(scope) {
                            // Valid class scope, declared member function
                            Some(SymbolTableEntry::Function(function)) => {
                                Ok(function)
                            },
                            // Valid class scope, identifier is not a member function
                            Some(entry) => {
                                return Err(SemanticError::IdentifierIsNotAMemberFunction(format!(
                                    "{}:{} Scope identifier {} names a {} and not a member function",
                                    validated_node.line(),
                                    validated_node.column(),
                                    validated_node.id(),
                                    entry
                                )))
                            },
                            // Valid class scope, undeclared identifier
                            None => {
                                return Err(SemanticError::UndefinedIdentifier(format!(
                                    "{}:{} Definition provided for undeclared member function {}::{}",
                                    validated_node.line(),
                                    validated_node.column(),
                                    validated_node.id(),
                                    scope,
                                )))
                            }
                        }
                    }
                    // Scope identifier exists but is not a class
                    Some(_) => {
                        return Err(SemanticError::InvalidScopeIdentifier(format!(
                            "{}:{} Scope identifier {} is not a class",
                            // "Definition provided for undeclared class members {}::{} at {}:{}",
                            validated_node.line(),
                            validated_node.column(),
                            validated_node.id(),
                        )));
                    }
                    // Scope identifier does not exist
                    None => {
                        return Err(SemanticError::UndefinedIdentifier(format!(
                            "{}:{} Definition provided for undeclared class member {}::{}",
                            validated_node.line(),
                            validated_node.column(),
                            validated_node.id(),
                            scope
                        )))
                    } // Scope is specifying an undefined class
                }
            }
            // Free function
            None => {
                let f = Function::new(
                    validated_node.id(),
                    validated_node.scope(),
                    validated_node.return_type(),
                    None,
                );

                if let SymbolTableEntry::Function(f) =
                    global_table.add_entry(SymbolTableEntry::Function(f))
                {
                    Ok(f)
                } else {
                    panic!("Free function was just created in symbol table and cannot be accessed");
                }
            }
        }
    }
}
