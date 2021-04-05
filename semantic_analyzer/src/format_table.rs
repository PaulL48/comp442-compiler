/// A way for a type to yield table-like lines that describe itself.
/// Similar to fmt::Display except designed specifically for nesting types
/// displayed in an text based table.
pub trait FormatTable {
    fn lines(&self, width: usize) -> Vec<String>;
}
