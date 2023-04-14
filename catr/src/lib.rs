use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Benjamin Billian <rust@benni.keksdose.email>")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines")
                .help("Number lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines")
                .help("Number nonblank lines"),
        )
        .get_matches();
    Ok(Config {
        files: matches.get_many("file").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                let mut line = String::new();
                let mut line_number = 0;
                while let Ok(bytes_read) = file.read_line(&mut line) {
                    if bytes_read == 0 {
                        break;
                    }
                    if config.number_lines || (config.number_nonblank_lines && line != "\n") {
                        line_number += 1;
                        print!("     {}\t{}", line_number, line);
                    } else {
                        print!("{}", line);
                    }
                    line.clear();
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
