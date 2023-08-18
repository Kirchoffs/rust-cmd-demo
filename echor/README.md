# Notes

## Dependencies
```
>> cargo add clap -F derive
>> cargo add --dev assert_cmd predicates
```

## Run
```
>> cargo run -- --help
>> cargo run -- Tom --omit-newline
>> cargo run -- Tom -n
```

```
>> cargo build
>> ./target/debug/echor Ben -n
```

## Test
```
>> bash mk-outs.sh
>> cargo test
```