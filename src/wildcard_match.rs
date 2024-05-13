use colored::*;
use core::fmt;
use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct MatchedLine {
    pub line: String,
    pub line_number: usize,
}

impl MatchedLine {
    pub fn new(line: String, line_number: usize) -> MatchedLine {
        MatchedLine { line, line_number }
    }
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.line_number.to_string().blue(),
            self.line.cyan()
        )
    }
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

    for (idx, string) in lines.enumerate() {
        if let Ok(line) = string {
            let words = line.split(' ');

            if words.into_iter().any(|word| is_match(word, pattern)) {
                result.push(MatchedLine::new(line, idx + 1));
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
