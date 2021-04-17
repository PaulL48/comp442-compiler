use crate::ast_validation::{ClassFunctionDeclaration, FunctionBody, FunctionDefinition};
use crate::format_table::FormatTable;

use crate::symbol_table::param::Param;
use crate::symbol_table::symbol_table::SymbolTable;

use crate::visibility::Visibility;

use derive_getters::Getters;
use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Function {
    id: String,
    scope: Option<String>,
    parameter_types: Vec<Param>,
    return_type: Option<String>,
    visibility: Option<Visibility>,
    symbol_table: SymbolTable,
    defined: bool,
    line: usize,
    column: usize,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function ")?;
        if let Some(visibility) = self.visibility {
            write!(f, "{} ", visibility)?;
        }
        write!(f, "{}{}", self.id, self.signature())
    }
}

impl FormatTable for Function {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut line = format!(
            "{:10}| {:12}| {:34}",
            "function",
            self.id,
            format!("{}: {}", self.signature(), self.return_type_as_string())
        );
        if let Some(visibility) = self.visibility {
            line.push_str(&format!("| {}", visibility))
        }
        result.push(line);
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        self.id == other.id && self.parameter_types == other.parameter_types
    }
}

impl Function {
    pub fn is_class_member(&self) -> bool {
        self.scope.is_some()
    }

    pub fn is_defined(&self) -> bool {
        self.defined
    }

    /// Returns a new function with manually set id and return type
    pub fn create_main(validated_node: &FunctionBody) -> Self {
        Function {
            id: "main".to_owned(),
            scope: None,
            parameter_types: Vec::new(),
            return_type: None,
            visibility: None,
            symbol_table: SymbolTable::new("main"),
            defined: true,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn from_definition(validated_node: &FunctionDefinition) -> Self {
        let (id, scope) = validated_node.get_corrected_scoped_id();

        Function {
            id: id.to_string(),
            scope: scope.map(|s| s.to_string()),
            parameter_types: validated_node
                .parameter_list()
                .parameters()
                .iter()
                .map(|x| Param::from(x))
                .collect(),
            return_type: validated_node.return_type().map(|t| t.to_string()),
            visibility: None, // Free functions have no visibility
            symbol_table: SymbolTable::scoped_new(validated_node.id(), scope),
            defined: true,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn from_declaration(validated_node: &ClassFunctionDeclaration, parent_class: &str) -> Self {
        Function {
            id: validated_node.id().to_string(),
            scope: Some(parent_class.to_string()),
            parameter_types: validated_node
                .parameter_list()
                .parameters()
                .iter()
                .map(|x| Param::from(x))
                .collect(),
            return_type: validated_node.return_type().map(|t| t.to_string()),
            visibility: Some(*validated_node.visibility()),
            symbol_table: SymbolTable::scoped_new(validated_node.id(), Some(parent_class)),
            defined: false,
            line: *validated_node.line(),
            column: *validated_node.column(),
        }
    }

    pub fn return_type_as_string(&self) -> String {
        match &self.return_type {
            Some(return_type) => return_type.clone(),
            None => "void".to_owned(),
        }
    }

    pub fn signature(&self) -> String {
        let mut result = String::new();
        result.push_str("(");
        let parameter_list_string = self
            .parameter_types
            .iter()
            .map(|p| p.type_string())
            .collect::<Vec<_>>()
            .join(",");
        result.push_str(&parameter_list_string);
        result.push(')');
        result
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}
