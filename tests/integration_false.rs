use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
};

#[allow(unused_imports)]
use rizzybox::*;

#[test]
fn false_is_failure() {
    // Arrange
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    // Act
    cmd.arg("false");

    // Assert
    cmd.assert().failure();
}

#[test]
fn false_argshift_does_work() {
    // Arrange
    let rizzybox_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let rizzybox_path = rizzybox_cmd.get_program();

    let temp_dir = env::temp_dir();
    let _ = symlink(
        rizzybox_path,
        format!("{}false", &temp_dir.to_string_lossy()),
    );

    let symlinked_bin = format!("{}false", &temp_dir.to_string_lossy());
    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::cargo_bin(&symlinked_bin).unwrap();

    // Assert
    assert_ne!(rizzybox_path, PathBuf::from(symlinked_bin));
    cmd.assert().failure();
}
