# Notes
### Code
#### Naive Code
```
fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file)
        .map_err(|e| anyhow!("{}: {e}", args.in_file))?;

    let mut lines: Vec<String> = Vec::new();
    let mut cnts: Vec<i32> = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = file.read_line(&mut line)?; // read_line() includes '\n'
        if bytes == 0 {
            break;
        }

        if lines.is_empty() || lines[lines.len() - 1] != line {
            lines.push(line);
            if args.count {
                cnts.push(1);
            }
        } else {
            if args.count {
                let cnts_len = cnts.len();
                cnts[cnts_len - 1] += 1;
            }
        }
    }

    if args.out_file.is_some() {
        let out_file = File::create(args.out_file.unwrap())?;
        let mut writer = BufWriter::new(out_file);
        for i in 0 .. lines.len() {
            if args.count {
                write!(writer, "   {} {}", cnts[i], lines[i]);
            } else {
                write!(writer, "{}", lines[i]);
            }
        }
    } else {
        for i in 0 .. lines.len() {
            if args.count {
                print!("   {} {}", cnts[i], lines[i]);
            } else {
                print!("{}", lines[i]);
            }
        }
    }

    Ok(())
}
```

#### Reference (and definitely better) Code
```
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
```

### Knowledge
#### Write trait
When a value implements the `Write` trait, it can be used with the `write!` and `writeln!` macro. The `Write` trait has a single method `write(&mut self, buf: &[u8]) -> Result<usize>`. The `write!` macro writes formatted data into a value that implements the `Write` trait.

#### Tempfile
```
use tempfile::NamedTempFile;

let out_file = NamedTempFile::new()?;
```

#### Closure
The difference between a closure and a function is that a closure can capture variables from the environment in which it is defined. The syntax for a closure is `|arg1, arg2, ...| { body }`.

#### How does `assert_cmd::Command` work?
```
let output = Command::cargo_bin("uniqr")
    .unwrap()
    .arg("uniq")
    .arg("tests/inputs/uniq.txt")
    .unwrap()
    .stdout(Stdio::piped())
    .output()
    .unwrap();
```

It looks for the executable in the directory target/debug, which is where Cargo places compiled binaries by default when you run `cargo build` or `cargo run`.

#### What is `'static` lifetime?
The 'static lifetime is the lifetime of the entire program. All string literals have the 'static lifetime.

#### What does `{count:>4}` mean in the string formatting?
It means that the count will be right-aligned and take up 4 spaces. If the count is less than 4 digits, the remaining spaces will be filled with spaces.
