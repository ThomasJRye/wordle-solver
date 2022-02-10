use std::env;
use std::fs;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);
}

fn getwords(wordList: String) -> Vec<String> {
    let mut iterator = 0;
    let mut length_read = 0;

    let mut words = Vec::<String>::new();

    let length = wordList.len();
    for i in wordList.chars() {
        if i.is_alphabetic() {
            iterator += 1;
            length_read += 1;
        } else if length_read + iterator < length {
            words.push(wordList[length_read..length_read + iterator].to_string());
            iterator = 0;
            length_read += 1;
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
