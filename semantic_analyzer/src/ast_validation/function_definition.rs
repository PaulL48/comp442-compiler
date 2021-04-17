//! Converts a string typed AST into well typed semantic components
//! This amounts to changing non-leaf nodes into structures that provide
//! strong guarantees and names for their contents
//! Generally only those nodes whose children are single variants are defined
//! Ex. function definitions will have children in this order: id, scope, parameter list, function body
//! Whereas a relational operator can have arithmetic operations as either side, or variables, or fcalls, etc..

//! Allow AST nodes to be viewed as a more strongly type version of themselves.
//! Given an AST node such as a function declaration, many things can be wrong with the AST
//! There may be less than 5 children, the variants of the children enum may be incorrect, the names may be incorrect, etc...
//! So this provides a validation that produces a more concrete view into a node or a failure that can be propagated

use crate::ast_validation::node_validator::{NodeValidator, ValidatorError};
use crate::ast_validation::{FunctionBody, ParameterList};
use crate::ast_validation::{ToSymbol, ViewAs};
use crate::symbol_table::rules;
use crate::symbol_table::{SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;
use std::fmt;

#[derive(Getters)]
pub struct FunctionDefinition<'a> {
    id: &'a str,
    scope: Option<&'a str>,
    parameter_list: ParameterList<'a>,
    return_type: Option<&'a str>,
    function_body: FunctionBody<'a>,
    line: usize,
    column: usize,
}

impl<'a> fmt::Display for FunctionDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function definition {}", self.id())
    }
}

impl<'a> ViewAs<'a> for FunctionDefinition<'a> {
    fn view_as(node: &'a Node) -> Result<FunctionDefinition<'a>, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Function definition").has_children(5)?;

        let id = validator.then_string()?;
        let scope = validator.then_optional_string()?;
        let parameter_list = validator.then_optional()?;
        let return_type = validator.then_optional_string()?;
        let function_body = validator.then()?;

        let parameter_list = match parameter_list {
            Some(list) => list,
            None => ParameterList::new(*node.line(), *node.column()),
        };

        Ok(FunctionDefinition {
            id,
            scope,
            parameter_list,
            return_type,
            function_body,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

use crate::symbol_table::Function;

impl<'a> std::cmp::PartialEq<Function> for FunctionDefinition<'a> {
    fn eq(&self, other: &Function) -> bool {
        if self.parameter_list.parameters().len() != other.parameter_types().len() {
            return false;
        }

        for (lp, rp) in self
            .parameter_list()
            .parameters()
            .iter()
            .zip(other.parameter_types().iter())
        {
            if lp.data_type() != rp.data_type() {
                return false;
            }
        }

        true
    }
}

impl<'a> FunctionDefinition<'a> {
    /// Convert a loosely typed AST node into a node with more validation
    // pub fn view_as(node: &'a Node) -> Result<FunctionDefinition<'a>, ValidatorError> {
    //     let mut validator = NodeValidator::new(node, "Function definition").has_children(5)?;

    //     let id = validator.then_string()?;
    //     let scope = validator.then_optional_string()?;
    //     let parameter_list = validator.then_optional()?;
    //     let return_type = validator.then_optional_string()?;
    //     let function_body = validator.then()?;

    //     let parameter_list = match parameter_list {
    //         Some(list) => list,
    //         None => ParameterList::new(*node.line(), *node.column())
    //     };

    //     Ok(FunctionDefinition {
    //         id,
    //         scope,
    //         parameter_list,
    //         return_type,
    //         function_body,
    //         line: *node.line(),
    //         column: *node.column(),
    //     })
    // }

    /// Return the actual id and scope of the function as (id, scope)
    /// Since the id of the function becomes the scopeSpec in the lexing
    /// this undoes that change and returns the scope as scope and id as id
    pub fn get_corrected_scoped_id(&self) -> (&'a str, Option<&'a str>) {
        match self.scope {
            Some(scope) => (scope, Some(self.id)),
            None => (self.id, None),
        }
    }
}

impl<'a> ToSymbol for FunctionDefinition<'a> {
    fn validate_entry(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        let (id, scope) = self.get_corrected_scoped_id();
        if let Some(scope) = scope {
            let class = if let Some(SymbolTableEntry::Class(class)) = context.get(scope) {
                class
            } else {
                return Err(SemanticError::new_defined_not_declared(
                    self.line(),
                    self.column(),
                    &self.to_string(),
                    scope,
                ));
            };
            println!("{:?}", class);

            let matching_entries = class.symbol_table().get_all(id);
            rules::function_redefines(
                self.id(),
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                &self.to_string(),
            )?;
            rules::warn_overloading_function(
                self.id(),
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                output,
            );

            if let None = rules::get_exact(id, &self.parameter_list, &matching_entries) {
                return Err(SemanticError::new_defined_not_declared(
                    self.line(),
                    self.column(),
                    id,
                    scope,
                ));
            }
        } else {
            let matching_entries = context.get_all(id);
            rules::function_redefines(
                id,
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                &self.to_string(),
            )?;
            rules::warn_overloading_function(
                id,
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                output,
            );
        }
        Ok(())
    }

    fn to_symbol(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let (id, scope) = self.get_corrected_scoped_id();
        if let Some(scope) = scope {
            let class = if let Some(SymbolTableEntry::Class(class)) = context.get(scope) {
                class
            } else {
                return Err(SemanticError::new_defined_not_declared(
                    self.line(),
                    self.column(),
                    &self.to_string(),
                    scope,
                ));
            };

            let matching_entries = class.symbol_table().get_all(id);
            rules::function_redefines(
                id,
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                &self.to_string(),
            )?;
            rules::warn_overloading_function(
                id,
                &self.parameter_list,
                &matching_entries,
                &self.line,
                &self.column,
                output,
            );

            if let Some(mut declaration) =
                rules::get_exact_clone(id, &self.parameter_list, &matching_entries)
            {
                let local_entries = self
                    .function_body()
                    .to_validated_symbol(declaration.symbol_table(), output)?;
                declaration.symbol_table_mut().extend(local_entries);
                return Ok(vec![SymbolTableEntry::Function(declaration)]);
            } else {
                panic!("Should invoke validation before symbol creation");
            }
        } else {
            let mut new_entry = Function::from_definition(self);
            let parameter_entries = self
                .parameter_list
                .to_validated_symbol(new_entry.symbol_table(), output)?;
            new_entry.symbol_table_mut().extend(parameter_entries);
            let local_entries = self
                .function_body
                .local_variable_list()
                .to_validated_symbol(new_entry.symbol_table(), output)?;
            new_entry.symbol_table_mut().extend(local_entries);
            Ok(vec![SymbolTableEntry::Function(new_entry)])
        }
    }
}
