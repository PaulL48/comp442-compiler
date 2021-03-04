mod cli_config;

use clap::{load_yaml, App};
use log::info;
use simplelog::*;
use path;
use cli_config::CliConfig;

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

fn main() {
    // CLI args processing ====================================================
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();
    let config = CliConfig::new(&matches);
    init_logging(LOGGING_SWITCH);

    for entry in path::directory("driver/src")
        .filter(|entry| path::is_file(entry) && path::extension(entry).unwrap_or("") == "rs")
    {
        println!("{}", entry);
    }
}
