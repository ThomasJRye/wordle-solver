use std::env;
use std::fs;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");
    println!("{}", ','.is_alphabetic());

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    let filtered = charfilter(shortwords, 'd');
    println!("{:?}", filtered.len());
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
