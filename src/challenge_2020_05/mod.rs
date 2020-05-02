#[cfg(test)]
mod tests;

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        if jewels.is_empty() || stones.is_empty() { return 0; }
        let mut cnt = 0;
        let mut j_set: HashSet<char> = HashSet::with_capacity(jewels.len());
        for j in jewels.chars() {
            j_set.insert(j);
        }
        for s in stones.chars() {
            if j_set.contains(&s) {
                cnt += 1;
            }
        }
        cnt
    }
}