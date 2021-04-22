mod macros;
mod moon_instructions;
mod preamble;
mod register;
mod visitor;

use ast::Node;
use output_manager::OutputConfig;
use semantic_analyzer::SemanticAnalysisResults;
pub use visitor::process;
