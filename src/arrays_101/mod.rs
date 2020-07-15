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

    // 1295. Find Numbers with Even Number of Digits.
    // https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for mut n in nums {
            n = n.abs();
            let mut len = 1;
            while n > 9 {
                len += 1;
                n /= 10;
            }
            if len > 0 && len % 2 == 0 {
                res += 1;
            }
        }
        res
    }

    // 977. Squares of a Sorted Array.
    // https://leetcode.com/problems/squares-of-a-sorted-array/
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        for i in 0..nums.len() {
            res[i] = nums[i] * nums[i];
        }
        res.sort_unstable();
        res
    }

    pub fn sorted_squares_v2(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.iter().map(|x| x * x).collect::<Vec::<i32>>();
        res.sort_unstable();
        res
    }
}