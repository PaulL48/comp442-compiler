// Transform grammar to LL(1) grammar
// Must be either a recursive descent predictive parser
// or a table driven predictive parser
// must have a textual output of the tree structure
// In the derivation output file, create a table like slide 23 from syntaxII.ppt
// outast can be a textual representation or a DOT file

// For state table based:
// One column for every token in language
// One row for every non-terminal

// Some sort of table + stack to track the tree

// Need tools to generate FIRST and FOLLOW sets of the symbols in the grammar
// This necessitates an internal representation of a grammar and production
mod grammar;
mod symbol;
mod parse_table;
mod parser;

pub use grammar::*;
pub use parse_table::*;
pub use parser::parse;
