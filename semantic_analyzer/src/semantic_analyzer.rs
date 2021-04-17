use crate::symbol_table;
use crate::symbol_table::symbol_table::SymbolTable;
use output_manager::{warn_write, OutputConfig};

pub struct SemanticAnalysisResults {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalysisResults {
    pub fn new() -> Self {
        SemanticAnalysisResults {
            symbol_table: SymbolTable::new("global"),
        }
    }
}

type Visitor = fn(&ast::Node, &mut SemanticAnalysisResults, &mut OutputConfig, &Vec<String>);
type EndOfPhaseCheck = fn(&mut SemanticAnalysisResults, &mut OutputConfig);

struct Phase {
    visitor: Visitor,
    end_of_phase: EndOfPhaseCheck,
}

impl Phase {
    pub fn new(visitor: Visitor, eopc: EndOfPhaseCheck) -> Self {
        Phase {
            visitor,
            end_of_phase: eopc,
        }
    }
}

pub fn analyze(root: &ast::Node, output_config: &mut OutputConfig) -> SemanticAnalysisResults {
    let phases: Vec<Vec<Phase>> = vec![vec![Phase::new(
        symbol_table::visitor::visit,
        symbol_table::visitor::end_of_phase,
    )]];

    let mut results: SemanticAnalysisResults = SemanticAnalysisResults::new();
    let current_scope = Vec::new();

    let _main_node = match root.data() {
        ast::Data::Children(children) => &children[2],
        _ => {
            panic!("No main in AST");
        }
    };

    for phase in phases {
        for visitor in phase {
            for node in root.dft() {
                (visitor.visitor)(node, &mut results, output_config, &current_scope);
            }
            (visitor.end_of_phase)(&mut results, output_config);
        }
    }

    output_config.flush_semantic_messages();

    // Write results to a file
    warn_write(
        &mut output_config.symbol_table_file,
        &output_config.symbol_table_path,
        &format!("{}", results.symbol_table),
    );

    results
}
