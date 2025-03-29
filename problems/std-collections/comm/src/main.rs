#![forbid(unsafe_code)]

use clap::Parser;
use std::path::PathBuf;
use std::{fs::File, io::BufRead, io::BufReader};
use std::collections::HashSet;

#[derive(Parser)]
#[command(about = "Print common lines in two files", long_about = None)]
struct Args {
    /// First file
    file1: PathBuf,
    /// Second file
    file2: PathBuf,
}

fn file_to_set(file_path: &PathBuf) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        set.insert(line.unwrap());
    }
    return set;
}

fn main() {
    let args = Args::parse();
    let file_path1 = args.file1;
    let file_path2 = args.file2;
    let set1 = file_to_set(&file_path1);
    let mut set_out = HashSet::<String>::new();
    let file2 = File::open(file_path2).unwrap();
    let reader = BufReader::new(file2);
    for line in reader.lines() {
        let line = line.unwrap();
        if set1.contains(&line) && !set_out.contains(&line) {
            println!("{}", &line);
            set_out.insert(line);
        }
    }
}
