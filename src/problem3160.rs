//! 3160. Find the Number of Distinct Colors Among the Balls
//!
//! Solved on 2025-02-06
//! https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/

pub struct Solution;
impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;
        let mut balls_colors = HashMap::new();
        let mut colors_counts = HashMap::<_, u32>::new();

        queries
            .into_iter()
            .map(|query| {
                let (ball, color) = (query[0], query[1]);

                if let Some(old_color) = balls_colors.insert(ball, color) {
                    if old_color != color {
                        match colors_counts.entry(old_color) {
                            Entry::Occupied(mut ent) => {
                                *ent.get_mut() -= 1;
                                if *ent.get() == 0 {
                                    ent.remove();
                                }
                            }
                            _ => unreachable!(),
                        }
                        *colors_counts.entry(color).or_default() += 1;
                    }
                } else {
                    *colors_counts.entry(color).or_default() += 1;
                }

                colors_counts.len() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    param_test!(test<Solution::query_results>(
        limit: i32,
        queries: &[[i32; 2]] {
            queries.iter().copied().map(Vec::from).collect::<Vec<_>>()
        },
    ) -> &[i32] {
        case1(4, &[[1,4],[2,5],[1,3],[3,4]]) => &[1,2,2,3],
        case2(4, &[[0,1],[1,2],[2,2],[3,4],[4,5]]) => &[1,2,2,3,4],
    });
}
