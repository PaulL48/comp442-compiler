
pub trait FormatTable {
    type Iter: Iterator<Item = String>;

    fn lines(&self, width: usize) -> Self::Iter;
}
