use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csvinfo").unwrap();
    cmd.arg("tests/data/404.csv");

    let assert = cmd.assert();
    assert.failure();

    let predicate_fn = predicate::str::contains("No such file or directory");
    cmd.assert().failure().stdout(predicate_fn);

    Ok(())
}

#[test]
fn simple_file() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csvinfo").unwrap();
    cmd.arg("tests/data/simple.csv");
    cmd.assert().success();
    Ok(())
}

#[test]
fn quoted_file() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csvinfo").unwrap();
    cmd.arg("tests/data/quoted.csv");
    cmd.arg("-q");
    cmd.arg("-s");

    // If quoted, properly, this will contain "City,State" as one of the field headers.
    let predicate_fn = predicate::str::contains("City,State");
    cmd.assert().success().stdout(predicate_fn);

    Ok(())
}

#[test]
#[ignore] // TODO: Decide if we want to allow for mixed quoted CSV files
fn mixed_file() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csvinfo").unwrap();
    cmd.arg("tests/data/mixed.csv");
    cmd.arg("-q");
    cmd.arg("-s");

    // If quoted, properly, this will contain "City,State" as one of the field headers.
    let predicate_fn = predicate::str::contains("City,State");
    cmd.assert().success().stdout(predicate_fn);

    // It will also not contain an unknown "unk" field
    let predicate_fn = predicate::str::contains("unk").not();
    cmd.assert().success().stdout(predicate_fn);

    Ok(())
}

#[test]
fn test_empty_field() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("csvinfo").unwrap();
    cmd.arg("tests/data/empty.csv");
    cmd.arg("-s"); // You need to skip headers on an empty test, else the header is considered

    let predicate_fn = predicate::str::contains("empty   Part Time");
    cmd.assert().success().stdout(predicate_fn);

    Ok(())
}
