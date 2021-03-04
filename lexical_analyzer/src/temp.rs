// #[macro_use]
// extern crate lazy_static;
// #[macro_use]
// extern crate log;
// #[macro_use]
// extern crate clap;
// mod cli_config;
// mod double_buffer;
// mod lexer;
// mod lexical_rule;
// mod token;
// mod utilities;

// use clap::App;
// use simplelog::*;

// use cli_config::CliConfig;

// use std::io::prelude::*;

// use lexer::Lexer;
// use std::fs;

// /// Development switch to easily turn terminal logging on or off
// const LOGGING_SWITCH: LevelFilter = LevelFilter::Info;

// fn init_logging(level: LevelFilter) {
//     TermLogger::init(level, Config::default(), TerminalMode::Mixed)
//         .expect("Could not create logging interface");
//     info!(
//         "Logging interface initialized to terminal at {} level",
//         level
//     );
// }

// fn main() {
//     // CLI args processing ====================================================
//     let cli_config = load_yaml!("cli.yml");
//     let matches = App::from_yaml(cli_config).get_matches();
//     let config = CliConfig::new(&matches);
//     init_logging(LOGGING_SWITCH);

//     // Generation of lexical analyzer =========================================
//     let rules = lexical_rule::LexicalRule::from_file(config.lex_tokens_file)
//         .expect("Failed to construct lexical rules");
//     let keywords = std::fs::read_to_string(config.keyword_file)
//         .expect("Could not open keywords file")
//         .lines()
//         .map(|keyword| keyword.to_string())
//         .collect();

//     let output_dir = std::path::Path::new(config.output_folder);
//     if !output_dir.exists() {
//         match fs::create_dir_all(output_dir) {
//             Err(err) => {
//                 error!(
//                     "Could not create output directory \"{:?}\": {}",
//                     output_dir, err
//                 );
//                 panic!();
//             }
//             _ => (),
//         }
//     }

//     let l = Lexer::new(rules, keywords);
//     for src_file in utilities::files_with_extension(config.source_folder, "src") {
//         let (mut token_file, mut error_file) =
//             get_token_and_error_files(&src_file, config.output_folder);
//         let mut previous_line = 1;
//         info!("Beginning lexical analysis of file {}", src_file);
//         for token in l.lex(&src_file) {
//             match token {
//                 Ok(token) => {
//                     if token.error_token {
//                         match error_file.write_all(
//                             format!("{}\n", lexer::LexingError::from(token.clone())).as_bytes(),
//                         ) {
//                             Err(err) => {
//                                 error!("Unable to write buffer to token file: {}", err);
//                                 panic!();
//                             }
//                             _ => (),
//                         }
//                     }

//                     if token.line > previous_line {
//                         let newlines;
//                         if token.line > previous_line + 1 {
//                             newlines = "\n\n";
//                         } else {
//                             newlines = "\n";
//                         }
//                         match token_file.write_all(newlines.as_bytes()) {
//                             Err(err) => {
//                                 error!("Unable to write buffer to token file: {}", err);
//                                 panic!();
//                             }
//                             _ => previous_line = token.line,
//                         }
//                     }
//                     match token_file.write_all(format!("{} ", token).as_bytes()) {
//                         Err(err) => {
//                             error!("Unable to write buffer to token file: {}", err);
//                             panic!();
//                         }
//                         _ => (),
//                     }
//                 }
//                 Err(err) => match error_file.write_all(format!("{}\n", err).as_bytes()) {
//                     Err(err) => {
//                         error!("Unable to write buffer to token file: {}", err);
//                         panic!();
//                     }
//                     _ => (),
//                 },
//             };
//         }
//     }
// }

// const TOKEN_EXTENSION: &str = ".outlextokens";
// const ERROR_EXTENSION: &str = ".outlexerrors";

// fn get_token_and_error_files(src_file: &String, output_folder: &str) -> (fs::File, fs::File) {
//     let file_name = utilities::file_name(&src_file);
//     let token_output = String::from(output_folder) + "/" + file_name + TOKEN_EXTENSION;
//     let error_output = String::from(output_folder) + "/" + file_name + ERROR_EXTENSION;
//     info!(
//         "Tokens will appear in {}. Errors will appear in {}",
//         token_output, error_output
//     );
//     let token_file = match std::fs::OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .create(true)
//         .open(token_output.clone())
//     {
//         Ok(file) => file,
//         Err(err) => {
//             error!("Unable to open file \"{}\": {}", token_output, err);
//             panic!();
//         }
//     };
//     let error_file = match std::fs::OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .create(true)
//         .open(error_output.clone())
//     {
//         Ok(file) => file,
//         Err(err) => {
//             error!("Unable to open file \"{}\": {}", error_output, err);
//             panic!();
//         }
//     };
//     (token_file, error_file)
// }
