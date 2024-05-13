use clap::Parser;
use colored::Colorize;
use std::{fmt, fs::File, io};

use crate::wildcard_match::search_in_file;

mod wildcard_match;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    pattern: String,
    filename: Vec<String>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.pattern, self.filename)
    }
}

fn log_occurances(file_name: &str, pattern: &str) -> Result<(), &'static str> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Err("unable to open file"),
    };

    let matches = search_in_file(file, pattern);

    if matches.is_empty() {
        return Err("no occurances is found");
    }

    println!("{}: ", file_name.green());
    matches
        .iter()
        .for_each(|match_entry| println!("{}", match_entry));

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file_names = args.filename;

    for file_name in file_names {
        match log_occurances(file_name.as_str(), &args.pattern) {
            Ok(_) => (),
            Err(err) => eprintln!("{}: \n{}", file_name.red(), err),
        }
    }

    Ok(())
}
