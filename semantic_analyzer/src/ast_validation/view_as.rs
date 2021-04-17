use crate::ast_validation::node_validator::ValidatorError;
use ast::Node;
use log::error;

pub trait ViewAs<'a>: Sized {
    fn view_as(node: &'a Node) -> Result<Self, ValidatorError>;
    fn try_view_as(node: &'a Node) -> Self {
        match ViewAs::view_as(node) {
            Ok(result) => result,
            Err(err) => {
                error!("{}", err);
                panic!();
            }
        }
    }
}

/*
pub fn select(node: &Node) {
    match node.name().as_str() {
        
    }

}


*/

