//! An intermediate storage area for an expression

use crate::format_table::FormatTable;
use derive_getters::Getters;
use std::fmt;
use crate::sizes;

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
            "{:10}| {:10}| {:10}| {:<10}|",
            "temp",
            self.id,
            self.data_type,
            self.bytes,
        )]
    }
}

impl Temporary {
    pub fn new(id: &str, data_type: &str, line: usize, column: usize) -> Self {
        Temporary {
            id: id.to_owned(),
            data_type: data_type.to_owned(),
            bytes: 0,
            line,
            column
        }
    }

    pub fn computed_size(&mut self) -> usize {
        let size = sizes::size_of(&self.data_type, &Vec::new());
        self.bytes = size;
        size
    }
}

