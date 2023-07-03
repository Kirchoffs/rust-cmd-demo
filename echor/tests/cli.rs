use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    match Command::cargo_bin("echor") {
        Err(e) => {
            println!("Failed to execute process: {}", e);
            panic!("Failed to execute process: {}", e)
        },
        Ok(mut cmd) => {
            println!("Executing: {:?}", cmd);
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("The following required arguments were not provided:"))
                .stderr(predicate::str::contains("TEXT"));
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello4() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}