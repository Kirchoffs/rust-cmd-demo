use std::error::Error;
use std::io::stdin;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser)]
#[derive(Debug)]
#[command(name = "catr")]
#[command(author = "Ben")]
#[command(version = "1.0.0")]
#[command(about = "Rust catr", long_about = None)]
pub struct Config {
    #[arg(value_name = "FILE NAMES", default_values_t = ["-".to_string()])]
    files: Vec<String>,

    #[arg(short = 'n', long, default_value_t = false)]
    number_lines: bool,

    #[arg(short = 'b', long, default_value_t = false)]
    number_nonblank_lines: bool,
}

type WrapResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> WrapResult<Config> {
    let config = Config::parse();
    let files = config.files;
    let number_lines = config.number_lines;
    let number_nonblank_lines = config.number_nonblank_lines;

    if number_lines && number_nonblank_lines {
        return Err("the argument '--number-nonblank' (-b) cannot be used with '--number' (-n)".into());
    }

    Ok(Config {
        files: files,
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    })
}

pub fn run(config: Config) -> WrapResult<()> {
    let mut line_number = 1;
    for filename in config.files {
        match open(&filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(line_content) => {
                            if !config.number_nonblank_lines && !config.number_lines {
                                println!("{}", line_content);
                            } else {
                                if !line_content.is_empty() {
                                    println!("{} {}", line_number, line_content);
                                    line_number += 1;
                                } else {
                                    if config.number_nonblank_lines {
                                        println!();
                                    } else {
                                        println!("{}", line_number);
                                        line_number += 1;
                                    }
                                }
                            }
                        },
                        Err(err) => eprintln!("Failed to read line: {}", err)
                    }
                }
            },
            Err(err) => eprintln!("Failed to open {}: {}", filename, err)
        }
    }

    Ok(())
}

fn open(filename: &str) -> WrapResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
