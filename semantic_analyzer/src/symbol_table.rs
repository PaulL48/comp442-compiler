use std::fmt;
use std::iter;
use crate::format_table::FormatTable;

enum SymbolTableEntry {
    Class(Class),
    Function(Function),
    Inherit(Inherit),
    Param(Param),
    Local(Local),
    Data(Data),
}

impl FormatTable for SymbolTableEntry {
    type Iter = impl Iterator<Item = String>;

    fn lines(&self) {
        match self {
            SymbolTableEntry::Class(a) => a.lines(),
            SymbolTableEntry::
        }
    }
}

struct SymbolTable {
    name: String,
    values: Vec<SymbolTableEntry>
}

impl SymbolTable {
    pub fn lines(&self, table_width: usize) -> impl Iterator<Item = String> {
        iter::once(self.header_bar(table_width))
            .chain(iter::once(format!("| {:1$}  |", self.title(), table_width - 5)))
            .chain(iter::once(self.header_bar(table_width)))
            .chain(self.value_lines(table_width))
            .chain(iter::once(self.header_bar(table_width)))
    }   

    fn value_lines(&self, table_width: usize) -> impl Iterator<Item = String> {
        // self.values.iter().map(|x| {
        //     match x {

        //     }
        //     format!("| {:1$}  |", x, table_width - 5)
        // })

        let mut a = iter::empty();
        for value in self.values {
            // value.lines()
        }
    }

    fn title(&self) -> String {
        format!("table: {}", self.name)
    }

    fn header_bar(&self, table_width: usize) -> String {
        format!("{:=<1$}", "", table_width)
    }
}

enum Visibility {
    Public,
    Private
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Private => write!(f, "private"),
        }
    }
}

struct Class {
    name: String,
    symbol_table: SymbolTable,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:10}|", "class");
        write!(f, " {}", self.name)
    }
}

// Writing these tables would require
// a way to print each line which means
// it would have to be aware of how many tables nested it is
// which technically is a number between 0 and 2

// something like
// println!("| {:1$}  |", table_width - 5)
// where for nesting 0, table_width is 83. 1 is 75. 2 is 66
// for a sub table 
// the interface could be something like
// for line in symbol_table.lines() {
//    println!("|    {:1$}  |", table_width - 5);
// }

// lines() would have to somehow manually output the table header
// and then chain with the innards of the table
// then chain with the table footer



struct Function {
    name: String,
    parameter_types: Vec<String>,
    return_type: String,
    visibility: Option<Visibility>,
    symbol_table: SymbolTable,
}

struct Inherit {
    names: Vec<String>
}

struct Param {
    name: String,
    data_type: String,
}

struct Local {
    name: String,
    data_type: String
}

struct Data {
    name: String,
    data_type: String,
    visibility: Visibility
}
