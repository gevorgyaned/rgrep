use clap::Parser;
use colored::Colorize;
use std::{ffi::OsString, fmt, io, path::PathBuf};

use crate::wildcard_match::log_occurances;

mod wildcard_match;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    wildcard: String,
    filename: Vec<String>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.wildcard, self.filename)
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file_names = args.filename;

    for file_name in file_names {
        match log_occurances(&PathBuf::from(OsString::from(&file_name)), &args.wildcard) {
            Ok(_) => (),
            Err(err) => println!("{}: \n{}", file_name.red(), err),
        }
    }

    Ok(())
}
