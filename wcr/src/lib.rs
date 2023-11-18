use std::{error::Error, io::BufRead};

use clap::Parser;
use std::io::{stdin, BufReader};
use std::fs::File;

type WrapResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[derive(Debug)]
pub struct Config {
    #[arg(value_name = "FILE NAMES", default_values_t = ["-".to_string()])]
    files: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    lines: bool,

    #[arg(short, long, default_value_t = false)]
    words: bool,

    #[arg(short = 'c', long, default_value_t = false)]
    bytes: bool,

    #[arg(short = 'm', long, default_value_t = false)]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> WrapResult<Config> {
    let config = Config::parse();
    let (mut lines, mut words, mut bytes, mut chars) = (
        config.lines,
        config.words,
        config.bytes,
        config.chars,
    );
    
    // Default to all if none are specified
    if [lines, words, bytes, chars].iter().all(|&x| !x) {
        lines = true;
        words = true;
        bytes = true;
    }

    if bytes && chars {
        return Err("The argument '--bytes' cannot be used with '--chars'".into());
    }

    Ok(Config {
        lines,
        words,
        bytes,
        chars,
        ..config
    })
}

fn count(mut file: impl BufRead) -> WrapResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
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

pub fn run(config: Config) -> WrapResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for file in &config.files {
        match open(file) {
            Err(err) => eprintln!("{}: {}", file, err),
            Ok(file_reader) => {
                if let Ok(file_info) = count(file_reader) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(file_info.num_lines, config.lines),
                        format_field(file_info.num_words, config.words),
                        format_field(file_info.num_bytes, config.bytes),
                        format_field(file_info.num_chars, config.chars),
                        if file == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", file)
                        }
                    );

                    total_lines += file_info.num_lines;
                    total_words += file_info.num_words;
                    total_bytes += file_info.num_bytes;
                    total_chars += file_info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }

    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

fn open(file: &str) -> WrapResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
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
