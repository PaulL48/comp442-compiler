pub enum SemanticError {
    MalformedAst(String), // This is an error in the lexer/parser/syntax analyzer
    InvalidScopeIdentifier(String),
    IdentifierIsNotAMemberFunction(String),
    UndefinedIdentifier(String),
}
