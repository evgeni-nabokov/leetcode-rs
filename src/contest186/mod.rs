#[cfg(test)]
mod tests;

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut zeros_sum: Vec<i32> = Vec::with_capacity(s.len());
        let mut ones_sum: Vec<i32> = Vec::with_capacity(s.len());
        let mut last = 0;
        for b in s.chars() {
            if b == '0' {
                last += 1;
            }
            zeros_sum.push(last);
        }
        last = 0;
        for b in s.chars().rev() {
            if b == '1' {
                last += 1;
            }
            ones_sum.push(last);
        }
        ones_sum.reverse();
        let mut max_score = 0;
        for i in 1..s.len() {
            max_score = max(max_score, zeros_sum[i - 1] + ones_sum[i]);
        }
        max_score
    }

    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        if nums.is_empty() { return Vec::new(); }
        let mut res: Vec<i32> = Vec::with_capacity(nums.len() ^ 2);
        let mut max_row_len = 0;
        for row_index in 0..nums.len() {
            for col_index in 0..=row_index {
                max_row_len = max(max_row_len, nums[row_index - col_index].len());
                if col_index < nums[row_index - col_index].len() {
                    res.push(nums[row_index - col_index][col_index]);
                }
            }
        }
        for col_index in 1..max_row_len {
            for i in 0..nums.len() {
                let row_index = nums.len() - i - 1;
                if col_index + i < nums[row_index].len() {
                    res.push(nums[row_index][col_index + i]);
                }
            }
        }
        res
    }
}