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

    let mut best_match = String::from(word);
    let mut best_score = 0;

    for candidate in candidates(word) {
        if let Some(score) = WORDS.get(&candidate) {
            if *score > best_score {
                best_score = *score;
                best_match = candidate;
            }
        }
    }

    return best_match;
}

/// Generate possible spelling corrections for `word`.
fn candidates(word: &str) -> HashSet<String> {
    let mut word_set = HashSet::new();
    word_set.insert(String::from(word));

    if WORDS.contains_key(word) {
        return word_set;
    } else if let Some(single_edits) = known(edits(word)) {
        return single_edits;
    } else if let Some(double_edits) = known(edits2(word)) {
        return double_edits;
    }

    return word_set;
}

/// The subset of `words` that appear in the dictionary of WORDS
fn known(words: HashSet<String>) -> Option<HashSet<String>> {
    let mut known_words = HashSet::new();

    for word in words {
        if WORDS.contains_key(&word) {
            known_words.insert(word);
        }
    }

    match known_words.is_empty() {
        false => return Some(known_words),
        true => return None,
    }
}

/// All edits that are one edit away from `word`.
fn edits(word: &str) -> HashSet<String> {
    // String containing lowercase letters in alphabetical order
    let letters = String::from_utf8((97..123).collect()).unwrap();

    // Construct vector of split variants of `word`
    // cat -> [("", "cat"), ("c", "at"), ("ca", "t")]
    let splits: Vec<_> = (0..word.len()).map(|i| (&word[..i], &word[i..])).collect();

    // Iterate through different edit permutations
    let mut all_edits = HashSet::new();

    for (left, right) in splits {
        // Deletions
        let mut deletion = String::from(left);
        deletion = deletion + &right[1..];
        all_edits.insert(deletion);

        // Transpositions
        if right.len() > 1 {
            let mut transposition = String::from(left);
            let middle: &String = &right[..2].chars().rev().collect();
            transposition = transposition + middle + &right[2..];
            all_edits.insert(transposition);
        }

        for letter in letters.chars() {
            // Replacements
            let mut replacement = String::from(left);
            replacement.push(letter);
            replacement = replacement + &right[1..];
            all_edits.insert(replacement);

            // Insertions
            let mut insertion = String::from(left);
            insertion.push(letter);
            insertion = insertion + right;
            all_edits.insert(insertion);
        }
    }

    // End-of-word insertions (appends?)
    for letter in letters.chars() {
        let mut insertion = String::from(word);
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

        // Use a punctuation-based test word to ensure unique edits
        let test_word = "!@#$%^&*()";

        for i in 1..(test_word.len() + 1) {
            assert_eq!(edits(&test_word[..i]).len(), 54*i + 25);
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
