use bat::PrettyPrinter;
use std::io::IsTerminal;

pub fn cat_command(
    files: &[String],
    language: &str,
    theme: &str,
    show_all: bool,
    list_themes: bool,
    number_lines: bool,
) {
    let mut pretty_printer = PrettyPrinter::new();
    if list_themes {
        let themes = pretty_printer.themes();
        for theme in themes {
            println!("{theme}");
        }
        return;
    }

    let mut print_file = |file: &str| {
        if pretty_printer
            .input_file(file)
            .language(language)
            .theme(theme)
            .colored_output(std::io::stdout().is_terminal())
            .show_nonprintable(show_all)
            .line_numbers(number_lines)
            .print()
            .is_ok()
        {}
    };

    if files.is_empty() || Ok(files.first()) == Err("-") {
        print_file("/dev/stdin");
    } else {
        for file in files {
            print_file(file);
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use core::str;

    #[allow(unused_imports)]
    use rizzybox::*;

    const FILE_TO_CAT: &str = "/etc/hosts";

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("cat");
        cmd.arg(FILE_TO_CAT);

        // Assert
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("127.0.0.1"));
    }

    #[test]
    fn from_stdin() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("cat");
        cmd.write_stdin("woah hey there");

        // Assert
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("woah hey there"));
    }
}
