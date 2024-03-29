use log::{error, info, warn};
use path;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;

const SYMBOL_TABLE_EXT: &str = "outsymboltable";
const SEMANTIC_ERROR_EXT: &str = "outsemanticerrors";
const DERIVATION_EXT: &str = "outderivation";
const AST_EXT: &str = "outast";
const PARSE_ERROR_EXT: &str = "outsyntaxerrors";
const LEX_ERROR_EXT: &str = "outlexerrors";
const CODE_EXT: &str = "moon";

pub struct ErrorMessage {
    line: usize,
    column: usize,
    message: String,
}

impl ErrorMessage {
    pub fn message(&self) -> &String {
        &self.message
    }
}

impl ErrorMessage {
    fn new(line: usize, column: usize, message: &str) -> Self {
        ErrorMessage {
            line,
            column,
            message: message.to_owned(),
        }
    }
}

impl Ord for ErrorMessage {
    fn cmp(&self, other: &ErrorMessage) -> cmp::Ordering {
        if self.line < other.line {
            cmp::Ordering::Less
        } else if self.line == other.line {
            if self.column < other.column {
                cmp::Ordering::Less
            } else if self.column == other.column {
                cmp::Ordering::Equal
            } else {
                cmp::Ordering::Greater
            }
        } else {
            cmp::Ordering::Greater
        }
    }
}

impl cmp::PartialOrd for ErrorMessage {
    fn partial_cmp(&self, other: &ErrorMessage) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ErrorMessage {
    fn eq(&self, other: &ErrorMessage) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl Eq for ErrorMessage {}

pub struct OutputConfig {
    pub code_path: String,
    pub code_exec: Vec<String>,
    pub code_data: Vec<String>,
    pub code_file: File,

    pub symbol_table_path: String,
    pub symbol_table_file: File,

    pub semantic_error_path: String,
    pub semantic_error_file: File,
    pub semantic_error_buffer: Vec<ErrorMessage>,

    pub derivation_path: String,
    pub derivation_file: File,

    pub ast_path: String,
    pub ast_file: File,

    pub syntax_error_path: String,
    pub syntax_error_file: File,

    pub lex_error_path: String,
}

fn panic_open(path: &str) -> File {
    match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
    {
        Ok(file) => file,
        Err(err) => {
            error!("Failed to open file \"{}\": {}", path, err);
            panic!();
        }
    }
}

pub fn warn_write(file: &mut File, path: &str, message: &str) {
    match file.write_all(message.as_bytes()) {
        Err(err) => {
            warn!("Warning: Failed to write to file \"{}\": {}", path, err);
        }
        _ => (),
    }
}

pub fn write_list<T: std::fmt::Display>(file: &mut File, path: &str, list: &Vec<T>) {
    warn_write(file, path, "{");
    if list.is_empty() {
        warn_write(file, path, "}\n");
        return;
    }

    warn_write(file, path, &format!("{}", list.first().unwrap()));
    for s in list.iter().skip(1) {
        warn_write(file, path, &format!(", {}", s));
    }

    warn_write(file, path, "}\n");
}

pub fn write_array<T: std::fmt::Display>(file: &mut File, path: &str, list: &Vec<T>) {
    warn_write(file, path, "[");
    if list.is_empty() {
        warn_write(file, path, "]\n");
        return;
    }

    warn_write(file, path, &format!("{}", list.first().unwrap()));
    for s in list.iter().skip(1) {
        warn_write(file, path, &format!(", {}", s));
    }

    warn_write(file, path, "]\n");
}

impl OutputConfig {
    pub fn new(source_file_path: &str, output_directory: &str) -> Self {
        path::touch_dir(output_directory);

        let source_file_name = path::file_name(source_file_path).unwrap();
        let output = output_directory.to_string() + "/" + source_file_name;
        let output_no_ext = path::replace_extension(&output, "").unwrap();

        let code_path = path::replace_extension(&output, CODE_EXT).unwrap();
        let symbol_table_path = path::replace_extension(&output, SYMBOL_TABLE_EXT).unwrap();
        let semantic_error_path = path::replace_extension(&output, SEMANTIC_ERROR_EXT).unwrap();
        let derivation_path = path::replace_extension(&output, DERIVATION_EXT).unwrap();
        let ast_path = path::replace_extension(&output, AST_EXT).unwrap();
        let syntax_error_path = path::replace_extension(&output, PARSE_ERROR_EXT).unwrap();
        let lex_error_path = path::replace_extension(&output, LEX_ERROR_EXT).unwrap();

        let code_file = panic_open(&code_path);
        let symbol_table_file = panic_open(&symbol_table_path);
        let semantic_error_file = panic_open(&semantic_error_path);
        let derivation_file = panic_open(&derivation_path);
        let ast_file = panic_open(&ast_path);
        let syntax_error_file = panic_open(&syntax_error_path);

        info!("Processing source file \"{}\"", source_file_path);
        info!("Outputs and error will appear in files named \"{}.*\" where the extension specifies the contents of the file", output_no_ext);
        // info!("Lexical errors will appear in \"{}\"", lex_error_path);
        // info!("Syntax errors will appear in \"{}\"", syntax_error_path);
        // info!("Grammar derivation will appear in \"{}\"", derivation_path);
        // info!("AST will appear in \"{}\"", ast_path);

        OutputConfig {
            code_path,
            code_file,
            code_exec: Vec::new(),
            code_data: Vec::new(),
            symbol_table_file,
            symbol_table_path,
            semantic_error_file,
            semantic_error_path,
            semantic_error_buffer: Vec::new(),
            derivation_file,
            derivation_path,
            ast_file,
            ast_path,
            syntax_error_file,
            syntax_error_path,
            lex_error_path,
        }
    }

    pub fn flush_semantic_messages(&mut self) {
        self.semantic_error_buffer.sort();
        for message in &self.semantic_error_buffer {
            warn_write(
                &mut self.semantic_error_file,
                &self.semantic_error_path,
                &format!("{}\n", message.message()),
            )
        }
    }

    pub fn add(&mut self, message: &str, line: usize, column: usize) {
        self.semantic_error_buffer
            .push(ErrorMessage::new(line, column, message))
    }

    pub fn add_exec(&mut self, line: &str) {
        self.code_exec.push(line.to_string());
    }

    pub fn add_data(&mut self, line: &str) {
        self.code_data.push(line.to_string());
    }

    pub fn flush_code(&mut self) {
        for line in &self.code_exec {
            warn_write(&mut self.code_file, &self.code_path, &line)
        }

        for line in &self.code_data {
            warn_write(&mut self.code_file, &self.code_path, &line)
        }
    }

    pub fn has_errors(&self) -> bool {
        for entry in &self.semantic_error_buffer {
            if let Some(position) = entry.message().find("error") {
                return true;
            }
        }
        return false;
    }
}
