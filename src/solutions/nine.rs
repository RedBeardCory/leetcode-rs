/// Given an integer x, return true if x is a
/// palindrome , and false otherwise.

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let mut x_clone = x.clone();
            let mut reverse_digits = Vec::<i32>::new();
            while x_clone != 0 {
                let digit = x_clone % 10;
                reverse_digits.push(digit.clone());
                x_clone = x_clone / 10;
            }
            let right_way_digits: Vec<i32> = reverse_digits.clone().into_iter().rev().collect();
            right_way_digits == reverse_digits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 121;
        assert!(Solution::is_palindrome(input));
    }

    #[test]
    fn test_2() {
        let input = -121;
        assert!(!Solution::is_palindrome(input));
    }

    #[test]
    fn test_3() {
        let input = 10;
        assert!(!Solution::is_palindrome(input));
    }
}
