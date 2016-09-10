extern crate ghoti;
use ghoti::correction;

#[test]
fn empty_correction() {
    assert_eq!(correction(""), "");
}

#[test]
fn correct_word() {
    assert_eq!(correction("the"), "the");
}

#[test]
fn single_insertion() {
    assert_eq!(correction("speling"), "spelling");
}

#[test]
fn double_replacement() {
    assert_eq!(correction("korrectud"), "corrected");
}

#[test]
fn single_replacement() {
    assert_eq!(correction("bycycle"), "bicycle");
}

#[test]
fn double_insertion() {
    assert_eq!(correction("inconvient"), "inconvenient");
}

#[test]
fn single_deletion() {
    assert_eq!(correction("arrainged"), "arranged");
}

#[test]
fn single_transposition() {
    assert_eq!(correction("peotry"), "poetry");
}

#[test]
fn transposition_and_deletion() {
    assert_eq!(correction("poetryy"), "poetry");
}

#[test]
fn no_plausible_correction() {
    assert_eq!(correction("ghotighoti"), "ghotighoti");
}
