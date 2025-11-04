use std::io::IsTerminal;

use anyhow::Result;
use bat::PrettyPrinter;

pub fn echo_command(
    disable_backslash_escapes: bool,
    enable_backslash_escapes: bool,
    language: &str,
    nonewline: bool,
    text: &[String],
    theme: &str,
) -> Result<()> {
    let word_args = text.join(" ");
    let mut text = word_args.to_string();
    if !nonewline {
        text.push('\n');
    }

    if disable_backslash_escapes || !enable_backslash_escapes {
        text = text.replace("\\\\", "\\");
        text = text.replace("\\a", "\x07");
        text = text.replace("\\b", "\x08");
        text = text.replace("\\c", "");
        text = text.replace("\\e", "\x1B");
        text = text.replace("\\f", "\x0C");
        text = text.replace("\\n", "\n");
        text = text.replace("\\r", "\r");
        text = text.replace("\\t", "\t");
        text = text.replace("\\v", "\x0B");
        text = text.replace("\\0", "\0");

        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some('x') = chars.peek() {
                    chars.next(); // consume 'x'
                    let mut hex = String::new();
                    if let Some(h1) = chars.next() {
                        hex.push(h1);
                    }
                    if let Some(h2) = chars.next() {
                        hex.push(h2);
                    }
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    } else {
                        result.push('\\');
                        result.push('x');
                        result.push_str(&hex);
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        text = result;
    }

    let mut pretty_printer = PrettyPrinter::new();
    pretty_printer
        .input_from_bytes(text.as_bytes())
        .language(language)
        .theme(theme)
        .colored_output(std::io::stdout().is_terminal())
        .print()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![expect(non_snake_case)]

    use assert_cmd::Command;

    #[allow(unused_imports)]
    use rizzybox::*;

    // TODO: add tests for --language and --theme args

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("echo");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("\n");
    }

    #[test]
    fn with_text_is_success() {
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
    fn with_multiple_text_is_success() {
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

    #[test]
    fn with_no_args_does_not_interpret_backslash_chars_by_default() {
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
    fn with__E_disables_interpretation_of_backslash_chars() {
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
    fn with__e_enables_interpretation_of_backslash_chars() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let c_hello_world = "\\x23\\x69\\x6e\\x63\\x6c\\x75\\x64\\x65\\x20\\x3c\\x73\\x74\\x64\\x69\\x6f\\x2e\\x68\\x3e\\x0a\\x0a\\x69\\x6e\\x74\\x20\\x6d\\x61\\x69\\x6e\\x28\\x29\\x20\\x7b\\x0a\\x20\\x20\\x20\\x20\\x70\\x72\\x69\\x6e\\x74\\x66\\x28\\x22\\x68\\x65\\x6c\\x6c\\x6f\\x2c\\x20\\x77\\x6f\\x72\\x6c\\x64\\x5c\\x6e\\x22\\x29\\x3b\\x0a\\x7d";

        // Act
        cmd.arg("echo");
        cmd.arg("-e");
        cmd.arg(c_hello_world);

        // Assert
        cmd.assert().success();
        cmd.assert()
            .stdout("#include <stdio.h>\n\nint main() {\n    printf(\"hello, world\\n\");\n}\n");
    }

    #[test]
    fn with___nonewline_does_not_output_a_newline() {
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
    fn with_all_args() {
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
}
