use std::borrow::BorrowMut;

pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sets = Vec::<Vec<i32>>::new();
        // add the starting set
        sets.push(vec![]);

        for num in nums.into_iter() {
            let mut new_sets = Vec::<Vec<i32>>::new();
            for set in sets.clone().into_iter() {
                let mut new_set = set.clone();
                new_set.append(vec![num].borrow_mut());
                new_sets.push(new_set);
            }
            sets.append(new_sets.borrow_mut());
        }
        sets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(Solution::subsets(nums), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        let expected = vec![vec![], vec![0]];
        assert_eq!(Solution::subsets(nums), expected);
    }
}
