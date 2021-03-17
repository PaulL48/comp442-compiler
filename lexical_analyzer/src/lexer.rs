use crate::double_buffer::{DoubleFixedBuffer, DoubleFixedBufferCursor, BUFFER_SIZE};
use crate::lexical_rule::{FusedRuleState, LexicalRule, RuleState};
use crate::token::Token;
use crate::utilities::is_start_of_codepoint;
use lazy_static::lazy_static;
use log::{error, trace, warn};
use regex_automata::DFA;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

pub struct Lexer {
    rules: Vec<LexicalRule>,
    keywords: Vec<String>,
}

/// An iterator over the lexical elements of a file
pub struct Lex<'a, T: Read> {
    lexer: &'a Lexer,
    source: DoubleFixedBuffer<T>,
    position: DoubleFixedBufferCursor,
    lookahead: DoubleFixedBufferCursor,
    line: usize,
    column: usize,
    previous_line: usize,
    previous_column: usize,
    lex_error_file: File,
}

#[derive(Debug, Clone)]
pub enum LexingError {
    LexemeTooLong(usize, usize),
    InvalidCharacter(String, usize, usize),
    ErrorToken(String, String, usize, usize),
}

impl From<Token> for LexingError {
    fn from(token: Token) -> Self {
        let lexeme = token.lexeme.replace("\n", "\\n").replace("\r", "\\r");
        LexingError::ErrorToken(token.token_type, lexeme, token.line, token.column)
    }
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::LexemeTooLong(line, col) => write!(
                f,
                "Lexical error: lexeme too long: line {} col {}",
                line, col
            ),
            LexingError::InvalidCharacter(lexeme, line, col) => write!(
                f,
                "Lexical error: Invalid character \"{}\": line {} col {}",
                lexeme, line, col
            ),
            LexingError::ErrorToken(token_type, lexeme, line, col) => write!(
                f,
                "Lexical error: {} \"{}\": line {} col {}",
                token_type, lexeme, line, col
            ),
        }
    }
}

const NEWLINE: u8 = b'\n';

impl Lexer {
    pub fn new(rules: Vec<LexicalRule>, keywords: Vec<String>) -> Self {
        assert!(rules.len() != 0, "Lexer must define one or more tokens");
        Lexer { rules, keywords }
    }

    pub fn lex(&self, source_path: &str, lex_error_path: &str) -> Lex<File> {
        let source = match File::open(source_path) {
            Ok(file) => file,
            Err(err) => {
                error!("Failed to open file \"{}\": {}", source_path, err);
                panic!();
            }
        };

        let lex_error_file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(lex_error_path)
        {
            Ok(file) => file,
            Err(err) => {
                warn!("Unable to open file \"{}\": {}", lex_error_path, err);
                warn!("Lexing errors will be unreported");
                panic!();
            }
        };
        Lex::new(self, source, lex_error_file)
    }
}

