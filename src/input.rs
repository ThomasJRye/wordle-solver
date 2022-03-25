mod input {

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
}
