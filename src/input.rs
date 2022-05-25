use crate::{getChars, getwords};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct LetterPos {
    letter: char,
    position: u8,
}

pub fn iterate(
    letters: Vec<char>,
    blocked_letters: Vec<char>,
    UnPlacedLetters: Vec<LetterPos>,
    PlacedLetters: Vec<LetterPos>,
    GoodLetters: Vec<LetterPos>,
    words: Vec<String>,
) {
    let blocked_letters = getChars(get_input("input blocked letters: "));

    let UnPlacedLetters: Vec<LetterPos> =
        get_letters(Vec::<LetterPos>::new(), "Enter placed letters as: A3");

    let PlacedLetters: Vec<LetterPos> =
        get_letters(Vec::<LetterPos>::new(), "Enter unplaced letters as: A3");

    let remaining_words = filter_words(
        words.clone(),
        blocked_letters.clone(),
        UnPlacedLetters.clone(),
        PlacedLetters.clone(),
    );

    println!("{}", remaining_words[0]);

    iterate(
        letters,
        blocked_letters,
        UnPlacedLetters,
        PlacedLetters,
        GoodLetters,
        words,
    );
}

fn get_input(message: &str) -> String {
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

    input = get_input("Enter unplaced letter as: A3");

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

fn get_letters(mut PlacedLetters: Vec<LetterPos>, message: &str) -> Vec<LetterPos> {
    let input;
    let mut letter;
    let mut position_char;

    input = get_input(message);

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

    return get_letters(PlacedLetters, message.clone());
}

fn filter_words(
    wordList: Vec<String>,
    blocked_letters: Vec<char>,
    UnPlacedLetters: Vec<LetterPos>,
    PlacedLetters: Vec<LetterPos>,
) -> Vec<String> {
    let mut good_words: Vec<String> = vec![];

    for word in wordList {
        if iter_function(word.clone(), PlacedLetters.clone(), &check_placed_letter)
            && check_blocked_letters(word.clone(), blocked_letters.clone())
            && iter_function(
                word.clone(),
                UnPlacedLetters.clone(),
                &check_unplaced_letter,
            )
        {
            good_words.push(word.clone());
        }
    }

    return good_words;
}

fn iter_function(
    word: String,
    letters: Vec<LetterPos>,
    f: &dyn Fn(String, LetterPos) -> bool,
) -> bool {
    for letter in letters {
        if !f(word.clone(), letter) {
            return false;
        }
    }

    return true;
}
fn check_placed_letter(word: String, letter: LetterPos) -> bool {
    for i in word.chars() {
        if word.find(letter.letter) == word.find(i) {
            return true;
        }
    }

    return false;
}

fn check_unplaced_letter(word: String, letter: LetterPos) -> bool {
    for i in word.chars() {
        if word.find(letter.letter) == word.find(i) {
            return false;
        } else {
            for i in word.chars() {
                if i == letter.letter {
                    return true;
                }
            }
        }
    }

    return false;
}

fn check_blocked_letters(word: String, blocked_letters: Vec<char>) -> bool {
    for i in word.chars() {
        for letter in blocked_letters.clone() {
            if i == letter {
                return false;
            }
        }
    }

    return true;
}

mod tests {
    use super::*;

    #[test]
    fn contains_test() {
        let placedLetter1 = LetterPos {
            letter: 'e',
            position: 1,
        };

        let placedLetter2 = LetterPos {
            letter: 'h',
            position: 0,
        };
        let placedLetters = vec![placedLetter1, placedLetter2];

        assert!(check_placed_letter("hello".to_string(), placedLetter1));
        assert!(check_placed_letter("hello".to_string(), placedLetter2));
        assert!(!check_placed_letter("balls".to_string(), placedLetter1));
        //assert!(!contains("fear".to_string(), 'b'));
    }

    #[test]
    fn contains_all_test() {}
}
