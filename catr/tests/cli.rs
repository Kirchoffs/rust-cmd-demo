use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const A: &str = "tests/inputs/a.txt";
const B: &str = "tests/inputs/b.txt";
const C: &str = "tests/inputs/c.txt";

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);

    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn a() -> TestResult {
    run(&[A], "tests/expected/a.out.txt")
}


#[test]
fn a_n() -> TestResult {
    run(&[A, "-n"], "tests/expected/a.out.n.txt")
}

#[test]
fn a_b() -> TestResult {
    run(&[A, "-b"], "tests/expected/a.out.b.txt")
}

#[test]
fn b() -> TestResult {
    run(&[B], "tests/expected/b.out.txt")
}

#[test]
fn b_n() -> TestResult {
    run(&[B, "-n"], "tests/expected/b.out.n.txt")
}

#[test]
fn b_b() -> TestResult {
    run(&[B, "-b"], "tests/expected/b.out.b.txt")
}

#[test]
fn c() -> TestResult {
    run(&[C], "tests/expected/c.out.txt")
}

#[test]
fn c_n() -> TestResult {
    run(&[C, "-n"], "tests/expected/c.out.n.txt")
}

#[test]
fn c_b() -> TestResult {
    run(&[C, "-b"], "tests/expected/c.out.b.txt")
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.out.txt")
}

#[test]
fn empty_n() -> TestResult {
    run(&[EMPTY, "-n"], "tests/expected/empty.out.n.txt")
}

#[test]
fn empty_b() -> TestResult {
    run(&[EMPTY, "-b"], "tests/expected/empty.out.b.txt")
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn stdin_c() -> TestResult {
    run_stdin(C, &["-"], "tests/expected/c.out.txt")
}

#[test]
fn stdin_c_n() -> TestResult {
    run_stdin(C, &["-", "-n"], "tests/expected/c.out.n.txt")
}

#[test]
fn stdin_c_b() -> TestResult {
    run_stdin(C, &["-", "-b"], "tests/expected/c.out.b.txt")
}