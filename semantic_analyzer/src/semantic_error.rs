use std::fmt;

pub enum SemanticError {
    // MalformedAst(String), // This is an error in the lexer/parser/syntax analyzer
    InvalidScopeIdentifier(String),
    IdentifierIsNotAMemberFunction(String),
    UndefinedIdentifier(String),
    IdentifierRedefinition(String)
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            SemanticError::InvalidScopeIdentifier(message) => message,
            SemanticError::IdentifierIsNotAMemberFunction(message) => message,
            SemanticError::UndefinedIdentifier(message) => message,
            SemanticError::IdentifierRedefinition(message) => message,
        };

        write!(f, "Semantic error: {}", message)
    }
}
