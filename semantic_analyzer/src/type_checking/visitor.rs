use crate::semantic_analyzer::SemanticAnalysisResults;
use ast::Node;
use output_manager::OutputConfig;
use crate::SemanticError;
use crate::type_checking::typing::get_type;

pub fn visit(node: &Node, current_data: &mut SemanticAnalysisResults, output_config: &mut OutputConfig, current_scope: &Vec<String>) {
    match node.name().as_str() {
        "assignOp" => {
            if let Err(err) = assign_op(node, current_data, output_config, current_scope) {
                err.write(output_config);
            }
        },
        _=>()
    }
}

fn assign_op(node: &Node, current_data: &mut SemanticAnalysisResults, output_config: &mut OutputConfig, current_scope: &Vec<String>) -> Result<(), SemanticError> {
    // This node will have a lhs and a rhs
    // lhs must be assignable
    // and of the same type as rhs
    // rhs side must have a consistent type
    match node.data() {
        ast::Data::Children(children) => {
            // TODO: Check here if this is a var or an fcall on children[0] lhs
            let lhs = &children[0];
            let rhs = &children[1];
            let lhs_type = get_type(lhs, &current_data.symbol_table, current_scope, output_config)?;
            let rhs_type = get_type(rhs, &current_data.symbol_table, current_scope, output_config)?;
            // println!("FROM Assign Op");
            // println!("lhs: {} | rhs: {}", lhs_type, rhs_type);
        },
        _ => {panic!()}
    }
    Ok(())
}


