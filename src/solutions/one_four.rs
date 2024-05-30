struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut letter_count = 1;
        let mut prefix = "";
        'main: loop {
            if &letter_count > &strs.get(0).unwrap().as_str().len() {
                break;
            }
            let current_prefix = &strs.get(0).unwrap().as_str()[0..letter_count];
            for i in 1..strs.len() {
                if &letter_count > &strs.get(i).unwrap().as_str().len()
                    || current_prefix != &strs.get(i).unwrap().as_str()[0..letter_count]
                {
                    break 'main;
                }
            }
            prefix = current_prefix;
            letter_count += 1;
        }
        prefix.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
        let expected = "fl";
        assert_eq!(Solution::longest_common_prefix(input), expected);
    }

    #[test]
    fn test_2() {
        let input = vec!["dog".to_owned(), "racecar".to_owned(), "car".to_owned()];
        let expected = "";
        assert_eq!(Solution::longest_common_prefix(input), expected);
    }

    #[test]
    fn test_3() {
        let input = vec!["ab".to_owned(), "a".to_owned()];
        let expected = "a";
        assert_eq!(Solution::longest_common_prefix(input), expected);
    }
}
