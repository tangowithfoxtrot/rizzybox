use std::io::IsTerminal;

use anyhow::Result;
use bat::PrettyPrinter;

pub(crate) fn echo_command(
    disable_backslash_escapes: &bool,
    enable_backslash_escapes: &bool,
    language: &str,
    nonewline: &bool,
    text: &str,
    theme: &str,
) -> Result<()> {
    let mut text = text.to_string();
    if !nonewline {
        text.push('\n');
    }

    if *disable_backslash_escapes || !*enable_backslash_escapes {
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
        for i in 0..=255 {
            text = text.replace(&format!("\\x{:02X}", i), &format!("{}", i as u8 as char));
        }
    }

    let mut pretty_printer = PrettyPrinter::new();
    pretty_printer
        .input_from_bytes(text.as_bytes())
        .language(language)
        .theme(theme)
        .colored_output(std::io::stdout().is_terminal())
        .show_nonprintable(*enable_backslash_escapes && *disable_backslash_escapes)
        .print()
        .unwrap();
    Ok(())
}
