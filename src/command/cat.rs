use std::io::IsTerminal;

use bat::PrettyPrinter;

pub(crate) fn cat_command(file: &[String], language: &str, theme: &str) {
    let mut pretty_printer = PrettyPrinter::new();
    for file in file.iter() {
        pretty_printer
            .input_file(file)
            .language(language)
            .theme(theme)
            .colored_output(std::io::stdout().is_terminal())
            .print()
            .unwrap();
    }
}
