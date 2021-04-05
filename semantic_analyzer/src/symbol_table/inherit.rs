use crate::format_table::FormatTable;
use crate::utils::separated_list;
use derive_getters::Getters;
use std::default::Default;
use std::fmt;

#[derive(Debug, Clone, Default, Getters)]
pub struct Inherit {
    names: Vec<String>,
}

impl fmt::Display for Inherit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.names.first() {
            write!(f, "{}", name)?;
        }

        for name in self.names.iter().skip(1) {
            write!(f, ", {}", name)?;
        }
        Ok(())
    }
}

impl FormatTable for Inherit {
    fn lines(&self, _: usize) -> Vec<String> {
        if self.names.is_empty() {
            vec![format!("{:10}| none", "inherit")]
        } else {
            vec![format!(
                "{:10}| {}",
                "inherit",
                separated_list(&self.names, ", ")
            )]
        }
    }
}

impl Inherit {
    pub fn new(id_list: &[&str]) -> Self {
        Inherit {
            names: id_list.iter().map(|x| x.to_string()).collect(),
        }
    }
}
