use crate::format_table::FormatTable;
use crate::symbol_table::*;
use crate::SemanticError;
use output_manager::OutputConfig;
use std::default::Default;
use std::fmt;

use crate::symbol_table::Class;
use maplit::hashset;
use std::collections::HashSet;

const GLOBAL_TABLE_WIDTH: usize = 83;
const TEMP_PREFIX: &str = "temp";
const ELSE_PREFIX: &str = "else";
const ENDIF_PREFIX: &str = "endif";
const GOWHILE_PREFIX: &str = "gowhile";
const ENDWHILE_PREFIX: &str = "endwhile";

#[derive(Debug, Clone)]
pub enum SymbolTableEntry {
    Class(class::Class),
    Function(function::Function),
    Inherit(inherit::Inherit),
    Param(param::Param),
    Local(local::Local),
    Data(data::Data),
    Literal(literal::Literal),
    Temporary(temporary::Temporary),
}

pub trait TableEntryGenerator<T> {
    fn add_symbol_table_entry(
        validated_node: &T,
        global_table: &mut SymbolTable,
        current_scope: &Vec<String>,
        output_config: &mut OutputConfig,
    ) -> Result<(), SemanticError>;
}

impl fmt::Display for SymbolTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolTableEntry::Class(class) => class.fmt(f),
            SymbolTableEntry::Function(function) => function.fmt(f),
            SymbolTableEntry::Inherit(inherit) => inherit.fmt(f),
            SymbolTableEntry::Param(param) => param.fmt(f),
            SymbolTableEntry::Local(local) => local.fmt(f),
            SymbolTableEntry::Data(data) => data.fmt(f),
            SymbolTableEntry::Literal(literal) => literal.fmt(f),
            SymbolTableEntry::Temporary(temporary) => temporary.fmt(f),
        }
    }
}

impl Default for SymbolTableEntry {
    fn default() -> Self {
        SymbolTableEntry::Local(local::Local::default())
    }
}

impl SymbolTableEntry {
    pub fn id(&self) -> Option<&str> {
        match self {
            SymbolTableEntry::Class(class) => Some(class.id()),
            SymbolTableEntry::Function(function) => Some(function.id()),
            SymbolTableEntry::Inherit(_) => None,
            SymbolTableEntry::Param(param) => Some(param.id()),
            SymbolTableEntry::Local(local) => Some(local.id()),
            SymbolTableEntry::Data(data) => Some(data.id()),
            SymbolTableEntry::Literal(literal) => Some(literal.id()),
            SymbolTableEntry::Temporary(temporary) => Some(temporary.id()),
        }
    }

    pub fn computed_size(&mut self) -> usize {
        match self {
            SymbolTableEntry::Class(class) => class.computed_size(),
            SymbolTableEntry::Function(function) => function.computed_size(),
            SymbolTableEntry::Inherit(_) => 0,
            SymbolTableEntry::Param(param) => param.computed_size(),
            SymbolTableEntry::Local(local) => local.computed_size(),
            SymbolTableEntry::Data(data) => data.computed_size(),
            SymbolTableEntry::Literal(literal) => literal.computed_size(),
            SymbolTableEntry::Temporary(temporary) => temporary.computed_size(),
        }
    }
}

