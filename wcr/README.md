# Notes

## Get Started
```
>> rustc --version
>> rustup update stable
>> rustup default stable
```

```
>> rustup update nightly
>> rustup default nightly
```

```
>> cargo new wcr
```

```
>> cargo add clap -F derive

>> cargo add --dev assert_cmd predicates rand
```

```
>> cargo run -- --help
```

## Test
```
>> chmod 755 mk-outs.sh
>> ./mk-outs.sh
```

```
>> cargo run -- tests/inputs/fox.txt
```

## Description
Program wcr will print the number of lines, words, and characters / bytes in a file. If no file is specified, it will read from standard input.

You can also specify the option -l, -w, or -c / -m to print only the number of lines, words, or characters / bytes, respectively.

## Rust Knowledge
### Conditional Compilation
`#[cfg(test)]` enables conditional compilation, so this module will be compiled only when running tests.

### BufRead
`BufRead::lines` will remove the line endings, while `BufRead::read_line` will keep them.

### Format
- {:>8} means right-aligned with width 8.
- {:<8} means left-aligned with width 8.
- {:^8} means centered with width 8.

## General Knowledge
### Common Permission Combinations
- __chmod 777__: This command grants read, write, and execute permissions to the file owner, group members, and other users. This allows any user to perform any operation on the file.

- __chmod 755__: The file owner has read, write, and execute permissions, while group members and other users have read and execute permissions. This is often used for executable files, allowing the file owner enough permissions to run it while restricting others to execution only.

- __chmod 644__: The file owner has read and write permissions, while group members and other users only have read permissions. This is typically used for regular files, allowing the file owner to edit the file while others can only read it.

### New Line
On Windows, newlines are two bytes (\r\n), while Linux newlines are one byte (\n).
