#![allow(dead_code)]
use anyhow::Result;

const ENG_PREFIXES: [&str; 5] = ["ex", "pre", "post", "re", "un"];
const ENG_SUFFIXES: [&str; 5] = ["ed", "ing", "er", "est", "ly"];

#[derive(Debug)]
struct Word<'a> {
    prefix: &'a str,
    stem: &'a str,
    suffix: &'a str,
}

impl<'a> Word<'a> {
    /// Constructor for a default `Word` with empty fields.
    fn new() -> Word<'a> {
        Self {
            prefix: "",
            stem: "",
            suffix: "",
        }
    }

    /// Constructs a `Word` object by analyzing the given word.
    fn from(word: &'a str) -> Word<'a> {
        let (prefix, without_prefix) = ENG_PREFIXES
            .iter()
            .find_map(|&prefix| word.strip_prefix(prefix).map(|stem| (prefix, stem)))
            .unwrap_or(("", word));

        let (stem, suffix) = ENG_SUFFIXES
            .iter()
            .find_map(|&suffix| {
                without_prefix
                    .strip_suffix(suffix)
                    .map(|stem| (stem, suffix))
            })
            .unwrap_or((without_prefix, ""));

        Self {
            prefix,
            stem,
            suffix,
        }
    }
}

pub(crate) fn stem_command(unstemmed_word: &str) -> Result<()> {
    let word = Word::from(unstemmed_word);
    println!("{}", word.stem);

    Ok(())
}
