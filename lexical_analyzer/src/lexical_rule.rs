use path::canonicalize;
use regex_automata;
use regex_automata::DFA;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const INPUT_FILE_SEPARATOR: char = '@';

#[derive(Debug)]
pub enum LexicalRuleParseError {
    IoError(std::io::Error),
    MalformedLine(String),
    InvalidFieldValue(String),
}

impl fmt::Display for LexicalRuleParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            LexicalRuleParseError::IoError(err) => format!("{}", err),
            LexicalRuleParseError::MalformedLine(message) => message.to_string(),
            LexicalRuleParseError::InvalidFieldValue(message) => message.to_string(),
        };
        write!(f, "{}", message)
    }
}

impl From<std::io::Error> for LexicalRuleParseError {
    fn from(error: std::io::Error) -> Self {
        LexicalRuleParseError::IoError(error)
    }
}

/// String values of a lexical rule
///
/// First stage of parsing a lexical rule from a file or string
#[derive(Debug)]
struct RawLexicalRule {
    regex: String,
    backtrack: String,
    error_token: String,
    output_token_name: String,
}

impl FromStr for RawLexicalRule {
    type Err = LexicalRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split(INPUT_FILE_SEPARATOR).map(|s| s.trim()).collect();
        if components.len() != 4 {
            return Err(LexicalRuleParseError::MalformedLine(format!(
                "Line is malformed \"{}\", should contain four fields separated by {}",
                s, INPUT_FILE_SEPARATOR
            )));
        }

        Ok(Self {
            regex: components[0].to_string(),
            error_token: components[1].to_string(),
            backtrack: components[2].to_string(),
            output_token_name: components[3].to_string(),
        })
    }
}

impl RawLexicalRule {
    fn from_file(path: &str) -> Result<Vec<Self>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rule_set = Vec::new();

        // To disambiguate error output we get the full path to the file being read
        let full_path = canonicalize(path).unwrap_or(path.to_string());
        info!("Extracting lexical rules from file \"{}\"", full_path);

        for (number, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    error!("Error while reading file \"{}\": {}", full_path, err);
                    panic!();
                }
            };

            if line.len() == 0 || line.starts_with("#") {
                continue;
            }

            match line.parse::<Self>() {
                Ok(val) => rule_set.push(val),
                Err(err) => {
                    warn!("Warning [{} l:{}]: {}", full_path, number + 1, err);
                    continue;
                }
            };
        }

        Ok(rule_set)
    }
}

#[derive(Debug)]
pub struct LexicalRule {
    pub dfa: regex_automata::DenseDFA<std::vec::Vec<usize>, usize>,
    pub is_error_token: bool,
    pub backtrack: bool,
    pub token_name: String,
}

impl LexicalRule {
    fn from_raw(raw_rule: RawLexicalRule) -> Result<Self, LexicalRuleParseError> {
        let dfa = regex_automata::dense::Builder::new()
            .anchored(true)
            .byte_classes(false)
            .case_insensitive(false)
            .ignore_whitespace(false)
            .minimize(true)
            .premultiply(false)
            .dot_matches_new_line(true)
            .build(&raw_rule.regex);

        let dfa = match dfa {
            Ok(dfa) => dfa,
            Err(err) => {
                return Err(LexicalRuleParseError::InvalidFieldValue(format!(
                    "Failed to construct DFA for regex \"{}\": {}",
                    &raw_rule.regex, err
                )))
            }
        };

        let is_err_token = match raw_rule.error_token.parse::<bool>() {
            Err(err) => {
                return Err(LexicalRuleParseError::InvalidFieldValue(format!(
                    "Invalid error token field \"{}\", should be \"true\" or \"false\": {}",
                    raw_rule.error_token, err
                )))
            }
            Ok(val) => val,
        };

        let backtrack = match raw_rule.backtrack.parse::<bool>() {
            Err(err) => {
                return Err(LexicalRuleParseError::InvalidFieldValue(format!(
                    "Invalid backtrack field \"{}\", should be \"true\" or \"false\": {}",
                    raw_rule.backtrack, err
                )))
            }
            Ok(val) => val,
        };

        Ok(Self {
            dfa,
            is_error_token: is_err_token,
            backtrack,
            token_name: raw_rule.output_token_name,
        })
    }

    pub fn from_file(path: &str) -> Result<Vec<Self>, LexicalRuleParseError> {
        let raw_rules = RawLexicalRule::from_file(path)?;
        let mut rules = Vec::new();
        for raw_rule in raw_rules {
            match LexicalRule::from_raw(raw_rule) {
                Ok(rule) => rules.push(rule),
                Err(err) => warn!("{}", err),
            }
        }
        info!(
            "Successfully read {} lexical rules from {}",
            rules.len(),
            path
        );
        Ok(rules)
    }
}

#[derive(Clone)]
pub struct FusedRuleState {
    pub is_error_token: bool,
    pub backtrack: bool,
    pub token_name: String,
    pub latest_match: usize,
}

impl From<(&LexicalRule, RuleState)> for FusedRuleState {
    fn from(r: (&LexicalRule, RuleState)) -> Self {
        FusedRuleState {
            is_error_token: r.0.is_error_token,
            backtrack: r.0.backtrack,
            token_name: r.0.token_name.clone(),
            latest_match: r.1.latest_match,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RuleState {
    state: usize,
    pub latest_match: usize,
}

impl RuleState {
    pub fn new(rule: &LexicalRule) -> Self {
        RuleState {
            state: rule.dfa.start_state(),
            latest_match: 0,
        }
    }

    pub fn advance(&mut self, rule: &LexicalRule, byte: u8) {
        self.state = rule.dfa.next_state(self.state, byte);
        if !self.is_dead(rule) {
            trace!("Rule {:?} generates a match on byte {}", self, byte);
            self.latest_match += 1;
        }
    }

    pub fn is_dead(&self, rule: &LexicalRule) -> bool {
        rule.dfa.is_dead_state(self.state)
    }
}
