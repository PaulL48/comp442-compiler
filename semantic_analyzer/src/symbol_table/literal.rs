use crate::format_table::FormatTable;
use derive_getters::Getters;
use std::fmt;
use crate::sizes;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i32),
    Real(f32),
    StrLit(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Integer(i) => write!(f, "{}", i),
            LiteralValue::Real(r) => write!(f, "{}", r),
            LiteralValue::StrLit(s) => write!(f, "{}", s),
        }
    }
}

impl Default for LiteralValue {
    fn default() -> Self {LiteralValue::Integer(1337)}
}

#[derive(Debug, Clone, Default, Getters)]
pub struct Literal {
    id: String,
    value: LiteralValue,
    bytes: usize,
    line: usize,
    column: usize,
}

impl FormatTable for Literal {
    fn lines(&self, _: usize) -> Vec<String> {
        vec![format!(
            "{:10}| {:10}| {:10}| {:<10}|",
            "literal",
            self.id,
            self.value.to_string(),
            self.bytes
        )]
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Literal value {} {}", self.id(), self.value)
    }
}

impl Literal {
    pub fn new(id: &str, value: &LiteralValue, line: usize, column: usize) -> Self {
        Literal {
            id: id.to_owned(),
            value: value.clone(),
            bytes: 0,
            line,
            column,
        }
    }

    pub fn computed_size(&mut self) -> usize {
        let size = match self.value {
            LiteralValue::Integer(_) => 4,
            LiteralValue::Real(_) => 4,
            LiteralValue::StrLit(_) => 4
        };
        self.bytes = size;
        size
    }
}



