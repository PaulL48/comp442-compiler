//! Application specific validation and parsing of CLI arguments

use clap::ArgMatches;

pub struct CliConfig<'a> {
    pub source_folder: &'a str,
    pub output_folder: &'a str,
    pub lex_tokens_file: &'a str,
    pub keyword_file: &'a str,
    pub grammar_file: &'a str,
}

impl<'a> CliConfig<'a> {
    pub fn new(matches: &'a ArgMatches) -> CliConfig<'a> {
        CliConfig {
            source_folder: matches.value_of("INPUT").unwrap_or("test_sources"),
            output_folder: matches.value_of("output").unwrap_or("test_output"),
            lex_tokens_file: matches
                .value_of("tokens")
                .unwrap_or("resources/lex_tokens.txt"),
            keyword_file: matches
                .value_of("keywords")
                .unwrap_or("resources/keywords.txt"),
            grammar_file: matches
                .value_of("grammar")
                .unwrap_or("resources/grammar.txt"),
        }
    }
}
