use std::borrow::Borrow;
use std::env;
use std::fs;
use std::io::{self, stdin, Write};

fn main() {
    let path = "index.txt";
    let contents = fs::read_to_string(path).expect("something wrong");

    let words = getwords(contents);

    let shortwords = fivelettersonly(words);

    let letters = letters_by_usage(&shortwords);

    let s = get_word_scores(shortwords, letters);
    println!("{:?}", s[0]);

    let mut justletters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut blockedletters = vec![];

    iterate(
        justletters,
        blockedletters,
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct LetterPos {
    letter: char,
    position: u8,
}

fn iterate(
    letters: Vec<char>,
    blocked_letters: Vec<char>,
    UnPlacedLetters: Vec<LetterPos>,
    PlacedLetters: Vec<LetterPos>,
    GoodLetters: Vec<LetterPos>,
) {
    let blocked_letters = getChars(get_input("input blocked letters: ".to_string()));

    let mut placeholder = Vec::<LetterPos>::new();
    let UnPlacedLetters: Vec<LetterPos> = get_placed_letters(placeholder);

    let mut placeholder = Vec::<LetterPos>::new();
    let mut PlacedLetters: Vec<LetterPos> = get_placed_letters(placeholder);
}

fn get_input(message: String) -> String {
    let mut first_line = String::new();
    println!("{}", message);
    std::io::stdin()
        .read_line(&mut first_line)
        .expect("Did not enter a valid String");

    return first_line;
}

fn block_letters(letters: Vec<char>, blocked_letters: Vec<char>) -> Vec<char> {
    let mut good_letters = Vec::new();

    for letter in letters {
        if blocked_letters.contains(&letter) {
            good_letters.push(letter);
        }
    }

    return good_letters;
}

fn get_unplaced_letters(mut UnPlacedLetters: Vec<LetterPos>) -> Vec<LetterPos> {
    let mut input;
    let mut letter;
    let mut position_char;

    input = get_input("Enter unplaced letter as: A3".to_string());

    if input.contains('/') {
        println!("{}", input);
        return UnPlacedLetters;
    } else {
        letter = input.chars().nth(0).unwrap();
        position_char = input.chars().nth(1).unwrap();

        match position_char.to_string().parse::<u8>() {
            Ok(_s) => UnPlacedLetters.push(LetterPos {
                letter: letter,
                position: _s,
            }),
            Err(_err) => println!("nan"),
        }
    }

    return get_unplaced_letters(UnPlacedLetters);
}

fn get_placed_letters(mut PlacedLetters: Vec<LetterPos>) -> Vec<LetterPos> {
    let mut input;
    let mut letter;
    let mut position_char;

    input = get_input("Enter placed letter as: A3".to_string());

    if input.contains('/') {
        println!("{}", input);
        return PlacedLetters;
    } else {
        letter = input.chars().nth(0).unwrap();
        position_char = input.chars().nth(1).unwrap();

        match position_char.to_string().parse::<u8>() {
            Ok(_s) => PlacedLetters.push(LetterPos {
                letter: letter,
                position: _s,
            }),
            Err(_err) => println!("nan"),
        }
    }

    return get_placed_letters(PlacedLetters);
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
