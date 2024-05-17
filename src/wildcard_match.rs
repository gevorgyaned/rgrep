use colored::*;
use core::fmt;
use std::{
    fs::{self, File},
    io::{self, BufRead}, path::PathBuf,
};

pub struct MatchedLine {
    pub line: Vec<String>,
    pub line_number: usize,
    pub word_number: usize,
}

impl MatchedLine {
    pub fn new(line: Vec<String>, line_number: usize, word_number: usize) -> MatchedLine {
        MatchedLine { line, line_number, word_number }
    }
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(
            f,
            "{}:",
            self.line_number.to_string().blue(),
        );

        for (idx, word) in self.line.iter().enumerate() {
            if idx == self.word_number {
                let _ = write!(f, "{}", word.red());
            } else {
                let _ = write!(f, "{}", word);
            }
        }

        Ok(())
    }
}

pub fn log_occurances(file_name: &PathBuf, wildcard: &str) -> Result<(), &'static str> {
    if file_name.is_dir() {
        println!("Directory {}: ", file_name.display().to_string().green());

        let file_names = match fs::read_dir(file_name) {
            Ok(f) => f,
            Err(_) => return Err("unable to read directory"),
        };

        for res_file_name in file_names {
            let file_name = match res_file_name {
                Ok(e) => e,
                Err(_) => return Err("unable to resolve"),
            };

            let _ = log_occurances(&file_name.path(), wildcard);
        }

        return Ok(());
    }

    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Err("unable to open file"),
    };

    let matches = search_in_file(file, wildcard);
    if matches.is_empty() {
        return Err("no occurances is found");
    }

    println!("{}: ", file_name.display().to_string().green());
    matches
        .iter()
        .for_each(|match_entry| println!("{}", match_entry));

    Ok(())
}

fn parse(input: &str) -> Option<Vec<String>> {
    if input.is_empty() {
        return None;
    }

    Some(input.split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect())
}

pub fn is_match(str: &str, pattern: &str) -> bool {
    let (str_len, pat_len) = (str.len(), pattern.len());

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut m: usize = 0;
    let mut start_idx: i32 = -1;

    while i < str_len {
        if j < pat_len
            && (pattern.chars().nth(j) == Some('?') || pattern.chars().nth(j) == str.chars().nth(i))
        {
            j += 1;
            i += 1;
        } else if j < pat_len && pattern.chars().nth(j) == Some('*') {
            start_idx = j as i32;
            m = i as usize;
            j += 1;
        } else if start_idx != -1 {
            j = start_idx as usize + 1;
            m += 1;
            i = m;
        } else {
            return false;
        }
    }

    while j < pat_len && pattern.chars().nth(j) == Some('*') {
        j += 1;
    }

    j == pat_len
}

pub fn search_in_file(file: File, pattern: &str) -> Vec<MatchedLine> {
    let mut result = Vec::new();
    let lines = io::BufReader::new(file).lines();

    for (line_idx, string) in lines.enumerate() {
        if let Ok(line) = string {
            let words = match parse(&line) {
                Some(w) => w,
                None => continue,
            };

            for (word_idx, word) in words.iter().enumerate() {
                if is_match(word.trim(), pattern) {
                    result.push(MatchedLine::new(
                        words.clone(), 
                        line_idx, 
                        word_idx));
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_match_ext() {
        assert!(is_match("annn.py", "*.py") == true);
    }
}
