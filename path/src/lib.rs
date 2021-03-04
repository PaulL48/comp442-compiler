//! Provides easier access to some of the functionality of std::fs and std::path that uses Path, PathBuf, ReadDir and DirEntry.
//! The functionality provided by Path and PathBuf suffer from conversion to and from OsString/OsStr
//! The functionality provided by ReadDir and DirEntry have very granular error checking but this creates
//! excess boilerplate during simple operations such as iteration of accessible files in a directory

mod directory_iterator;

pub use crate::directory_iterator::directory;
use std::path::Path;
use log::error;
use std::str::FromStr;

pub fn extension(path: &str) -> Option<&str> {
    if let Some(extension) = Path::new(path).extension() {
        if let Some(extension) = extension.to_str() {
            return Some(extension);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn file_name(path: &str) -> Option<&str> {
    if let Some(file_name) = Path::new(path).file_name() {
        if let Some(file_name) = file_name.to_str() {
            return Some(file_name);
        }
    }
    return None
}

pub fn canonicalize(path: &str) -> std::io::Result<String> {
    let canonical = Path::new(path).canonicalize()?;
    if let Some(canonical) = canonical.to_str() {
        if let Ok(canonical) = String::from_str(canonical) {
            return Ok(canonical);
        } else {
            error!("Failed to convert to String {:?}", canonical);
            panic!();
        }
    } else {
        error!("Failed to convert to UTF-8 {:?}", canonical);
        panic!();
    }
}
