use std::fs;
use assert_cmd::Command;
use anyhow::Result;
use tempfile::NamedTempFile;

type TestResult = Result<(), Box<dyn std::error::Error>>;

struct Test {
    input: &'static str,
    output: &'static str,
    output_count: &'static str,
}

const PRG: &str = "uniqr";

const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    output: "tests/expected/empty.txt.out",
    output_count: "tests/expected/empty.txt.c.out",
};

const ONE: Test = Test {
    input: "tests/inputs/one.txt",
    output: "tests/expected/one.txt.out",
    output_count: "tests/expected/one.txt.c.out",
};

const TWO: Test = Test {
    input: "tests/inputs/two.txt",
    output: "tests/expected/two.txt.out",
    output_count: "tests/expected/two.txt.c.out",
};

const THREE: Test = Test {
    input: "tests/inputs/two.txt",
    output: "tests/expected/two.txt.out",
    output_count: "tests/expected/two.txt.c.out",
};

const SKIP: Test = Test {
    input: "tests/inputs/skip.txt",
    output: "tests/expected/skip.txt.out",
    output_count: "tests/expected/skip.txt.c.out",
};

const T1: Test = Test {
    input: "tests/inputs/t1.txt",
    output: "tests/expected/t1.txt.out",
    output_count: "tests/expected/t1.txt.c.out",
};

const T2: Test = Test {
    input: "tests/inputs/t2.txt",
    output: "tests/expected/t2.txt.out",
    output_count: "tests/expected/t2.txt.c.out",
};

const T3: Test = Test {
    input: "tests/inputs/t3.txt",
    output: "tests/expected/t3.txt.out",
    output_count: "tests/expected/t3.txt.c.out",
};

const T4: Test = Test {
    input: "tests/inputs/t4.txt",
    output: "tests/expected/t4.txt.out",
    output_count: "tests/expected/t4.txt.c.out",
};

const T5: Test = Test {
    input: "tests/inputs/t5.txt",
    output: "tests/expected/t5.txt.out",
    output_count: "tests/expected/t5.txt.c.out",
};

const T6: Test = Test {
    input: "tests/inputs/t6.txt",
    output: "tests/expected/t6.txt.out",
    output_count: "tests/expected/t6.txt.c.out",
};

fn run(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(PRG)?
        .arg(test.input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

fn run_count(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.output_count)?;
    let output = Command::cargo_bin(PRG)?
        .args([test.input, "-c"])
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

fn run_stdin(test: &Test) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

fn run_stdin_count(test: &Test) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output_count)?;
    let output = Command::cargo_bin(PRG)?
        .arg("--count")
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

fn run_out_file(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.output)?;
    let out_file = NamedTempFile::new()?;
    let out_path = out_file.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args(&[test.input, out_path])
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(out_path)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

fn run_out_file_count(test: &Test) -> TestResult {
    let expected = fs::read_to_string(test.output_count)?;
    let out_file = NamedTempFile::new()?;
    let out_path = out_file.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args(&[test.input, out_path, "-c"])
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(out_path)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

fn run_stdin_out_file_count(test: &Test) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output_count)?;
    let out_file = NamedTempFile::new()?;
    let out_path = out_file.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args(&["-", out_path, "-c"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(out_path)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&EMPTY)
}

#[test]
fn empty_count() -> TestResult {
    run_count(&EMPTY)
}

#[test]
fn empty_stdin() -> TestResult {
    run_stdin(&EMPTY)
}

#[test]
fn empty_stdin_count() -> TestResult {
    run_stdin_count(&EMPTY)
}

#[test]
fn empty_out_file() -> TestResult {
    run_out_file(&EMPTY)
}

#[test]
fn empty_out_file_count() -> TestResult {
    run_out_file_count(&EMPTY)
}

#[test]
fn empty_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&EMPTY)
}

#[test]
fn one() -> TestResult {
    run(&ONE)
}

#[test]
fn one_count() -> TestResult {
    run_count(&ONE)
}

#[test]
fn one_stdin() -> TestResult {
    run_stdin(&ONE)
}

#[test]
fn one_stdin_count() -> TestResult {
    run_stdin_count(&ONE)
}

#[test]
fn one_out_file() -> TestResult {
    run_out_file(&ONE)
}

#[test]
fn one_out_file_count() -> TestResult {
    run_out_file_count(&ONE)
}

#[test]
fn one_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&ONE)
}

#[test]
fn two() -> TestResult {
    run(&TWO)
}

#[test]
fn two_count() -> TestResult {
    run_count(&TWO)
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(&TWO)
}

#[test]
fn two_stdin_count() -> TestResult {
    run_stdin_count(&TWO)
}

#[test]
fn two_out_file() -> TestResult {
    run_out_file(&TWO)
}

