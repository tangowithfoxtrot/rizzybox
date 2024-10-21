use bat::PrettyPrinter;

pub(crate) fn cat_command(file: &str, language: &str, theme: &str) {
    PrettyPrinter::new()
        .input_file(file)
        .language(language)
        .theme(theme)
        .print()
        .unwrap();
}
