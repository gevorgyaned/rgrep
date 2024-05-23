use colored::*;
use wildcard::*;
use core::fmt;
use std::{
    fs::{self, File}, io::{self, BufRead}, path::{Path, PathBuf}, sync::{mpsc::{self, Sender}, Arc, Mutex}
};

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
        let _ = write!(f, "{}:", self.line_number.to_string().blue(),);

        for (idx, word) in self.line.iter().enumerate() {
            if idx == self.word_number {
                let _ = write!(f, "{} ", word.red());
            } else {
                let _ = write!(f, "{} ", word);
            }
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
    pub sender: Mutex<mpsc::Sender<(Vec<MatchedLine>, PathBuf)>>,
    pub receiver: mpsc::Receiver<(Vec<MatchedLine>, PathBuf)>
}

impl Matcher {
    pub fn build(filenames: Vec<PathBuf>, wildcard: String) -> Matcher {
        let (sender, receiver) = mpsc::channel();

        Matcher {
            threadpool: ThreadPool::new(8),
            filenames,
            wildcard,
            sender: Mutex::new(sender),
            receiver,
        }
    }

    pub fn log_occurances(&self, file_name: &Path, matches: Vec<MatchedLine>) {
        if matches.is_empty() {
            println!("no occurances is found");
            return;
        }

        println!("{}: ", file_name.display().to_string().green());
        matches
            .iter()
            .for_each(|match_entry| println!("{}", match_entry));
    }

    pub fn execute(&self) -> Result<(), &'static str> {
        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        for filename in &self.filenames {
            if filename.is_dir() {
                let sender = Arc::clone(&sender);
                self.handle_directory(filename.clone(), &sender);
            } else if filename.is_file() {
                let sender = Arc::clone(&sender);
                self.handle_regular_file(filename.clone(), &sender);
            }
        }

        drop(sender);

        for (matched_line, file_name) in receiver {
            self.log_occurances(&file_name, matched_line);
        }
        
        Ok(())
    }

    fn handle_directory(&self, file_name: PathBuf, sender: &Arc<Mutex<Sender<(Vec<MatchedLine>, PathBuf)>>>) {
        let entries = match fs::read_dir(file_name) {
            Ok(e) => e,
            Err(_) => todo!(),
        };

        for dir_entry  in entries {
            if let Ok(dir_entry) = dir_entry {
                let dir_entry = dir_entry.path();

                if dir_entry.is_dir() {
                    self.handle_directory(dir_entry, sender);
                } else if dir_entry.is_file() {
                    self.handle_regular_file(dir_entry, sender);
                }
            }
        }
    }

    fn handle_regular_file(&self, filename: PathBuf, sender: &Arc<Mutex<Sender<(Vec<MatchedLine>, PathBuf)>>>) {
        let sender = Arc::clone(sender);
        let wildcard = self.wildcard.clone();

        self.threadpool.execute(move || {
            let file = File::open(filename.clone()).unwrap();
    
            let matches = search_in_file(file, wildcard.clone().as_str());
    
            sender.lock().unwrap().send((matches, filename)).unwrap();
        });
    }
}
