use assert_cmd::Command;
use std::{
    env::{self},
    os::unix::fs::symlink,
    path::PathBuf,
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
    let rizzybox_path = PathBuf::from(rizzybox_cmd.get_program());

    let temp_dir = env::temp_dir();
    let symlink_path = temp_dir.join("true");
    let _ = symlink(rizzybox_path, &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::new(&symlinked_bin);

    // Assert
    cmd.assert().success();
}
