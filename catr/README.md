# Notes

## Dependencies
```
>> cargo add clap -F derive
>> cargo add --dev assert_cmd predicates rand
```

## Run & Test
```
>> cargo run -- tests/inputs/*.txt
>> cargo run -- tests/inputs/*.txt -n
```

```
>> touch cannot-touch-this && chmod 000 cannot-touch-this
>> cargo run -- blargh cant-touch-this tests/inputs/a.txt
```

```
>> cat tests/inputs/c.txt | cargo run
>> cargo run -- < tests/inputs/c.txt
>> cargo run < tests/inputs/c.txt
>> cargo run -- -n < tests/inputs/c.txt
```

The bash shell will expand the file glob *.txt into all files that end with the extension .txt.

## Rust Details
### BufRead
```
fn open(filename: &str) -> WrapResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
```

If File::open is successful, the result will be a filehandle, which is a mechanism for reading the contents of a file. Both a filehandle and std::io::stdin implement the BufRead trait, which means the values will, for instance, respond to the BufRead::lines function to produce lines of text. 