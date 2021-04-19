//! An intermediate storage area for an expression

use crate::format_table::FormatTable;
use derive_getters::Getters;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Temporary {
    id: String,
    data_type: String,
    bytes: usize,
    line: usize,
    column: usize,
}

impl fmt::Display for Temporary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Temporary value {} {}", self.data_type, self.id)
    }
}

impl FormatTable for Temporary {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:10}| {:10}| {:3}",
            "temp",
            self.id,
            self.data_type(),
            self.bytes,
        )]
    }
}

impl Temporary {

}

