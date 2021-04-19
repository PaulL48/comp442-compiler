mod cli_config;

use clap::{load_yaml, App};
use cli_config::CliConfig;
use code_gen;
use lexical_analyzer::{lexer::Lexer, lexical_rule::LexicalRule};
use log::{error, info};
use output_manager::OutputConfig;
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

fn main() -> std::io::Result<()> {
    // CLI args processing ====================================================
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();
    let config = CliConfig::new(&matches);
    init_logging(LOGGING_SWITCH);

    let output_dir = std::path::Path::new(config.output_folder);
    if !output_dir.exists() {
        if let Err(err) = std::fs::create_dir_all(output_dir) {
            error!(
                "Could not create output directory \"{:?}\": {}",
                output_dir, err
            );
            panic!();
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
        let result = parse(
            &mut l.lex(&source_file, &oc.lex_error_path),
            &g,
            &parse_table,
            &mut oc,
        );

        if let Some(ast) = result {
            let mut result = semantic_analyzer::analyze(&ast, &mut oc);
            // TODO: Add check if the semantic analysis failed or not

            code_gen::process(&ast, &mut result, &mut oc)
        }
    }

    Ok(())
}
