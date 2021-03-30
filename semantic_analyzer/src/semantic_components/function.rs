use crate::format_table::FormatTable;
use crate::symbol_table::SymbolTable;
use crate::visibility::Visibility;
use crate::utils::separated_list;
use std::default::Default;

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub visibility: Option<Visibility>,
    pub symbol_table: SymbolTable,
}

impl FormatTable for Function {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut line = format!("{:10}| {:12}| {:34}", "function", self.name, self.signature());
        match self.visibility {
            Some(visibility) => line.push_str(&format!("| {}", visibility)),
            _ => (),
        }
        result.push(line);
        for l in self.symbol_table.lines(width - 8) {
            result.push(format!("   {}", l));
        }
        result
    }
}



impl Function {
    fn signature(&self) -> String {
        format!("({}): {}", separated_list(&self.parameter_types, ", "), self.return_type)
    }

    // pub fn new(name: &str) -> Self {
    //     Function {
    //         name: name.to_string(),
    //         parameter_types: Vec::new(),
    //         return_type: "".to_string(),
    //         visibility: None,
    //         symbol_table: SymbolTable::new(name, parent_scopes),
    //     }
    // }

    pub fn add_parameter(&mut self, data_type: &str) -> &mut Self {
        self.parameter_types.push(data_type.to_string());
        self
    }

    pub fn add_return_type(&mut self, data_type: &str) -> &mut Self {
        self.return_type = data_type.to_string();
        self
    }


}
