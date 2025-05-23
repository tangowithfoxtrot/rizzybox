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
    let symlink_path = temp_dir.join("false");
    let _ = symlink(rizzybox_path.clone(), &symlink_path);
    let symlinked_bin = symlink_path.to_string_lossy().to_string();

    let _cleanup = TestCleanup {
        file: Some(symlinked_bin.clone()),
    };

    // Act
    let mut cmd = Command::cargo_bin(&symlinked_bin).unwrap();

    // Assert
    assert_ne!(rizzybox_path, PathBuf::from(symlinked_bin));
    cmd.assert().failure();
}