impl FormatTable for SymbolTableEntry {
    fn lines(&self, width: usize) -> Vec<String> {
        match self {
            SymbolTableEntry::Class(c) => c.lines(width),
            SymbolTableEntry::Function(f) => f.lines(width),
            SymbolTableEntry::Inherit(i) => i.lines(width),
            SymbolTableEntry::Param(p) => p.lines(width),
            SymbolTableEntry::Local(l) => l.lines(width),
            SymbolTableEntry::Data(d) => d.lines(width),
            SymbolTableEntry::Literal(literal) => literal.lines(width),
            SymbolTableEntry::Temporary(temporary) => temporary.lines(width),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub name: String,
    pub values: Vec<SymbolTableEntry>,
    pub scope: Option<String>,

    temp_var_count: usize,
    if_else_count: usize,
    while_count: usize,
}

/// Helper type for inheritance based searches
/// Determines whether the initial supplied class is searched
pub enum Search {
    Inclusive,
    Exclusive,
}

impl FormatTable for SymbolTable {
    fn lines(&self, width: usize) -> Vec<String> {
        let mut result = vec![
            self.header_bar(width),
            format!("| {:1$}  |", self.title(), width - 5),
            self.header_bar(width),
        ];
        result.extend(
            self.values
                .iter()
                .flat_map(|x| x.lines(width))
                .map(|x| format!("| {:1$}  |", x, width - 5)),
        );
        result.push(self.header_bar(width));
        result
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in self.lines(GLOBAL_TABLE_WIDTH) {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}

impl Extend<SymbolTableEntry> for SymbolTable {
    fn extend<T: IntoIterator<Item = SymbolTableEntry>>(&mut self, iter: T) {
        for elem in iter {
            self.add_entry(elem);
        }
    }
}

impl SymbolTable {
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Add an entry to the symbol table and return a mutable reference to it
    pub fn add_entry(&mut self, entry: SymbolTableEntry) -> &mut SymbolTableEntry {
        self.values.push(entry);
        self.values.last_mut().unwrap()
    }

    /// Return the title string of this symbol table
    fn title(&self) -> String {
        let mut title = "".to_string();
        if let Some(scope) = &self.scope {
            title.push_str(&scope);
            title.push_str("::");
        }
        format!("table: {}{}", title, self.name)
    }

    /// Return the line of "=" that makes up the horizontal rules in the table string
    fn header_bar(&self, table_width: usize) -> String {
        format!("{:=<1$}", "", table_width)
    }

    pub fn get_next_temporary(&mut self) -> String {
        let result = format!("{}{}", TEMP_PREFIX, self.temp_var_count);
        self.temp_var_count += 1;
        result
    }

    pub fn get_next_if_else_label(&mut self) -> (String, String) {
        let result1 = format!("{}__{}{}", self.name, ELSE_PREFIX, self.if_else_count);
        let result2 = format!("{}__{}{}", self.name, ENDIF_PREFIX, self.if_else_count);
        self.if_else_count += 1;
        return (result1, result2);
    }

    pub fn get_next_while_label(&mut self) -> (String, String) {
        let result1 = format!("{}__{}{}", self.name, GOWHILE_PREFIX, self.while_count);
        let result2 = format!("{}__{}{}", self.name, ENDWHILE_PREFIX, self.while_count);
        self.while_count += 1;
        return (result1, result2);
    }

    // pub fn get_previous_mangled_name(&self) -> String {
    //     format!("{}__{}{}", self.name, TEMP_PREFIX, self.temp_var_count - 1)
    // }

    // pub fn mangle(&self, id: &str) -> String {
    //     format!("{}__{}", self.name, id)
    // }

    pub fn get_function<'a, T: AsRef<str>>(
        &'a self,
        id: &str,
        parameters: &[T],
    ) -> Option<&'a Function> {
        // Select the correct overload
        let matches = self.get_all(id);
        for entry in matches {
            match entry {
                SymbolTableEntry::Function(function) => {
                    if function.signature_matches(id, parameters) {
                        return Some(function);
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn mangle_function<T: AsRef<str>>(&self, id: &str, parameters: &[T]) -> String {
        let mut result = String::new();
        result.push_str(id);
        for parameter in parameters {
            result.push_str(parameter.as_ref());
        }
        result
    }

    // pub fn get_internal_variable_prefix(&mut self) -> String {
    //     let mut result = String::new();
    //     result.push_str("__");
    //     if let Some(scope) = &self.scope {
    //         result.push_str(&format!("{}_", scope));
    //     }
    //     result.push_str(&format!(
    //         "{}_{}{}",
    //         self.name, TEMP_PREFIX, self.temp_var_count
    //     ));
    //     self.temp_var_count += 1;
    //     result
    // }

    pub fn new(name: &str) -> Self {
        SymbolTable {
            name: name.to_string(),
            scope: None,
            values: Vec::new(),
            temp_var_count: 0,
            if_else_count: 0,
            while_count: 0,
        }
    }

    pub fn scoped_new(name: &str, scope: Option<&str>) -> Self {
        SymbolTable {
            name: name.to_string(),
            scope: scope.map(|x| x.to_string()),
            values: Vec::new(),
            temp_var_count: 0,
            if_else_count: 0,
            while_count: 0,
        }
    }

    pub fn get(&self, id: &str) -> Option<&SymbolTableEntry> {
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SymbolTableEntry> {
        for entry in &mut self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Return the list of all entries in this table with the supplied identifier
    pub fn get_all(&self, id: &str) -> Vec<&SymbolTableEntry> {
        let mut result = Vec::new();
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    result.push(entry);
                }
            }
        }
        result
    }

    pub fn get_all_mut(&mut self, id: &str) -> Vec<&mut SymbolTableEntry> {
        let mut result = Vec::new();
        for entry in &mut self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    result.push(entry);
                }
            }
        }
        result
    }

    pub fn replace_class_function_declaration(&mut self, definition: Function) {
        if let Some(scope) = definition.scope() {
            // Should have a scope
            if let Some(SymbolTableEntry::Class(class)) = self.get_mut(scope) {
                class.symbol_table_mut().replace_declaration(definition);
            }
        }
    }

    fn replace_declaration(&mut self, definition: Function) {
        let position = self.values.iter().position(|x| {
            if let SymbolTableEntry::Function(function) = x {
                return *function == definition;
            }
            false
        });

        if let Some(position) = position {
            self.values[position] = SymbolTableEntry::Function(definition);
        } else {
            panic!("Asking to replace nonexistent declaration");
        }
    }

    /// Return the list of entries that share the identifier from the supplied class' inheritance hierarchy
    /// The purpose of this is to allow a check at the definition of a class member for shadowing, and overriding
    /// Checking for redefinition and overloading should be done with get_all
    pub fn get_all_inherited<'a>(
        class: &'a Class,
        id: &str,
        global_table: &'a SymbolTable,
    ) -> Vec<&'a SymbolTableEntry> {
        let mut result = Vec::new();
        let mut visited = hashset! {class.id().clone()};

        // search the inherited classes recursively
        for inherited in class.inheritance_list() {
            if let Some(SymbolTableEntry::Class(class)) = global_table.get(inherited) {
                if visited.contains(class.id()) {
                    continue;
                }
                result.extend(class.symbol_table().get_all_inherited_aux(
                    class,
                    id,
                    global_table,
                    &mut visited,
                ))
            }
        }
        result
    }

    /// Implementation of get_all_inherited
    fn get_all_inherited_aux<'a>(
        &'a self,
        class: &'a Class,
        id: &str,
        global_table: &'a SymbolTable,
        visited: &mut HashSet<String>,
    ) -> Vec<&'a SymbolTableEntry> {
        let mut result = Vec::new();

