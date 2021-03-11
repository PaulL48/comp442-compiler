mod cli_config;

use clap::{load_yaml, App};
use cli_config::CliConfig;
use lexical_analyzer::{lexer::Lexer, lexical_rule::LexicalRule};
use log::info;
use path;
use simplelog::*;
use syntactic_analyzer::{Grammar, ParseTable};

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

fn main() {
    // CLI args processing ====================================================
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();
    let config = CliConfig::new(&matches);
    init_logging(LOGGING_SWITCH);

    let rules = LexicalRule::from_file(config.lex_tokens_file).expect("Failed to build rule set");
    let l = Lexer::new(rules, vec![]);
    let g = Grammar::from_reader(File::open(config.grammar_file).unwrap()).unwrap();
    let parse_table = ParseTable::from_grammar(&g);
    parse_table.parse(&g, l, "resources/test.src");

    // for entry in path::directory("driver/src")
    //     .filter(|entry| path::is_file(entry) && path::extension(entry).unwrap_or("") == "rs")
    // {
    //     println!("{}", entry);
    // }
}
