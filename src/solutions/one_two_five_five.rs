use std::collections::HashMap;

/// Given a list of words, list of  single letters (might be repeating) and score of every character.
///
/// Return the maximum score of any valid set of words formed by using the given letters (words[i] cannot be used two or more times).
///
/// It is not necessary to use all characters in letters and each letter can only be used once. Score of letters 'a', 'b', 'c', ... ,'z' is given by score[0], score[1], ... , score[25] respectively.

struct Solution {}

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        // out of the words, check if they are possible to spell with the set of the letters to narrow the set
        let possible_words: Vec<String> = words
            .into_iter()
            .filter(|x| Self::is_possible_to_spell(&x, &letters))
            .collect();

        if possible_words.len() == 0 {
            // no possible combinations, we can exit early
            0
        } else {
            // build hashmap of scores to letters
            let alphabet = vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ];
            let mut score_map: HashMap<char, i32> = HashMap::new();
            for (i, char) in alphabet.into_iter().enumerate() {
                score_map.insert(char, *score.get(i).unwrap());
            }
            let mut res = 0;
            let mut index = 0;
            let mut current_score = 0;
            // then we can use a dfs to get the best score for each combination of words and return the score
            Self::dfs_best_score(
                &mut res,
                &letters,
                &score_map,
                &mut index,
                &mut current_score,
            )
        }
    }

    pub fn dfs_best_score(
        res: &mut i32,
        letters: &Vec<char>,
        score_map: &HashMap<char, i32>,
        index: &mut i32,
        current_score: &mut i32,
    ) -> i32 {
        // end condition
        // inside if, check if score is greater, if so replace it. We don't care about the exact sets
    }

    fn is_possible_to_spell(word: &String, letters: &Vec<char>) -> bool {
        let mut letters_copy = letters.clone();
        for char in word.chars() {
            if letters_copy.contains(&char) {
                letters_copy.remove(letters_copy.iter().position(|x| *x == char).unwrap());
            } else {
                return false;
            }
        }
        true
    }

    fn calc_score(word: &String, score_map: &HashMap<char, i32>) -> i32 {
        let mut score = 0;
        for char in word.chars() {
            score += score_map.get(&char).unwrap();
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec![
            "dog".to_owned(),
            "cat".to_owned(),
            "dad".to_owned(),
            "good".to_owned(),
        ];
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let expected = 21;
        assert_eq!(Solution::max_score_words(words, letters, score), expected);
    }

    #[test]
    fn test_2() {
        let words = vec![
            "xxxz".to_owned(),
            "ax".to_owned(),
            "bx".to_owned(),
            "cx".to_owned(),
        ];
        let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ];
        let expected = 27;
        assert_eq!(Solution::max_score_words(words, letters, score), expected);
    }

    #[test]
    fn test_3() {
        let words = vec!["leetcode".to_owned()];
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ];
        let expected = 0;
        assert_eq!(Solution::max_score_words(words, letters, score), expected);
    }
}