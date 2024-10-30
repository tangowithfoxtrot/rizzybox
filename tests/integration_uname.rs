#![allow(non_snake_case)]
use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
};

#[allow(unused_imports)]
use rizzybox::*;

#[test]
fn uname_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("uname");

    // Assert
    cmd.assert().success();
}

#[test]
fn uname_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = rizzybox_cmd.get_program();

    let temp_dir = env::temp_dir();
    let _ = symlink(
        rizzybox_path,
        format!("{}uname", &temp_dir.to_string_lossy()),
    );

    let symlinked_bin = format!("{}uname", &temp_dir.to_string_lossy());
    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);

    // Assert
    assert_ne!(rizzybox_path, PathBuf::from(symlinked_bin));
    cmd.assert().success();
}

#[test]
fn uname_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("uname");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}
