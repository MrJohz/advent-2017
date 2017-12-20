#![feature(conservative_impl_trait)]

use std::collections::HashSet;

const PASS_PHRASES: &'static str = include_str!("inputs/day4.txt");

fn pass_phrases() -> impl Iterator<Item=Vec<String>> {
    PASS_PHRASES.lines()
        .map(|line| line.split_whitespace()
            .map(|string| string.to_owned())
            .collect())
}

fn valid(phrase: Vec<String>) -> bool {
    let mut found = HashSet::with_capacity(phrase.len());
    for word in phrase {
        let mut sorted_word = word
            .chars()
            .collect::<Vec<_>>();

        sorted_word.sort_by(|a, b| a.cmp(b));
        let word = String::from(word);

        println!("{:?}", word);

        if found.insert(sorted_word) == false {
            return false
        }
    }

    true
}

fn main() {
    let mut valid_count = 0;

    for phrase in pass_phrases() {
        if valid(phrase) {
            valid_count += 1;
        }
    }

    println!("{:?}", valid_count);
}
