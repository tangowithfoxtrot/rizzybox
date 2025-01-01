use anyhow::Result;
use bat::PrettyPrinter;
use std::io::IsTerminal;

pub(crate) fn cat_command(
    files: &[String],
    language: &str,
    theme: &str,
    show_all: &bool,
    list_themes: &bool,
) -> Result<()> {
    let mut pretty_printer = PrettyPrinter::new();
    if *list_themes {
        let themes = pretty_printer.themes();
        for theme in themes {
            println!("{theme}");
        }
        return Ok(());
    }

    let mut print_file = |file: &str| {
        if pretty_printer
            .input_file(file)
            .language(language)
            .theme(theme)
            .colored_output(std::io::stdout().is_terminal())
            .show_nonprintable(*show_all)
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

    Ok(())
}
