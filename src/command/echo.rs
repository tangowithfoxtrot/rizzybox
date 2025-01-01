use std::io::IsTerminal;

use anyhow::Result;
use bat::PrettyPrinter;

pub fn echo_command(
    disable_backslash_escapes: &bool,
    enable_backslash_escapes: &bool,
    language: &str,
    nonewline: &bool,
    text: &[String],
    theme: &str,
) -> Result<()> {
    let word_args = text.join(" ");
    let mut text = word_args.to_string();
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
