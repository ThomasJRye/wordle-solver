pub mod scoring {
    use crate::letter::Letter;

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Word {
        word: String,
        score: u32,
    }

    pub fn get_word_score(word: String, letters: &Vec<Letter>) -> u32 {
        let mut score: u32 = 0;

        for letter in letters {
            if word.contains(letter.letter) {
                score += letter.uses;
            }
        }

        return score;
    }

    pub fn get_word_scores(words: Vec<String>, letters: Vec<Letter>) -> Vec<Word> {
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
}
