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

const LONG_PATH: &str = "/var/home/username/.local/bin/mybinary";
const ANOTHER_PATH: &str = "/var/home/username/.local/bin/mycoolerbinary";
const PATH_WITH_SUFFIX: &str = "/var/home/username/git/coolproject/main.c";

#[test]
fn basename_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg(LONG_PATH);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("mybinary\n");
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn basename_argshift_does_work() {
    // FIXME: this is yikes, but I think running the tests in QEMU is
    // preventing us from being able to create the symlinks we need
    if cfg!(target_os = "linux")
        && std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "x86_64"
    {
        eprintln!("Skipping test on non-x86_64 Linux");
        return;
    }

    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

    let temp_dir = env::temp_dir();
    let symlink_path = temp_dir.join("basename");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);
    cmd.arg(LONG_PATH);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("mybinary\n");
}

#[test]
fn basename_with___multiple_arg_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg("--multiple");
    cmd.arg(LONG_PATH);
    cmd.arg(ANOTHER_PATH);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("mybinary\nmycoolerbinary\n");
}

#[test]
fn basename_with___suffix_arg_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg("--suffix");
    cmd.arg(".c");
    cmd.arg(PATH_WITH_SUFFIX);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("main\n");
}

#[test]
fn basename_with___zero_arg_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg("--zero");
    cmd.arg(LONG_PATH);

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout(predicates::str::ends_with("mybinary\0"));
    // TODO: figure out how to assert the absence of a newline
}

#[test]
fn basename_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}

#[test]
fn basename_with_all_args_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("basename");
    cmd.arg("-a"); // --multiple
    cmd.arg("-s"); // --suffix
    cmd.arg(".c");
    cmd.arg("-z"); // --zero
    cmd.arg(LONG_PATH);
    cmd.arg(ANOTHER_PATH);
    cmd.arg(PATH_WITH_SUFFIX);

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains(
        "mybinary\0mycoolerbinary\0main\0",
    ));
}
