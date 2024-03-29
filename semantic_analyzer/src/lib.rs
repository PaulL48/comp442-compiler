mod format_table;
mod memory_size_visitor;
mod semantic_analyzer;
mod semantic_error;
mod type_check_visitor;
mod utils;
mod visibility;

pub use semantic_analyzer::analyze;
pub use semantic_analyzer::SemanticAnalysisResults;
use semantic_error::SemanticError;

pub mod symbol_table {
    pub mod class;
    pub mod data;
    pub mod function;
    pub mod inherit;
    pub mod literal;
    pub mod local;
    pub mod param;
    pub mod rules;
    pub mod sizes;
    pub mod symbol_table;
    pub mod temporary;
    pub mod utils;
    pub mod visitor;

    pub use class::Class;
    pub use data::Data;
    pub use function::Function;
    pub use inherit::Inherit;
    pub use literal::{Literal, LiteralValue};
    pub use local::Local;
    pub use param::Param;
    pub use symbol_table::{SymbolTable, SymbolTableEntry};
    pub use temporary::Temporary;
}

mod ast_validation {
    pub mod class_declaration;
    pub mod class_member;
    pub mod class_member_list;
    pub mod dimension_list;
    pub mod function_body;
    pub mod function_declaration;
    pub mod function_definition;
    pub mod function_parameter;
    pub mod inheritance_list;
    pub mod node_validator;
    pub mod parameter_list;
    pub mod program_root;
    pub mod to_symbol;
    pub mod variable;
    pub mod variable_list;
    pub mod view_as;

    pub use class_declaration::ClassDeclaration;
    pub use class_member::{ClassFunctionDeclaration, ClassMember, ClassVariable};
    pub use class_member_list::ClassMemberList;
    pub use dimension_list::DimensionList;
    pub use function_body::FunctionBody;
    // pub use function_declaration::FunctionDeclaration;
    pub use function_definition::FunctionDefinition;
    pub use function_parameter::FunctionParameter;
    pub use inheritance_list::InheritanceList;
    pub use node_validator::NodeValidator;
    pub use node_validator::ValidatorError;
    pub use parameter_list::ParameterList;
    pub use program_root::ProgramRoot;
    pub use to_symbol::ToSymbol;
    pub use variable::Variable;
    pub use variable_list::VariableList;
    pub use view_as::ViewAs;
}

mod type_checking {
    pub mod typing;
    pub mod visitor;

    // pub use visitor::visit;
}

pub use symbol_table::*;
pub use visibility::Visibility;
