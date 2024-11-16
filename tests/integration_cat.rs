#![allow(non_snake_case)]
use assert_cmd::Command;
use core::str;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
};

#[allow(unused_imports)]
use rizzybox::*;

const FILE_TO_CAT: &str = "/etc/hosts";

#[test]
fn cat_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("cat");
    cmd.arg(FILE_TO_CAT);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("127.0.0.1"));
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn cat_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

    let temp_dir = env::temp_dir();
    let symlink_path = temp_dir.join("cat");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);
    cmd.arg(FILE_TO_CAT);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("127.0.0.1"));
}

#[test]
fn cat_from_stdin_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("cat");
    cmd.write_stdin("woah hey there");

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout(predicates::str::contains("woah hey there"));
}

#[test]
fn cat_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("cat");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}
