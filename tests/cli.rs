use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csv_utils").unwrap();
    cmd.arg("tests/data/404.csv");

    let assert = cmd.assert();
    assert.failure();

    let predicate_fn = predicate::str::contains("No such file or directory");
    cmd.assert().failure().stderr(predicate_fn);
    Ok(())
}

#[test]
fn simple_file() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csv_utils").unwrap();
    cmd.arg("tests/data/simple.csv");
    cmd.assert().success();
    Ok(())
}
