#![allow(non_snake_case)]
use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
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

#[test]
fn echo_with_multiple_text_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("echo");
    cmd.arg("henlo");
    cmd.arg("world");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("henlo world\n");
}

/// tests the ability to invoke `rizzybox COMMAND` as `COMMAND` directly
#[test]
fn echo_argshift_does_work() {
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
    let symlink_path = temp_dir.join("echo");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

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
    let chello_world = "\\x23\\x69\\x6e\\x63\\x6c\\x75\\x64\\x65\\x20\\x3c\\x73\\x74\\x64\\x69\\x6f\\x2e\\x68\\x3e\\x0a\\x0a\\x69\\x6e\\x74\\x20\\x6d\\x61\\x69\\x6e\\x28\\x29\\x20\\x7b\\x0a\\x20\\x20\\x20\\x20\\x70\\x72\\x69\\x6e\\x74\\x66\\x28\\x22\\x68\\x65\\x6c\\x6c\\x6f\\x2c\\x20\\x77\\x6f\\x72\\x6c\\x64\\x5c\\x6e\\x22\\x29\\x3b\\x0a\\x7d";

    // Act
    cmd.arg("echo");
    cmd.arg("-e");
    cmd.arg(chello_world);

    // Assert
    cmd.assert().success();
    cmd.assert()
        .stdout("#include <stdio.h>\n\nint main() {\n    printf(\"hello, world\\n\");\n}\n");
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
    cmd.arg("weirdoutput");

    // Assert
    cmd.assert().success();
    cmd.assert().stdout("weirdoutput");
}
