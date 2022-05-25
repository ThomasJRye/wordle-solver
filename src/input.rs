#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct LetterPos {
    letter: char,
    position: u8,
}
pub mod input {
    use super::LetterPos;
    use crate::get_word_scores;
    use crate::letter::Letter;

    pub fn iterate(
        letters: Vec<Letter>,
        mut blocked_letters: Vec<char>,
        mut un_placed_letters: Vec<LetterPos>,
        mut placed_letters: Vec<LetterPos>,
        GoodLetters: Vec<LetterPos>,
        words: Vec<String>,
    ) {
        blocked_letters.append(&mut get_chars(get_input("input blocked letters: ")));

        println!("");
        println!("Letters must be written one at a time. With letter first followed by index.");
        println!("If there is a good letter(green) A in the first spot write A0");
        println!("Write / when all placed letters are written");

        placed_letters.append(&mut get_letters(
            Vec::<LetterPos>::new(),
            "Enter placed letters:",
        ));

        println!("");
        println!("");
        println!("Now repeat for unplaced letters(yellow)");
        un_placed_letters.append(&mut get_letters(
            Vec::<LetterPos>::new(),
            "Enter unplaced letters on at a time: ",
        ));

        let remaining_words = filter_words(
            words.clone(),
            blocked_letters.clone(),
            un_placed_letters.clone(),
            placed_letters.clone(),
        );
        println!("");
        println!("");
        println!("Remaining words: {:?}", remaining_words.clone());
        println!("");
        let word_scores = get_word_scores(remaining_words.clone(), letters.clone());

        println!("Best {:?}", word_scores[0]);
        iterate(
            letters.clone(),
            blocked_letters,
            un_placed_letters,
            placed_letters,
            GoodLetters,
            remaining_words,
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

    fn get_letters(mut placed_letters: Vec<LetterPos>, message: &str) -> Vec<LetterPos> {
        let input;
        let letter;
        let position_char;

        input = get_input(message);

        if input.contains('/') {
            println!("{}", input);
            return placed_letters;
        } else {
            letter = input.chars().nth(0).unwrap();
            position_char = input.chars().nth(1).unwrap();

            match position_char.to_string().parse::<u8>() {
                Ok(_s) => placed_letters.push(LetterPos {
                    letter: letter,
                    position: _s,
                }),
                Err(_err) => println!("nan"),
            }
        }

        return get_letters(placed_letters, message.clone());
    }

    fn filter_words(
        wordList: Vec<String>,
        blocked_letters: Vec<char>,
        un_placed_letters: Vec<LetterPos>,
        placed_letters: Vec<LetterPos>,
    ) -> Vec<String> {
        let mut good_words: Vec<String> = vec![];

        for word in wordList {
            let contains_placed_letter =
                iter_function(word.clone(), placed_letters.clone(), &check_placed_letter);
            let no_blocked_letters = check_blocked_letters(word.clone(), blocked_letters.clone());
            let contains_unplaced_letter = iter_function(
                word.clone(),
                un_placed_letters.clone(),
                &check_unplaced_letter,
            );

            if contains_placed_letter && no_blocked_letters && contains_unplaced_letter {
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
        if letters.len() == 0 {
            return true;
        }
        for letter in letters {
            if !f(word.clone(), letter) {
                return false;
            }
        }

        return true;
    }
    fn check_placed_letter(word: String, letter: LetterPos) -> bool {
        let letter_position = word.chars().position(|c| c == letter.letter);

        let pos: u8;

        match letter_position {
            Option::Some(val) => pos = val as u8,
            Option::None => return false,
        }

        if pos == letter.position {
            return true;
        } else {
            return false;
        }
    }

    fn check_unplaced_letter(word: String, letter: LetterPos) -> bool {
        let letter_position = word.chars().position(|c| c == letter.letter);

        let pos: u8;

        match letter_position {
            Option::Some(val) => pos = val as u8,
            Option::None => return false,
        }

        if pos == letter.position {
            return false;
        } else {
            return true;
        }
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

    fn get_chars(line: String) -> Vec<char> {
        let mut letters = Vec::<char>::new();

        for i in line.chars() {
            if i.is_alphabetic() {
                letters.push(i);
            }
        }

        return letters;
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
            let placedLetter3 = LetterPos {
                letter: 'c',
                position: 1,
            };
            let placed_letters = vec![placedLetter1, placedLetter2];

            assert!(check_placed_letter("hello".to_string(), placedLetter1));
            assert!(check_placed_letter("hello".to_string(), placedLetter2));
            assert!(!check_placed_letter("balls".to_string(), placedLetter1));
            assert!(!check_placed_letter("balls".to_string(), placedLetter1));
            assert!(!check_unplaced_letter("balls".to_string(), placedLetter1));
            assert!(check_unplaced_letter("cadee".to_string(), placedLetter3));

            let placedLetter0 = LetterPos {
                letter: 'h',
                position: 1,
            };
            println!(
                "check_placed_letter={:?}",
                check_placed_letter("zymic".to_string(), placedLetter0)
            );
            println!(
                "check_unplaced_letter={:?}",
                check_unplaced_letter("hello".to_string(), placedLetter0)
            );
        }
    }
}
