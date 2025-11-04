pub fn basename_command(mut multiple: bool, name: &[String], suffix: Option<&String>, zero: bool) {
    let delimiter = '/';

    for name in name {
        let split_output = name.rsplit_once(delimiter);
        let mut output = if let Some((_, right_string)) = split_output {
            right_string
        } else {
            name
        };

        if let Some(suffix) = suffix {
            multiple = true;

            if output.ends_with(suffix) {
                output = output.trim_end_matches(suffix);
            }
        }

        print!("{output}{}", if zero { "\0" } else { "\n" });

        if !multiple {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #![expect(non_snake_case)]

    use assert_cmd::Command;
    use core::str;

    #[allow(unused_imports)]
    use rizzybox::*;

    const LONG_PATH: &str = "/var/home/username/.local/bin/mybinary";
    const ANOTHER_PATH: &str = "/var/home/username/.local/bin/mycoolerbinary";
    const PATH_WITH_SUFFIX: &str = "/var/home/username/git/coolproject/main.c";

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("basename");
        cmd.arg(LONG_PATH);

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("mybinary\n");
    }

    #[test]
    fn with___multiple_arg() {
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
    fn with___suffix_arg() {
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
    fn with___zero_arg() {
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
    fn with_all_args() {
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
}
