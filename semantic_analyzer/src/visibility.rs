use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    Public,
    Private,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Private => write!(f, "private"),
            Visibility::Public => write!(f, "public"),
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}
