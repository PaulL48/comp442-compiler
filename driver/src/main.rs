mod cli_config;

use clap::{load_yaml, App};
use cli_config::CliConfig;
use lexical_analyzer::{lexer::Lexer, lexical_rule::LexicalRule};
use log::{error, info};
use output_manager::OutputConfig;
use path;
use semantic_analyzer::symbol_table::{
    class::Class, data::Data, function::Function, local::Local, symbol_table::SymbolTable,
    symbol_table::SymbolTableEntry,
};
use simplelog::*;
use syntactic_analyzer::{parse, Grammar, ParseTable};

/// Development switch to easily turn terminal logging on or off
const LOGGING_SWITCH: LevelFilter = LevelFilter::Info;

fn init_logging(level: LevelFilter) {
    TermLogger::init(level, Config::default(), TerminalMode::Mixed)
        .expect("Could not create logging interface");
    info!(
        "Logging interface initialized to terminal at {} level",
        level
    );
}

use std::fs::File;

// TODO: dimLists don't work in the syntactic analyzer

fn main() -> std::io::Result<()> {
    // let dst: SymbolTable = SymbolTable {name: "global".to_string(), .. Default::default()};
    // println!("{}", dst);

    // let st = SymbolTable {
    //     name: "global".to_string(),
    //     parent_scopes: vec![],
    //     values: vec![
    //         SymbolTableEntry::Data(Data {
    //             name: "d1".to_string(),
    //             data_type: "float".to_string(),
    //             visibility: Visibility::Public,
    //         }),
    //         SymbolTableEntry::Local( Local {
    //             name: "l1".to_string(),
    //             data_type: "integer".to_string()
    //         }),
    //         SymbolTableEntry::Function( Function {
    //             name: "f1".to_string(),
    //             parameter_types: vec!["float".to_string(), "integer".to_string()],
    //             return_type: "void".to_string(),
    //             visibility: None,
    //             symbol_table: SymbolTable {
    //                 name: "f1".to_string(),
    //                 parent_scopes: vec!["".to_string()],
    //                 values: vec![
    //                     SymbolTableEntry::Data(Data {
    //                         name: "d1".to_string(),
    //                         data_type: "float".to_string(),
    //                         visibility: Visibility::Public,
    //                     }),
    //                     SymbolTableEntry::Local( Local {
    //                         name: "l1".to_string(),
    //                         data_type: "integer".to_string()
    //                     })
    //                 ]
    //             }
    //         }),
    //         SymbolTableEntry::Class( Class {
    //             name: "c1".to_string(),
    //             symbol_table: SymbolTable {
    //                 name: "c1".to_string(),
    //                 parent_scopes: vec![],
    //                 values: vec![
    //                     SymbolTableEntry::Data(Data {
    //                         name: "d1".to_string(),
    //                         data_type: "float".to_string(),
    //                         visibility: Visibility::Public,
    //                     }),
    //                     SymbolTableEntry::Data(Data {
    //                         name: "d2".to_string(),
    //                         data_type: "string".to_string(),
    //                         visibility: Visibility::Private,
    //                     }),
    //                     SymbolTableEntry::Function( Function {
    //                         name: "f1".to_string(),
    //                         parameter_types: vec!["float".to_string(), "integer".to_string()],
    //                         return_type: "void".to_string(),
    //                         visibility: None,
    //                         symbol_table: SymbolTable {
    //                             name: "f1".to_string(),
    //                             parent_scopes: vec!["c1".to_string()],
    //                             values: vec![
    //                                 SymbolTableEntry::Local( Local {
    //                                     name: "l1".to_string(),
    //                                     data_type: "integer".to_string()
    //                                 })
    //                             ]
    //                         }
    //                     }),
    //                     SymbolTableEntry::Function( Function {
    //                         name: "f1".to_string(),
    //                         parameter_types: vec!["float".to_string(), "integer".to_string()],
    //                         return_type: "void".to_string(),
    //                         visibility: None,
    //                         symbol_table: SymbolTable {
    //                             name: "f1".to_string(),
    //                             parent_scopes: vec!["c1".to_string()],
    //                             values: vec![
    //                                 SymbolTableEntry::Local( Local {
    //                                     name: "l1".to_string(),
    //                                     data_type: "integer".to_string()
    //                                 })
    //                             ]
    //                         }
    //                     })
    //                 ]
    //             }
    //         })
    //     ]
    // };
    // println!("{}", st);
    // return Ok(());

    // CLI args processing ====================================================
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();
    let config = CliConfig::new(&matches);
    init_logging(LOGGING_SWITCH);

    let output_dir = std::path::Path::new(config.output_folder);
    if !output_dir.exists() {
        match std::fs::create_dir_all(output_dir) {
            Err(err) => {
                error!(
                    "Could not create output directory \"{:?}\": {}",
                    output_dir, err
                );
                panic!();
            }
            _ => (),
        }
    }

    let rules = LexicalRule::from_file(config.lex_tokens_file).expect("Failed to build rule set");
    let keywords = std::fs::read_to_string(config.keyword_file)
        .expect("Could not open keywords file")
        .lines()
        .map(|keyword| keyword.to_string())
        .collect();
    let l = Lexer::new(rules, keywords);
    info!(
        "Extracting grammar productions from file \"{}\"",
        config.grammar_file
    );
    let g = Grammar::from_reader(File::open(config.grammar_file)?)?;
    let parse_table = ParseTable::from_grammar(&g);

    for source_file in path::directory(config.source_folder)
        .filter(|x| path::is_file(x) && path::extension(x).unwrap_or("") == "src")
    {
        let mut oc = OutputConfig::new(&source_file, config.output_folder);
        parse(
            &mut l.lex(&source_file, &oc.lex_error_path),
            &g,
            &parse_table,
            &mut oc,
        );
    }

    Ok(())
}
