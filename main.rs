use std::borrow::Borrow;
use std::env;
use std::fs;

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    let letters = letters_by_usage(&shortwords);

    let s = get_word_scores(shortwords, letters);
    println!("{:?}", s);
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

fn contains_all(word: String, letters: String) -> bool {
    for i in letters.chars() {
        if !word.contains(i) {
            return false;
        }
    }

    return true;
}

fn letter_counter(words: Vec<String>, letter: char) -> u32 {
    let mut count: u32 = 0;
    for word in words {
        if word.contains(letter) {
            count += 1;
        }
    }

    return count;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Letter {
    letter: char,
    uses: u32,
}

fn letters_by_usage(words: &Vec<String>) -> Vec<Letter> {
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
    let mut letters2 = letters.clone();
    let mut scored_words = Vec::new();

    let mut word_score: u32 = 0;

    for word in words {
        let mut word2 = word.clone();
        scored_words.push(Word {
            word: word,
            score: get_word_score(word2, &letters2),
        })
    }

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
