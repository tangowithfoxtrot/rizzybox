use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
};

#[allow(unused_imports)]
use rizzybox::*;

#[test]
fn true_is_success() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("true");

    // Assert
    cmd.assert().success();
}

#[test]
fn true_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = rizzybox_cmd.get_program();

    let temp_dir = env::temp_dir();
    let _ = symlink(
        &rizzybox_path,
        format!("{}true", &temp_dir.to_string_lossy()),
    );

    let symlinked_bin = format!("{}true", &temp_dir.to_string_lossy());

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);

    // Assert
    cmd.assert().success();
}
