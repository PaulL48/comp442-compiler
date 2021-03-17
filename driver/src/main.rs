mod cli_config;

use clap::{load_yaml, App};
use cli_config::CliConfig;
use lexical_analyzer::{lexer::Lexer, lexical_rule::LexicalRule};
use log::{error, info};
use path;
use simplelog::*;
use syntactic_analyzer::{parse, Grammar, ParseTable, Symbol, unexpanded_follow};

/// Development switch to easily turn terminal logging on or off
const LOGGING_SWITCH: LevelFilter = LevelFilter::Trace;

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
    info!("Extracting grammar productions from file \"{}\"", config.grammar_file);
    let g = Grammar::from_reader(File::open(config.grammar_file)?)?;
    println!("{:?}", g.first(&Symbol::NonTerminal("FuncOrAssignStatIdnestVarTail".to_string())));
    println!("{:?}", g.follow(&Symbol::NonTerminal("FuncOrAssignStatIdnestVarTail".to_string())));
    println!("{:?}", g.follow(&Symbol::NonTerminal("FuncOrAssignStatIdnest".to_string())));

    println!("{:?}", g.follow(&Symbol::NonTerminal("IndiceRep".to_string())));
    let parse_table = ParseTable::from_grammar(&g);
    println!("{:?}", unexpanded_follow(g.productions(), g.start(), &Symbol::NonTerminal("IndiceRep".to_string())));
    println!("{:?}", parse_table.table[&Symbol::NonTerminal("FuncOrAssignStatIdnestVarTail".to_string())]);
    parse(
        &mut l.lex("resources/bubblesort.src", "lex_errors.ole"),
        &g,
        &parse_table,
    );

    Ok(())
}
