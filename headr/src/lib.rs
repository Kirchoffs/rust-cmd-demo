use std::{error::Error, io::{BufRead, BufReader, Read, stdin}, fs::File};
use clap::Parser;

type WrapResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[derive(Debug)]
pub struct Config {
    #[arg(value_name = "FILE NAMES", default_values_t = ["-".to_string()])]
    files: Vec<String>,

    #[arg(short = 'n', long, default_value_t = 10)]
    lines: usize,

    #[arg(short = 'c', long)]
    bytes: Option<usize>,
}

pub fn get_args() -> WrapResult<Config> {
    let config = Config::parse();

    let files = config.files;
    let lines = config.lines;
    let bytes = config.bytes;

    if lines == 0 {
        return Err("lines must be positive".into());
    }

    if bytes == Some(0) {
        return Err("bytes must be positive".into());
    }

    Ok(Config {
        files,
        lines,
        bytes,
    })  
}

pub fn run(config: Config) -> WrapResult<()> {
    let multiple_file_flag = config.files.len() > 1;

    for (idx, file) in config.files.iter().enumerate() {
        let mut last_file_flag = false;
        if idx == config.files.len() - 1 {
            last_file_flag = true;
        }
        match open(file) {
            Ok(reader) => {
                if let Err(err) = head(reader, file, config.lines, config.bytes, multiple_file_flag, last_file_flag) {
                    eprintln!("Error: {err}");
                }
            },
            Err(err) => {
                eprintln!("Failed to open {file}: {err}");
                eprintln!();
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> WrapResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn head(reader: Box<dyn BufRead>, file: &str, lines: usize, bytes: Option<usize>, multiple_file_flag: bool, last_file_flag: bool) -> WrapResult<()> {
    if multiple_file_flag {
        println!("==> {file} <==");
    }

    if let Some(num_bytes) = bytes {
        head_bytes(reader, num_bytes)?;
    } else {
        head_lines(reader, lines)?;
    }

    if !last_file_flag {
        println!();
    }
    
    Ok(())
}

fn head_bytes(reader: Box<dyn BufRead>, num_bytes: usize) -> WrapResult<()> {
    let mut buffer = vec![0; num_bytes];
    let mut handle = reader.take(num_bytes as u64);
    let num_bytes_read = handle.read(&mut buffer)?;
    let content = String::from_utf8_lossy(&buffer[..num_bytes_read]);
    print!("{content}");
    Ok(())
}

fn head_lines(mut reader: Box<dyn BufRead>, num_lines: usize) -> WrapResult<()> {
    let mut line = String::new();

    for _ in  0 .. num_lines {
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{line}");
        line.clear();
    }

    Ok(())
}

fn parse_positive_int(val: &str) -> WrapResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => {
            Ok(n)
        },
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("6");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 6);

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0");

    let res = parse_positive_int("-1");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "-1");

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo");
}