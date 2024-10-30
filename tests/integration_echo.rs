#![allow(non_snake_case)]
use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
};

#[allow(unused_imports)]
use rizzybox::*;

// TODO: add tests for --language and --theme args

#[test]
fn echo_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("\n");
}

#[test]
fn echo_with_text_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("henlo");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("henlo\n");
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn echo_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = rizzybox_cmd.get_program();

    let temp_dir = env::temp_dir();
    let _ = symlink(
        rizzybox_path,
        format!("{}echo", &temp_dir.to_string_lossy()),
    );

    let symlinked_bin = format!("{}echo", &temp_dir.to_string_lossy());
    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);
    cmd.arg("using `echo` like a normal binary B)");

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout("using `echo` like a normal binary B)\n");
}

#[test]
fn echo_with_no_args_does_not_interpret_backslash_chars_by_default() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("\r");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("\r\n");
}

#[test]
fn echo_with__E_disables_interpretation_of_backslash_chars() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("-E");
    cmd.arg("\r");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("\r\n");
}

#[test]
fn echo_with__e_enables_interpretation_of_backslash_chars() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("-e");
    cmd.arg("\r");

    // Assert
    cmd.assert().success();
    // FIXME: idk if this tests it properly
    cmd.assert().stdout("␍␊\n");
}

#[test]
fn echo_with___nonewline_does_not_output_a_newline() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("--nonewline");
    cmd.arg("howdy");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("howdy");
    // TODO: figure out how to assert the absence of a newline
}

#[test]
fn echo_with___help_prints_help() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("--help");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout(predicates::str::contains("Usage:"));
}

#[test]
fn echo_with_all_args_works() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("-e");
    cmd.arg("-n"); // --nonewline
    cmd.arg("weirdoutput\r");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("weirdoutput␍");
}
