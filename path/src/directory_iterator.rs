use std::str::FromStr;

// for file in directory("foo")
// for file in directory_entries("foo")
// for file in directory_contents("foo")
// for file in directory("foo").filter(|entry| is_file(entry)).filter(|file| extension(file) == ".src")
// for file in directory("foo").files().filter(|file| extension(file) == ".src")
// for file in directory("foo").filter(|entry| is_file(entry) && extension(entry) == ".src") {

use log::error;
use std::fs::ReadDir;

pub struct Directory {
    iter: ReadDir,
}

pub fn directory(path: &str) -> Directory {
    Directory::new(path)
}

/// A directory iterator created via directory(...)
/// Panics if a non-directory path is supplied
/// Provides error-tolerance during iteration, meaning errors will be ignored and
/// iteration will skip files with insufficient permissions
impl Directory {
    fn new(path: &str) -> Self {
        let iter = match std::fs::read_dir(path) {
            Ok(iter) => iter,
            Err(err) => {
                error!("Failed to open directory \"{}\": {}", path, err);
                panic!();
            }
        };

        Directory { iter }
    }
}

impl Iterator for Directory {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok(entry)) = self.iter.next() {
            if let Some(path) = entry.path().to_str() {
                if let Ok(path) = String::from_str(path) {
                    return Some(path);
                }
            }
            return None;
        } else {
            return None;
        }
    }
}
