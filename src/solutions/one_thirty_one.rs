use std::{borrow::BorrowMut, slice::Chunks};

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
        let mut res: Vec<Vec<String>> = Vec::new();
        Self::dfs(&mut res, &s, 0, &mut Vec::<String>::new());
        return res;
    }

    /// depth first search
    pub fn dfs(res: &mut Vec<Vec<String>>, s: &String, ind: usize, cur: &mut Vec<String>) {
        if ind >= s.len() {
            res.push(cur.to_vec());
            return;
        }

        for i in ind..s.len() {
            // TIL: Slices in rust are [starting_index..length]
            //   so "abc"[0..1] == "a" and "abc"[0..2] == "ab"
            let sub_string = s.as_str()[ind..(i + 1)].to_owned();

            if Self::is_palindrome(&sub_string) {
                cur.push(sub_string);
                Self::dfs(res, s, i + 1, cur);
                cur.pop();
            }
        }
    }

    fn is_palindrome(s: &String) -> bool {
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
        assert!(Solution::is_palindrome(&input));

        let input = String::from("abba");
        assert!(Solution::is_palindrome(&input));

        // uneven amount of input
        let input = String::from("abbba");
        assert!(Solution::is_palindrome(&input));
    }

    #[test]
    fn test_invalid_palindrome() {
        let input = String::from("ab");
        assert!(!Solution::is_palindrome(&input));

        let input = String::from("abbar");
        assert!(!Solution::is_palindrome(&input));
    }
}
