use std::slice::Chunks;

/// Palindrome Partitioning
///
/// Given a string s, partition s such that every
/// substring of the partition is a palindrome.
/// Return all possible palindrome partitioning of s.
///
/// Constraints:
/// 1 <= s.len() <= 16
/// s contains only lowercase English letters.

struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut strings: Vec<Vec<String>> = Vec::new();
        let mut i = 1;
        // split string up in increments
        while i <= s.len() {
            let char_arr = s.chars().collect::<Vec<char>>();
            let rev_char_arr = s.chars().rev().collect::<Vec<char>>();
            let chunks = char_arr.chunks(i);
            let rev_chunks = rev_char_arr.chunks(i);
            let mut set: Vec<String> = Vec::new();
            if Self::check_all_chunks(chunks.clone()) {
                chunks.for_each(|chunk| set.push(chunk.iter().collect()))
            }

            if !set.is_empty() {
                strings.push(set.clone());
            }

            let mut rev_set: Vec<String> = Vec::new();
            if Self::check_all_chunks(rev_chunks.clone()) {
                rev_chunks
                    .rev()
                    .for_each(|chunk| rev_set.push(chunk.iter().collect()))
            }

            if !rev_set.is_empty() && &rev_set != &set {
                strings.push(rev_set);
            }

            i += 1;
        }
        return strings;
    }

    fn check_all_chunks(mut chunks: Chunks<char>) -> bool {
        chunks.all(|c| Self::is_palindrome(c.iter().collect()))
    }

    fn is_palindrome(s: String) -> bool {
        let mut s2 = s.clone();
        for c1 in s.chars() {
            let c2 = s2.pop();
            match c2 {
                Some(c) => {
                    if c1 != c {
                        return false;
                    }
                }
                None => return false,
            }

            // can exit early if we have gone past the halfway mark
            if s2.len() <= (s.len() / 2) {
                break;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("aab");
        let expected = vec![vec!["a", "a", "b"], vec!["aa", "b"]];
        assert_eq!(Solution::partition(input), expected);
    }

    #[test]
    fn test_2() {
        let input = String::from("a");
        let expected = vec![vec!["a"]];
        assert_eq!(Solution::partition(input), expected);
    }

    #[test]
    fn test_3() {
        let input = String::from("cdd");
        let expected = vec![vec!["c", "d", "d"], vec!["c", "dd"]];
        assert_eq!(Solution::partition(input), expected);
    }

    #[test]
    fn test_palindrome() {
        let input = String::from("a");
        assert!(Solution::is_palindrome(input));

        let input = String::from("abba");
        assert!(Solution::is_palindrome(input));

        // uneven amount of input
        let input = String::from("abbba");
        assert!(Solution::is_palindrome(input));
    }

    #[test]
    fn test_invalid_palindrome() {
        let input = String::from("ab");
        assert!(!Solution::is_palindrome(input));

        let input = String::from("abbar");
        assert!(!Solution::is_palindrome(input));
    }
}
