use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(f);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("Line {}: \"{}\"", index, line);
        }
    }
}
