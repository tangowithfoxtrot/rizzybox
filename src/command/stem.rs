use std::collections::HashSet;

use anyhow::{bail, Result};

const ENG_PREFIXES: [&str; 5] = ["ex", "pre", "post", "re", "un"];
const ENG_SUFFIXES: [&str; 5] = ["ed", "ing", "er", "est", "ly"];

enum EngConsonant {
    B,
    C,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
    V,
    W,
    X,
    Y,
    Z,
}

impl EngConsonant {
    fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'B' => Some(EngConsonant::B),
            'C' => Some(EngConsonant::C),
            'D' => Some(EngConsonant::D),
            'F' => Some(EngConsonant::F),
            'G' => Some(EngConsonant::G),
            'H' => Some(EngConsonant::H),
            'J' => Some(EngConsonant::J),
            'K' => Some(EngConsonant::K),
            'L' => Some(EngConsonant::L),
            'M' => Some(EngConsonant::M),
            'N' => Some(EngConsonant::N),
            'P' => Some(EngConsonant::P),
            'Q' => Some(EngConsonant::Q),
            'R' => Some(EngConsonant::R),
            'S' => Some(EngConsonant::S),
            'T' => Some(EngConsonant::T),
            'V' => Some(EngConsonant::V),
            'W' => Some(EngConsonant::W),
            'X' => Some(EngConsonant::X),
            'Y' => Some(EngConsonant::Y),
            'Z' => Some(EngConsonant::Z),
            _ => None,
        }
    }

    fn is_consonant(c: char) -> bool {
        Self::from_char(c).is_some()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Word<'a> {
    prefix: &'a str,
    stem: &'a str,
    suffix: &'a str,
}

impl<'a> Word<'a> {
    #[allow(dead_code)]
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

        let (mut stem, suffix) = ENG_SUFFIXES
            .iter()
            .find_map(|&suffix| {
                without_prefix
                    .strip_suffix(suffix)
                    .map(|stem| (stem, suffix))
            })
            .unwrap_or((without_prefix, ""));

        // Account for stems that end with consonants, which usually have their
        // last letter duplicated when a suffix is added.
        // Example: "biggest" would normally return "bigg" because of how "est"
        // is recognized as a suffix, so we remove the extraneous 'g' to make it
        // "big". This is likely not always correct.
        if EngConsonant::is_consonant(stem.chars().last().unwrap()) {
            stem = stem.strip_suffix(|_: char| true).unwrap_or(stem)
        }

        // Account for words like "configured", which would normally be reduced
        // to "configur" with our suffix-removal strategy. We use a wordlist
        // to find the shortest match for a real word and return that. This is not
        // totally accurate. "ingest" would actually just return "in" because it
        // meets these conditions.
        let wordlist = read_wordlist().expect("wordlist should always exist");
        let mut potential_stem_matches: HashSet<&str> = HashSet::new();
        for word in wordlist {
            if word.starts_with(stem) {
                potential_stem_matches.insert(word);
            }
        }
        stem = potential_stem_matches
            .iter()
            .min_by_key(|s| s.len())
            .map_or(stem, |v| v);

        Self {
            prefix,
            stem,
            suffix,
        }
    }
}

fn read_wordlist() -> Result<HashSet<&'static str>> {
    // Include the wordlist in the binary.
    // FIXME: this makes the binary much larger :/
    let data = include_str!("../res/mthesaur.csv");
    let mut word_set: HashSet<&str> = HashSet::new();

    for val in data.split(',') {
        word_set.insert(val);
    }

    Ok(word_set)
}

pub(crate) fn stem_command(unstemmed_word: &str) -> Result<()> {
    // bail early if user invokes with `stem ''` so we can unwrap safely
    if unstemmed_word.is_empty() {
        bail!("cannot find the stem of an empty word")
    }

    let word = Word::from(unstemmed_word);
    println!("{}", word.stem);

    Ok(())
}
