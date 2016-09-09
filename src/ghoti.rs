#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
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


pub fn correction(word: &str) -> String {
    if word == "" {
        return String::from("");
    }

    let random_word = WORDS.keys().next().unwrap();
    return random_word.clone();
}


#[test]
fn empty_correction() {
    assert_eq!(correction(""), "");
}

#[test]
fn bad_correction() {
    assert!(correction("ghoti") != "");
}
