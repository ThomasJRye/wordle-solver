use std::env;
use std::fs;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");
    println!("{}", ','.is_alphabetic());

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    println!("{:?}", words_by_usage(&shortwords));
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

fn letter_counter(words: Vec<String>, letter: char) -> usize {
    let mut count: usize = 0;
    for word in words {
        if word.contains(letter) {
            count += 1;
        }
    }

    return count;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Letter {
    letter: char,
    uses: usize,
}

fn words_by_usage(words: &Vec<String>) -> Vec<Letter> {
    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut letters = Vec::<Letter>::new();

    for letter in ASCII_LOWER {
        letters.push(Letter {
            letter: letter,
            uses: letter_counter(words.to_vec(), letter),
        })
    }

    letters.sort_by(|a, b| b.uses.cmp(&a.uses));
    return letters;
}

fn get_words_with_most_used_letters(words: &Vec<String>) -> Vec<String> {
    let mut all_words = words_by_usage(words);
    let mut letters = Vec::<char>::new();
    let mut words: Vec<String> = Vec::<String>::new();

    let best_letter_structs = &all_words[0..4];

    for letter in best_letter_structs {
        letters.push(letter.letter);
    }

    for word in words {
        if contains(word, letters) {
            words.push(word)
        }
    }
    return words;
}
