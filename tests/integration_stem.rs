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
fn stem_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("\n");
}

#[test]
fn stem_with_text_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");
    cmd.arg("henlo");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("henlo\n");
}

#[test]
fn stem_with_multiple_text_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");
    cmd.arg("henlo");
    cmd.arg("world");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("henlo world\n");
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn stem_argshift_does_work() {
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
    let symlink_path = temp_dir.join("stem");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);
    cmd.arg("using `stem` like a normal binary B)");

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout("using `stem` like a normal binary B)\n");
}

#[test]
fn stem_with___no_newline_does_not_output_newline() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");
    cmd.arg("--nonewline");
    cmd.arg("preconfigured");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("configure");
}

#[test]
/// stem will never have 100% accuracy. This test will only serve as a way
/// to detect regressions. Words in this test should always work.
fn stem_stems_words_correctly() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let words = &[
        "big",
        "bigger",
        "biggest",
        "configure",
        "configured",
        "configuring",
        "honestly",
        "preconfigured",
    ];

    // Act
    cmd.arg("stem");
    cmd.args(words);

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout("big big big configure configure configure honest configure\n");
}

#[test]
fn stem_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}

#[test]
fn stem_with_all_args_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("stem");
    cmd.arg("-n"); // --nonewline
    cmd.arg("biggest");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("big");
}
