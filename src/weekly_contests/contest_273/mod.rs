#[cfg(test)]
mod tests;

use std::collections::HashMap;

struct Solution;

impl Solution {
    // 2119. A Number After a Double Reversal.
    // https://leetcode.com/problems/a-number-after-a-double-reversal/
    // Time complexity: O(1).
    // Space complexity: O(1).
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }

    // 2121. Intervals Between Identical Elements.
    // https://leetcode.com/problems/intervals-between-identical-elements/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();
        for i in 0..arr.len() {
            map.entry(arr[i]).or_insert(vec![]).push(i);
        }

        let mut res = vec![0; arr.len()];

        for (_, indices) in map {
            let mut curr_sum = 0;

            for i in 1..indices.len() {
                curr_sum += (indices[0] as i64 - indices[i] as i64).abs();
            }

            res[indices[0]] = curr_sum;

            for i in 1..indices.len() {
                let diff = (indices[i - 1] as i64 - indices[i] as i64).abs();
                curr_sum += diff * (2 * i as i64 - indices.len() as i64);
                res[indices[i]] = curr_sum;
            }
        }

        res
    }
}