#[test]
fn two_out_file_count() -> TestResult {
    run_out_file_count(&TWO)
}

#[test]
fn two_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&TWO)
}

#[test]
fn three() -> TestResult {
    run(&THREE)
}

#[test]
fn three_count() -> TestResult {
    run_count(&THREE)
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(&THREE)
}

#[test]
fn three_stdin_count() -> TestResult {
    run_stdin_count(&THREE)
}

#[test]
fn three_out_file() -> TestResult {
    run_out_file(&THREE)
}

#[test]
fn three_out_file_count() -> TestResult {
    run_out_file_count(&THREE)
}

#[test]
fn three_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&THREE)
}

#[test]
fn skip() -> TestResult {
    run(&SKIP)
}

#[test]
fn skip_count() -> TestResult {
    run_count(&SKIP)
}

#[test]
fn skip_stdin() -> TestResult {
    run_stdin(&SKIP)
}

#[test]
fn skip_stdin_count() -> TestResult {
    run_stdin_count(&SKIP)
}

#[test]
fn skip_out_file() -> TestResult {
    run_out_file(&SKIP)
}

#[test]
fn skip_out_file_count() -> TestResult {
    run_out_file_count(&SKIP)
}

#[test]
fn skip_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&SKIP)
}

#[test]
fn t1() -> TestResult {
    run(&T1)
}

#[test]
fn t1_count() -> TestResult {
    run_count(&T1)
}

#[test]
fn t1_stdin() -> TestResult {
    run_stdin(&T1)
}

#[test]
fn t1_stdin_count() -> TestResult {
    run_stdin_count(&T1)
}

#[test]
fn t1_out_file() -> TestResult {
    run_out_file(&T1)
}

#[test]
fn t1_out_file_count() -> TestResult {
    run_out_file_count(&T1)
}

#[test]
fn t1_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T1)
}

#[test]
fn t2() -> TestResult {
    run(&T2)
}

#[test]
fn t2_count() -> TestResult {
    run_count(&T2)
}

#[test]
fn t2_stdin() -> TestResult {
    run_stdin(&T2)
}

#[test]
fn t2_stdin_count() -> TestResult {
    run_stdin_count(&T2)
}

#[test]
fn t2_out_file() -> TestResult {
    run_out_file(&T2)
}

#[test]
fn t2_out_file_count() -> TestResult {
    run_out_file_count(&T2)
}

#[test]
fn t2_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T2)
}

#[test]
fn t3() -> TestResult {
    run(&T3)
}

#[test]
fn t3_count() -> TestResult {
    run_count(&T3)
}

#[test]
fn t3_stdin() -> TestResult {
    run_stdin(&T3)
}

#[test]
fn t3_stdin_count() -> TestResult {
    run_stdin_count(&T3)
}

#[test]
fn t3_out_file() -> TestResult {
    run_out_file(&T3)
}

#[test]
fn t3_out_file_count() -> TestResult {
    run_out_file_count(&T3)
}

#[test]
fn t3_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T3)
}

#[test]
fn t4() -> TestResult {
    run(&T4)
}

#[test]
fn t4_count() -> TestResult {
    run_count(&T4)
}

#[test]
fn t4_stdin() -> TestResult {
    run_stdin(&T4)
}

#[test]
fn t4_stdin_count() -> TestResult {
    run_stdin_count(&T4)
}

#[test]
fn t4_out_file() -> TestResult {
    run_out_file(&T4)
}

#[test]
fn t4_out_file_count() -> TestResult {
    run_out_file_count(&T4)
}

#[test]
fn t4_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T4)
}

#[test]
fn t5() -> TestResult {
    run(&T5)
}

#[test]
fn t5_count() -> TestResult {
    run_count(&T5)
}

#[test]
fn t5_stdin() -> TestResult {
    run_stdin(&T5)
}

#[test]
fn t5_stdin_count() -> TestResult {
    run_stdin_count(&T5)
}

#[test]
fn t5_out_file() -> TestResult {
    run_out_file(&T5)
}

#[test]
fn t5_out_file_count() -> TestResult {
    run_out_file_count(&T5)
}

#[test]
fn t5_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T5)
}

#[test]
fn t6() -> TestResult {
    run(&T6)
}

#[test]
fn t6_count() -> TestResult {
    run_count(&T6)
}

#[test]
fn t6_stdin() -> TestResult {
    run_stdin(&T6)
}

#[test]
fn t6_stdin_count() -> TestResult {
    run_stdin_count(&T6)
}

#[test]
fn t6_out_file() -> TestResult {
    run_out_file(&T6)
}

#[test]
fn t6_out_file_count() -> TestResult {
    run_out_file_count(&T6)
}

#[test]
fn t6_stdin_out_file_count() -> TestResult {
    run_stdin_out_file_count(&T6)
}
