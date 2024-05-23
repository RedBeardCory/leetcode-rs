use std::borrow::BorrowMut;

/// Problem 2597: The Number of Beautiful Subsets
///
/// You are given an array nums of positive integers and a positive integer k.
///
/// A subset of nums is beautiful if it does not contain two integers with an absolute difference equal to k.
///
/// Return the number of non-empty beautiful subsets of the array nums.
///
/// A subset of nums is an array that can be obtained by deleting some (possibly none) elements from nums.
/// Two subsets are different if and only if the chosen indices to delete are different.
///
/// Constraints
/// 1 <= nums.length <= 20
/// 1 <= nums[i], k <= 1000

struct Solution {}

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        // TODO: This was a bruteforce solution. Need to practice more intelligent ways of doing things
        // generate all subsets
        let all_subsets: Vec<Vec<i32>> = Self::get_all_subsets(nums);

        all_subsets
            .into_iter()
            .filter(|x| Self::is_beautiful(&x, &k))
            .count() as i32
    }

    pub fn get_all_subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

    fn is_beautiful(nums: &Vec<i32>, k: &i32) -> bool {
        if nums.is_empty() {
            return false;
        } else {
            let mut sorted_nums = nums.clone();
            sorted_nums.sort();
            for num in sorted_nums.clone().into_iter() {
                let partner = num + *k as i32;
                match sorted_nums.binary_search(&partner) {
                    Ok(_) => return false,
                    Err(_) => continue,
                }
            }
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 4, 6];
        let k = 2;
        let expected = 4;
        assert_eq!(Solution::beautiful_subsets(nums, k), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let k = 1;
        let expected = 1;
        assert_eq!(Solution::beautiful_subsets(nums, k), expected);
    }
}
