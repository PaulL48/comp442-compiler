use std::fs::File;
use path;
use log::{error, warn, info};
use std::io::prelude::*;

const DERIVATION_EXT: &str = "outderivation";
const AST_EXT: &str = "outast";
const PARSE_ERROR_EXT: &str = "outsyntaxerrors";
const LEX_ERROR_EXT: &str = "outlexerrors";

pub struct OutputConfig {
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
        _ => ()
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

        let derivation_path = path::replace_extension(&output, DERIVATION_EXT).unwrap();
        let ast_path = path::replace_extension(&output, AST_EXT).unwrap();
        let syntax_error_path = path::replace_extension(&output, PARSE_ERROR_EXT).unwrap();
        let lex_error_path = path::replace_extension(&output, LEX_ERROR_EXT).unwrap();

        let derivation_file = panic_open(&derivation_path);
        let ast_file = panic_open(&ast_path);
        let syntax_error_file = panic_open(&syntax_error_path);

        info!("Processing source file \"{}\"", source_file_path);
        info!("Lexical errors will appear in \"{}\"", lex_error_path);
        info!("Syntax errors will appear in \"{}\"", syntax_error_path);
        info!("Grammar derivation will appear in \"{}\"", derivation_path);
        info!("AST will appear in \"{}\"", ast_path);

        OutputConfig {
            derivation_file,
            derivation_path,
            ast_file,
            ast_path,
            syntax_error_file,
            syntax_error_path,
            lex_error_path
        }
    }
}


