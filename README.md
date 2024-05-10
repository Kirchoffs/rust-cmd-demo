# rust-cmd-demo
Reference: __Command-Line Rust__ by Ken Youens-Clark

## Rust Notes
### Rust Code Linter - Clippy
```
>> cargo clippy
```

## Other Notes
### New Line
CRLF (0xD 0xA) or LF (0xA)

### Command Line
#### Check if a directory exists, and create it if it doesn't
```
[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

[[ ! -d $OUT_DIR ]] && mkdir -p $OUT_DIR

[[ ! -d ${OUT_DIR} ]] && mkdir -p ${OUT_DIR}
```

#### Check the exit status
```
>> echo $?
```
