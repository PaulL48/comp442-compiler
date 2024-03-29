use std::fmt;

pub enum SemanticError {
    InvalidScopeIdentifier(usize, usize, String),
    IdentifierIsNotAMemberFunction(usize, usize, String),
    UndefinedIdentifier(usize, usize, String),
    IdentifierRedefinition(usize, usize, String),
    DuplicateInheritance(usize, usize, String),
    DeclaredButNotDefined(usize, usize, String),
    DefinedButNotDeclared(usize, usize, String),
    FunctionOverload(usize, usize, String), // This isn't really an error,
    MissingDimension(usize, usize, String),
    CyclicInheritance(usize, usize, String),
    TypeError(usize, usize, String),
    InvalidArrayIndex(usize, usize, String),
    IncorrectNumberOfArguments(usize, usize, String),
    NoMatchingOverload(usize, usize, String),
    InvalidRelOp(usize, usize, String),

    BinaryMismatchedTypes(usize, usize, String),
    UndefinedType(usize, usize, String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (l, c, message) = match self {
            SemanticError::InvalidScopeIdentifier(l, c, message) => (l, c, message),
            SemanticError::IdentifierIsNotAMemberFunction(l, c, message) => (l, c, message),
            SemanticError::UndefinedIdentifier(l, c, message) => (l, c, message),
            SemanticError::IdentifierRedefinition(l, c, message) => (l, c, message),
            SemanticError::DuplicateInheritance(l, c, message) => (l, c, message),
            SemanticError::DeclaredButNotDefined(l, c, message) => (l, c, message),
            SemanticError::DefinedButNotDeclared(l, c, message) => (l, c, message),
            SemanticError::BinaryMismatchedTypes(l, c, message) => (l, c, message),
            SemanticError::UndefinedType(l, c, message) => (l, c, message),
            SemanticError::MissingDimension(l, c, message) => (l, c, message),
            SemanticError::CyclicInheritance(l, c, message) => (l, c, message),
            SemanticError::TypeError(l, c, message) => (l, c, message),
            SemanticError::InvalidArrayIndex(l, c, message) => (l, c, message),
            SemanticError::IncorrectNumberOfArguments(l, c, message) => (l, c, message),
            SemanticError::NoMatchingOverload(l, c, message) => (l, c, message),
            SemanticError::InvalidRelOp(l, c, message) => (l, c, message),

            SemanticError::FunctionOverload(l, c, message) => {
                return write!(f, "Semantic warning: {}:{} {}", l, c, message);
            }
        };

        write!(f, "Semantic error: {}:{} {}", l, c, message)
    }
}

impl SemanticError {
    pub fn line(&self) -> usize {
        match &self {
            SemanticError::InvalidScopeIdentifier(l, _, _) => *l,
            SemanticError::IdentifierIsNotAMemberFunction(l, _, _) => *l,
            SemanticError::UndefinedIdentifier(l, _, _) => *l,
            SemanticError::IdentifierRedefinition(l, _, _) => *l,
            SemanticError::DuplicateInheritance(l, _, _) => *l,
            SemanticError::DeclaredButNotDefined(l, _, _) => *l,
            SemanticError::DefinedButNotDeclared(l, _, _) => *l,
            SemanticError::BinaryMismatchedTypes(l, _, _) => *l,
            SemanticError::UndefinedType(l, _, _) => *l,
            SemanticError::FunctionOverload(l, _, _) => *l,
            SemanticError::MissingDimension(l, _, _) => *l,
            SemanticError::CyclicInheritance(l, _, _) => *l,
            SemanticError::TypeError(l, _, _) => *l,
            SemanticError::InvalidArrayIndex(l, _, _) => *l,
            SemanticError::IncorrectNumberOfArguments(l, _, _) => *l,
            SemanticError::NoMatchingOverload(l, _, _) => *l,
            SemanticError::InvalidRelOp(l, _, _) => *l,
        }
    }

    pub fn col(&self) -> usize {
        match &self {
            SemanticError::InvalidScopeIdentifier(_, c, _) => *c,
            SemanticError::IdentifierIsNotAMemberFunction(_, c, _) => *c,
            SemanticError::UndefinedIdentifier(_, c, _) => *c,
            SemanticError::IdentifierRedefinition(_, c, _) => *c,
            SemanticError::DuplicateInheritance(_, c, _) => *c,
            SemanticError::DeclaredButNotDefined(_, c, _) => *c,
            SemanticError::DefinedButNotDeclared(_, c, _) => *c,
            SemanticError::BinaryMismatchedTypes(_, c, _) => *c,
            SemanticError::UndefinedType(_, c, _) => *c,
            SemanticError::FunctionOverload(_, c, _) => *c,
            SemanticError::MissingDimension(_, c, _) => *c,
            SemanticError::CyclicInheritance(_, c, _) => *c,
            SemanticError::TypeError(_, c, _) => *c,
            SemanticError::InvalidArrayIndex(_, c, _) => *c,
            SemanticError::IncorrectNumberOfArguments(_, c, _) => *c,
            SemanticError::NoMatchingOverload(_, c, _) => *c,
            SemanticError::InvalidRelOp(_, c, _) => *c,
        }
    }

    /// Create a new message about the redefinition of some element printed as already_exists
    /// by some element printed as tried_to_add
    pub fn new_redefinition(
        line: &usize,
        column: &usize,
        tried_to_add: &str,
        already_exists: &str,
    ) -> SemanticError {
        SemanticError::IdentifierRedefinition(
            *line,
            *column,
            format!(
                "Identifier \"{}\" is already defined in this scope as \"{}\"",
                tried_to_add, already_exists
            ),
        )
    }

