mod moon_instructions;
mod register;
mod visitor;
mod macros;
mod preamble;

use ast::Node;
use output_manager::OutputConfig;
use semantic_analyzer::SemanticAnalysisResults;
pub use visitor::process;

