use anyhow::Result;
use bat::PrettyPrinter;
use std::io::IsTerminal;

pub(crate) fn cat_command(files: &[String], language: &str, theme: &str) -> Result<()> {
    let mut pretty_printer = PrettyPrinter::new();
    let mut print_file = |file: &str| {
        pretty_printer
            .input_file(file)
            .language(language)
            .theme(theme)
            .colored_output(std::io::stdout().is_terminal())
            .print()
            .unwrap();
    };

    if files.is_empty() || files.first().unwrap() == "-" {
        print_file("/dev/stdin");
    } else {
        for file in files {
            print_file(file);
        }
    }

    Ok(())
}
