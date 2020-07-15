#[cfg(test)]
mod tests;

use std::cmp::max;

struct Solution;

impl Solution {
    // 485. Max Consecutive Ones.
    // https://leetcode.com/problems/max-consecutive-ones/
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut one_cntr = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                one_cntr += 1;
            } else {
                max_len = max(max_len, one_cntr);
                one_cntr = 0;
            }
        }
        max(max_len, one_cntr)
    }
}