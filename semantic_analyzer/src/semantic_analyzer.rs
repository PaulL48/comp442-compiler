use crate::symbol_table::symbol_table::SymbolTable;
use crate::symbol_table;
use ast;

pub struct SemanticAnalysisResults {
    symbol_table: SymbolTable,
}

impl SemanticAnalysisResults {
    pub fn new() -> Self {
        SemanticAnalysisResults {
            symbol_table: SymbolTable::new("global", &None),
        }
    }
}

pub fn analyze(root: &ast::Node) {
    let phases: Vec<Vec<fn(&ast::Node, &mut SemanticAnalysisResults)>> =
        vec![vec![symbol_table::visitor::visit]];

    let mut results: SemanticAnalysisResults = SemanticAnalysisResults::new();

    for phase in phases {
        for visitor in phase {
            for node in root.dft() {
                visitor(node, &mut results);
            }
        }
    }
}
