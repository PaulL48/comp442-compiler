mod moon_instructions;
mod register;
mod visitor;

use ast::Node;
use semantic_analyzer::SemanticAnalysisResults;
use output_manager::OutputConfig;

pub fn process(node: &Node, current_results: &mut SemanticAnalysisResults, output: &mut OutputConfig) {
    visitor::visit(node, &current_results.symbol_table, output)
}
