use anyhow::Result;
use assert_cmd::Command;
use std::fs;

const PRG: &str = "todo-cli-3";
const TODO_EXAMPLE: &str = "new todo";

#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "-help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicates::str::contains("Usage"));
    }

    Ok(())
}

fn delete_db(db_name: &str) -> Result<()> {
    // clear db before running tests
    fs::remove_file(db_name)?;
    Ok(())
}

/// Create the db with a basic todo
fn setup_db_with_content(db_name: &str) -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(format!("-d={}", db_name))
        .args(&["add", TODO_EXAMPLE])
        .assert();

    Ok(())
}

fn validate_output(
    args: &[&str],
    expected_file: &str,
    db_name: &str,
    read_from_stderr: bool,
) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .arg(format!("-d={}", db_name))
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let std = if read_from_stderr {
        output.stderr
    } else {
        output.stdout
    };
    let cli_output = String::from_utf8(std).expect("invalid UTF-8");
    assert_eq!(cli_output, expected);

    delete_db(db_name)?;

    Ok(())
}

#[test]
fn empty_list() -> Result<()> {
    validate_output(
        &["list"],
        "tests/expected/list.empty.out",
        "empty.db.txt",
        false,
    )
}

#[test]
fn add_cmd() -> Result<()> {
    validate_output(
        &["add", "new todo"],
        "tests/expected/add.out",
        "add.db.txt",
        false,
    )
}

#[test]
fn edit_cmd() -> Result<()> {
    let db_name = "edit.db.txt";
    setup_db_with_content(db_name)?;
    validate_output(
        &["edit", "1", "todo edited"],
        "tests/expected/edit.out",
        db_name,
        false,
    )
}

#[test]
fn edit_empty_list() -> Result<()> {
    validate_output(
        &["edit", "123", "todo edited"],
        "tests/expected/edit.empty.list.out",
        "edit.empty.list.out",
        true,
    )
}

#[test]
fn invalid_edit() -> Result<()> {
    let db_name = "edit.invalid.txt";
    setup_db_with_content(db_name)?;
    validate_output(
        &["edit", "123", "todo edited"],
        "tests/expected/edit.invalid.out",
        db_name,
        true,
    )
}

#[test]
fn delete_cmd() -> Result<()> {
    let db_name = "delete.db.txt";
    setup_db_with_content(db_name)?;
    validate_output(
        &["delete", "1"],
        "tests/expected/delete.out",
        db_name,
        false,
    )
}

#[test]
fn invalid_delete() -> Result<()> {
    let db_name = "delete.invalid.db.txt";
    setup_db_with_content(db_name)?;
    validate_output(
        &["delete", "123"],
        "tests/expected/delete.invalid.out",
        db_name,
        true,
    )
}