        // search the current class
        result.extend(class.symbol_table().get_all(id));
        visited.insert(class.id().clone());

        // search the inherited classes recursively
        for inherited in class.inheritance_list() {
            if let Some(SymbolTableEntry::Class(class)) = global_table.get(inherited) {
                if visited.contains(class.id()) {
                    continue;
                    // Signal a cyclic inheritance
                }

                result.extend(class.symbol_table().get_all_inherited_aux(
                    class,
                    id,
                    global_table,
                    visited,
                ))
            }
        }

        result
    }

    pub fn inherit_list_has_cycles(&self, global_table: &SymbolTable) -> bool {
        self.inherit_list_has_cycles_aux(&self.collect_inherits(), global_table, &mut Vec::new())
    }

    fn inherit_list_has_cycles_aux(
        &self,
        inherit_list: &Vec<String>,
        global_table: &SymbolTable,
        visited: &mut Vec<String>,
    ) -> bool {
        for inherit in inherit_list {
            if visited.contains(&inherit.to_string()) {
                return true;
            }
        }

        visited.extend(inherit_list.iter().map(|x| x.to_string()));
        for inherit in inherit_list {
            // get the class, get the inherit list and call again
            if let Some(entry) = global_table.get(inherit) {
                if let SymbolTableEntry::Class(class) = entry {
                    if class.symbol_table().inherit_list_has_cycles_aux(
                        &class.symbol_table().collect_inherits(),
                        global_table,
                        visited,
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Return all inherit entries
    fn collect_inherits(&self) -> Vec<String> {
        for entry in &self.values {
            if let SymbolTableEntry::Inherit(inherit) = entry {
                return inherit.names().clone();
            }
        }
        return Vec::new();
    }

    pub fn contains(&self, id: &str) -> bool {
        for entry in &self.values {
            if let Some(entry_id) = entry.id() {
                if entry_id == id {
                    return true;
                }
            }
        }
        false
    }
}
