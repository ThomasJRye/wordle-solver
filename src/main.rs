mod input;
mod letter;
mod scoring;

use self::input::LetterPos;
use crate::input::input::iterate;
use crate::scoring::scoring::get_word_scores;

use std::fs;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    let letters = letter::letters_by_usage(&shortwords);

    let s = get_word_scores(shortwords.clone(), letters.clone());
    println!("{:?}", s[0]);

    iterate(
        letters.clone(),
        vec![],
        Vec::<input::LetterPos>::new(),
        Vec::<LetterPos>::new(),
        Vec::<LetterPos>::new(),
        shortwords,
    );

    println!("{:?}", s[0]);
}

fn getwords(word_list: String) -> Vec<String> {
    let mut iterator = 0;
    let mut length_read = 0;

    let mut words = Vec::<String>::new();

    let length = word_list.len();

    let _word: String = "".to_string();

    for i in word_list.chars() {
        if i.is_alphabetic() {
            iterator += 1;
        } else if length_read + iterator < length {
            words.push(word_list[length_read..length_read + iterator].to_string());

            length_read = length_read + iterator;
            length_read += 1;
            iterator = 0;
        }
    }

    return words;
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
