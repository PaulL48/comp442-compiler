use crate::ast_validation::{FunctionBody, NodeValidator, ToSymbol, ValidatorError, ViewAs};
use crate::symbol_table::{Function, SymbolTable, SymbolTableEntry};
use crate::SemanticError;
use ast::Node;
use derive_getters::Getters;
use output_manager::OutputConfig;

#[derive(Getters)]
pub struct ProgramRoot<'a> {
    _class_declaration_list: &'a Node,
    _function_definition_list: &'a Node,
    main: FunctionBody<'a>,
}

impl<'a> ViewAs<'a> for ProgramRoot<'a> {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError> {
        let mut validator = NodeValidator::new(node, "Program root").has_children(3)?;

        let _class_declaration_list = validator.then_node()?;
        let _function_definition_list = validator.then_node()?;
        let main = validator.then()?;

        Ok(ProgramRoot {
            _class_declaration_list,
            _function_definition_list,
            main,
        })
    }
}

impl ToSymbol for ProgramRoot<'_> {
    fn validate_entry(
        &self,
        _context: &SymbolTable,
        _output: &mut OutputConfig,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn to_symbol(
        &self,
        _context: &SymbolTable,
        output: &mut OutputConfig,
    ) -> Result<Vec<SymbolTableEntry>, SemanticError> {
        let mut new_entry = Function::create_main(&self.main);

        let local_entries = self
            .main()
            .local_variable_list()
            .to_validated_symbol(new_entry.symbol_table(), output)?;
        new_entry.symbol_table_mut().extend(local_entries);
        Ok(vec![SymbolTableEntry::Function(new_entry)])
    }
}
