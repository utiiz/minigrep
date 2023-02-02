//! # Minigrep
//!
//! `minigrep` is a CLI tool to search a string in a file
use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query     = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

/// Search the query string inside the contents and return a list of string
///
///# Examples
///
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// Safe, fast, productive.
/// Pick three.
/// Duct one.";
///
/// assert_eq!(minigrep::search(query, contents), vec!["Safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search the case insensitive query string inside the contents and return a list of string
///
///# Examples
///
/// ```
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// Safe, fast, productive.
/// Pick three.
/// Rust all the way.";
///
/// assert_eq!(minigrep::search_case_insensitive(query, contents), vec!["Rust:", "Rust all the way."]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
Safe, fast, productive.
Pick three.
Duct one.";

        assert_eq!(search(query, contents), vec!["Safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
Safe, fast, productive.
Pick three.
Rust all the way.";

        assert_eq!(search_case_insensitive(query, contents), vec!["Rust:", "Rust all the way."]);
    }
}
