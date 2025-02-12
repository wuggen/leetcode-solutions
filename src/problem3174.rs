//! Clear Digits
//!
//! Solved on 2025-02-09
//! <https://leetcode.com/problems/clear-digits>

pub struct Solution;
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                res.pop();
            } else {
                res.push(c)
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test!(test<Solution::clear_digits>(s: &str) -> &str {
        case1("abc") => "abc",
        case2("cb34") => "",
    });
}
