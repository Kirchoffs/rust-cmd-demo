use clap::Parser;
use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{self, BufReader, BufRead, Write}
};

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file)
        .map_err(|e| anyhow!("{}: {}", args.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> Result<()> {
        if count > 0 {
            if args.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    print(count, &previous)?;

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,
    #[arg(short, long)]
    count: bool,
}
