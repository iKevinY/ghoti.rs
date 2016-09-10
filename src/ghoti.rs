#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::fs::File;

use regex::Regex;


lazy_static! {
    static ref WORDS: HashMap<String, usize> = {
        let mut f = File::open("data/big.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        // Count occurrences of words found in `big.txt`
        let mut words = HashMap::new();

        for word in Regex::new(r"\W+").unwrap().split(&s) {
            let count = words.entry(word.to_lowercase()).or_insert(0);
            *count += 1;
        }

        return words;
    };
}


/// Most probable spelling correction for `word`.
pub fn correction(word: &str) -> String {
    if word == "" {
        return String::from("");
    }

    let mut best_match = word.to_string();
    let mut best_score = 0;

    for candidate in candidates(word) {
        if let Some(score) = WORDS.get(&candidate) {
            if *score > best_score {
                best_score = *score;
                best_match = candidate;
            }
        }
    }

    return best_match.to_string();
}

/// Generate possible spelling corrections for `word`.
fn candidates(word: &str) -> HashSet<String> {
    let mut word_set = HashSet::new();
    word_set.insert(word.to_string());

    if let Some(set) = known(word_set) {
        return set;
    } else if let Some(set) = known(edits(word)) {
        return set;
    } else if let Some(set) = known(edits2(word)) {
        return set;
    } else {
        let mut word_set = HashSet::new();
        word_set.insert(word.to_string());
        return word_set;
    }
}

/// The subset of `words` that appear in the dictionary of WORDS
fn known(words: HashSet<String>) -> Option<HashSet<String>> {
    let mut known_words = HashSet::new();

    for word in words {
        if WORDS.contains_key(&word) {
            known_words.insert(word);
        }
    }

    match known_words.len() {
        0 => None,
        _ => Some(known_words),
    }
}

/// All edits that are one edit away from `word`.
fn edits(word: &str) -> HashSet<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz";

    // Construct vector of split variants of `word`
    // ghoti -> [("", "ghoti"), ("g", "hoti"), ... ("ghot", "i")]
    let mut splits: Vec<(&str, &str)> = Vec::new();
    for i in 0..word.len() {
        splits.push((&word[..i], &word[i..]));
    }

    let mut all_edits: HashSet<String> = HashSet::new();

    // Iterate through different edit permutations
    for (i, &(left, right)) in splits.iter().enumerate() {
        // Deletions
        let mut deletion = left.to_string();
        deletion = deletion + &right[1..];
        all_edits.insert(deletion);

        // Transpositions
        // Skip the final split to ensure `right` contains > 1 character
        if i < splits.len() - 1 {
            let mut transposition = left.to_string();
            transposition.push(right.chars().nth(1).unwrap());
            transposition.push(right.chars().nth(0).unwrap());
            transposition = transposition + &right[2..];
            all_edits.insert(transposition);
        }

        for letter in letters.chars() {
            // Replacements
            let mut replacement = left.to_string();
            replacement.push(letter);
            replacement = replacement + &right[1..];
            all_edits.insert(replacement);

            // Insertions
            let mut insertion = left.to_string();
            insertion.push(letter);
            insertion = insertion + right;
            all_edits.insert(insertion);
        }
    }

    // Insertions at end of word (appends?)
    for letter in letters.chars() {
        let mut insertion = word.to_string();
        insertion.push(letter);
        all_edits.insert(insertion);
    }

    return all_edits;
}

/// All edits that are two edits away from `word`
fn edits2(word: &str) -> HashSet<String> {
    let mut all_edits = HashSet::new();

    for single_edits in edits(word) {
        for edit in edits(&single_edits) {
            all_edits.insert(edit);
        }
    }

    return all_edits;
}


#[cfg(test)]
mod tests {
    use super::{candidates, edits};

    #[test]
    fn edit_count() {
        // Words of length n > 1 will have:
        // - n deletions
        // - n-1 transpositions
        // - 26n replacements
        // - 26(n+1) insertions
        // = 54n + 25 total edits

        let test_word = "!@#$%^&*()";

        for i in 1..(test_word.len() + 1) {
            assert_eq!(edits(&test_word[..i]).len(), 54 * i + 25);
        }
    }

    #[test]
    fn possible_candidates() {
        let candidates = candidates("ct");
        assert!(candidates.contains("cat"));
        assert!(candidates.contains("cot"));
        assert!(candidates.contains("cut"));
        assert!(!candidates.contains("czt"));
    }
}
