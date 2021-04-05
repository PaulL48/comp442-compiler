mod format_table;
mod semantic_analyzer;
mod semantic_error;
mod symbol_table_creator;
mod utils;
mod visibility;

use semantic_error::SemanticError;

pub mod symbol_table {
    pub mod class;
    pub mod data;
    pub mod function;
    pub mod inherit;
    pub mod local;
    pub mod param;
    pub mod symbol_table;
    pub mod visitor;
}

mod ast_validation {
    pub mod dimension_list;
    pub mod function_body;
    pub mod function_definition;
    pub mod function_parameter;
    pub mod node_validator;
    pub mod parameter_list;
    pub mod variable;
    pub mod variable_list;
    pub mod view_as;

    pub use dimension_list::DimensionList;
    pub use function_body::FunctionBody;
    pub use function_definition::FunctionDefinition;
    pub use function_parameter::FunctionParameter;
    pub use node_validator::NodeValidator;
    pub use node_validator::ValidatorError;
    pub use parameter_list::ParameterList;
    pub use variable::Variable;
    pub use variable_list::VariableList;
}

pub use symbol_table::*;
pub use visibility::Visibility;
