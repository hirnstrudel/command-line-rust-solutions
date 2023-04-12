use std::error::Error;
use clap::{App, Arg}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .version("0.1.0")
    .author("Benjamin Billian <rust@benni.keksdose.email>")
    .about("Rust cat")
    // What goes here?
    .get_matches();
    Ok(Config {
    files: ...,
    number_lines: ...,
    number_nonblank_lines: ...,
    })
    }

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}
