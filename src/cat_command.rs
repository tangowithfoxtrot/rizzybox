use bat::PrettyPrinter;

pub(crate) fn cat_command(file: &str) {
    PrettyPrinter::new()
        .input_file(file)
        .language("env")
        .print()
        .unwrap();
}
