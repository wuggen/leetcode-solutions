//! Count Number of Bad Pairs
//!
//! <https://leetcode.com/problems/count-number-of-bad-pairs/>

pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;

        let mut vals = HashMap::<i32, i64>::new();
        let mut good_pairs = 0;
        let total_pairs = (nums.len() * (nums.len() - 1) / 2) as i64;

        for (j, x) in nums.into_iter().enumerate() {
            let val = x - j as i32;
            let prev_occurrences = vals.entry(val).or_default();
            good_pairs += *prev_occurrences;
            *prev_occurrences += 1;
        }

        total_pairs - good_pairs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test! {
        test<Solution::count_bad_pairs>(nums: &[i32]) -> i64 {
            case1(&[4,1,3,3]) => 5,
            case2(&[1,2,3,4,5]) => 0,
        }
    }
}
