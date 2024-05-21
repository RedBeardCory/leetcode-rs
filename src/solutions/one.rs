use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                if nums.get(i).unwrap() + nums.get(j).unwrap() == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("No solution possible");
    }

    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        for (i, x) in nums.into_iter().enumerate() {
            let compliment = target - x;
            if hash_map.contains_key(&compliment) {
                return vec![i as i32, *hash_map.get(&compliment).unwrap()];
            }
            hash_map.insert(x, i as i32);
        }
        panic!("No solution possible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
