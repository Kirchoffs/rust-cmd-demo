# Notes

## Dependencies
```
>> cargo add clap -F derive
>> cargo add --dev assert_cmd predicates rand
```

## Test & Run
```
>> cargo clippy
```

```
>> cargo run -- tests/inputs/*
>> cargo run -- tests/inputs/* -n 2
```

## Code Details
### Match with Guard
```
match val.parse::<usize>() {
    Ok(n) if n > 0 => {
        Ok(n)
    },
    _ => Err(val.into()),
}
```

```
match val.parse::<usize>() {
    Ok(n) => {
        if n > 0 {
            Ok(n)
        } else {
            Err(From::from(n))
        }
    },
    _ => {
        Err(From::from(val)),
    }
}
```

### From & Into
#### Code in the project

```
fn parse_positive_int(val: &str) -> WrapResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}
```

```
fn parse_positive_int(val: &str) -> WrapResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}
```

```
fn parse_positive_int(val: &str) -> WrapResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(Into::into(val)),
    }
}
```

#### Example code for Into trait
```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Into<u64> for Rectangle {
    fn into(self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    let area: u64 = rectangle.into();
    println!("Area: {}", area);
}
```

### UTF-8
`String::from_utf8_lossy` is a method used to convert a slice of bytes (&[u8]) into a String, even if the bytes are not valid UTF-8. If the input byte slice contains valid UTF-8 data, it will be converted directly into a String. However, if the byte slice contains invalid UTF-8 sequences, the from_utf8_lossy method replaces those invalid sequences with the Unicode replacement character (U+FFFD), which is used to represent "replacement" or "unknown" characters.

### Preserving Line Endings While Reading a File
Not Preserving Line Endings:
```
match open(file_path) {
    Ok(reader) => {
        for line in reader.lines() {
            print!("{}", line.unwrap());
        }
    },
    Err(error) => {
        println!("Error: {}", error);
    }
}
```

Preserving Line Endings:
```
match open(file_path) {
    Ok(mut reader) => {
        let mut line = String::new();
        loop {
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            print!("{}", line);
        }
    },
    Err(error) => {
        println!("Error: {}", error);
    }
}
```

### Unknown Character
The unknown character produced by String::from_utf8_lossy (b'\xef\xbf\xbd') is not exactly the same output produced by the BSD head (b'\xc3').

### Pointer
#### Box Pointer
