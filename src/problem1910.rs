//! 1910. Remove All Occurrences of a Substring
//!
//! Solved 2025-02-11
//! https://leetcode.com/problems/remove-all-occurrences-of-a-substring

pub struct Solution;
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut res = String::with_capacity(s.len());
        for c in s.chars() {
            res.push(c);
            if res.ends_with(&part) {
                res.truncate(res.len() - part.len());
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test! {
        test<Solution::remove_occurrences>(s: &str, part: &str) -> &str {
            case1("daabcbaabcbc", "abc") => "dab",
            case2("axxxxyyyyb", "xy") => "ab",
        }
    }
}
