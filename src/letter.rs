#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Letter {
    pub letter: char,
    pub uses: u32,
}

pub fn letters_by_usage(words: &Vec<String>) -> Vec<Letter> {
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

fn letter_counter(words: Vec<String>, letter: char) -> u32 {
    let mut count: u32 = 0;
    for word in words {
        if word.contains(letter) {
            count += 1;
        } else {
        }
    }

    return count;
}
