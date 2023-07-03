# Notes

## During the dev
```
>> cargo add clap -F derive
>> cargo add --dev assert_cmd predicates
```

## Run the command
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