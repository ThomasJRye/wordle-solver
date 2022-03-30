mod input;
mod letter;

use std::borrow::Borrow;
use std::env;
use std::fs;
use std::io::{self, stdin, Write};

use letter::Letter;

use crate::input::LetterPos;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    let letters = letter::letters_by_usage(&shortwords);

    let s = get_word_scores(shortwords, letters);
    println!("{:?}", s[0]);

    let justletters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    input::iterate(
        justletters,
        vec![],
        Vec::<LetterPos>::new(),
        Vec::<LetterPos>::new(),
        Vec::<LetterPos>::new(),
    );

    println!("{:?}", s[0]);
}

fn getwords(wordList: String) -> Vec<String> {
    let mut iterator = 0;
    let mut length_read = 0;

    let mut words = Vec::<String>::new();

    let length = wordList.len();

    let mut word: String = "".to_string();

    for i in wordList.chars() {
        if i.is_alphabetic() {
            iterator += 1;
        } else if length_read + iterator < length {
            words.push(wordList[length_read..length_read + iterator].to_string());

            length_read = length_read + iterator;
            length_read += 1;
            iterator = 0;
        }
    }

    return words;
}

fn getChars(line: String) -> Vec<char> {
    let mut letters = Vec::<char>::new();

    for i in line.chars() {
        if i.is_alphabetic() {
            letters.push(i);
        }
    }

    return letters;
}

fn fivelettersonly(words: Vec<String>) -> Vec<String> {
    let mut shortwords = Vec::<String>::new();

    for word in words {
        if word.len() == 5 {
            shortwords.push(word.to_string());
        }
    }

    return shortwords;
}

fn charfilter(words: Vec<String>, letter: char) -> Vec<String> {
    let mut filteredwords = Vec::<String>::new();

    for word in words {
        if word.contains(letter) {
            filteredwords.push(word.to_string());
        }
    }

    return filteredwords;
}

fn contains(word: String, letter: char) -> bool {
    for i in word.chars() {
        if i == letter {
            return true;
        }
    }

    return false;
}

fn contains_all(word: String, letters: String) -> bool {
    for i in letters.chars() {
        if !word.contains(i) {
            return false;
        }
    }

    return true;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Word {
    word: String,
    score: u32,
}

fn get_word_score(word: String, letters: &Vec<Letter>) -> u32 {
    let mut score: u32 = 0;

    for letter in letters {
        if word.contains(letter.letter) {
            score += letter.uses;
        }
    }

    return score;
}

fn get_word_scores(words: Vec<String>, letters: Vec<Letter>) -> Vec<Word> {
    let letters2 = letters.clone();
    let mut scored_words = Vec::new();

    for word in words {
        let word2 = word.clone();
        scored_words.push(Word {
            word: word,
            score: get_word_score(word2, &letters2),
        })
    }

    scored_words.sort_by(|a, b| b.score.cmp(&a.score));
    return scored_words;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_test() {
        assert!(contains("fear".to_string(), 'a'));
        assert!(!contains("fear".to_string(), 'b'));
    }

    #[test]
    fn contains_all_test() {
        assert!(contains_all("fear".to_string(), "eafr".to_string()));
        assert!(!contains_all("fear".to_string(), "b".to_string()));
    }
}
