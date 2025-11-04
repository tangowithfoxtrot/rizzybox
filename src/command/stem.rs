use std::collections::HashSet;

const ENG_PREFIXES: [&str; 5] = ["ex", "pre", "post", "re", "un"];
const ENG_SUFFIXES: [&str; 5] = ["ed", "ing", "er", "est", "ly"];

const WORDLIST: &str = include_str!("../res/mthesaur.csv");

enum EngArticle {
    A,
    An,
    The,
}

impl EngArticle {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase() {
            val if val == *"A" => Some(EngArticle::A),
            val if val == *"AN" => Some(EngArticle::An),
            val if val == *"THE" => Some(EngArticle::The),
            _ => None,
        }
    }
}

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
struct Word<'a> {
    prefix: &'a str,
    stem: &'a str,
    suffix: &'a str,
}

impl<'a> Word<'a> {
    #[expect(dead_code)]
    /// Constructor for a default `Word` with empty fields.
    fn new() -> Word<'a> {
        Self {
            prefix: "",
            stem: "",
            suffix: "",
        }
    }

    /// Constructs a `Word` object by analyzing the given word.
    fn from(word: &'a str, wordlist: &'a HashSet<&str>) -> Word<'a> {
        // don't stem articles
        if EngArticle::from_str(word).is_some() {
            return Self {
                prefix: "",
                stem: word,
                suffix: "",
            };
        }

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
        if EngConsonant::is_consonant(stem.chars().last().unwrap_or('.')) // '.' is arbitrary
            && (suffix.eq("ing") || suffix.eq("est") || suffix.eq("er") || suffix.eq("ed"))
        {
            stem = stem.strip_suffix(|_: char| true).unwrap_or(stem);
        }

        // Account for words like "configured", which would normally be reduced
        // to "configur" with our suffix-removal strategy. We use a wordlist
        // to find the shortest match for a real word and return that. This is not
        // totally accurate. "ingest" would actually just return "in" because it
        // meets these conditions.
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

impl std::fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.prefix, self.stem, self.suffix)
    }
}

fn read_wordlist() -> HashSet<&'static str> {
    let mut word_set: HashSet<&str> = HashSet::new();

    for val in WORDLIST.trim().split(',') {
        word_set.insert(val);
    }

    word_set
}

pub fn stem_command(nonewline: bool, unstemmed_words: &[String]) {
    let mut to_print = Vec::new();
    let wordlist = read_wordlist();

    for unstemmed_word in unstemmed_words {
        let word = Word::from(unstemmed_word, &wordlist);
        to_print.push(word.stem);
    }

    print!(
        "{}",
        to_print
            .into_iter()
            .collect::<Vec<&str>>()
            .join(" ")
            .trim_ascii_end()
    );

    if !nonewline {
        println!();
    }
}

#[cfg(test)]
mod tests {
    #![expect(non_snake_case)]

    use assert_cmd::Command;

    #[allow(unused_imports)]
    use rizzybox::*;

    #[test]
    fn success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("stem");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("\n");
    }

    #[test]
    fn with_text_is_success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("stem");
        cmd.arg("henlo");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("henlo\n");
    }

    #[test]
    fn with_multiple_strings_is_success() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("stem");
        cmd.arg("henlo");
        cmd.arg("world");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("henlo world\n");
    }

    #[test]
    fn with___no_newline_does_not_output_newline() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("stem");
        cmd.arg("--nonewline");
        cmd.arg("preconfigured");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("configure");
    }

    #[test]
    /// stem will never have 100% accuracy. This test will only serve as a way
    /// to detect regressions. Words in this test should always work.
    fn stems_words_correctly() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let words = &[
            "big",
            "bigger",
            "biggest",
            "configure",
            "configured",
            "configuring",
            "honestly",
            "preconfigured",
        ];

        // Act
        cmd.arg("stem");
        cmd.args(words);

        // Assert
        cmd.assert().success();
        cmd.assert()
            .stdout("big big big configure configure configure honest configure\n");
    }

    #[test]
    fn with_all_args() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act
        cmd.arg("stem");
        cmd.arg("-n"); // --nonewline
        cmd.arg("biggest");

        // Assert
        cmd.assert().success();
        cmd.assert().stdout("big");
    }
}
