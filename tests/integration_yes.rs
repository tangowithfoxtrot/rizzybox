#![allow(non_snake_case)]
use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
    time::SystemTime,
};

#[allow(unused_imports)]
use rizzybox::*;

// Because the `yes` command will run indefinitely by default, tests should always
// use either the --amount or --duration args so that the command can exit successfully

#[test]
fn yes_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("yes").arg("--duration").arg("1");

    // Assert
    cmd.assert().success();
}

#[test]
fn yes_argshift_does_work() {
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
    let symlink_path = temp_dir.join("yes");
    let _ = symlink(rizzybox_path.clone(), &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::cargo_bin(&symlinked_bin).unwrap();
    cmd.arg("--duration").arg("1");

    // Assert
    assert_ne!(rizzybox_path, PathBuf::from(symlinked_bin));
    cmd.assert().success();
}

#[test]
fn yes_with___amount_outputs_the_specified_amount_of_text() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("yes");
    cmd.arg("--amount");
    cmd.arg("789");

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout(predicates::str::contains("y\n".repeat(789)));
}

#[test]
fn yes_with___duration_runs_for_the_specified_duration() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("yes");
    cmd.arg("--duration");
    cmd.arg("1");

    // Assert
    let start = SystemTime::now();
    cmd.assert().success();
    let elapsed = start.elapsed().unwrap();
    assert!(elapsed.as_secs() >= 1);
    cmd.assert().stdout(predicates::str::contains("y"));
}

#[test]
fn yes_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("yes");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}
