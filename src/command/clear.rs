pub fn clear_command() {
    println!("\x1b[2J\x1b[H");
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[allow(unused_imports)]
    use rizzybox::*;

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("clear");

        // Assert
        cmd.assert().success();
        // TODO: is there a better way to test this?
        cmd.assert().stdout("\u{1b}[2J\u{1b}[H\n");
    }
}
