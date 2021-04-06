use std::fmt;
use output_manager::{OutputConfig, warn_write};

pub enum SemanticError {
    // MalformedAst(String), // This is an error in the lexer/parser/syntax analyzer
    InvalidScopeIdentifier(String),
    IdentifierIsNotAMemberFunction(String),
    UndefinedIdentifier(String),
    IdentifierRedefinition(String),
    DuplicateInheritance(String),
    DeclaredButNotDefined(String),
    DefinedButNotDeclared(String),
    FunctionOverload(String), // This isn't really an error
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            SemanticError::InvalidScopeIdentifier(message) => message,
            SemanticError::IdentifierIsNotAMemberFunction(message) => message,
            SemanticError::UndefinedIdentifier(message) => message,
            SemanticError::IdentifierRedefinition(message) => message,
            SemanticError::DuplicateInheritance(message) => message,
            SemanticError::DeclaredButNotDefined(message) => message,
            SemanticError::DefinedButNotDeclared(message) => message,

            SemanticError::FunctionOverload(message) => {
                return write!(f, "Semantic warning: {}", message);
            }
        };

        write!(f, "Semantic error: {}", message)
    }
}

impl SemanticError {
    pub fn write(&self, output_manager: &mut OutputConfig) {
        warn_write(&mut output_manager.semantic_error_file, &output_manager.semantic_error_path, &format!("{}\n", self));
    }
}
