//! Max Sum of a Pair With Equal Sum of Digits
//!
//! Solved on 2025-02-12
//! <https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits>

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
