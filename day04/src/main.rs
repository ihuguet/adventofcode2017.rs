use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, BTreeMap};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in reader.lines() {
        let (part1_ok, part2_ok) = validate_passphrase(&line.unwrap());
        if part1_ok { part1_count += 1; }
        if part2_ok { part2_count += 1; }
    }

    println!("Part1: {} valid passphrases", part1_count);
    println!("Part2: {} valid passphrases", part2_count);
}

fn validate_passphrase(passphrase: &str) -> (bool, bool) {
    let mut seen_words = HashSet::new();
    let mut seen_anagrams = HashSet::new();
    let mut repeated_words = false;
    let mut repeated_anagrams = false;

    for word in passphrase.split_ascii_whitespace() {
        if !seen_words.insert(word) {
            repeated_words = true;
            repeated_anagrams = true; // implied if word is repeated
            break;
        }

        let anagram = anagram_from_str(word);
        if !seen_anagrams.insert(anagram) {
            repeated_anagrams = true;
        }
    }

    (!repeated_words, !repeated_anagrams)
}

fn anagram_from_str(word: &str) -> BTreeMap<char, i32> {
    let mut anagram = BTreeMap::new();

    for ch in word.chars() {
        anagram.entry(ch)
            .and_modify(|v| *v += 1)
            .or_insert(0);
    }

    anagram
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passphrase_word_validation() {
        passphrase_word_test("aa bb cc dd ee", true);
        passphrase_word_test("aa bb cc dd aa", false);
        passphrase_word_test("aa bb cc dd aaa", true);
    }

    fn passphrase_word_test(passphrase: &str, expects: bool) {
        let (result, _) = validate_passphrase(passphrase);
        assert_eq!(result, expects, "Passphrase \"{}\" word validation: expected {}, got {}",
            passphrase, expects, result);
    }

    #[test]
    fn passphrase_anagram_validation() {
        passphrase_anagram_test("abcde fghij", true);
        passphrase_anagram_test("abcde xyz ecdab", false);
        passphrase_anagram_test("a ab abc abd abf abj", true);
        passphrase_anagram_test("iiii oiii ooii oooi oooo", true);
        passphrase_anagram_test("oiii ioii iioi iiio", false);
    }

    fn passphrase_anagram_test(passphrase: &str, expects: bool) {
        let (_, result) = validate_passphrase(passphrase);
        assert_eq!(result, expects, "Passphrase \"{}\" anagram validation: expected {}, got {}",
            passphrase, expects, result);
    }
}
