use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    #[arg(help = "Input file(s)",value_name = "FILE",default_values_t = vec!["-".to_string()])]
    files: Vec<String>,

    #[arg(short, long, help = "Show line count")]
    lines: bool,

    #[arg(short, long, help = "Show word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Show byte count", conflicts_with = "chars")]
    bytes: bool,

    #[arg(short = 'm', long, help = "Show character count")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run(mut config: Config) -> MyResult<()> {
    set_defaults(&mut config);

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let file_info = count(file)?;
                total_lines += file_info.num_lines;
                total_words += file_info.num_words;
                total_chars += file_info.num_chars;
                total_bytes += file_info.num_bytes;

                println!(
                    "{}{}{}{}{}",
                    format_field(file_info.num_lines, config.lines),
                    format_field(file_info.num_words, config.words),
                    format_field(file_info.num_chars, config.chars),
                    format_field(file_info.num_bytes, config.bytes),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", filename)
                    }
                );
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_chars, config.chars),
            format_field(total_bytes, config.bytes),
        );
    }

    Ok(())
}

fn set_defaults(config: &mut Config) {
    if [config.lines, config.words, config.bytes, config.chars]
        .iter()
        .all(|v| v == &false)
    {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += bytes;
        num_chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
