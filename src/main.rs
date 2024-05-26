use clap::Parser;
use wildcard_match::Matcher;
use std::{fmt, io, path::PathBuf};

use crate::threadpool::*;

mod threadpool;
mod wildcard_match;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    wildcard: String,
    filename: Vec<PathBuf>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.wildcard, self.filename)
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file_names = args.filename;
    
    let matcher = Matcher::build(file_names, args.wildcard);

    match matcher.execute() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    };

    Ok(())
}
