//! Special Array I
//!
//! Solved on 2025-01-31
//! <https://leetcode.com/problems/special-array-i>

pub struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|ns| ns[0] % 2 != ns[1] % 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test! {
        test<Solution::is_array_special>(nums: &[i32]) -> bool {
            case1(&[1]) => true,
            case2(&[2, 1, 4]) => true,
            case3(&[4, 3, 1, 6]) => false,
        }
    }
}