    pub fn new_overload(line: &usize, column: &usize, id: &str) -> SemanticError {
        SemanticError::FunctionOverload(
            *line,
            *column,
            format!("Function provides an overload for \"{}\"", id),
        )
    }

    pub fn new_defined_not_declared(
        line: &usize,
        column: &usize,
        function: &str,
        missing_scope: &str,
    ) -> SemanticError {
        SemanticError::DefinedButNotDeclared(
            *line,
            *column,
            format!(
                "Member function is defining an undeclared identifier \"{}\" in the scope \"{}\"",
                function, missing_scope
            ),
        )
    }

    // pub fn new_identifier_redefinition(
    //     line: &usize,
    //     column: &usize,
    //     function: &str,
    //     scope: &str,
    // ) -> SemanticError {
    //     SemanticError::IdentifierRedefinition(
    //         *line,
    //         *column,
    //         format!(
    //             "Function \"{}\" is already defined for the scope {}",
    //             function, scope
    //         ),
    //     )
    // }

    pub fn new_duplicate_inheritance(
        line: &usize,
        column: &usize,
        class: &str,
        id: &str,
    ) -> SemanticError {
        SemanticError::DuplicateInheritance(
            *line,
            *column,
            format!(
                "Duplicate inheritance of identifier {} for class {}",
                id, class
            ),
        )
    }

    pub fn new_missing_dimension(line: &usize, column: &usize, identifier: &str) -> SemanticError {
        SemanticError::MissingDimension(
            *line,
            *column,
            format!("Missing dimension for array \"{}\"", identifier),
        )
    }

    pub fn new_cyclic_inheritance(line: &usize, column: &usize, class_repr: &str) -> SemanticError {
        SemanticError::CyclicInheritance(
            *line,
            *column,
            format!(
                "Class has a cyclic inheritance hierarchy \"{}\"",
                class_repr
            ),
        )
    }

    pub fn new_declared_but_not_defined(
        line: &usize,
        column: &usize,
        function: &str,
    ) -> SemanticError {
        SemanticError::DeclaredButNotDefined(
            *line,
            *column,
            format!("Member function \"{}\" missing definition", function),
        )
    }

    pub fn new_override(line: &usize, column: &usize, function: &str) -> SemanticError {
        SemanticError::FunctionOverload(
            *line,
            *column,
            format!(
                "Member function \"{}\" provides override for inherited method",
                function
            ),
        )
    }

    pub fn new_shadowing(
        line: &usize,
        column: &usize,
        entry: &str,
        shadows: &str,
    ) -> SemanticError {
        SemanticError::FunctionOverload(
            *line,
            *column,
            format!("\"{}\" shadows inherited member \"{}\"", entry, shadows),
        )
    }

    pub fn new_binary_type_error(
        line: &usize,
        column: &usize,
        lht: &str,
        rht: &str,
    ) -> SemanticError {
        SemanticError::BinaryMismatchedTypes(
            *line,
            *column,
            format!(
                "Type error: types of binary operation do not match \"{}\", \"{}\"",
                lht, rht,
            ),
        )
    }

    pub fn new_undefined_type(line: &usize, column: &usize, data_type: &str) -> SemanticError {
        SemanticError::UndefinedType(
            *line,
            *column,
            format!("Type error: specified type is undefined \"{}\"", data_type),
        )
    }

    pub fn new_undefined_identifier(line: &usize, column: &usize, id: &str) -> SemanticError {
        SemanticError::UndefinedIdentifier(
            *line,
            *column,
            format!("Undefined identifier \"{}\"", id),
        )
    }

    pub fn new_invalid_array_index(line: &usize, column: &usize, data_type: &str) -> SemanticError {
        SemanticError::InvalidArrayIndex(
            *line,
            *column,
            format!("Invalid array index \"{}\"", data_type),
        )
    }

    pub fn new_invalid_array_dimension(
        line: &usize,
        column: &usize,
        supplied_dimension: &usize,
        actual_dimension: &usize,
    ) -> SemanticError {
        SemanticError::InvalidArrayIndex(
            *line,
            *column,
            format!(
                "Incorrect number of dimensions, has {} but supplied {}",
                actual_dimension, supplied_dimension
            ),
        )
    }

    pub fn new_incorrect_number_arguments(
        line: usize,
        column: usize,
        supplied: usize,
        actual: usize,
    ) -> SemanticError {
        SemanticError::IncorrectNumberOfArguments(
            line,
            column,
            format!(
                "Incorrect number of arguments supplied, expected {} but got {}",
                actual, supplied
            ),
        )
    }

    pub fn new_incorrect_type(
        line: usize,
        column: usize,
        supplied: &str,
        expected: &str,
    ) -> SemanticError {
        SemanticError::TypeError(
            line,
            column,
            format!(
                "Incorrect type found \"{}\" but was expecting \"{}\"",
                supplied, expected
            ),
        )
    }

    pub fn new_no_overload(
        line: usize,
        column: usize,
        id: &str,
        parameters: &str,
    ) -> SemanticError {
        SemanticError::NoMatchingOverload(
            line,
            column,
            format!(
                "No overloads found for function \"{}\" that match the parameters ({})",
                id, parameters
            ),
        )
    }

    pub fn new_invalid_relop(line: usize, column: usize, data_type: &str) -> SemanticError {
        SemanticError::InvalidRelOp(
            line,
            column,
            format!(
                "Comparison operators must be between integers or floats, compared \"{}\"",
                data_type
            ),
        )
    }

    // pub fn write(&self, output_manager: &mut OutputConfig) {
    //     warn_write(&mut output_manager.semantic_error_file, &output_manager.semantic_error_path, &format!("{}\n", self));
    // }
}
