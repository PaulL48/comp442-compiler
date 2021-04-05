mod format_table;
mod semantic_analyzer;
mod semantic_error;
mod utils;
mod visibility;

use semantic_error::SemanticError;
pub use semantic_analyzer::analyze;

pub mod symbol_table {
    pub mod class;
    pub mod data;
    pub mod function;
    pub mod inherit;
    pub mod local;
    pub mod param;
    pub mod symbol_table;
    pub mod visitor;
    pub mod entrypoint;

    pub use class::Class;
    pub use data::Data;
    pub use function::Function;
    pub use inherit::Inherit;
    pub use local::Local;
    pub use param::Param;
    pub use symbol_table::SymbolTable;
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
    pub mod program_root;
    pub mod class_declaration;
    pub mod class_member_list;
    pub mod inheritance_list;
    pub mod class_member;

    pub use dimension_list::DimensionList;
    pub use function_body::FunctionBody;
    pub use function_definition::FunctionDefinition;
    pub use function_parameter::FunctionParameter;
    pub use node_validator::NodeValidator;
    pub use node_validator::ValidatorError;
    pub use parameter_list::ParameterList;
    pub use variable::Variable;
    pub use variable_list::VariableList;
    pub use program_root::ProgramRoot;
    pub use view_as::ViewAs;
    pub use class_declaration::ClassDeclaration;
    pub use class_member::{ClassFunctionDeclaration, ClassVariable, ClassMember};
    pub use class_member_list::ClassMemberList;
    pub use inheritance_list::InheritanceList;
}

pub use symbol_table::*;
pub use visibility::Visibility;
