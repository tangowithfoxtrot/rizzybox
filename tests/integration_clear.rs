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
fn clear_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("clear");

    // Assert
    cmd.assert().success();
    // TODO: is there a better way to test this?
    cmd.assert().stdout("\u{1b}[2J\u{1b}[H\n");
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn clear_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

    let temp_dir = env::temp_dir();
    let symlink_path = temp_dir.join("clear");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("\u{1b}[2J\u{1b}[H\n");
}
