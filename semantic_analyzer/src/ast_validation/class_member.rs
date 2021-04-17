use crate::ast_validation::{DimensionList, NodeValidator, ParameterList, ValidatorError, ViewAs, ToSymbol};
use crate::Visibility;
use ast::Node;
use derive_getters::Getters;
use crate::symbol_table::{SymbolTable, SymbolTableEntry, Function, Data};
use output_manager::OutputConfig;
use crate::SemanticError;
use crate::symbol_table::rules;
use std::fmt;

#[derive(Getters, Debug)]
pub struct ClassFunctionDeclaration<'a> {
    visibility: Visibility,
    id: &'a str,
    parameter_list: ParameterList<'a>,
    return_type: Option<&'a str>,
    line: usize,
    column: usize,
}

#[derive(Getters)]
pub struct ClassVariable<'a> {
    visibility: Visibility,
    data_type: &'a str,
    id: &'a str,
    dimension_list: DimensionList,
    line: usize,
    column: usize,
}

impl fmt::Display for ClassFunctionDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: This can be improved
        write!(f, "Class member function {}", self.id())
    }
}

impl fmt::Display for ClassVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: This can be improved
        write!(f, "Class member variable {}", self.id())
    }
}

pub enum ClassMember<'a> {
    FunctionDeclaration(ClassFunctionDeclaration<'a>),
    Variable(ClassVariable<'a>),
}

impl<'a> ViewAs<'a> for ClassMember<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        // This gives visibility + [varDecl | funcDecl]
        let mut validator = NodeValidator::new(node, "Class member").has_children(2)?;
        let visibility = validator.then_optional_string()?;
        let interior_node = validator.then_node()?;

        let visibility = match visibility {
            Some(s) => {
                match s {
                    "public" => Visibility::Public,
                    "private" => Visibility::Private,
                    _ => {
                        // syntactic analysis should prevent this from happening
                        return Err(ValidatorError::MalformedAst(format!(
                            "Visibility node should be \"public\" or \"private\", found \"{}\"",
                            s
                        )));
                    }
                }
            }
            None => Visibility::Private,
        };

        // This is the only node I'm aware of that needs to disambiguate based on the
        // node name itself
        match interior_node.name().as_str() {
            "varDecl" => {
                // the internal varDecl will generate a 'data' symbol table entry
                let mut internal_validator =
                    NodeValidator::new(interior_node, "Class variable").has_children(3)?;

                let data_type = internal_validator.then_string()?;
                let id = internal_validator.then_string()?;
                let dimension_list = internal_validator.then()?;

                return Ok(ClassMember::Variable(ClassVariable {
                    visibility,
                    data_type,
                    id,
                    dimension_list,
                    line: *interior_node.line(),
                    column: *interior_node.column(),
                }));
            }
            "funcDecl" => {
                let mut internal_validator =
                    NodeValidator::new(interior_node, "Function declaration").has_children(3)?;

                let id = internal_validator.then_string()?;
                let parameter_list = internal_validator.then_optional()?;
                let return_type = internal_validator.then_optional_string()?;

                let parameter_list = match parameter_list {
                    None => ParameterList::new(*interior_node.line(), *interior_node.column()),
                    Some(list) => list,
                };

                return Ok(ClassMember::FunctionDeclaration(ClassFunctionDeclaration {
                    visibility,
                    id,
                    parameter_list,
                    return_type,
                    line: *interior_node.line(),
                    column: *interior_node.column(),
                }));
            }
            _ => {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node requires second child to be a varDecl or a funcDecl, found {:?}",
                    node.name(),
                    interior_node,
                )))
            }
        }
    }
}

impl<'a> ToSymbol for ClassMember<'a> {
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        match self {
            ClassMember::FunctionDeclaration(declaration) => declaration.validate_entry(context, output),
            ClassMember::Variable(variable) => variable.validate_entry(context, output),
        }
    }

    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        match self {
            ClassMember::FunctionDeclaration(declaration) => declaration.to_symbol(context, output),
            ClassMember::Variable(variable) => variable.to_symbol(context, output),
        }
    }
}

impl ToSymbol for ClassFunctionDeclaration<'_> {
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        let matching_entries = context.get_all(self.id());
        rules::function_redefines(self.id(), self.parameter_list(), &matching_entries, self.line(), self.column(), &self.to_string())?;
        rules::warn_overloading_function(self.id(), self.parameter_list(), &matching_entries, self.line(), self.column(), output);
        Ok(())
    }

    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let mut new_entry = Function::from_declaration(self, &context.name);
        let parameter_entries = self.parameter_list.to_validated_symbol(context, output)?;
        new_entry.symbol_table_mut().extend(parameter_entries);
        Ok(vec![SymbolTableEntry::Function(new_entry)])
    }
}

impl ToSymbol for ClassVariable<'_> {
    fn validate_entry(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<(), SemanticError> {
        // Check for redefinitions
        let matching_entries = context.get_all(self.id());
        rules::id_redefines(self.id(), &matching_entries, self.line(), self.column(), &self.to_string())?;
        rules::mandatory_dimensions(&self.dimension_list, self.id())?;
        Ok(())
    }

    fn to_symbol(&self, context: &SymbolTable, output: &mut OutputConfig) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        Ok(vec![SymbolTableEntry::Data(Data::from(self))])
    }
}
