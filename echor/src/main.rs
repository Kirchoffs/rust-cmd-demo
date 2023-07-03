use clap::Parser;

#[derive(Parser)]
#[command(name = "echor")]
#[command(author = "Ben")]
#[command(version = "1.0.0")]
#[command(about = "Rust echo", long_about = None)]
struct Cli {
    #[arg(required = true, value_name = "TEXT SEQUENCE")]
    text: Vec<String>,
    
    #[arg(short = 'n', long, default_value_t = false)]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cli.text;
    let omit_newline = cli.omit_newline;
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}