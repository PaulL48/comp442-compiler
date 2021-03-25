pub trait FormatTable {
    fn lines(&self, width: usize) -> Vec<String>;
}
