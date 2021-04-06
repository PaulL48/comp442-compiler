use crate::symbol_table;
use crate::symbol_table::symbol_table::SymbolTable;
use output_manager::{OutputConfig, warn_write};

pub struct SemanticAnalysisResults {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalysisResults {
    pub fn new() -> Self {
        SemanticAnalysisResults {
            symbol_table: SymbolTable::new("global", &None),
        }
    }
}

pub fn analyze(root: &ast::Node, output_config: &mut OutputConfig) -> SemanticAnalysisResults {
    let phases: Vec<Vec<fn(&ast::Node, &mut SemanticAnalysisResults, &mut OutputConfig)>> =
        vec![vec![symbol_table::visitor::visit]];

    let mut results: SemanticAnalysisResults = SemanticAnalysisResults::new();

    for phase in phases {
        for visitor in phase {
            for node in root.dft() {
                visitor(node, &mut results, output_config);
            }
        }
    }

    // Check the symbol table for class functions that haven't yet been defined
    results.symbol_table.check_declared_but_not_defined_functions(output_config);

    // Write results to a file 
    warn_write(&mut output_config.symbol_table_file, &output_config.symbol_table_path, &format!("{}", results.symbol_table));

    results
}
