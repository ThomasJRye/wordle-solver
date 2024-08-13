#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct LetterPos {
    letter: char,
    position: u8,
}
pub mod input {
    use super::LetterPos;
    use crate::letter::Letter;
    use crate::{get_word_scores, letter};

    pub fn iterate(
        letters: Vec<Letter>,
        mut blocked_letters: Vec<char>,
        mut un_placed_letters: Vec<LetterPos>,
        mut placed_letters: Vec<LetterPos>,
        good_letters: Vec<LetterPos>,
        words: Vec<String>,
    ) {
        blocked_letters.append(&mut get_chars(get_input("input blocked letters: ")));
    
        println!("");
        println!("If there is a good letter(green) A in the first spot write A0");
        println!("Write / when all placed letters are written");
        println!("Letters must be written one at a time. With letter first followed by index.");

        placed_letters.append(&mut get_letters(
            Vec::<LetterPos>::new(),
            "Enter placed letters:",
        ));
    
        println!("");
        println!("");
        println!("Now repeat for unplaced letters(yellow)");
        un_placed_letters.append(&mut get_letters(
            Vec::<LetterPos>::new(),
            "Enter unplaced letters one at a time: ",
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
    
        // Base case to exit recursion
        if remaining_words.is_empty() {
            println!("No more words remaining. Ending iteration.");
            return;
        }
    
        // Continue recursion if there are still words left
        iterate(
            letters.clone(),
            blocked_letters,
            un_placed_letters,
            placed_letters,
            good_letters,
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
        word_list: Vec<String>,
        blocked_letters: Vec<char>,
        un_placed_letters: Vec<LetterPos>,
        placed_letters: Vec<LetterPos>,
    ) -> Vec<String> {
        let mut good_words: Vec<String> = vec![];

        for word in word_list {
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
    fn check_placed_letter(word: String, letter_struct: LetterPos) -> bool {
        let letter = letter_struct.letter;

        match word.matches(letter).count() {
            1 => return check_one_placed_letter(word, letter_struct),
            2 => return check_two_placed_letters(word, letter_struct),
            _ => return false,
        }
    }

    fn check_one_placed_letter(word: String, letter_struct: LetterPos) -> bool {
        let letter_position = word.chars().position(|c| c == letter_struct.letter);

        let pos: u8;

        match letter_position {
            Option::Some(val) => pos = val as u8,
            Option::None => return false,
        }

        if pos == letter_struct.position {
            return true;
        } else {
            return false;
        }
    }

    fn check_two_placed_letters(mut word: String, letter_struct: LetterPos) -> bool {
        let letter_position1_option = word.chars().position(|c| c == letter_struct.letter);

        let letter_position1;

        match letter_position1_option {
            Option::Some(val) => letter_position1 = val as u8,
            Option::None => return false,
        }

        let mut new_word = String::from(word);
        //let range: Range<Idx> = std::ops::Range {
        //    start: letter_position1.into(),
        //    end: letter_position1.into() + 1,
        //};

        let lower: usize = letter_position1.into();
        let higher: usize = letter_position1 as usize + 1;
        new_word.replace_range(lower..higher, "?");

        return check_one_placed_letter(new_word, letter_struct);
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
            println!("How many s in asset?={:?}", "aet".matches('s').count());
        }
    }
}
