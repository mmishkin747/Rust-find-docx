use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("rfd")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("rfd")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_err(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("rfd")?
        .args(args)
        .assert()
        .stderr(expected);
    Ok(())
}

#[test]
fn hello_1() -> TestResult {
    run(&["tests/input/hello.docx"], "tests/expected/hello.txt")
}

/* 
#[test]
fn no_open() -> TestResult{
    run_err(&["tests/input/no_open.docx"], "tests/expected/no_open.txt")
}
*/

#[test]
fn walk_hello() -> TestResult{
    run(&["tests/input/a"], "tests/expected/walk_hello.txt")
}

#[test]
fn find_text() -> TestResult{
    run(&["tests/input/a", "-p", "Test"], "tests/expected/find_text.txt")
}

#[test]
fn find_text_insensitive() -> TestResult{
    run(&["tests/input/a", "-p", "test", "-i"], "tests/expected/find_text.txt")
}
