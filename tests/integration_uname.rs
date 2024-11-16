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
    let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

    let temp_dir = env::temp_dir();
    let symlink_path = temp_dir.join("uname");
    let _ = symlink(rizzybox_path.clone(), &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

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