impl<'a, T: Read> Iterator for Lex<'a, T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_token = self.next_token();
        if let None = next_token {
            return None;
        }

        loop {
            match next_token.clone() {
                Some(Ok(token)) => {
                    if token.error_token || token.token_type == "blockcmt" || token.token_type == "inlinecmt" {
                        match self
                            .lex_error_file
                            .write_all(format!("{}\n", LexingError::from(token.clone())).as_bytes())
                        {
                            Err(err) => {
                                warn!("Failed to write to lexical error file: {}", err);
                            }
                            _ => next_token = self.next_token(),
                        }
                    } else {
                        return Some(token);
                    }
                }
                Some(Err(err)) => {
                    match self
                        .lex_error_file
                        .write_all(format!("{}\n", err).as_bytes())
                    {
                        Err(err) => {
                            warn!("Failed to write to lexical error file: {}", err);
                        }
                        _ => next_token = self.next_token(),
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}

impl<'a, T: Read> Lex<'a, T> {
    fn new(lexer: &'a Lexer, source: T, lex_error_file: File) -> Self {
        Lex {
            lexer,
            source: DoubleFixedBuffer::new(source),
            position: DoubleFixedBufferCursor::new(),
            lookahead: DoubleFixedBufferCursor::new(),
            line: 1,
            column: 1,
            previous_line: 1,
            previous_column: 1,
            lex_error_file,
        }
    }

    /// Move through arbitrary whitespace and produce a token or an error
    /// If the end of input is reached, None is returned
    fn next_token(&mut self) -> Option<Result<Token, LexingError>> {
        if self.position == self.source.end_of_input() {
            return None;
        }

        self.consume_whitespace();
        if self.position == self.source.end_of_input() {
            return None;
        }

        let rule_states = self.advance_rules();
        let best = match self.select_best(rule_states) {
            Ok(rule_state) => rule_state,
            Err(err) => return Some(Err(err)),
        };

        let token = Token::new(
            &self.lexer.keywords,
            &best.token_name,
            best.is_error_token,
            &self.source,
            &self.position,
            best.latest_match,
            self.previous_line,
            self.previous_column,
        );

        self.previous_line = self.line;
        self.previous_column = self.column;
        self.position = self.lookahead;

        Some(Ok(token))
    }

    /// Move the position cursor to the first non-whitespace character
    /// following its initial position
    fn consume_whitespace(&mut self) {
        lazy_static! {
            // Failure to construct this regex is not a logged error, it's a malformed program
            static ref WHITESPACE_RE: regex_automata::DenseDFA<std::vec::Vec<usize>, usize> = regex_automata::dense::Builder::new()
                .anchored(true)
                .longest_match(true)
                .build(r"\s+")
                .expect("Failed to build whitespace consuming regex");
        }

        let start_state = WHITESPACE_RE.start_state();
        let mut state = WHITESPACE_RE.next_state(start_state, self.source[self.position]);
        while self.position != self.source.end_of_input() && !WHITESPACE_RE.is_dead_state(state) {
            self.advance_character_position(self.source[self.position]);
            self.position.advance(&mut self.source);
            state = WHITESPACE_RE.next_state(state, self.source[self.position]);
        }

        self.previous_line = self.line;
        self.previous_column = self.column;
    }

    /// March a lookahead cursor starting at position, feedings the rule
    /// DFAs until the end of input or until all DFAs are in a dead state
    fn advance_rules(&mut self) -> Vec<FusedRuleState> {
        let mut all_dfas_dead = false;
        let mut bytes_consumed = 0;
        let mut rule_states: Vec<(&LexicalRule, RuleState)> = self
            .lexer
            .rules
            .iter()
            .map(|rule| (rule, RuleState::new(&rule)))
            .collect();
        self.lookahead = self.position;
        while self.lookahead != self.source.end_of_input()
            && bytes_consumed < BUFFER_SIZE
            && !all_dfas_dead
        {
            trace!("Processing byte {}", self.source[self.lookahead]);
            all_dfas_dead = true;
            for (rule, state) in &mut rule_states {
                state.advance(rule, self.source[self.lookahead]);
                all_dfas_dead &= state.is_dead(rule);
            }

            if !all_dfas_dead {
                self.advance_character_position(self.source[self.lookahead]);
                self.lookahead.advance(&mut self.source);
                bytes_consumed += 1;
            }
        }
        trace!(
            "All DFAs dead on start {}, lookahead {}",
            self.position,
            self.lookahead
        );
        rule_states
            .iter()
            .map(|rule_state| FusedRuleState::from(*rule_state))
            .collect()
    }

    /// Return the best candidate from a list of rule states
    fn select_best<'b>(
        &mut self,
        rule_states: Vec<FusedRuleState>,
    ) -> Result<FusedRuleState, LexingError> {
        if rule_states.iter().all(|rs| rs.latest_match == 0) {
            let mut bytes = Vec::new();
            trace!(
                "Consuming invalid character byte {} at {}",
                self.source[self.position],
                self.position
            );
            bytes.push(self.source[self.position]);
            self.advance_character_position(self.source[self.position]);
            self.position.advance(&mut self.source);
            while !is_start_of_codepoint(self.source[self.position]) {
                trace!(
                    "Consuming invalid character byte {} at {}",
                    self.source[self.position],
                    self.position
                );
                bytes.push(self.source[self.position]);
                self.advance_character_position(self.source[self.position]);
                self.position.advance(&mut self.source);
            }
            let invalid_character = match std::str::from_utf8(&bytes) {
                Ok(slice) => slice.to_string(),
                Err(err) => {
                    warn!("Invalid UTF-8 byte sequence {:?}: {}", bytes, err);
                    "".to_string()
                }
            };
            return Err(LexingError::InvalidCharacter(
                invalid_character,
                self.previous_line,
                self.previous_column,
            ));
        } else if rule_states.iter().any(|rs| rs.latest_match == BUFFER_SIZE) {
            return Err(LexingError::LexemeTooLong(self.line, self.column));
        }

        let mut furthest_match = rule_states.first().unwrap();
        for state in &rule_states {
            if state.latest_match > furthest_match.latest_match {
                furthest_match = state;
            }
        }
        Ok(furthest_match.clone())
    }

    /// Advance the line and column index based on the content of the utf-8 byte
    fn advance_character_position(&mut self, byte: u8) {
        if is_start_of_codepoint(byte) {
            if byte == NEWLINE {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}
