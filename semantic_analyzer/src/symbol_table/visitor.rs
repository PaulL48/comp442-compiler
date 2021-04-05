//! Given an AST node, build a symbol table

use crate::ast_validation::FunctionDefinition;
use crate::semantic_error::SemanticError;
use crate::symbol_table::function;
use crate::symbol_table::symbol_table::SymbolTable;
use ast::Node;
use log::error;
use crate::semantic_analyzer::SemanticAnalysisResults;

pub fn visit(node: &Node, current_data: &mut SemanticAnalysisResults) {

}


// A function definition requires the global symbol table
// If it is a member function, it must get its visibility from the
// class symbol table
pub fn function_declaration(
    node: &ast::Node,
    global_table: &mut SymbolTable,
) -> Result<(), SemanticError> {
    match FunctionDefinition::view_as(node) {
        Ok(validated_node) => {
            function::Function::convert(&validated_node, global_table)?;
        },
        Err(validation_error) => {
            error!("{}", validation_error);
            panic!();
        }
    }

    Ok(())
}
