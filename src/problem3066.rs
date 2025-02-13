//! Minimum Operations to Exceed Threshold Value II
//!
//! Solved on 2025-02-12
//!
//! <https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/>

pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let k = k as u64;
        let mut nums: BinaryHeap<Reverse<u64>> =
            nums.into_iter().map(|n| Reverse(n as u64)).collect();
        let mut n = 0;

        loop {
            if nums.peek().unwrap().0 >= k {
                break n;
            }

            let a = nums.pop().unwrap().0;
            let b = nums.pop().unwrap().0;

            nums.push(Reverse(a * 2 + b));

            n += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test! {
        test<Solution::min_operations>(nums: &[i32], k: i32) -> i32 {
            case1(&[2,11,10,1,3], 10) => 2,
            case2(&[1,1,2,4,9], 20) => 4,
            case3(&[999999999,999999999,999999999], 1000000000) => 2,
        }
    }
}
