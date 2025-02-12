//! 2342. Max Sum of a Pair With Equal Sum of Digits
//!
//! Solved on 2025-02-12
//! https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits
//!
//! You are given a 0-indexed array nums consisting of positive integers. You
//! can choose two indices i and j, such that i != j, and the sum of digits of the
//! number nums[i] is equal to that of nums[j].
//!
//! Return the maximum value of nums[i] + nums[j] that you can obtain over all
//! possible indices i and j that satisfy the conditions.
//!
//! Example 1:
//!
//! Input: nums = [18,43,36,13,7]
//! Output: 54
//! Explanation: The pairs (i, j) that satisfy the conditions are:
//! - (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
//! - (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
//!
//! So the maximum sum that we can obtain is 54.
//!
//! Example 2:
//!
//! Input: nums = [10,12,19,14]
//! Output: -1
//! Explanation: There are no two numbers that satisfy the conditions, so we return -1.
//!
//! Constraints:
//! - 1 <= nums.length <= 10^5
//! - 1 <= nums[i] <= 10^9

pub struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        // let mut sums: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        // for n in nums {
        //     let sum = sum_of_digits(n);
        //     sums.entry(sum).or_default().push(n);
        // }

        // sums.into_values()
        //     .filter_map(|mut ns| {
        //         let a = ns.pop()?;
        //         let b = ns.pop()?;
        //         Some(a + b)
        //     })
        //     .max()
        //     .unwrap_or(-1)

        let mut res = -1;
        let mut largest_digit_sums = [0; 81];
        for n in nums {
            let digit_sum = (sum_of_digits(n) as usize) - 1;
            let largest = &mut largest_digit_sums[digit_sum];
            if *largest > 0 {
                res = i32::max(res, n + *largest);
            }

            *largest = i32::max(*largest, n);
        }

        res
    }
}

#[inline]
fn sum_of_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    param_test!(test<Solution::maximum_sum>(nums: &[i32]) -> i32 {
        case1(&[18,43,36,13,7]) => 54,
        case2(&[10,12,19,14]) => -1,
        case3(&[229,398,269,317,420,464,491,218,439,153,482,169,411,93,147,50,347,210,251,366,401]) => 973,
    });

    param_test!(test_sum_of_digits<sum_of_digits>(n: i32) -> i32 {
        sod1(0) => 0,
        sod2(7) => 7,
        sod3(12) => 3,
        sod4(55) => 10,
        sod5(145) => 10,
    });
}
