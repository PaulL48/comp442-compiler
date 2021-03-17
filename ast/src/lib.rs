// So as the semantic stack is built,
// anytime an element is popped off the stack it becomes a child in the ast being built.
mod ast;

pub use crate::ast::{Data, Node};
