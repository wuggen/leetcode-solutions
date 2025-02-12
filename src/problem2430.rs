//! Maximum Deletions on a String
//!
//! Solved 2025-02-11
//! <https://leetcode.com/problems/maximum-deletions-on-a-string>

pub struct Solution;
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let mut dp = Vec::<Option<i32>>::with_capacity(s.len());
        dp.push(Some(0));
        for i in 1..s.len() {
            let tail = &s[i..];
            let next_n = dp
                .iter()
                .enumerate()
                .rev()
                .take(tail.len())
                .filter_map(|(prev_i, n)| {
                    n.and_then(|n| {
                        let prefix = &s[prev_i..i];
                        if tail.is_empty() || tail.starts_with(prefix) {
                            Some(n + 1)
                        } else {
                            None
                        }
                    })
                })
                .max();
            dp.push(next_n);
        }

        dp.into_iter().max().flatten().unwrap_or(0) + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test! {
        test<Solution::delete_string>(s: &str) -> i32 {
            case1("abcabcdabc") => 2,
            case2("aaabaab") => 4,
            case3("aaaaa") => 5,
            case4("abaaa") => 1,
        }
    }
}
