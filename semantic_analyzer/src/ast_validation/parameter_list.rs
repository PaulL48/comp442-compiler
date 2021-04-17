use crate::ast_validation::{FunctionParameter, NodeValidator, ToSymbol, ValidatorError, ViewAs};
use crate::symbol_table::{Param, SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;

#[derive(Getters, Debug)]
pub struct ParameterList<'a> {
    parameters: Vec<FunctionParameter<'a>>,
    line: usize,
    column: usize,
}

impl PartialEq<Vec<Param>> for ParameterList<'_> {
    fn eq(&self, other: &Vec<Param>) -> bool {
        if self.parameters.len() != other.len() {
            return false;
        }

        for (lp, rp) in self.parameters.iter().zip(other.iter()) {
            if lp.data_type() != rp.data_type() {
                return false;
            }
        }

        true
    }
}

impl<'a> ViewAs<'a> for ParameterList<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let validator = NodeValidator::new(node, "Parameter list");

        let parameters = validator.then_list_of()?;

        Ok(ParameterList {
            parameters,
            line: *node.line(),
            column: *node.column(),
        })
    }
}

impl ToSymbol for ParameterList<'_> {
    fn validate_entry(
        &self,
        _context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        // The list of parameters itself, cannot be invalid at this point
        Ok(())
    }

    fn to_symbol(
        &self,
        context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let mut results = Vec::new();
        for parameter in self.parameters() {
            // parameter.validate_entry(context, output)?;
            let entries = parameter.to_validated_symbol(context, output)?;
            results.extend(entries);
        }
        Ok(results)
    }
}

impl<'a> ParameterList<'a> {
    pub fn new(line: usize, column: usize) -> Self {
        ParameterList {
            parameters: Vec::new(),
            line,
            column,
        }
    }

    pub fn same_as(&self, string_list: &Vec<String>) -> bool {
        if self.parameters.len() != string_list.len() {
            // println!("Lists have different length {:?}, {:?}", self.parameters, string_list);
            return false;
        }

        self.parameters
            .iter()
            .zip(string_list)
            .all(|(lhs, rhs)| lhs.as_symbol_string() == *rhs)
    }
}
