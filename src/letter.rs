mod letter {
    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Letter {
        letter: char,
        uses: u32,
    }

    fn letters_by_usage(words: &Vec<String>) -> Vec<Letter> {
        static ASCII_LOWER: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
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
}
