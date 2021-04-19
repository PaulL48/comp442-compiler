mod moon_instructions;
mod register;
mod visitor;

use ast::Node;
use output_manager::OutputConfig;
use semantic_analyzer::SemanticAnalysisResults;

pub fn process(
    node: &Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    visitor::visit(node, &current_results.symbol_table, output)
}
