use colored::*;
use wildcard::*;
use core::fmt;
use std::{
    fs::{self, File}, io::{self, BufRead, BufWriter}, path::{Path, PathBuf}, sync::mpsc::{self, Sender}
};
use std::io::Write;

extern crate wildcard;

use crate::ThreadPool;

pub struct MatchedLine {
    pub line: Vec<String>,
    pub line_number: usize,
    pub word_number: usize,
}

impl MatchedLine {
    pub fn new(line: Vec<String>, line_number: usize, word_number: usize) -> MatchedLine {
        MatchedLine {
            line,
            line_number,
            word_number,
        }
    }
}

impl fmt::Display for MatchedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "{}:", self.line_number.to_string().blue());

        for (idx, word) in self.line.iter().enumerate() {
            write!(f, "{} ", if idx == self.word_number { 
                word.green() 
            } else { 
                word.white() 
            }).unwrap();
        }

        Ok(())
    }
}


pub fn search_in_file(file: File, wildcard: &str) -> Vec<MatchedLine> {
    let mut result = Vec::new();
    let lines = io::BufReader::new(file).lines();

    for (line_idx, string) in lines.enumerate() {
        if let Ok(line) = string {
            let words = match parse(&line) {
                Some(w) => w,
                None => continue,
            };

            for (word_idx, word) in words.iter().enumerate() {
                if is_match(word.trim(), wildcard) {
                    result.push(MatchedLine::new(words.clone(), line_idx, word_idx));
                    break;
                }
            }
        }
    }

    result
}

pub struct Matcher {
    pub threadpool: ThreadPool,
    pub wildcard: String,
    pub filenames: Vec<PathBuf>,
}

impl Matcher {
    pub fn build(filenames: Vec<PathBuf>, wildcard: String) -> Matcher {
        Matcher {
            threadpool: ThreadPool::new(8),
            filenames,
            wildcard,
        }
    }

    pub fn log_occurances(&self, file_name: &Path, matches: Vec<MatchedLine>, buffer: &mut BufWriter<io::StdoutLock<'static>>) {
        if matches.is_empty() {
            writeln!(buffer, "no occurrences found").unwrap();
            return;
        }

        writeln!(buffer, "{}: ", file_name.display().to_string().green()).unwrap();
        for match_entry in &matches {
            writeln!(buffer, "{}", match_entry).unwrap();
        }
    }

    pub fn execute(&self) -> Result<(), &'static str> {
        let (sender, receiver) = mpsc::channel();

        for filename in &self.filenames {
            if filename.is_dir() {
                self.handle_directory(filename.clone(), sender.clone());
            } else if filename.is_file() {
                self.handle_regular_file(filename.clone(), sender.clone());
            }
        }

        drop(sender);

        let stdout = io::stdout().lock();
        let mut buffer = BufWriter::new(stdout);

        for (matched_line, file_name) in receiver {
            self.log_occurances(&file_name, matched_line, &mut buffer);
        }

        buffer.flush().unwrap();
        
        Ok(())
    }

    fn handle_directory(&self, file_name: PathBuf, sender: Sender<(Vec<MatchedLine>, PathBuf)>) {
        let entries = match fs::read_dir(file_name) {
            Ok(e) => e,
            Err(_) => todo!(),
        };

        for dir_entry  in entries.flatten() {
            let dir_entry = dir_entry.path();

            if dir_entry.is_dir() {
                self.handle_directory(dir_entry, sender.clone());
            } else if dir_entry.is_file() {
                self.handle_regular_file(dir_entry, sender.clone());
            }
        }
    }

    fn handle_regular_file(&self, filename: PathBuf, sender: Sender<(Vec<MatchedLine>, PathBuf)>) {
        let wildcard = self.wildcard.clone();

        self.threadpool.execute(move || {
            let file = File::open(filename.clone()).unwrap();
    
            let matches = search_in_file(file, wildcard.clone().as_str());
    
            sender.send((matches, filename)).unwrap();
        });
    }
}
