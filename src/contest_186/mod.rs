#[cfg(test)]
mod tests;

use std::cmp::max;
use std::collections::VecDeque;

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

    // 1424. Diagonal Traverse II.
    // https://leetcode.com/problems/diagonal-traverse-ii/
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        if nums.is_empty() { return vec![]; }
        let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(10000);
        let mut res: Vec<i32> = Vec::with_capacity(10000);
        queue.push_front((0, 0));
        while !queue.is_empty() {
            let (i, j) = queue.pop_back().unwrap();
            res.push(nums[i][j]);
            if j == 0 && i < nums.len() - 1 && j < nums[i + 1].len() {
                queue.push_front((i + 1, j));
            }
            if j < nums[i].len() - 1 {
                queue.push_front((i, j + 1));
            }
        }
        res
    }
}